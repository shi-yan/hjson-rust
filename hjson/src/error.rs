//! JSON Errors
//!
//! This module is centered around the `Error` and `ErrorCode` types, which represents all possible
//! `serde_hjson` errors.

use std::error;
use std::fmt;
use std::io;
use std::result;
use std::string::FromUtf8Error;
use std::num::ParseIntError;

use serde::de;
use serde::ser;

/// The errors that can arise while parsing a JSON stream.
#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    /// Catchall for syntax error messages
    Custom(String),

    /// EOF while parsing a list.
    EOFWhileParsingList,

    /// EOF while parsing an object.
    EOFWhileParsingObject,

    /// EOF while parsing a string.
    EOFWhileParsingString,

    /// EOF while parsing a JSON value.
    EOFWhileParsingValue,

    /// Expected this character to be a `':'`.
    ExpectedColon,

    /// Expected this character to be either a `','` or a `]`.
    ExpectedListCommaOrEnd,

    /// Expected this character to be either a `','` or a `}`.
    ExpectedObjectCommaOrEnd,

    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,

    /// Expected this character to start a JSON value.
    ExpectedSomeValue,

    /// Invalid hex escape code.
    InvalidEscape,

    /// Invalid number.
    InvalidNumber,

    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,

    /// Object key is not a string.
    KeyMustBeAString,

    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,

    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,

    /// Unexpected end of hex excape.
    UnexpectedEndOfHexEscape,

    /// Found a punctuator character when expecting a quoteless string.
    PunctuatorInQlString,
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //use std::fmt::Debug;

        match *self {
            ErrorCode::Custom(ref msg) => write!(f, "{}", msg),
            ErrorCode::EOFWhileParsingList => "EOF while parsing a list".fmt(f),
            ErrorCode::EOFWhileParsingObject => "EOF while parsing an object".fmt(f),
            ErrorCode::EOFWhileParsingString => "EOF while parsing a string".fmt(f),
            ErrorCode::EOFWhileParsingValue => "EOF while parsing a value".fmt(f),
            ErrorCode::ExpectedColon => "expected `:`".fmt(f),
            ErrorCode::ExpectedListCommaOrEnd => "expected `,` or `]`".fmt(f),
            ErrorCode::ExpectedObjectCommaOrEnd => "expected `,` or `}`".fmt(f),
            ErrorCode::ExpectedSomeIdent => "expected ident".fmt(f),
            ErrorCode::ExpectedSomeValue => "expected value".fmt(f),
            ErrorCode::InvalidEscape => "invalid escape".fmt(f),
            ErrorCode::InvalidNumber => "invalid number".fmt(f),
            ErrorCode::InvalidUnicodeCodePoint => "invalid unicode code point".fmt(f),
            ErrorCode::KeyMustBeAString => "key must be a string".fmt(f),
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                "lone leading surrogate in hex escape".fmt(f)
            }
            ErrorCode::TrailingCharacters => "trailing characters".fmt(f),
            ErrorCode::UnexpectedEndOfHexEscape => "unexpected end of hex escape".fmt(f),
            ErrorCode::PunctuatorInQlString => {
                "found a punctuator character when expecting a quoteless string".fmt(f)
            }
        }
    }
}

/// This type represents all possible errors that can occur when serializing or deserializing a
/// value into JSON.
#[derive(Debug)]
pub enum Error {
    /// The JSON value had some syntatic error.
    Syntax(ErrorCode, usize, usize),

    /// Some IO error occurred when serializing or deserializing a value.
    Io(io::Error),

    /// Some UTF8 error occurred while serializing or deserializing a value.
    FromUtf8(FromUtf8Error),

    /// Some error occurred while deserializing a number.
    ParseIntError(ParseIntError),
}

impl error::Error for Error {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(..) => "syntax error",
            Error::Io(ref error) => error.description(),
            Error::FromUtf8(ref error) => error.description(),
            Error::ParseIntError(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::FromUtf8(ref error) => Some(error),
            Error::ParseIntError(ref error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref code, line, col) => {
                write!(fmt, "{:?} at line {} column {}", code, line, col)
            }
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
            Error::FromUtf8(ref error) => fmt::Display::fmt(error, fmt),
            Error::ParseIntError(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        Error::ParseIntError(error)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Syntax(ErrorCode::Custom(msg.to_string()), 0, 0)
    }
}

impl ser::Error for Error {
    /// Raised when there is general error when deserializing a type.
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Syntax(ErrorCode::Custom(msg.to_string()), 0, 0)
    }
}

/// Helper alias for `Result` objects that return a JSON `Error`.
pub type Result<T> = result::Result<T, Error>;
