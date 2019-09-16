use std::fmt;

use crate::core::Method;
use crate::uri::Uri;
use super::Header;

use std::collections::HashMap;

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::To(name, uri, params) => write_named_header("To", name, uri, params, f),
            Header::From(name, uri, params) => write_named_header("From", name, uri, params, f),
            Header::Contact(name, uri, params) => write_named_header("Contact", name, uri, params, f),
            Header::ReplyTo(name, uri, params) => write_named_header("Reply-To", name, uri, params, f),
            Header::CSeq(num, method) => write!(f, "CSeq: {} {}", num, method),
            Header::MaxForwards(num) => write!(f, "Max-Forwards: {}", num),
            Header::Expires(num) => write!(f, "Expires: {}", num),
            Header::Accept(methods) => write_method_array_header("Accept", f, methods),
            Header::Allow(methods) => write_method_array_header("Allow", f, methods),
            Header::ContentEncoding(ty) => write_simple_field("Content-Encoding", ty, f),
            Header::ContentLength(len) => write_simple_field("Content-Length", len, f),
            Header::ContentType(ty) => write_simple_field("Content-Type", ty, f),
            Header::UserAgent(agent) => write_simple_field("User-Agent", agent, f),
            Header::CallId(call_id) => write_simple_field("Call-ID", call_id, f),
            Header::ContentLanguage(lang) => write_simple_field("Content-Language", lang, f),
            Header::AcceptLanguage(lang) => write_simple_field("Accept-Language", lang, f),
            Header::AcceptEncoding(ty) => write_simple_field("Accept-Encoding", ty, f),
            Header::AlertInfo(data) => write_simple_field("Alert-Info", data, f),
            Header::ErrorInfo(data) => write_simple_field("Error-Info", data, f),
            Header::AuthenticationInfo(data) => write_simple_field("Authentication-Info", data, f),
            Header::Authorization(data) => write_simple_field("Authorization", data, f),
            Header::CallInfo(data) => write_simple_field("Call-Info", data, f),
            Header::InReplyTo(data) => write_simple_field("In-Reply-To", data, f),
            Header::ContentDisposition(data) => write_simple_field("Content-Disposition", data, f),
            Header::Date(string) => write_simple_field("Date", string, f),
            Header::MinExpires(exp) => write_simple_field("Min-Expires", exp, f),
            Header::MimeVersion(exp) => write_simple_field("MIME-Version", exp, f),
            Header::Organization(org) => write_simple_field("Organization", org, f),
            Header::ProxyAuthenticate(data) => write_simple_field("Proxy-Authenticate", data, f),
            Header::ProxyAuthorization(data) => write_simple_field("Proxy-Authorization", data, f),
            Header::ProxyRequire(data) => write_simple_field("Proxy-Require", data, f),
            Header::Require(data) => write_simple_field("Require", data, f),
            Header::RetryAfter(data) => write_simple_field("Retry-After", data, f),
            Header::Route(data) => write_simple_field("Route", data, f),
            Header::Subject(data) => write_simple_field("Subject", data, f),
            Header::RecordRoute(data) => write_simple_field("Record-Route", data, f),
            Header::Server(data) => write_simple_field("Server", data, f),
            Header::Supported(data) => write_string_array_header("Supported", f, data),
            Header::Timestamp(data) => write_simple_field("Timestamp", data, f),
            Header::Unsupported(data) => write_simple_field("Unsupported", data, f),
            Header::Warning(data) => write_simple_field("Warning", data, f),
            Header::Via(data) => write_simple_field("Via", data, f),
            Header::Priority(data) => write_simple_field("Priority", data, f),
            Header::WwwAuthenticate(data) => write!(f, "WWW-Authenticate: {}", data)
        }
    }
}

fn write_simple_field<D: fmt::Display>(header: &str, data: D, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", header, data)
}

fn write_named_header(header: &str, name: &Option<String>, uri: &Uri, params: &HashMap<String, String>, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: ", header)?;
    if let Some(name) = name {
        if name.contains(' ') {
            write!(f, "\"{}\" <{}>", name, uri)?;
        } else {
            write!(f, "{} <{}>", name, uri)?;
        }
    } else {
        write!(f, "<{}>", uri)?;
    }
    for (key, value) in params.iter() {
        write!(f, ";{}={}", key, value)?;
    }
    Ok(())
}

fn write_method_array_header(name: &str, f: &mut fmt::Formatter, v: &Vec<Method>) -> fmt::Result {
    write!(f, "{}: ", name)?;
    for (index, method) in v.iter().enumerate() {
        if index == 0 {
            write!(f, "{}", method)?;
        } else {
            write!(f, ",{}", method)?;
        }
    }
    Ok(())
}

fn write_string_array_header(name: &str, f: &mut fmt::Formatter, v: &Vec<String>) -> fmt::Result {
    write!(f, "{}: ", name)?;
    for (index, method) in v.iter().enumerate() {
        if index == 0 {
            write!(f, "{}", method)?;
        } else {
            write!(f, ",{}", method)?;
        }
    }
    Ok(())
}
