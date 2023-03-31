wai_bindgen_rust::export!("wasm-example.wai");

pub struct WasmExample;

impl wasm_example::WasmExample for WasmExample {
    fn hello(input: String) -> String {
        input
    }
}

