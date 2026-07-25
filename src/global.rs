use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
extern "C" {
    /// manually trigger garbage collection
    #[wasm_bindgen]
    pub fn gc(quick: Option<bool>);
}
