//! q01 spike — positive-regression tests for CubeCL 0.10 emit idioms.
//!
//! These are the **compiling** patterns the emit fix must produce. The broken
//! patterns that motivated this spike (raw f64-literal × generic-F arithmetic,
//! 1-tuple `let`-binding inside `-> (F,)`, and `(F × ≥13)` returns) are
//! documented in `.planning/quick/260515-q01-cse-chunk-arity-cap-12/SPIKE-FINDINGS.md`
//! with full error transcripts. They were removed from this file so
//! `cargo test -p libxc-kernel-math` stays green.
//!
//! Located alongside `spike_tuple_return_cube.rs` (see that file for rationale:
//! verify/ would OOM; libxc-kernel-math depends only on cubecl + libm).

#![allow(non_snake_case, dead_code, unused_variables)]

use cubecl::prelude::*;

// -----------------------------------------------------------------------------
// At-ceiling tuple return — `MAX_TUPLE_ARITY = 12` in tools/translate_v2/cse.py
// must NOT be raised without re-validating this. CubeType is implemented for
// tuples up to arity 12 in cubecl 0.10; arity 13+ fails to derive (E0277).
// -----------------------------------------------------------------------------
#[cube]
fn ceiling_twelve_tuple<F: Float>(x: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    (x, x, x, x, x, x, x, x, x, x, x, x)
}

// -----------------------------------------------------------------------------
// F::new() literal-coercion idiom — emit.py / per_functional.py must wrap
// every f64 literal in chunk bodies via `F::new(<literal>)` because CubeCL 0.10
// does NOT auto-coerce f64 literals against generic `F: Float`:
//   raw `let t = 0.173e2 * x;` (x:F) → E0277 "Mul<F> not implemented for {float}"
//
// These three confirm that F::new wrapping makes the same operations compile.
// -----------------------------------------------------------------------------
#[cube]
fn F_new_literal_mul<F: Float>(x: F) -> F {
    F::new(0.17315859105681463759e2) * x
}

#[cube]
fn F_new_literal_div<F: Float>(x: F) -> F {
    x / F::new(0.5e2)
}

#[cube]
fn F_new_literal_add<F: Float>(x: F) -> F {
    F::new(0.25e1) + x
}

// -----------------------------------------------------------------------------
// Scalar single-output chunk return — emit.py / per_functional.py must emit
// `-> F` (not `-> (F,)`) for chunks with len(outputs) == 1. The 1-tuple return
// ABI is broken with `let` bindings under cubecl 0.10:
//   `let b = a;` inside `-> (F,)` → E0308 "expected (NativeExpand<F>,), found
//   NativeExpand<F>" — the macro infers `b`'s type from the return type.
//
// `-> F` with the same body compiles fine.
// -----------------------------------------------------------------------------
#[cube]
fn scalar_return_alias<F: Float>(a: F) -> F {
    let b = a;
    b
}

#[cube]
fn scalar_return_compute<F: Float>(a: F) -> F {
    let b = a + a;
    b
}
