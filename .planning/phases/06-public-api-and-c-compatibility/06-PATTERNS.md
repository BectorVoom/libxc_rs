# Phase 6: Public API and C Compatibility - Pattern Map

**Mapped:** 2026-05-06
**Files analyzed:** 14 (11 src/ files + 1 include/ + 2 verify/tests/)
**Analogs found (in this codebase):** 11 / 14
**Net-new (no analog in libxc_rs):** 3 / 14 — `compat/c_layout.rs`, `compat/raw_handle.rs`, `include/xc.h`

The libxc_rs codebase has **zero pre-existing `extern "C"` / `#[repr(C)]` / `Box::into_raw` / `catch_unwind` / `thread_local!` usage in the production tree** (`src/`). All FFI patterns Phase 6 needs are net-new at the `unsafe` boundary level. The closest analog for *raw FFI* in the workspace is `verify/src/lib.rs` and `verify/src/oracle_ffi.rs`, which **call** libxc through generated `libxc-sys` bindings — useful as a *consumer-side* model of the same FFI surface Phase 6 will *expose*. Wherever an in-tree analog exists for a non-FFI concern (typed errors, registry lookups, sealed dispatch shape, threshold/ext-param setters, hybrid query API, oracle-test harness layout), Phase 6 should mirror it 1:1.

## File Classification

| New / Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---------------------|------|-----------|----------------|---------------|
| `src/api/builder.rs` | builder | request-response | `src/functional/lifecycle.rs` (`Functional::new`) + `src/functional/config.rs` (setters) | role-match (builder is net-new shape; wrappee is exact) |
| `src/api/batch.rs` | service / driver | batch | `src/eval/workspace.rs` (`EvaluationWorkspace::new`) + `src/functional/evaluate.rs` | exact (wrappee is in-tree) |
| `src/api/evaluate.rs` | sealed-trait dispatcher | request-response | `src/functional/params.rs` (trait + Box dispatch) + `src/functional/evaluate.rs` (per-family arms) | role-match (sealed-trait pattern is net-new; family arms are exact) |
| `src/api/mod.rs` (update) | module barrel | n/a | current `src/api/mod.rs`, `src/output/mod.rs`, `src/functional/mod.rs` | exact |
| `src/compat/c_layout.rs` | FFI types | layout | NONE in `src/`; libxc-sys `xc_func_type` shape (`verify/src/lib.rs:31`) is the consumer | net-new |
| `src/compat/raw_handle.rs` | FFI lifecycle / handle | request-response | NONE in `src/` (no `Box::into_raw` exists); libxc reference at `libxc-master/src/functionals.c:224-391` | net-new |
| `src/compat/macros.rs` (`extern_c_wrapper!`) | declarative macro | n/a | `src/eval/dispatch.rs` (`ten_arm_dispatch!`), `src/eval/workspace.rs` (`pop!` macro) | role-match (declarative-macro shape is in-tree; the catch_unwind body is net-new) |
| `src/compat/legacy_eval.rs` | FFI evaluator wrapper | request-response | `src/functional/evaluate.rs` (wrappee), `src/output/mod.rs` (Option-of-mut-slice constructor) | exact (wrappee is in-tree) |
| `src/compat/ids.rs` | FFI registry getters | request-response | `src/registry/mod.rs` (`lookup_by_id`, `lookup_by_name`, `version`, `version_string`) | exact |
| `src/compat/info.rs` | FFI info accessors | request-response | `src/meta/mod.rs` (`FunctionalMeta`) + `src/registry/mod.rs::lookup_by_id` | exact |
| `src/compat/hybrid.rs` | FFI hybrid getters | request-response | `src/functional/hybrid.rs` (`Functional::cam_coefficients`, `nlc_coefficients`, etc.) | exact (wrappee is in-tree) |
| `src/compat/library.rs` | FFI version constants | request-response | `src/registry/mod.rs::version`/`version_string`/`reference_string` | exact |
| `src/compat/errno.rs` | FFI thread-local error state | request-response | `src/error/mod.rs` (`LibxcRsError` enum) | role-match (thread-local + `discriminant()` are net-new; error enum is in-tree) |
| `src/compat/removed.rs` (rewrite) | FFI errno mapping | request-response | `src/registry/removed.rs` (data) + `src/registry/mod.rs:13-29` (consumer) | exact |
| `src/error/mod.rs` (extend) | error enum | n/a | itself (existing 18 variants set the pattern) | exact |
| `include/xc.h` | C header | n/a | NONE in `libxc_rs/`; `libxc-master/src/xc.h` is the *reference* (not a project analog) | net-new |
| `verify/tests/compat_smoke.rs` | integration test | request-response | `verify/tests/lda_oracle.rs` (`FunctionalTestCase`, runner shape) | exact |

## Pattern Assignments

### `src/api/builder.rs`
**Role:** Owned-self builder that wraps `Functional::new` + Phase-5 setter chain into one chained-config call site.
**Analog:** `src/functional/lifecycle.rs` (constructor) + `src/functional/config.rs` (setters) — the builder is a *thin wrapper* over both.
**Pattern to replicate:**
- Constructor returns `Result<Functional, LibxcRsError>` (mirror `Functional::new(id, spin) -> Result<Self, LibxcRsError>` exactly — `lifecycle.rs:31`).
- Setter signatures and error types come from `config.rs` 1:1: `set_density_threshold(&mut self, v: f64)`, `set_ext_param(&mut self, name, val) -> Result<(), LibxcRsError>`, `set_ext_params(&mut self, vals) -> Result<(), LibxcRsError>`.
- Doc-comment style: `# Errors` blocks listing typed `LibxcRsError` variants per setter (see `config.rs:62-64`).
- No interior mutability (D-13 / `lifecycle.rs:30` "Functional" doc).
**Code excerpt** (`src/functional/lifecycle.rs:31-52`):
```rust
pub fn new(id: FunctionalId, spin: Spin) -> Result<Self, LibxcRsError> {
    let meta: &'static FunctionalMeta = lookup_by_id(id.raw())?;
    let dims = match meta.family {
        Family::Lda => Dimensions::lda(spin),
        Family::Gga => Dimensions::gga(spin),
        Family::Mgga => Dimensions::mgga(spin),
    };
    let ext_params: Option<Box<[f64]>> = if meta.ext_params.is_empty() {
        None
    } else {
        Some(meta.ext_params.iter()
            .map(|spec| spec.default_value)
            .collect::<Vec<f64>>().into_boxed_slice())
    };
    let params: Box<dyn FunctionalParams> = construct_params(meta.id, ext_params.as_deref())?;
    // ...
}
```
**Differences for Phase 6:**
- Builder accumulates configuration in private fields (`id`, `spin`, optional `density_threshold`, `ext_params: Vec<(String, f64)>`, ...) and only invokes `Functional::new` + setters from `.build()`. Setter errors surface from `.build()`, not mid-chain (the recommended owned-self chain in CONTEXT § Specifics).
- Add `pub use api::FunctionalBuilder;` re-export to `src/lib.rs` (mirror the existing `pub use functional::{Functional, ...}` block at `lib.rs:33`).

---

### `src/api/batch.rs`
**Role:** Owns an `EvaluationWorkspace` plus `(spin, np_max)`; `evaluate<I: EvaluateInput>` forwards to `input.dispatch(...)` after a `BatchOverflow` guard.
**Analog:** `src/eval/workspace.rs` (workspace constructor) + `src/functional/evaluate.rs` (wrappee shape) — `BatchEvaluator` literally owns one `EvaluationWorkspace` and re-uses Phase 5's evaluate methods through the sealed trait.
**Pattern to replicate:**
- Constructor signature mirrors `EvaluationWorkspace::new(np: usize, spin: Spin) -> Self` (`workspace.rs:170`) — same arg order + same eager-allocation discipline.
- Workspace is sized once for MGGA-superset (`workspace.rs:171-178`); BatchEvaluator inherits this property by composition.
- `evaluate` is `&mut self` (BatchEvaluator owns the workspace mutably) but takes `&Functional` (D-A3-2: read-only borrow, mirrors `Functional::evaluate_lda(&self, ...)` at `evaluate.rs:34`).
- Family/spin guard returns `LibxcRsError::SpinMismatch` / `FamilyMismatch` (already in `error/mod.rs:39-50`) — no new error machinery beyond the new `BatchOverflow` variant.
**Code excerpt** (`src/eval/workspace.rs:165-180`):
```rust
impl EvaluationWorkspace {
    /// Create a new workspace with scratch sized for MGGA superset.
    pub fn new(np: usize, spin: Spin) -> Self {
        let dims = Dimensions::mgga(spin);
        let total = dims.total_output_components() * np;
        Self {
            scratch: vec![0.0; total],
            np,
            spin,
            dims,
        }
    }
    pub fn zero_scratch(&mut self) { self.scratch.fill(0.0); }
    pub fn np(&self) -> usize { self.np }
    pub fn spin(&self) -> Spin { self.spin }
}
```
**Code excerpt** (`src/functional/evaluate.rs:34-54` — the wrappee):
```rust
pub fn evaluate_lda(
    &self,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    if self.auxiliaries.is_empty() {
        let lda_fn = LdaFunctional::from_id(self.meta.id)?;
        dispatch_lda(lda_fn, input, order, output, &*self.params, &self.thresholds)
    } else {
        evaluate_mixed_lda_functional(self, input, order, output, workspace)
    }
}
```
**Differences for Phase 6:**
- BatchEvaluator stores `np_max` and checks `input.np() > np_max` before dispatching (new guard, returns `LibxcRsError::BatchOverflow`).
- `evaluate` is monomorphic on `I: EvaluateInput` (sealed trait from `api/evaluate.rs`) — the workspace, output, and family-specific dispatch are all hidden behind `input.dispatch(...)`.

---

### `src/api/evaluate.rs` (NEW)
**Role:** Sealed `EvaluateInput` trait with three impls (`LdaInput`, `GgaInput`, `MggaInput`); each impl's `dispatch` calls the family-specific `Functional::evaluate_*` method. Provides type-safe dispatch with zero `unsafe`.
**Analog:** `src/functional/params.rs` (`FunctionalParams` trait + `Box<dyn ...>` dispatch) for the *trait shape*; `src/functional/evaluate.rs:34-102` for the *per-family arms*.
**Pattern to replicate:**
- Sealed-trait pattern: `mod sealed { pub trait Sealed {} }` then `pub trait EvaluateInput: sealed::Sealed`. Mirrors the closed-set discipline of `LdaFunctional`/`GgaFunctional`/`MggaFunctional` enums (`src/model/lda_functional.rs:28`) — the input families are also a closed set of three.
- Per-family dispatch arm body matches `Functional::evaluate_{lda,gga,mgga}` exactly (`evaluate.rs:34-102`); the trait impl is a one-liner `functional.evaluate_lda(self, order, output, workspace)`.
- Family-mismatch guard inside each impl uses the existing `LibxcRsError::FamilyMismatch { id, expected, actual }` variant (`error/mod.rs:39-44`) — no new error needed for this code path.
**Code excerpt** (`src/functional/params.rs` — the closest in-tree dyn-dispatch pattern):
```rust
// (params.rs is 117 lines; its FunctionalParams trait is the in-tree
//  precedent for `pub trait T: ... { fn method(...) -> Result<(), E>; }`
//  consumed via `Box<dyn T>` from the Functional handle.)
pub trait FunctionalParams: std::any::Any + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;
}
```
**Code excerpt** (`src/functional/evaluate.rs:58-78` — GGA arm shape to mirror in the trait impl):
```rust
pub fn evaluate_gga(
    &self,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    if self.auxiliaries.is_empty() {
        let gga_fn = GgaFunctional::from_id(self.meta.id)?;
        dispatch_gga(gga_fn, input, order, output, &*self.params, &self.thresholds)
    } else {
        evaluate_mixed_gga(self, input, order, output, workspace)
    }
}
```
**Differences for Phase 6:**
- Sealed-trait pattern itself is net-new in this codebase. Pull the canonical shape from any standard sealed-trait reference (CONTEXT § Specifics, RESEARCH § Pattern 4).
- Trait uses GAT (`type Output<'a>`) for the per-impl Output bundle, which `params.rs` does not — the closest `'_` lifetime example in src/ is the `LdaScratch<'a>` family in `eval/workspace.rs:15-126`.
- Each impl's `dispatch` adds a family-mismatch guard *before* forwarding (Phase-5 evaluate methods don't need this guard because they're already family-typed; the trait does because users pick the family at call time).

---

### `src/api/mod.rs` (UPDATE)
**Role:** Module barrel — registers `pub mod evaluate;` alongside `batch` and `builder`, and re-exports the public surface.
**Analog:** existing `src/api/mod.rs` (3 lines) + `src/output/mod.rs:1-3` + `src/functional/mod.rs:9-19`.
**Pattern to replicate:**
- One `pub mod` line per submodule (already-established style).
- Public re-exports use `pub use submodule::TypeName;` (mirror `src/lib.rs:21-35`).
**Code excerpt** (`src/functional/mod.rs:9-19`):
```rust
pub mod config;
pub mod evaluate;
pub mod hybrid;
pub mod lifecycle;
pub mod params;
// ...
pub use hybrid::{classify_hybrid, CamCoefficients, NlcCoefficients};
pub use params::{FunctionalParams, NoParams};
```
**Differences for Phase 6:**
- Add `pub mod evaluate;`, then `pub use batch::BatchEvaluator;`, `pub use builder::FunctionalBuilder;`, `pub use evaluate::EvaluateInput;`. Mirror this in `src/lib.rs:33-35` for top-level visibility.

---

### `src/compat/c_layout.rs` (REWRITE)
**Role:** Forward-declares the opaque `xc_func_type` and `xc_func_info_type` Rust types that the C ABI sees; emits compile-time layout assertions guaranteeing zero-size opaque structs.
**Analog:** **NONE in src/.** No production-tree file uses `#[repr(C)]`. The closest *consumer-side* shape is `verify/src/lib.rs:31` reading through libxc-sys's `xc_func_type`. The **reference shape** is `libxc-master/src/xc.h` lines 175-189 (forward-decl + opaque pointers).
**Pattern to replicate:**
- Compile-time layout assertions go in `const _: () = assert!(...);` blocks — this idiom is not yet used in src/, so anchor it on the in-tree `#[repr(u8)]` discriminant tests in `src/model/mod.rs:178-196` (`assert_eq!(Family::Lda as u8, 1)` etc., showing the team's preference for compile-time-equivalent layout invariants over runtime asserts).
- File-level doc comment style follows `src/functional/mod.rs:1-7` ("// Runtime Functional handle: owns ...") — one-paragraph summary at the top.
**Code excerpt** (`src/model/mod.rs:11-17` + `:178-196` — closest in-tree precedent for `#[repr(u8)]` + numeric-stable type assertions):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Family {
    Lda  = 1,
    Gga  = 2,
    Mgga = 4,
}
// ...
#[test]
fn test_family_repr() {
    assert_eq!(Family::Lda as u8, 1);
    assert_eq!(Family::Gga as u8, 2);
    assert_eq!(Family::Mgga as u8, 4);
}
```
**Differences for Phase 6:**
- This is **net-new pattern** — `#[repr(C)]` zero-size struct + `PhantomData<(*mut u8, std::marker::PhantomPinned)>` + `const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);`. RESEARCH § Pattern 1 (lines 295-305) gives the exact shape; copy it.
- Document the reasoning inline so future readers understand why `#[repr(C)]` plus zero-size opaque is chosen over `repr(transparent)` over `Box<FunctionalSlot>` (CONTEXT D-A1-1 / D-A1-4: opaque-by-design, all introspection goes through accessors).

---

### `src/compat/raw_handle.rs` (REWRITE)
**Role:** Owns the `FunctionalSlot` enum (Empty / Initialized(Functional)) and the four lifecycle entry points `xc_func_alloc/init/end/free`. The pointer cast `*mut xc_func_type ↔ *mut FunctionalSlot` is layout-safe because the C type is opaque.
**Analog:** **NONE in src/.** No `Box::into_raw` / `Box::from_raw` exists in the production tree. The **reference** is libxc's `xc_func_alloc/init/end/free` in `libxc-master/src/functionals.c:224-391` (consumer-side mirror at `verify/src/lib.rs:20-37`).
**Pattern to replicate:**
- File-level docstring + per-fn doc style mirrors `src/functional/lifecycle.rs:1-7` and `:18-30` (top-of-file paragraph; per-fn `# Arguments` / `# Errors` blocks).
- The *internal* state machine (Empty / Initialized) mirrors the typed-state pattern of `src/output/mod.rs` (each Output field is `Option<&mut [f64]>` — "absent / present" is encoded in the type system, not in a sentinel value). `FunctionalSlot::Empty / Initialized(Functional)` is the same idea applied to lifecycle.
- Drop semantics from `src/functional/lifecycle.rs:152-159` (`impl Drop for Functional` is explicit no-op; document why) — `xc_func_free` becomes the explicit drop-via-`Box::from_raw`.
**Code excerpt** (`src/functional/lifecycle.rs:152-159` — the in-tree precedent for "explicit drop with rationale"):
```rust
impl Drop for Functional {
    /// No-op per D-15. All fields (Box, Vec, &'static) auto-drop. Implemented
    /// explicitly so downstream readers do not wonder whether there is FFI or
    /// other resource cleanup that needs to happen.
    fn drop(&mut self) {
        // Intentionally empty.
    }
}
```
**Code excerpt** (RESEARCH § Pattern 1 lines 313-356 — full lifecycle to copy verbatim):
```rust
pub(crate) enum FunctionalSlot {
    Empty,
    Initialized(crate::Functional),
}

#[unsafe(no_mangle)]
pub extern "C" fn xc_func_alloc() -> *mut xc_func_type {
    let slot = Box::new(FunctionalSlot::Empty);
    Box::into_raw(slot) as *mut xc_func_type
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_init(p: *mut xc_func_type, functional: i32, nspin: i32) -> i32 {
    extern_c_wrapper!(p, "xc_func_init", { /* ... */ })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_free(p: *mut xc_func_type) {
    if p.is_null() { return; }
    unsafe { drop(Box::from_raw(p as *mut FunctionalSlot)); }
}
```
**Differences for Phase 6:**
- All `Box::into_raw` / `Box::from_raw` / `std::ptr::write` calls are net-new and confined to this file (BUILD-04 unsafe budget).
- The `as_initialized() / as_mut_initialized()` helpers (returning `Result<&Functional, LibxcRsError::UninitializedHandle>`) are net-new — model their signature on `Functional::ext_param(&self, name) -> Result<f64, LibxcRsError>` from `src/functional/config.rs:39-54`.

---

### `src/compat/macros.rs` (NEW) — `extern_c_wrapper!`
**Role:** One declarative macro every extern "C" entry point uses to enforce: NULL handle check, `catch_unwind` body, errno set on `Err` or panic, return `i32`.
**Analog:** `src/eval/dispatch.rs:313-447` (`ten_arm_dispatch!`) and `src/eval/workspace.rs:496-502` (`pop!`) — both demonstrate the team's idiomatic `macro_rules!` style: `($arg:expr, $name:literal, $body:block) => {{ ... }}`. The *body* of `extern_c_wrapper!` (catch_unwind, downcast panic payload, set thread-local errno) is net-new; only the *macro shape* is in-tree.
**Pattern to replicate:**
- Doc comment above the macro explains "What", "When to use", "Caller contract" (mirror `dispatch.rs:309-312`).
- Use `($var:expr, $literal:literal, $body:block) => {{ ... }}` shape (no fragments beyond `expr`/`literal`/`block` — keeps the macro hygienic and grep-able).
- Place the macro at module level with `#[macro_export]` so the per-file extern "C" functions can reach it from any compat submodule.
**Code excerpt** (`src/eval/workspace.rs:494-502` — the in-tree macro-shape precedent):
```rust
let mut cursor = buf;

macro_rules! pop {
    ($field:ident) => {{
        let (out, rest) = cursor.split_at_mut(d.$field as usize * np);
        cursor = rest;
        out
    }};
}
```
**Code excerpt** (`src/eval/dispatch.rs:313-339` — the in-tree multi-arm macro pattern):
```rust
macro_rules! ten_arm_dispatch {
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($exc_u:tt)::+], [$($vxc_u:tt)::+], /* ... */
        params = ( $( $scalar:expr ),* $(,)? )
    ) => {
        ten_arm_dispatch!( /* delegate to 2nd arm */ )
    };
    ( /* canonical arm */ ) => {{
        // Body that all call sites share.
    }};
}
```
**Differences for Phase 6:**
- The macro body uses `std::panic::catch_unwind(AssertUnwindSafe(|| $body))` — net-new in src/. RESEARCH § Pattern 2 (lines 374-419) gives the full implementation; copy it.
- Errno mechanism (`compat::errno::set_error`, `compat::errno::discriminant`) is an internal collaborator implemented in `compat/errno.rs` (see below).

---

### `src/compat/legacy_eval.rs` (REWRITE)
**Role:** The 33 `xc_lda_*` / `xc_gga_*` / `xc_mgga_*` evaluation functions plus 4 threshold setters and 5 ext-param setters. Each builds a typed Input/Output bundle from raw C pointers and forwards to `Functional::evaluate_*`.
**Analog:** `src/functional/evaluate.rs` (the wrappee) for the evaluation forwarding; `src/output/mod.rs:51-75` for the `Option<&'a mut [f64]>` constructor target; `verify/src/lib.rs:49-90` for the consumer-side shape of how raw pointers + `np` reconstruct typed buffers.
**Pattern to replicate:**
- For each evaluation arm: (1) read `np` from C, (2) look up `dims = Dimensions::<family>(spin)`, (3) for each `*mut f64` arg map NULL→`None` / non-NULL→`Some(slice::from_raw_parts_mut(...))`, (4) construct typed Output, (5) forward to `f.evaluate_*`. RESEARCH § Pattern 3 (lines 433-464) is the full template.
- Buffer-length math: `np * dims.<field> as usize` exactly mirrors `src/output/mod.rs::validate_output_field` (lines 14-25). Use `usize::checked_mul` per RESEARCH § Security threat table (line 851).
- Every function takes `*const xc_func_type` or `*mut xc_func_type` and goes through the `extern_c_wrapper!` macro — uniform NULL check + panic boundary.
- Threshold/ext_param setters wrap `Functional::set_density_threshold` etc. from `src/functional/config.rs:153-167` — the Rust setters take `&mut self` so the FFI wrapper does `unsafe { (&mut *p).as_mut_initialized()?.set_*(...) }`.
**Code excerpt** (`src/functional/evaluate.rs:34-54` — the wrappee, replicated for context):
```rust
pub fn evaluate_lda(
    &self, input: &LdaInput, order: DerivativeOrder,
    output: &mut LdaOutput, workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    if self.auxiliaries.is_empty() {
        let lda_fn = LdaFunctional::from_id(self.meta.id)?;
        dispatch_lda(lda_fn, input, order, output, &*self.params, &self.thresholds)
    } else {
        evaluate_mixed_lda_functional(self, input, order, output, workspace)
    }
}
```
**Code excerpt** (`src/output/mod.rs:51-75` — the constructor shape that NULL→None feeds into):
```rust
impl<'a> LdaOutput<'a> {
    pub fn new(
        zk: Option<&'a mut [f64]>,
        vrho: Option<&'a mut [f64]>,
        v2rho2: Option<&'a mut [f64]>,
        v3rho3: Option<&'a mut [f64]>,
        v4rho4: Option<&'a mut [f64]>,
        np: usize, spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::lda(spin);
        validate_output_field(&zk, "zk", np, dims.zk as usize)?;
        // ...
        Ok(Self { zk, vrho, v2rho2, v3rho3, v4rho4 })
    }
}
```
**Code excerpt** (`verify/src/lib.rs:70-90` — consumer-side raw-pointer shape, mirror in inverse):
```rust
unsafe {
    let func = oracle_ffi::xc_func_alloc();
    if func.is_null() { bail!("xc_func_alloc returned null"); }
    let ret = oracle_ffi::xc_func_init(func, func_id, spin);
    if ret != 0 {
        oracle_ffi::xc_func_free(func);
        bail!("xc_func_init failed with code {ret} for func_id={func_id}, spin={spin}");
    }
    // ... call evaluation, then xc_func_end + xc_func_free
}
```
**Differences for Phase 6:**
- Each compat function allocates a per-call `EvaluationWorkspace::new(np, spin)` (RESEARCH § Pattern 3 line 460 — C callers don't get the BatchEvaluator's workspace-reuse ergonomics; this is acceptable because the Rust API path is the recommended one).
- The setter wrappers must walk `self.auxiliaries` to apply thresholds recursively (RESEARCH Pitfall 4 + Wave 0 gap line 823 — the wrappee in `config.rs:153-167` currently doesn't recurse; Phase 6 either fixes the wrappee or replicates the recursion in the FFI wrapper).
- All `unsafe { slice::from_raw_parts_mut(ptr, len) }` calls are net-new, confined to this file.

---

### `src/compat/ids.rs` (REWRITE)
**Role:** The 8 discovery functions (`xc_functional_get_number`, `xc_functional_get_name`, `xc_family_from_id`, `xc_number_of_functionals`, `xc_max_func_aux`, `xc_func_set_dens_threshold` etc.) — wraps the static registry one-to-one.
**Analog:** `src/registry/mod.rs:12-92` — every compat function in this file is a 1:1 wrapper of an existing registry function.
**Pattern to replicate:**
- Per-fn doc comment matches `registry/mod.rs:9-12` (one-line summary + complexity note + reference to design decision).
- Error path: `lookup_by_id` returns `Result<&'static FunctionalMeta, LibxcRsError>`; the FFI wrapper translates to `i32` errno via the `extern_c_wrapper!` discriminant mapping.
- Name-return path follows libxc convention: return `*const c_char` pointing into a thread-local cache (RESEARCH Pitfall 2 lines 583-594) — wraps `meta.name: &'static str` from `src/meta/mod.rs:53`.
**Code excerpt** (`src/registry/mod.rs:12-35` — the wrappee):
```rust
pub fn lookup_by_id(id: u16) -> Result<&'static FunctionalMeta, LibxcRsError> {
    if let Some(&(_, replacement_id)) = removed::REMOVED_IDS.iter().find(|&&(r, _)| r == id) {
        let replacement_name = if replacement_id > 0 { /* ... */ } else { "none" };
        return Err(LibxcRsError::RemovedFunctionalId {
            removed_id: id,
            replacement_id,
            replacement_name,
        });
    }
    by_id::REGISTRY_BY_ID.get(id as usize)
        .and_then(|opt| *opt)
        .ok_or(LibxcRsError::UnknownFunctionalId(id))
}
```
**Code excerpt** (`src/registry/mod.rs:40-69` — name lookup + counts to wrap):
```rust
pub fn lookup_by_name(name: &str) -> Result<FunctionalId, LibxcRsError> {
    let upper = name.to_ascii_uppercase();
    if let Some(&(_, id)) = removed::NAME_ALIASES.iter()
        .find(|&&(n, _)| n.eq_ignore_ascii_case(&upper)) {
        return Ok(FunctionalId(id));
    }
    by_name::REGISTRY_BY_NAME
        .binary_search_by_key(&upper.as_str(), |&(n, _)| n)
        .map(|idx| FunctionalId(by_name::REGISTRY_BY_NAME[idx].1))
        .map_err(|_| LibxcRsError::UnknownFunctionalName(name.to_string()))
}

pub fn functional_count() -> usize { 649 }
```
**Differences for Phase 6:**
- C strings: convert `&str` → CString → `*const c_char` via thread-local cache. RESEARCH Pitfall 2 documents the contract ("valid until next call on this thread"). Net-new code path.
- Family-from-id wraps `meta.family as u8` (the `#[repr(u8)]` from `src/model/mod.rs:11-17` makes this a zero-cost cast).

---

### `src/compat/info.rs` (NEW)
**Role:** The 10 `xc_func_info_get_*` accessors + 4 `xc_func_reference_get_*` accessors. `xc_func_info_type*` is `&'static FunctionalMeta as *const xc_func_info_type` (D-A1-4 — opaque, accessor-only).
**Analog:** `src/meta/mod.rs` (`FunctionalMeta` field set) + `src/registry/mod.rs::lookup_by_id` (the source of `&'static FunctionalMeta`).
**Pattern to replicate:**
- Each accessor is `unsafe extern "C" fn xc_func_info_get_<field>(info: *const xc_func_info_type) -> <ty>` and dereferences to `&'static FunctionalMeta` via pointer cast.
- Field set comes from `src/meta/mod.rs:51-70` (`name`, `kind`, `family`, `flags`, `references`, `ext_params`, `default_density_threshold`, `auxiliaries`, `hybrid_terms`, `nlc_params`, `max_order`, `hybrid_type`).
- For `&'static str` fields (`name`, `references[i].citation/doi/key`), use `CString`/`*const c_char` pattern from `compat/ids.rs::xc_functional_get_name` — same thread-local-cache contract.
**Code excerpt** (`src/meta/mod.rs:50-70` — the source of every accessor return value):
```rust
#[derive(Debug, PartialEq)]
pub struct FunctionalMeta {
    pub id: FunctionalId,
    pub name: &'static str,
    pub kind: Kind,
    pub family: Family,
    pub flags: FunctionalFlags,
    pub references: &'static [Reference],
    pub ext_params: &'static [ExtParamSpec],
    pub default_density_threshold: f64,
    pub auxiliaries: &'static [(FunctionalId, f64)],
    pub hybrid_terms: &'static [HybridTerm],
    pub nlc_params: Option<(f64, f64)>,
    pub max_order: DerivativeOrder,
    pub hybrid_type: HybridType,
}
```
**Differences for Phase 6:**
- Pointer cast `*const xc_func_info_type → &'static FunctionalMeta` is net-new; the `&'static` lifetime is sound because metadata lives in `.rodata` for the program lifetime (Phase 1 D-01).
- `xc_func_info_get_n_ext_params` returns `meta.ext_params.len() as i32` — trivial wrapper, just type cast.

---

### `src/compat/hybrid.rs` (NEW)
**Role:** The 7 hybrid/aux/nlc accessors (`xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`, `xc_nlc_coef`, `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`) plus the 2 `xc_gga_ak13_*` helpers (per RESEARCH Pitfall 9).
**Analog:** `src/functional/hybrid.rs` — the wrappee is fully in-tree; every compat function is a 1:1 wrapper of a `Functional` method.
**Pattern to replicate:**
- Wrap `Functional::hybrid_type()` (`hybrid.rs:84-86`) — return `meta.hybrid_type as i32` (the `HybridType` enum doesn't have `#[repr(u8)]` — Phase 6 must either add `#[repr(i32)]` to match libxc's `xc_hyb_type` int contract, or do an explicit `match` mapping in compat).
- Wrap `Functional::cam_coefficients()` (`hybrid.rs:112-150`) — write `omega/alpha/beta` through three `*mut f64` output pointers (NULL → skip).
- Wrap `Functional::exx_coefficient()` (`hybrid.rs:97-102`) — single `f64` return, no out-pointer.
- Wrap `auxiliary_functionals()` / `mix_coefficients()` (`hybrid.rs:165-175`) — slice-out-via-pointer-write pattern.
**Code excerpt** (`src/functional/hybrid.rs:84-150` — the full wrappee surface, condensed):
```rust
impl Functional {
    pub fn hybrid_type(&self) -> HybridType { self.meta.hybrid_type }

    pub fn exx_coefficient(&self) -> Option<f64> {
        if self.hybrid_type() != HybridType::Hybrid { return None; }
        self.meta.hybrid_terms.first().map(|t| t.coefficient)
    }

    pub fn cam_coefficients(&self) -> Option<CamCoefficients> {
        let terms = self.meta.hybrid_terms;
        match terms.len() {
            1 => { /* ... */ }
            2 => { /* ... */ }
            _ => None,
        }
    }

    pub fn nlc_coefficients(&self) -> Option<NlcCoefficients> {
        self.meta.nlc_params.map(|(b, c)| NlcCoefficients { b, c })
    }

    pub fn auxiliary_functionals(&self) -> &[Functional] { &self.auxiliaries }
    pub fn mix_coefficients(&self) -> &[f64] { &self.mix_coefficients }
}
```
**Differences for Phase 6:**
- `HybridType` enum from `src/model/mod.rs:50-58` has no `#[repr(...)]` annotation — compat must provide the int mapping (probably as a `fn hybrid_type_to_int(t: HybridType) -> i32` helper).
- `*mut f64` writes use `unsafe { ptr.write(val) }` with NULL guard — net-new pattern.

---

### `src/compat/library.rs` (NEW)
**Role:** The 5 library-info functions (`xc_version`, `xc_version_string`, `xc_reference`, `xc_reference_doi`, `xc_reference_key`).
**Analog:** `src/registry/mod.rs:79-92` — already has `version()`, `version_string()`, `reference_string()`. This is the cleanest 1:1 wrap in the entire phase.
**Pattern to replicate:**
- Existing const-tuple return: `version() -> (u32, u32, u32) = (7, 0, 0)` becomes the C function `void xc_version(int *major, int *minor, int *micro)` writing to the three out-pointers (with NULL guards per RESEARCH § Pattern 3).
- `version_string()` returns `&'static str = "7.0.0"`. The C function returns `*const c_char` from a `const CStr` (no thread-local needed — the &'static input has 'static lifetime so a single `static VERSION_CSTR: &CStr = c"7.0.0";` works).
**Code excerpt** (`src/registry/mod.rs:79-92` — the wrappee):
```rust
pub fn version() -> (u32, u32, u32) {
    (7, 0, 0)
}

pub fn version_string() -> &'static str {
    "7.0.0"
}

pub fn reference_string() -> &'static str {
    "libxc_rs: Rust reimplementation of libxc 7.0.0"
}
```
**Differences for Phase 6:**
- `xc_reference_doi` / `xc_reference_key` are not yet in registry — Phase 6 either adds them as `pub fn reference_doi() -> &'static str` and `pub fn reference_key() -> &'static str` siblings, or hardcodes them inline.
- `static FOO: &CStr = c"...";` is the cleanest no-allocation approach (Rust 2024 supports the `c"..."` literal syntax — confirms with the project's Edition 2024 setting in CLAUDE.md).

---

### `src/compat/errno.rs` (NEW)
**Role:** Thread-local errno cell (`Option<CString>` + `i32` discriminant) plus public extern "C" accessors `xc_rs_last_error_code()` and `xc_rs_last_error_message()`. Internal `set_error()` and `discriminant()` helpers consumed by `extern_c_wrapper!`.
**Analog:** `src/error/mod.rs` is the **wrappee** (the source of `LibxcRsError` variants and their text); the thread-local + discriminant mapping itself is **net-new**.
**Pattern to replicate:**
- Match-arm-per-variant pattern for `discriminant(&LibxcRsError) -> i32` mirrors the test-cases in `src/error/mod.rs:121-222` (every variant gets a tag).
- `LibxcRsError::Display` text is already provided by `#[error("...")]` (`src/error/mod.rs:5-119`) — `set_error(code, &e.to_string())` reuses the thiserror-generated message.
- Fallback: when the cell is empty, `xc_rs_last_error_message()` returns a static empty `&CStr` so the caller never sees a NULL pointer.
**Code excerpt** (`src/error/mod.rs:1-120` — the wrappee, all 18 variants share the `#[error("...")]` shape):
```rust
#[derive(Debug, thiserror::Error)]
pub enum LibxcRsError {
    #[error("unknown functional ID: {0}")]
    UnknownFunctionalId(u16),

    #[error("removed functional ID {removed_id}; use {replacement_id} ({replacement_name}) instead")]
    RemovedFunctionalId { /* ... */ },

    #[error("functional {id} does not support derivative order {order:?} (max: {max:?})")]
    UnsupportedDerivativeOrder { /* ... */ },

    #[error("input buffer '{field}' size mismatch: expected {expected}, got {actual}")]
    InputBufferSizeMismatch { /* ... */ },

    // ... 14 more variants
}
```
**Differences for Phase 6:**
- New variants to add (`error/mod.rs` extension): `BatchOverflow { requested, capacity }`, `UninitializedHandle`, `Panicked { message: String }`. (RESEARCH § Wave 0 line 820.) Use the existing variant style verbatim.
- `discriminant(&self) -> i32` method on `LibxcRsError` (CONTEXT § Specifics line 246-247): document the negative-int contract in the same comment block as `error/mod.rs:5` "thiserror v2 at the library boundary."
- `thread_local! { static LAST_ERROR: RefCell<Option<(i32, CString)>> = RefCell::new(None); }` — net-new in this codebase.

---

### `src/compat/removed.rs` (REWRITE)
**Role:** Surface "removed functional" errors through the int errno mechanism, exposing the replacement-id payload.
**Analog:** `src/registry/removed.rs` (the static data) + `src/registry/mod.rs:13-29` (the consumer that constructs `LibxcRsError::RemovedFunctionalId`).
**Pattern to replicate:**
- Removed-ID logic already lives in `src/registry/mod.rs:13-29`; `compat/removed.rs` just exposes the same error path through the int errno layer (no re-implementation needed).
- The `RemovedFunctionalId { removed_id, replacement_id, replacement_name }` variant from `src/error/mod.rs:8-13` already carries everything the C caller needs — the discriminant maps to `LIBXC_RS_REMOVED_ID` and the message (via `Display`) includes the replacement name.
**Code excerpt** (`src/registry/removed.rs:5-7` — the static data, never duplicated):
```rust
pub(crate) static REMOVED_IDS: &[(u16, u16)] = &[
    (104, 0),
];
```
**Code excerpt** (`src/registry/mod.rs:13-29` — the consumer to mirror in compat):
```rust
if let Some(&(_, replacement_id)) = removed::REMOVED_IDS.iter().find(|&&(r, _)| r == id) {
    let replacement_name = if replacement_id > 0 {
        by_id::REGISTRY_BY_ID.get(replacement_id as usize)
            .and_then(|opt| opt.as_ref())
            .map(|m| m.name)
            .unwrap_or("unknown")
    } else { "none" };
    return Err(LibxcRsError::RemovedFunctionalId {
        removed_id: id, replacement_id, replacement_name,
    });
}
```
**Differences for Phase 6:**
- The work here is small: `compat/removed.rs` becomes either (a) a stub that re-exports `compat::ids::xc_functional_get_number` for callers that want to discover the replacement, or (b) one helper `fn removed_id_replacement(id: u16) -> Option<(u16, &'static str)>` that the errno layer can call to enrich the thread-local message. CONTEXT § Discretion (last bullet) leaves the choice open.

---

### `src/error/mod.rs` (EXTEND)
**Role:** Add `BatchOverflow`, `UninitializedHandle`, `Panicked` variants and a `discriminant() -> i32` method.
**Analog:** itself — the existing 18-variant enum sets the pattern (file is 222 lines, all variants follow `#[error("...")]` + struct fields + `Display` test).
**Pattern to replicate:**
- Each new variant: `#[error("...")]` line, then either a tuple form or a struct form with named fields. Pick struct form for variants with >1 field (mirrors `BatchOverflow { requested, capacity }`'s shape against existing `ExtParamCountMismatch { id, expected, actual }` at `error/mod.rs:58-63`).
- Each new variant gets a Display test at the bottom of the file (mirror `error/mod.rs:125-220`).
- The new `discriminant(&self) -> i32` method goes in an `impl LibxcRsError` block at the bottom of the file (style precedent: `src/registry/mod.rs:79-87` — short pure-fn methods grouped after the data type).
**Code excerpt** (`src/error/mod.rs:58-63` — the precedent for "new variant with named fields"):
```rust
#[error("external parameter count mismatch for {id}: expected {expected}, got {actual}")]
ExtParamCountMismatch {
    id: FunctionalId,
    expected: usize,
    actual: usize,
},
```
**Differences for Phase 6:**
- `discriminant(&self) -> i32` is the new method; it's a `match self` returning a per-variant negative integer. Document the C-ABI mapping in the docstring (per CONTEXT § Specifics line 246-247).
- `Panicked { message: String }` is the only variant with a heap-allocated payload (mirror the `String` field of `UnknownFunctionalName(String)` at `error/mod.rs:15-16`). Document that the message can be lossy if the panic payload isn't `String` / `&str` (RESEARCH § Pattern 2 lines 395-401).

---

### `include/xc.h` (NEW)
**Role:** C header file — hand-written, ~250 lines, ~100 declarations covering the ~85-function FFI surface.
**Analog:** **NONE in libxc_rs.** The **reference** is `libxc-master/src/xc.h` (the file Phase 6 mirrors at the source level). RESEARCH § Recommended Project Structure lines 269-270 places this at `include/xc.h`.
**Pattern to replicate:**
- Comment style: top-of-file MPL-2.0 boilerplate from `libxc-master/src/xc.h:1-7`. Include guards `#ifndef _XC_H ... #define _XC_H ... #endif`. `extern "C" { ... }` for C++ compatibility.
- Constant macros (`XC_UNPOLARIZED`, `XC_FAMILY_LDA`, `XC_FLAGS_HAVE_EXC`, ...) lifted verbatim from `libxc-master/src/xc.h:30-66` so source-compatible C callers Just Work.
- Forward decl `typedef struct xc_func_type xc_func_type;` and `typedef struct xc_func_info_type xc_func_info_type;` (D-A1-1 / D-A1-4: opaque).
- One declaration per extern "C" function in `compat/`. Group by category: lifecycle, discovery, info, evaluation × 3 families × 5 derivative orders, hybrid, library, errno.
**Code excerpt** (`libxc-master/src/xc.h:1-66` — the *reference* shape, condensed):
```c
/* Copyright (C) 2006-2007 M.A.L. Marques
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 */
#ifndef _XC_H
#define _XC_H
#ifdef __cplusplus
extern "C" {
#endif

const char *xc_reference();
void xc_version(int *major, int *minor, int *micro);
const char *xc_version_string();

#define XC_UNPOLARIZED          1
#define XC_POLARIZED            2
#define XC_FAMILY_LDA           1
#define XC_FAMILY_GGA           2
#define XC_FAMILY_MGGA          4
#define XC_FLAGS_HAVE_EXC         (1 <<  0)
#define XC_FLAGS_HAVE_VXC         (1 <<  1)
// ...
```
**Differences for Phase 6:**
- All `void` returns from libxc become `int` returns (D-A4-1) — the **one** signature-level departure from strict drop-in. Document this loudly at the top of the file ("This header is a SOURCE-level drop-in for libxc 7.0.0; binary-level compatibility is intentionally not pursued").
- New declarations not in libxc: `int xc_rs_last_error_code(void);` and `const char *xc_rs_last_error_message(void);` (the errno accessors, D-A4-1).
- Discriminant table for the int return codes belongs at the top of the file as `#define LIBXC_RS_OK 0`, `#define LIBXC_RS_PANIC -1`, etc. (CONTEXT § Specifics line 246).

---

### `verify/tests/compat_smoke.rs` (NEW)
**Role:** Minimum-viable Rust integration test that exercises the full FFI lifecycle: alloc → init → evaluate → end → free, comparing the result against the typed-API `Functional::evaluate_lda` path.
**Analog:** `verify/tests/lda_oracle.rs` — same harness shape (per-functional row + comparator + parametrized loop). Existing `verify/src/lib.rs:20-37` is the **inverse** (it calls libxc's FFI; compat_smoke calls libxc_rs's FFI).
**Pattern to replicate:**
- File-level docstring matches `verify/tests/lda_oracle.rs:1-22` (purpose, tolerance tiers, skip behaviour, mod-level fix notes).
- `struct FunctionalTestCase { id: i32, name: &'static str }` (same row shape as `lda_oracle.rs:35-38`).
- One test function per scenario: lifecycle round-trip, evaluate-vs-typed-API parity, NULL-output-skip, deferred-id error path, panic-via-uninitialized-handle, threshold setter round-trip. Each scenario is a single `#[test]`.
- Tolerance comparison: reuse the `rel_err_with_floor` helper convention from `lda_oracle.rs` if oracle parity is being checked; otherwise direct bit-equality `assert_eq!(zk_ffi[i], zk_typed[i])` because this test compares *libxc_rs to itself*, not to the C oracle.
**Code excerpt** (`verify/tests/lda_oracle.rs:35-50` — the harness layout):
```rust
struct FunctionalTestCase {
    id: i32,
    name: &'static str,
}

const LDA_FUNCTIONALS: &[FunctionalTestCase] = &[
    FunctionalTestCase { id: 1, name: "lda_x" },
    FunctionalTestCase { id: 2, name: "lda_c_wigner" },
    // ...
];
```
**Code excerpt** (`verify/src/lib.rs:20-37` — the FFI-call shape to mirror, but pointing at *libxc_rs::compat* instead of *libxc-sys*):
```rust
pub fn oracle_func_flags(func_id: i32, spin: i32) -> Option<i32> {
    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() { return None; }
        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            return None;
        }
        let info = (*func).info;
        let flags = if info.is_null() { 0 } else { (*info).flags as i32 };
        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
        Some(flags)
    }
}
```
**Differences for Phase 6:**
- Imports come from `libxc_rs::compat::*` (the new extern "C" surface) instead of `libxc_sys::*` (the verify-only oracle bindings). The function signatures are bit-identical; only the symbol-resolution target changes.
- `unsafe` blocks are net-new in `verify/tests/` for the libxc_rs-side FFI surface — the existing `lda_oracle.rs` is fully safe Rust; oracle FFI lives in `verify/src/lib.rs`.
- A small set of tests (3-5) is sufficient — RESEARCH § Validation Architecture lines 794-806 lists the 8 categories; the smoke test covers categories 1, 2, 4, 8 (lifecycle + evaluate parity + NULL handling + errno round-trip).

---

## Shared Patterns

These cross-cut multiple Phase 6 files; the planner should apply each in every relevant plan.

### Shared Pattern 1: thiserror v2 + typed `LibxcRsError` at the boundary
**Source:** `src/error/mod.rs:1-120`
**Apply to:** every public Rust API (`api/builder.rs`, `api/batch.rs`, `api/evaluate.rs`) and every result-returning compat helper (`compat/raw_handle.rs::as_initialized`, `compat/legacy_eval.rs` evaluation builders).
**Rule:** every fallible function returns `Result<T, LibxcRsError>`; never `String` or `Box<dyn Error>`. Errors propagate via `?`; the FFI layer is the **only** place that converts to `i32`.

### Shared Pattern 2: `Send + Sync` enforcement at the type level
**Source:** `src/functional/mod.rs:78-89` — `assert_send_sync::<Functional>()` compile-test.
```rust
#[cfg(test)]
mod thread_safety_tests {
    use super::Functional;
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn functional_is_send_sync() {
        assert_send_sync::<Functional>();
    }
}
```
**Apply to:** `BatchEvaluator` (it owns a `Vec<f64>` workspace; should be `Send + Sync`), `FunctionalBuilder` (intermediate state; trivially `Send + Sync`), `FunctionalSlot` (owns `Functional`; *should* be `Send + Sync`, but the C-ABI handle is *not* — RESEARCH Pitfall 11). Every new struct gets the same compile-time assertion test.

### Shared Pattern 3: Per-fn `# Errors` doc-comment block
**Source:** `src/functional/config.rs:62-64`
```rust
/// Bulk-set all ext_params. Length must match `meta.ext_params.len()`.
/// # Errors
/// `ExtParamCountMismatch` if `vals.len() != meta.ext_params.len()`.
pub fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> { /* ... */ }
```
**Apply to:** every new `pub fn` in `api/` and every internal helper in `compat/` that returns `Result`. The rustdoc `# Errors` heading is consistent across the codebase; Phase 6 must not break this convention.

### Shared Pattern 4: Layout assertions via `const _: () = assert!(...)`
**Source:** RESEARCH § Pattern 1 line 305 (`const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);`); in-tree precedent at `src/model/mod.rs:178-196` (runtime `assert_eq!(Family::Lda as u8, 1)` in tests).
**Apply to:** `compat/c_layout.rs` (zero-size assertion on `xc_func_type` and `xc_func_info_type`), `compat/raw_handle.rs` (size/align of `FunctionalSlot` if needed for `ptr::write` correctness).
**Rule:** prefer compile-time assertion over runtime test; only fall back to `#[test]` when the assertion depends on a constant the Rust compiler doesn't `const`-evaluate.

### Shared Pattern 5: Auto-generated header ("DO NOT EDIT") banner
**Source:** `src/registry/by_id.rs:1` ("//! Auto-generated by xtask generate-registry. DO NOT EDIT."); `src/registry/removed.rs:1`; `src/meta/generated.rs:1` (etc.).
**Apply to:** any Phase 6 file that the planner chooses to generate via xtask (CONTEXT § Discretion bullets allow either codegen or hand-write for the 83 extern "C" bodies and the C header).
**Rule:** generated files start with the exact "Auto-generated by xtask <subcommand>. DO NOT EDIT." comment so future readers don't hand-edit them.

---

## No Analog Found

Files where the closest match is libxc itself (consumer-side via libxc-sys) rather than an in-tree precedent. The planner should reference RESEARCH.md § Pattern 1-4 (lines 277-528) for the exact shapes; this PATTERNS.md captures the *non-FFI* analogs the planner should layer on top.

| File | Role | Reason |
|------|------|--------|
| `src/compat/c_layout.rs` | FFI types | No `#[repr(C)]` exists in production tree; `#[repr(u8)]` enums in `model/mod.rs` are the closest discriminant-stable type pattern. |
| `src/compat/raw_handle.rs` | FFI lifecycle | No `Box::into_raw`/`Box::from_raw` exists in src/. The opaque-handle pattern is documented at the design-doc level (CONTEXT § Specifics + RESEARCH § Pattern 1). |
| `include/xc.h` | C header | No C header exists in libxc_rs. Reference is `libxc-master/src/xc.h`; copy declaration shape but not declarations themselves (Phase 6 has its own opaque-only surface). |

---

## Metadata

**Analog search scope:** `/home/user/Documents/workspace/libxc_rs/src/`, `/home/user/Documents/workspace/libxc_rs/verify/`, `/home/user/Documents/workspace/libxc_rs/xtask/`, `/home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h` (reference only).
**Files scanned:** 14 Phase 6 targets + 18 in-tree analogs read in full or excerpt (lifecycle, config, evaluate, hybrid, params, mod for `functional/`; mod for `error/`, `registry/`, `meta/`, `model/`, `input/`, `output/`, `eval/workspace`, `eval/dispatch`, `kernel/launch`, `lib`; `verify/tests/lda_oracle.rs`, `verify/src/lib.rs`).
**Pattern extraction date:** 2026-05-06

## PATTERN MAPPING COMPLETE

Phase 6 is a wrapping phase: 11 of the 14 new/modified files have direct in-tree analogs that Phase 6 should mirror line-for-line — typed-error discipline (`error/mod.rs`), workspace-owning evaluator shape (`eval/workspace.rs`), per-family evaluate methods (`functional/evaluate.rs`), hybrid getter API (`functional/hybrid.rs`), registry lookups (`registry/mod.rs`), and the verify-tests harness layout (`verify/tests/lda_oracle.rs`). The other 3 files — `compat/c_layout.rs`, `compat/raw_handle.rs`, and `include/xc.h` — introduce net-new patterns (`#[repr(C)]` opaque types, `Box::into_raw` lifecycle, hand-written C header) that have no in-tree precedent; for those, RESEARCH.md § Pattern 1-2 (lines 277-419) supplies the exact shapes to copy. The single declarative-macro convention (`extern_c_wrapper!` in `compat/macros.rs`) is the bridge between the two worlds — its outer `macro_rules!` shape mirrors `eval/dispatch.rs::ten_arm_dispatch!` line-for-line, while its body (`catch_unwind` + thread-local errno) is fully net-new and confined to a single audit-able file. With this mapping, the planner can route every Phase 6 plan to a concrete analog file plus a 5-20 line excerpt, and every `unsafe` block in the implementation has a documented justification.
