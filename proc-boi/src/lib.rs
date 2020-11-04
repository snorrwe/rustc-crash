#![crate_type = "proc-macro"]

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Problematic)]
pub fn problematic(_input: TokenStream) -> TokenStream {
    let rogue_ident = quote::format_ident!(",");
    let result = quote! {
        struct Aux {
            #rogue_ident : i32
        }
    };
    TokenStream::from(result)
}
