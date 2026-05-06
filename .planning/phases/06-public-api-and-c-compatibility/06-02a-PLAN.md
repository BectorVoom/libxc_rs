---
phase: 06-public-api-and-c-compatibility
plan: 02a
type: execute
wave: 2
depends_on: ["06-01"]
files_modified:
  - src/compat/mod.rs
  - src/compat/c_layout.rs
  - src/compat/raw_handle.rs
  - src/compat/macros.rs
  - src/compat/errno.rs
  - src/compat/legacy_eval.rs
  - src/error/mod.rs
  - src/functional/config.rs
  - src/lib.rs
autonomous: true
requirements: [COMPAT-01, COMPAT-02, COMPAT-03]
tags: [ffi, extern-c, opaque-handle, lifecycle, errno, catch-unwind, threshold-aux-propagation, ext-params-default, discriminant-mapping]

must_haves:
  truths:
    - "C caller can run xc_func_alloc → xc_func_init(p, 1, 1) → xc_func_end(p) → xc_func_free(p) without leaking memory and without UB"
    - "C caller calling xc_func_init twice on the same handle drops the first Functional before installing the second (no leak — Pitfall 1)"
    - "Caller calling any extern C function with NULL xc_func_type* gets LIBXC_RS_NULL_HANDLE errno (negative int return), not a segfault"
    - "Forcing a panic inside a compat shim returns LIBXC_RS_PANIC errno (-1) and never propagates UB across the FFI boundary"
    - "xc_rs_last_error_code() / xc_rs_last_error_message() round-trip the most recent thread-local error after any non-ok return"
    - "xc_func_set_dens_threshold(b3lyp_p, 1e-12) propagates the threshold to all 4 auxiliary functionals of B3LYP (Pitfall 4 fix)"
    - "xc_func_set_ext_params(p, vals) substitutes any value equal to -999998888.0 with that ext_param's default_value (Pitfall 10)"
    - "xc_func_init with nspin ∉ {1, 2} returns LibxcRsError::InvalidSpin(nspin) errno (no placeholder SpinMismatch)"
    - "compat::c_layout::xc_func_type has size_of == 0 (compile-asserted) and the discriminant constants Family::Lda as i32 == 1 etc. match libxc's XC_FAMILY_* values"
    - "LibxcRsError::discriminant() returns a unique negative integer for every one of the 25 variants — no `_ => -99` catch-all arm and no fallback `_ => -N` arm of any kind (the discriminant fn is total + exhaustive)"
    - "cache_cstring(s: &'static str) returns a thread-local pointer that remains stable across ≥ 649 distinct insertions (HashMap-keyed, not single-slot)"
    - "All `unsafe` blocks introduced in this plan live exclusively under src/compat/* — verified by grep gate"
  artifacts:
    - path: "src/compat/c_layout.rs"
      provides: "Opaque #[repr(C)] xc_func_type / xc_func_info_type / func_reference_type + repr-C constant assertions"
      contains: "pub struct xc_func_type"
      contains: "const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0)"
      min_lines: 60
    - path: "src/compat/raw_handle.rs"
      provides: "FunctionalSlot enum + xc_func_alloc/init/end/free/get_info + as_initialized accessors"
      contains: "enum FunctionalSlot"
      contains: "extern \"C\" fn xc_func_alloc"
      contains: "extern \"C\" fn xc_func_init"
      contains: "extern \"C\" fn xc_func_end"
      contains: "extern \"C\" fn xc_func_free"
      contains: "extern \"C\" fn xc_func_get_info"
      min_lines: 140
    - path: "src/compat/macros.rs"
      provides: "extern_c_wrapper! declarative macro for catch_unwind + errno + int return"
      contains: "macro_rules! extern_c_wrapper"
      min_lines: 70
    - path: "src/compat/errno.rs"
      provides: "Thread-local errno cell + cache_cstring HashMap + xc_rs_last_error_* + 25-code constants table"
      contains: "thread_local!"
      contains: "extern \"C\" fn xc_rs_last_error_code"
      contains: "extern \"C\" fn xc_rs_last_error_message"
      contains: "pub fn cache_cstring"
      contains: "HashMap"
      min_lines: 140
    - path: "src/compat/legacy_eval.rs"
      provides: "4 threshold setters + 5 ext_params setters/getters; 33 evaluate fns added in 06-03"
      contains: "extern \"C\" fn xc_func_set_dens_threshold"
      contains: "extern \"C\" fn xc_func_set_zeta_threshold"
      contains: "extern \"C\" fn xc_func_set_sigma_threshold"
      contains: "extern \"C\" fn xc_func_set_tau_threshold"
      contains: "extern \"C\" fn xc_func_set_ext_params"
      contains: "extern \"C\" fn xc_func_get_ext_params"
      contains: "extern \"C\" fn xc_func_set_ext_params_name"
      contains: "extern \"C\" fn xc_func_get_ext_params_name"
      contains: "extern \"C\" fn xc_func_get_ext_params_value"
      contains: "LIBXC_EXT_PARAMS_DEFAULT"
    - path: "src/error/mod.rs"
      provides: "discriminant(&self) -> i32 method on LibxcRsError mapping every variant to a unique negative integer (25 variants total — see <interfaces> table)"
      contains: "pub fn discriminant"
      contains: "InvalidSpin"
    - path: "src/functional/config.rs"
      provides: "Pitfall 4 fix: set_density/zeta/sigma/tau_threshold walk self.auxiliaries recursively"
      contains: "self.auxiliaries.iter_mut()"
    - path: "src/lib.rs"
      provides: "Adds `pub mod compat;` declaration (compat directory exists but is unwired today) plus `pub use compat::{xc_func_type, xc_func_info_type};`"
      contains: "pub mod compat"
      contains: "pub use compat::"
  key_links:
    - from: "src/compat/raw_handle.rs::xc_func_init"
      to: "src/functional/lifecycle.rs::Functional::new"
      via: "Functional::new(FunctionalId::from_raw(functional as u16)?, spin)? written into FunctionalSlot::Initialized"
      pattern: "FunctionalSlot::Initialized\\(Functional::new"
    - from: "src/compat/macros.rs::extern_c_wrapper!"
      to: "src/compat/errno.rs::set_error"
      via: "set_error(code, &error.to_string()) on Err or panic"
      pattern: "errno::set_error"
    - from: "src/compat/errno.rs::discriminant"
      to: "src/error/mod.rs::LibxcRsError::discriminant"
      via: "Match-arm-per-variant (exhaustive, no `_` arm) returning negative i32"
      pattern: "fn discriminant"
    - from: "src/functional/config.rs::set_density_threshold"
      to: "self.auxiliaries[i].set_density_threshold"
      via: "Recursive fanout — Pitfall 4 fix"
      pattern: "auxiliaries\\.iter_mut.*set_density_threshold"
    - from: "src/compat/legacy_eval.rs::xc_func_set_ext_params"
      to: "Pitfall 10 substitution: vals[i] == LIBXC_EXT_PARAMS_DEFAULT → meta.ext_params[i].default_value"
      via: "Pre-loop substitution before forwarding to f.set_ext_params"
      pattern: "LIBXC_EXT_PARAMS_DEFAULT"
---

<objective>
Build the compat-layer infrastructure that 06-02b (accessors) and 06-03 (evaluators) depend on: opaque types, lifecycle (alloc/init/end/free/get_info), the `extern_c_wrapper!` panic+errno macro, thread-local errno + 25-code discriminant table, the HashMap-keyed `cache_cstring` helper, the 4 threshold setters, and the 5 ext_params setters/getters. Plus the Phase-5 Pitfall 4 fix (threshold setters walk auxiliaries) and the Pitfall 10 fix (XC_EXT_PARAMS_DEFAULT magic substitution) and the new `LibxcRsError::InvalidSpin` consumer in `xc_func_init`.

Purpose: Phase 6 wraps the Phase-5 `Functional` runtime handle in a Layer-1 C ABI. This plan ships the Wave-2 infrastructure layer; 06-02b adds the read-only accessors (discovery / info / library / hybrid / removed / AK13) in Wave 3, and 06-03 adds the 35 evaluate functions + the C header in Wave 4. Every extern "C" function in this plan uses the same `extern_c_wrapper!` macro for uniform NULL handling + `catch_unwind` + thread-local errno. The opaque-pointer pattern (`Box<FunctionalSlot>` behind `*mut xc_func_type`) keeps all `unsafe` confined to `src/compat/*`.

Output: 6 files under `src/compat/` (5 modules + 1 macros file) plus the partial `compat::legacy_eval` (threshold + ext_params setters; the 35 evaluate fns come in 06-03). 1 method addition on `LibxcRsError` (`discriminant() -> i32`, exhaustive, 25 variants). 4 modified threshold setters in `src/functional/config.rs`. `pub mod compat;` + `pub use compat::*;` re-export added to `src/lib.rs`. Approximately **22-25 extern "C" functions** exported from the cdylib in this plan (5 lifecycle + 4 thresholds + 5 ext_params + 2 errno = 16 — plus the supporting `pub fn cache_cstring` and `pub fn discriminant` helpers used by 06-02b/06-03; see Specifics).

This plan executes in Wave 2; 06-02b follows in Wave 3 because they share `src/compat/mod.rs` and the macros module. 06-01 (Wave 1) must ship first — this plan extends `src/error/mod.rs` (touched by 06-01-T1) and `src/lib.rs` (touched by 06-01-T2).
</objective>

<execution_context>
@/home/user/Documents/workspace/libxc_rs/.claude/get-shit-done/workflows/execute-plan.md
@/home/user/Documents/workspace/libxc_rs/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@/home/user/Documents/workspace/libxc_rs/.planning/PROJECT.md
@/home/user/Documents/workspace/libxc_rs/.planning/ROADMAP.md
@/home/user/Documents/workspace/libxc_rs/.planning/STATE.md
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-CONTEXT.md
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-PATTERNS.md
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-VALIDATION.md
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-01-PLAN.md
@/home/user/Documents/workspace/libxc_rs/CLAUDE.md
@/home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h

<interfaces>
<!-- Phase-5 wrappee surface (frozen except for the documented Pitfall 4 fix) -->

From src/functional/lifecycle.rs:
```rust
impl Functional { pub fn new(id: FunctionalId, spin: Spin) -> Result<Self, LibxcRsError>; }
```

From src/functional/config.rs (THIS PLAN MODIFIES THE 4 THRESHOLD SETTERS to walk auxiliaries):
```rust
impl Functional {
    pub fn set_density_threshold(&mut self, v: f64);  // current: self.thresholds.density = v;
    pub fn set_zeta_threshold(&mut self, v: f64);
    pub fn set_sigma_threshold(&mut self, v: f64);
    pub fn set_tau_threshold(&mut self, v: f64);
    pub fn ext_params(&self) -> Option<&[f64]>;
    pub fn ext_param(&self, name: &str) -> Result<f64, LibxcRsError>;
    pub fn ext_param_by_index(&self, idx: usize) -> Result<f64, LibxcRsError>;
    pub fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;
    pub fn set_ext_param(&mut self, name: &str, val: f64) -> Result<(), LibxcRsError>;
    pub fn set_ext_param_by_index(&mut self, idx: usize, val: f64) -> Result<(), LibxcRsError>;
}
```

From src/functional/mod.rs:
```rust
impl Functional {
    pub fn meta(&self) -> &'static FunctionalMeta;
    pub fn spin(&self) -> Spin;
    pub fn thresholds(&self) -> &Thresholds;          // public accessor — used by tests
    pub fn auxiliary_functionals(&self) -> &[Functional];   // immutable; the field src/functional/mod.rs:44 is `pub(crate) auxiliaries: Vec<Functional>` (mutable iter_mut() is the Pitfall 4 fix path)
}
```

From src/model/mod.rs:
```rust
#[repr(u8)] pub enum Family { Lda = 1, Gga = 2, Mgga = 4 }
#[repr(u8)] pub enum Kind   { Exchange = 0, Correlation = 1, ExchangeCorrelation = 2, Kinetic = 3 }
#[repr(u8)] pub enum Spin   { Unpolarized = 1, Polarized = 2 }
pub struct FunctionalId(pub(crate) u16);
impl FunctionalId {
    pub fn raw(self) -> u16;
    pub fn from_raw(raw: u16) -> Result<Self, LibxcRsError>;
}
```

From src/error/mod.rs (after 06-01-T1 — total 25 variants):
```rust
#[derive(Debug, thiserror::Error)]
pub enum LibxcRsError {
    UnknownFunctionalId(u16),
    RemovedFunctionalId { removed_id, replacement_id, replacement_name },
    UnknownFunctionalName(String),
    UnsupportedDerivativeOrder { id, order, max },
    InputBufferSizeMismatch { field, expected, actual },
    OutputBufferSizeMismatch { field, expected, actual },
    FamilyMismatch { id, expected, actual },
    SpinMismatch { expected, actual },
    ExtParamNotFound { id, name },
    ExtParamCountMismatch { id, expected, actual },
    GpuNotAvailable { reason },
    DeviceCapabilityMismatch { device },
    AllBelowThreshold { np, threshold },
    WorkspaceMismatch { expected_np, actual_np, expected_spin, actual_spin },
    KernelLaunchFailed { reason },
    UnsupportedFunctional { id, reason },
    ExtParamIndexOutOfRange { id, index, count },
    UnknownExtParamName { id, name },
    AuxiliaryInitFailed { parent_id, aux_id, source },
    PropagationConflict { id, parent_name, aux_slot, aux_name },
    BatchOverflow { requested, capacity },         // added in 06-01-T1
    UninitializedHandle,                            // added in 06-01-T1
    Panicked { message },                            // added in 06-01-T1
    InvalidSpin(i32),                                // added in 06-01-T1; consumed in 06-02a-T3
}
```

<!-- libxc reference signatures (verified by direct read of libxc-master/src/xc.h) -->

```c
/* Lifecycle (5) */
xc_func_type *xc_func_alloc();
int   xc_func_init(xc_func_type *p, int functional, int nspin);
int   xc_func_end (xc_func_type *p);    /* libxc: void; we change to int per D-A4-1 */
void  xc_func_free(xc_func_type *p);    /* keep void — no error path */
const xc_func_info_type *xc_func_get_info(const xc_func_type *p);

/* Threshold setters (4) — libxc: void; we change to int */
int xc_func_set_dens_threshold(xc_func_type *p, double t_dens);
int xc_func_set_zeta_threshold(xc_func_type *p, double t_zeta);
int xc_func_set_sigma_threshold(xc_func_type *p, double t_sigma);
int xc_func_set_tau_threshold(xc_func_type *p, double t_tau);

/* Ext params (5) */
int    xc_func_set_ext_params      (xc_func_type *p, const double *ext_params);
int    xc_func_get_ext_params      (const xc_func_type *p, double *ext_params);
int    xc_func_set_ext_params_name (xc_func_type *p, const char *name, double par);
double xc_func_get_ext_params_name (const xc_func_type *p, const char *name);
double xc_func_get_ext_params_value(const xc_func_type *p, int number);

/* Errno accessors (2) — libxc_rs-specific per D-A4-1 */
int    xc_rs_last_error_code();
const char *xc_rs_last_error_message();
```

<!-- LibxcRsError::discriminant() — full 25-variant table, exhaustive (no `_` arm) -->

The errno constants below are public from `compat::errno`. They are also `#define`'d into `include/xc.h` by 06-03's header task — that file's LIBXC_RS_* block must mirror this list **verbatim** (cross-plan contract).

```rust
// src/compat/errno.rs constants (this plan creates these)
pub const LIBXC_RS_OK:                              i32 =   0;
pub const LIBXC_RS_PANIC:                           i32 =  -1;
pub const LIBXC_RS_NULL_HANDLE:                     i32 =  -2;
pub const LIBXC_RS_UNINITIALIZED_HANDLE:            i32 =  -3;
pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_ID:           i32 =  -4;
pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_NAME:         i32 =  -5;
pub const LIBXC_RS_REMOVED_FUNCTIONAL_ID:           i32 =  -6;
pub const LIBXC_RS_UNKNOWN_EXT_PARAM_NAME:          i32 =  -7;
pub const LIBXC_RS_EXT_PARAM_INDEX_OUT_OF_RANGE:    i32 =  -8;
pub const LIBXC_RS_EXT_PARAM_COUNT_MISMATCH:        i32 =  -9;
pub const LIBXC_RS_FAMILY_MISMATCH:                 i32 = -10;
pub const LIBXC_RS_SPIN_MISMATCH:                   i32 = -11;
pub const LIBXC_RS_INPUT_BUFFER_SIZE_MISMATCH:      i32 = -12;
pub const LIBXC_RS_OUTPUT_BUFFER_SIZE_MISMATCH:     i32 = -13;
pub const LIBXC_RS_BATCH_OVERFLOW:                  i32 = -14;
pub const LIBXC_RS_UNSUPPORTED_DERIVATIVE_ORDER:    i32 = -15;
pub const LIBXC_RS_UNSUPPORTED_FUNCTIONAL:          i32 = -16;
pub const LIBXC_RS_EXT_PARAM_NOT_FOUND:             i32 = -17;
pub const LIBXC_RS_GPU_NOT_AVAILABLE:               i32 = -18;
pub const LIBXC_RS_DEVICE_CAPABILITY_MISMATCH:      i32 = -19;
pub const LIBXC_RS_ALL_BELOW_THRESHOLD:             i32 = -20;
pub const LIBXC_RS_WORKSPACE_MISMATCH:              i32 = -21;
pub const LIBXC_RS_KERNEL_LAUNCH_FAILED:            i32 = -22;
pub const LIBXC_RS_AUXILIARY_INIT_FAILED:           i32 = -23;
pub const LIBXC_RS_PROPAGATION_CONFLICT:            i32 = -24;
pub const LIBXC_RS_INVALID_SPIN:                    i32 = -25;
```

The `LibxcRsError::discriminant` match below is total — no `_ =>` arm:

```rust
impl LibxcRsError {
    pub fn discriminant(&self) -> i32 {
        match self {
            Self::UnknownFunctionalId(_)            =>  -4,
            Self::RemovedFunctionalId { .. }        =>  -6,
            Self::UnknownFunctionalName(_)          =>  -5,
            Self::UnsupportedDerivativeOrder { .. } => -15,
            Self::InputBufferSizeMismatch { .. }    => -12,
            Self::OutputBufferSizeMismatch { .. }   => -13,
            Self::FamilyMismatch { .. }             => -10,
            Self::SpinMismatch { .. }               => -11,
            Self::ExtParamNotFound { .. }           => -17,
            Self::ExtParamCountMismatch { .. }      =>  -9,
            Self::GpuNotAvailable { .. }            => -18,
            Self::DeviceCapabilityMismatch { .. }   => -19,
            Self::AllBelowThreshold { .. }          => -20,
            Self::WorkspaceMismatch { .. }          => -21,
            Self::KernelLaunchFailed { .. }         => -22,
            Self::UnsupportedFunctional { .. }      => -16,
            Self::ExtParamIndexOutOfRange { .. }    =>  -8,
            Self::UnknownExtParamName { .. }        =>  -7,
            Self::AuxiliaryInitFailed { .. }        => -23,
            Self::PropagationConflict { .. }        => -24,
            Self::BatchOverflow { .. }              => -14,
            Self::UninitializedHandle               =>  -3,
            Self::Panicked { .. }                   =>  -1,
            Self::InvalidSpin(_)                    => -25,
        }
    }
}
```

There are **24 match arms** (one per variant) plus the implicit `()` for any future variant — adding a new variant must produce a compile error. NO `_` arm. The `discriminant_all_variants_unique` test (T1) constructs every variant via a macro and asserts uniqueness.

<!-- libxc XC_EXT_PARAMS_DEFAULT magic constant (Pitfall 10) -->
```rust
pub const LIBXC_EXT_PARAMS_DEFAULT: f64 = -999998888.0;  // libxc-master/src/xc.h:72
```
</interfaces>

</context>

<tasks>

<task id="06-02a-T1" type="auto">
  <name>Task 1: Wave-0 infrastructure — c_layout (opaque types) + macros (extern_c_wrapper!) + errno (thread-local + cache_cstring HashMap + 25-constant table) + LibxcRsError::discriminant() + Pitfall 4 fix in functional/config.rs + module wiring (lib.rs gets `pub mod compat;`)</name>
  <files>
    src/compat/mod.rs,
    src/compat/c_layout.rs,
    src/compat/macros.rs,
    src/compat/errno.rs,
    src/error/mod.rs,
    src/functional/config.rs,
    src/lib.rs
  </files>
  <read_first>
    - /home/user/Documents/workspace/libxc_rs/src/compat/c_layout.rs (current placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/compat/mod.rs (current placeholder — overwrite to declare submodules)
    - /home/user/Documents/workspace/libxc_rs/src/error/mod.rs (read in full — 25 variants total after 06-01-T1; this task adds the exhaustive `discriminant()` match)
    - /home/user/Documents/workspace/libxc_rs/src/functional/config.rs lines 153-167 (current 4 threshold setters — do NOT walk auxiliaries; this task adds the Pitfall 4 fix)
    - /home/user/Documents/workspace/libxc_rs/src/functional/mod.rs lines 30-49 (the `pub(crate) auxiliaries: Vec<Functional>` field; iter_mut() works directly)
    - /home/user/Documents/workspace/libxc_rs/src/functional/hybrid.rs lines 165-175 (immutable accessor)
    - /home/user/Documents/workspace/libxc_rs/src/lib.rs (does NOT currently declare `pub mod compat;`; this task adds it next to the other `pub mod ...` lines)
    - /home/user/Documents/workspace/libxc_rs/src/model/mod.rs (Family/Kind/Spin repr-u8 layout)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h lines 30-102 (XC_FAMILY_*, XC_UNPOLARIZED, XC_POLARIZED, XC_EXCHANGE, XC_FLAGS_HAVE_*, XC_HYB_*, XC_EXT_PARAMS_DEFAULT, XC_MAX_REFERENCES)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/functionals.c lines 400-410 (libxc reference: xc_func_set_dens_threshold walks p->func_aux[i] recursively)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 277-419 (Patterns 1+2)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 595-614 (Pitfall 4)
  </read_first>
  <action>
    This task bundles four tightly-coupled pieces because they cannot ship in partial states: (a) c_layout opaque types, (b) macros that reference c_layout's xc_func_type for NULL checks and errno's `set_error`/`discriminant` for the panic path, (c) errno constants + thread-local + cache_cstring + the LibxcRsError::discriminant() method that errno wraps, (d) the Pitfall 4 fix on Phase-5 threshold setters which is a one-line touch per setter and must ship before 06-02a-T3 wires xc_func_set_dens_threshold. Lifecycle (alloc/init/end/free/get_info) is in T2; threshold + ext_params extern Cs are in T3.

    **Step 1 — `src/compat/c_layout.rs` (REWRITE):**

    ```rust
    //! C-ABI types and layout assertions for the libxc_rs compat layer.
    //!
    //! Per CONTEXT D-A1-1 / D-A1-4: `xc_func_type` and `xc_func_info_type` are
    //! **opaque** at the C boundary. The C header forward-declares
    //! `typedef struct xc_func_type xc_func_type;` and never exposes any field;
    //! the Rust pointer secretly references a `Box<FunctionalSlot>` (for
    //! `xc_func_type`) or `&'static FunctionalMeta` (for `xc_func_info_type`).
    //!
    //! Compile-time assertions guarantee the opaque structs are zero-sized
    //! and that the Rust enum discriminants match libxc's `XC_*` integer constants.

    use crate::model::{Family, Kind, Spin};

    /// Opaque forward-declared functional handle. C callers see `*mut xc_func_type`;
    /// Rust treats it as `*mut FunctionalSlot` after pointer cast.
    #[repr(C)]
    pub struct xc_func_type {
        _opaque: [u8; 0],
        _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
    }

    /// Opaque forward-declared info handle. C callers see `*const xc_func_info_type`;
    /// Rust treats it as `*const FunctionalMeta` after pointer cast.
    #[repr(C)]
    pub struct xc_func_info_type {
        _opaque: [u8; 0],
        _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
    }

    /// Opaque reference-struct handle. C callers see `*const func_reference_type`;
    /// Rust treats it as `*const Reference` after pointer cast.
    #[repr(C)]
    pub struct func_reference_type {
        _opaque: [u8; 0],
        _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
    }

    // --- Compile-time layout assertions ---
    const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);
    const _: () = assert!(std::mem::size_of::<xc_func_info_type>() == 0);
    const _: () = assert!(std::mem::size_of::<func_reference_type>() == 0);

    // libxc XC_FAMILY_* must match Rust enum repr-u8 values.
    const _: () = assert!(Family::Lda  as u8 == 1);
    const _: () = assert!(Family::Gga  as u8 == 2);
    const _: () = assert!(Family::Mgga as u8 == 4);

    // libxc XC_UNPOLARIZED / XC_POLARIZED.
    const _: () = assert!(Spin::Unpolarized as u8 == 1);
    const _: () = assert!(Spin::Polarized   as u8 == 2);

    // libxc XC_EXCHANGE / XC_CORRELATION / XC_EXCHANGE_CORRELATION / XC_KINETIC.
    const _: () = assert!(Kind::Exchange            as u8 == 0);
    const _: () = assert!(Kind::Correlation         as u8 == 1);
    const _: () = assert!(Kind::ExchangeCorrelation as u8 == 2);
    const _: () = assert!(Kind::Kinetic             as u8 == 3);

    /// libxc magic constant per Pitfall 10 (substituted with per-spec default
    /// in `compat::legacy_eval::xc_func_set_ext_params`).
    pub const LIBXC_EXT_PARAMS_DEFAULT: f64 = -999998888.0;

    /// XC_MAX_REFERENCES — libxc-master/src/xc.h
    pub const XC_MAX_REFERENCES: usize = 5;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn opaque_size_zero() {
            assert_eq!(std::mem::size_of::<xc_func_type>(), 0);
            assert_eq!(std::mem::size_of::<xc_func_info_type>(), 0);
            assert_eq!(std::mem::size_of::<func_reference_type>(), 0);
        }

        #[test]
        fn repr_constants_match_libxc() {
            assert_eq!(Family::Lda  as i32, 1);
            assert_eq!(Family::Gga  as i32, 2);
            assert_eq!(Family::Mgga as i32, 4);
            assert_eq!(Spin::Unpolarized as i32, 1);
            assert_eq!(Spin::Polarized   as i32, 2);
            assert_eq!(Kind::Exchange            as i32, 0);
            assert_eq!(Kind::Correlation         as i32, 1);
            assert_eq!(Kind::ExchangeCorrelation as i32, 2);
            assert_eq!(Kind::Kinetic             as i32, 3);
        }

        #[test]
        fn ext_params_default_constant() {
            assert_eq!(LIBXC_EXT_PARAMS_DEFAULT, -999998888.0);
        }
    }
    ```

    **Step 2 — extend `src/error/mod.rs` with the EXHAUSTIVE `discriminant()` method.**

    Use the full 24-arm match from `<interfaces>` above verbatim — every variant of the 24 maps to a unique negative integer in the 25-code table (LIBXC_RS_OK=0 is the success sentinel; not used in this match). NO `_ =>` catch-all of any kind. Adding a new variant later must produce a compile error pointing at this match (which is the goal — "exhaustive" is a feature, not a constraint to work around).

    Add a `discriminant_all_variants_unique` test using a `for_each_variant!` macro that constructs every variant with sentinel field values and asserts (a) discriminant is negative, (b) the set of all 24 codes has exactly 24 distinct values:

    ```rust
    #[cfg(test)]
    mod discriminant_tests {
        use super::*;
        use crate::model::{DerivativeOrder, Family, FunctionalId, Spin};

        /// Construct one of every variant. Sentinel field values are arbitrary.
        fn all_variants() -> Vec<LibxcRsError> {
            let id = FunctionalId::from_raw(1).unwrap();
            vec![
                LibxcRsError::UnknownFunctionalId(0),
                LibxcRsError::RemovedFunctionalId { removed_id: 0, replacement_id: 0, replacement_name: "" },
                LibxcRsError::UnknownFunctionalName(String::new()),
                LibxcRsError::UnsupportedDerivativeOrder { id, order: DerivativeOrder::Exc, max: DerivativeOrder::Exc },
                LibxcRsError::InputBufferSizeMismatch { field: "", expected: 0, actual: 0 },
                LibxcRsError::OutputBufferSizeMismatch { field: "", expected: 0, actual: 0 },
                LibxcRsError::FamilyMismatch { id, expected: Family::Lda, actual: Family::Lda },
                LibxcRsError::SpinMismatch { expected: Spin::Unpolarized, actual: Spin::Unpolarized },
                LibxcRsError::ExtParamNotFound { id, name: String::new() },
                LibxcRsError::ExtParamCountMismatch { id, expected: 0, actual: 0 },
                LibxcRsError::GpuNotAvailable { reason: String::new() },
                LibxcRsError::DeviceCapabilityMismatch { device: String::new() },
                LibxcRsError::AllBelowThreshold { np: 0, threshold: 0.0 },
                LibxcRsError::WorkspaceMismatch { expected_np: 0, actual_np: 0, expected_spin: Spin::Unpolarized, actual_spin: Spin::Unpolarized },
                LibxcRsError::KernelLaunchFailed { reason: String::new() },
                LibxcRsError::UnsupportedFunctional { id, reason: "" },
                LibxcRsError::ExtParamIndexOutOfRange { id, index: 0, count: 0 },
                LibxcRsError::UnknownExtParamName { id, name: String::new() },
                LibxcRsError::AuxiliaryInitFailed { parent_id: id, aux_id: id, source: Box::new(LibxcRsError::UnknownFunctionalId(0)) },
                LibxcRsError::PropagationConflict { id, parent_name: "", aux_slot: 0, aux_name: "" },
                LibxcRsError::BatchOverflow { requested: 0, capacity: 0 },
                LibxcRsError::UninitializedHandle,
                LibxcRsError::Panicked { message: String::new() },
                LibxcRsError::InvalidSpin(0),
            ]
        }

        #[test]
        fn discriminant_all_variants_unique() {
            let codes: Vec<i32> = all_variants().iter().map(|e| e.discriminant()).collect();
            // Every code is negative.
            for (i, c) in codes.iter().enumerate() {
                assert!(*c < 0, "variant index {i} returned non-negative discriminant {c}");
            }
            // All codes are distinct.
            let mut sorted = codes.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), codes.len(),
                "discriminant codes are not unique: codes={codes:?}");
            assert_eq!(codes.len(), 24, "expected 24 LibxcRsError variants in 06-02a; got {}", codes.len());
        }
    }
    ```

    **Step 3 — `src/compat/errno.rs` (NEW):**

    ```rust
    //! Thread-local errno + extern "C" accessors + cache_cstring helper for the libxc_rs compat layer.
    //!
    //! Every fallible extern "C" function on the C ABI returns an `int` (negative
    //! for error). Caller can then call `xc_rs_last_error_code()` /
    //! `xc_rs_last_error_message()` to retrieve the typed discriminant + the
    //! Display-formatted error message for the most recent error on this thread.

    use crate::error::LibxcRsError;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ffi::{c_char, CString};
    use std::pin::Pin;

    // === 25 errno constants — mirror LibxcRsError::discriminant() table ===

    pub const LIBXC_RS_OK:                              i32 =   0;
    pub const LIBXC_RS_PANIC:                           i32 =  -1;
    pub const LIBXC_RS_NULL_HANDLE:                     i32 =  -2;
    pub const LIBXC_RS_UNINITIALIZED_HANDLE:            i32 =  -3;
    pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_ID:           i32 =  -4;
    pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_NAME:         i32 =  -5;
    pub const LIBXC_RS_REMOVED_FUNCTIONAL_ID:           i32 =  -6;
    pub const LIBXC_RS_UNKNOWN_EXT_PARAM_NAME:          i32 =  -7;
    pub const LIBXC_RS_EXT_PARAM_INDEX_OUT_OF_RANGE:    i32 =  -8;
    pub const LIBXC_RS_EXT_PARAM_COUNT_MISMATCH:        i32 =  -9;
    pub const LIBXC_RS_FAMILY_MISMATCH:                 i32 = -10;
    pub const LIBXC_RS_SPIN_MISMATCH:                   i32 = -11;
    pub const LIBXC_RS_INPUT_BUFFER_SIZE_MISMATCH:      i32 = -12;
    pub const LIBXC_RS_OUTPUT_BUFFER_SIZE_MISMATCH:     i32 = -13;
    pub const LIBXC_RS_BATCH_OVERFLOW:                  i32 = -14;
    pub const LIBXC_RS_UNSUPPORTED_DERIVATIVE_ORDER:    i32 = -15;
    pub const LIBXC_RS_UNSUPPORTED_FUNCTIONAL:          i32 = -16;
    pub const LIBXC_RS_EXT_PARAM_NOT_FOUND:             i32 = -17;
    pub const LIBXC_RS_GPU_NOT_AVAILABLE:               i32 = -18;
    pub const LIBXC_RS_DEVICE_CAPABILITY_MISMATCH:      i32 = -19;
    pub const LIBXC_RS_ALL_BELOW_THRESHOLD:             i32 = -20;
    pub const LIBXC_RS_WORKSPACE_MISMATCH:              i32 = -21;
    pub const LIBXC_RS_KERNEL_LAUNCH_FAILED:            i32 = -22;
    pub const LIBXC_RS_AUXILIARY_INIT_FAILED:           i32 = -23;
    pub const LIBXC_RS_PROPAGATION_CONFLICT:            i32 = -24;
    pub const LIBXC_RS_INVALID_SPIN:                    i32 = -25;

    thread_local! {
        static LAST_ERROR: RefCell<Option<(i32, CString)>> = const { RefCell::new(None) };
        // HashMap-keyed cache: per-thread, indexed by `&'static str` (i.e. the
        // FunctionalMeta name slice's pointer + length identity is stable, so
        // hashing/eq on the &str works as expected). Storing `Pin<Box<CString>>`
        // ensures the heap address backing `as_ptr()` does NOT move when the
        // HashMap rehashes (Box's heap allocation is itself stable; Pin just
        // documents the invariant — alternative: store `Box<CString>` and use
        // `as_ptr()`, equivalent semantics).
        static CSTRING_CACHE: RefCell<HashMap<&'static str, Pin<Box<CString>>>> =
            RefCell::new(HashMap::new());
    }

    static EMPTY_CSTRING: &std::ffi::CStr = c"";

    /// Set the thread-local errno code + message. Called by `extern_c_wrapper!`
    /// on every Err / panic path.
    pub fn set_error(code: i32, msg: &str) {
        let cstring = CString::new(msg).unwrap_or_else(|_| {
            // Truncate at first NUL byte (CString::new rejects interior NULs).
            let bytes: Vec<u8> = msg.bytes().take_while(|&b| b != 0).collect();
            CString::new(bytes).unwrap_or_default()
        });
        LAST_ERROR.with(|cell| *cell.borrow_mut() = Some((code, cstring)));
    }

    /// Map a `LibxcRsError` to its C-ABI integer discriminant. Wraps
    /// [`LibxcRsError::discriminant`] for use by the wrapper macro.
    pub fn discriminant(e: &LibxcRsError) -> i32 { e.discriminant() }

    /// Get-or-insert a thread-local CString for a `&'static str` name.
    /// The returned pointer is stable across HashMap rehashes (Box's heap
    /// allocation does not move; only the HashMap's internal pointer-table moves).
    /// Lifetime: valid until the thread exits OR the HashMap is explicitly cleared
    /// (which we never do in production — only in tests).
    ///
    /// Used by `compat::ids::xc_functional_get_name`, `compat::info::xc_func_info_get_*`,
    /// `compat::ids::xc_available_functional_names`, etc.
    pub fn cache_cstring(s: &'static str) -> *const c_char {
        CSTRING_CACHE.with(|cell| {
            let mut map = cell.borrow_mut();
            // Use `entry` to insert if missing.
            let pinned = map
                .entry(s)
                .or_insert_with(|| Pin::new(Box::new(CString::new(s).unwrap_or_default())));
            pinned.as_ref().get_ref().as_ptr()
        })
    }

    /// Retrieve the most recent error code on this thread, or `LIBXC_RS_OK`.
    #[unsafe(no_mangle)]
    pub extern "C" fn xc_rs_last_error_code() -> i32 {
        LAST_ERROR.with(|cell| {
            cell.borrow().as_ref().map(|(code, _)| *code).unwrap_or(LIBXC_RS_OK)
        })
    }

    /// Retrieve the most recent error message on this thread.
    /// Returns a pointer to a thread-local `CString`; valid until the next
    /// error-setting call on this thread. Never returns NULL — when no error
    /// has been recorded, returns a static empty C string.
    #[unsafe(no_mangle)]
    pub extern "C" fn xc_rs_last_error_message() -> *const c_char {
        LAST_ERROR.with(|cell| match cell.borrow().as_ref() {
            Some((_, cstr)) => cstr.as_ptr(),
            None => EMPTY_CSTRING.as_ptr(),
        })
    }

    #[cfg(test)] mod tests {
        use super::*;
        use crate::error::LibxcRsError;

        #[test]
        fn errno_round_trip() {
            set_error(-7, "unknown ext param 'alpha'");
            assert_eq!(xc_rs_last_error_code(), -7);
            unsafe {
                let p = xc_rs_last_error_message();
                let s = std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
                assert!(s.contains("alpha"));
            }
        }

        #[test]
        fn discriminant_uses_libxc_rs_error_method() {
            assert_eq!(discriminant(&LibxcRsError::UnknownFunctionalId(42)),  -4);
            assert_eq!(discriminant(&LibxcRsError::Panicked { message: String::new() }), -1);
            assert_eq!(discriminant(&LibxcRsError::InvalidSpin(7)), -25);
            assert_eq!(discriminant(&LibxcRsError::UninitializedHandle), -3);
        }

        /// Verify the HashMap-keyed cache is stable across ≥ 649 distinct insertions.
        /// Pre-commits the cache shape: single-slot would corrupt under load
        /// (every call to xc_available_functional_names overwrites the prior name).
        #[test]
        fn cache_cstring_holds_649_pointers_stable() {
            std::thread::spawn(|| {
                // Collect 649 distinct &'static str via the registry.
                let names: Vec<&'static str> = crate::registry::all_functional_ids()
                    .filter_map(|fid| crate::registry::lookup_by_id(fid.raw()).ok().map(|m| m.name))
                    .collect();
                assert!(names.len() >= 649, "registry must have ≥ 649 names; got {}", names.len());
                // Insert all 649 (or more) and snapshot pointers.
                let mut ptrs: Vec<*const c_char> = names.iter().map(|n| cache_cstring(n)).collect();
                // Insert one more (forces rehash if HashMap is near load factor).
                let extra: &'static str = "this_is_a_distinct_test_name_for_rehash_safety";
                ptrs.push(cache_cstring(extra));
                // Re-fetch every pointer; must equal the snapshot (Box heap allocation stable).
                for (i, n) in names.iter().enumerate() {
                    let p_now = cache_cstring(n);
                    assert_eq!(p_now, ptrs[i],
                        "pointer for name {n:?} (index {i}) moved across rehash: {:p} -> {:p}",
                        ptrs[i], p_now);
                    // Bonus: pointer still resolves to the right C string.
                    let s = unsafe { std::ffi::CStr::from_ptr(p_now).to_string_lossy() };
                    assert_eq!(s, *n, "C string at cached pointer differs from key");
                }
            }).join().unwrap();
        }
    }
    ```

    **Step 4 — `src/compat/macros.rs` (NEW)** — uses the template from RESEARCH § Pattern 2:

    ```rust
    //! `extern_c_wrapper!` — uniform wrapper for every extern "C" entry point.

    /// Wrap an extern "C" body in NULL-handle check + `catch_unwind` + errno set + i32 return.
    ///
    /// Forms:
    ///   1. With handle: `extern_c_wrapper!(p, "fn_name", { body returning Result<i32, LibxcRsError> })`
    ///   2. No handle:   `extern_c_wrapper!(_, "fn_name", { body })`
    #[macro_export]
    macro_rules! extern_c_wrapper {
        ($p:expr, $name:literal, $body:block) => {{
            if $p.is_null() {
                $crate::compat::errno::set_error(
                    $crate::compat::errno::LIBXC_RS_NULL_HANDLE,
                    concat!($name, ": null xc_func_type pointer"),
                );
                return $crate::compat::errno::LIBXC_RS_NULL_HANDLE;
            }
            $crate::__extern_c_wrapper_body!($name, $body)
        }};
        (_, $name:literal, $body:block) => {{ $crate::__extern_c_wrapper_body!($name, $body) }};
    }

    /// Internal: shared catch_unwind body for both forms.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __extern_c_wrapper_body {
        ($name:literal, $body:block) => {{
            let result: ::std::result::Result<i32, $crate::LibxcRsError> =
                ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $body))
                .unwrap_or_else(|payload| {
                    let msg = if let Some(s) = payload.downcast_ref::<&str>() { (*s).to_string() }
                              else if let Some(s) = payload.downcast_ref::<String>() { s.clone() }
                              else { "unknown panic in libxc_rs compat layer".to_string() };
                    $crate::compat::errno::set_error(
                        $crate::compat::errno::LIBXC_RS_PANIC,
                        &format!("{}: panic — {}", $name, msg),
                    );
                    Err($crate::LibxcRsError::Panicked { message: msg })
                });
            match result {
                Ok(code) => code,
                Err(e) => {
                    let code = $crate::compat::errno::discriminant(&e);
                    $crate::compat::errno::set_error(code, &e.to_string());
                    code
                }
            }
        }};
    }

    #[cfg(test)] mod tests {
        use crate::compat::errno;

        #[unsafe(no_mangle)]
        unsafe extern "C" fn __test_compat_panic_fn() -> i32 {
            crate::extern_c_wrapper!(_, "__test_compat_panic_fn", {
                panic!("test panic from wrapper");
                #[allow(unreachable_code)]
                Ok::<i32, crate::LibxcRsError>(0)
            })
        }

        #[test]
        fn catch_panic_returns_errno() {
            let code = unsafe { __test_compat_panic_fn() };
            assert_eq!(code, errno::LIBXC_RS_PANIC);
            assert_eq!(errno::xc_rs_last_error_code(), errno::LIBXC_RS_PANIC);
            unsafe {
                let p = errno::xc_rs_last_error_message();
                let s = std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
                assert!(s.contains("test panic from wrapper"), "got: {s}");
            }
        }
    }
    ```

    **Step 5 — Pitfall 4 fix in `src/functional/config.rs`:**

    Tests-first. Add (or extend) `#[cfg(test)] mod tests` with four propagation tests:

    ```rust
    #[test]
    fn threshold_propagates_to_aux_density() {
        use crate::functional::Functional;
        use crate::model::{FunctionalId, Spin};
        let id = FunctionalId::from_name("hyb_gga_xc_b3lyp").unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(!f.auxiliary_functionals().is_empty(), "B3LYP must have auxiliaries");
        f.set_density_threshold(1e-12);
        assert_eq!(f.thresholds().density, 1e-12);
        for aux in f.auxiliary_functionals() {
            assert_eq!(aux.thresholds().density, 1e-12,
                "aux {} did not receive threshold", aux.meta().name);
        }
    }
    ```

    Repeat for zeta/sigma/tau (4 tests total). Run — must FAIL. Commit: `test(06-02a): add threshold-aux-propagation failing tests (Pitfall 4)`.

    Then for each of the 4 setters in `src/functional/config.rs:153-167`, change the body from `self.thresholds.X = v;` to:

    ```rust
    pub fn set_density_threshold(&mut self, v: f64) {
        self.thresholds.density = v;
        for aux in self.auxiliaries.iter_mut() {
            aux.set_density_threshold(v);
        }
    }
    ```

    Recursion is naturally depth-first because `aux.set_density_threshold(v)` walks its own auxiliaries. Repeat for zeta/sigma/tau. Re-run tests — must PASS. Commit: `fix(06-02a): threshold setters walk auxiliaries (Pitfall 4)`.

    **Step 6 — wire `src/compat/mod.rs`:**

    ```rust
    //! Layer-1 C ABI compatibility. All `unsafe` lives here per BUILD-04 / COMPAT-03.

    pub mod c_layout;
    pub mod errno;
    #[macro_use]
    pub mod macros;
    pub mod raw_handle;     // populated in 06-02a-T2
    pub mod legacy_eval;    // populated in 06-02a-T3 (setters) + 06-03 (evaluators)
    // Submodules added in 06-02b (Wave 3): ids, info, library, hybrid, removed.

    pub use c_layout::{xc_func_type, xc_func_info_type, func_reference_type, LIBXC_EXT_PARAMS_DEFAULT};
    pub use errno::{xc_rs_last_error_code, xc_rs_last_error_message};
    ```

    **Step 7 — wire `src/lib.rs`:** `src/lib.rs` does NOT currently declare `pub mod compat;` (the compat directory exists but is unwired). Add `pub mod compat;` alongside the existing `pub mod ...` declarations (06-01-T2 already added `pub mod api;`; mirror the placement). Also add `pub use compat::{xc_func_type, xc_func_info_type};` next to the existing api re-export. Do NOT remove or reorder other re-exports.

    **Step 8 — verify:**

    ```bash
    cargo test -p libxc_rs --lib compat::c_layout compat::errno compat::macros error::discriminant_tests functional::config::tests
    cargo build -p libxc_rs --release
    cargo clippy -p libxc_rs --no-deps -- -D warnings
    ```

    All green. Commit: `feat(06-02a): compat infrastructure (c_layout/errno/macros) + 25-variant discriminant + Pitfall 4 + lib.rs wiring`.

    Do NOT implement the 5 lifecycle functions in this task (T2). Do NOT implement threshold/ext_params extern Cs in this task (T3).
  </action>
  <verify>
    <automated>cargo test -p libxc_rs --lib compat::c_layout compat::errno compat::macros error::discriminant_tests functional::config::tests 2>&1 | tail -40</automated>
  </verify>
  <acceptance_criteria>
    - `grep -c '#\[repr(C)\]' src/compat/c_layout.rs` >= 3
    - `grep -c 'pub struct xc_func_type' src/compat/c_layout.rs` == 1
    - `grep -c 'pub struct xc_func_info_type' src/compat/c_layout.rs` == 1
    - `grep -c 'const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0)' src/compat/c_layout.rs` >= 1
    - `grep -c 'pub const LIBXC_EXT_PARAMS_DEFAULT' src/compat/c_layout.rs` == 1
    - `grep -c 'macro_rules! extern_c_wrapper' src/compat/macros.rs` == 1
    - `grep -c 'catch_unwind' src/compat/macros.rs` >= 1
    - `grep -c 'thread_local!' src/compat/errno.rs` >= 1
    - `grep -c 'HashMap' src/compat/errno.rs` >= 1
    - `grep -c 'pub fn cache_cstring' src/compat/errno.rs` == 1
    - `grep -c 'Pin<Box<CString>>' src/compat/errno.rs` >= 1
    - `grep -c 'pub const LIBXC_RS_OK' src/compat/errno.rs` == 1
    - `grep -c 'pub const LIBXC_RS_INVALID_SPIN' src/compat/errno.rs` == 1
    - Errno constants count: `grep -cE '^pub const LIBXC_RS_[A-Z_]+:\s*i32' src/compat/errno.rs` == 26  (LIBXC_RS_OK + 25 negative codes)
    - `grep -c '#\[unsafe\(no_mangle\)\]' src/compat/errno.rs` >= 2
    - `grep -c 'extern "C" fn xc_rs_last_error_code' src/compat/errno.rs` == 1
    - `grep -c 'extern "C" fn xc_rs_last_error_message' src/compat/errno.rs` == 1
    - `grep -c 'pub fn discriminant' src/error/mod.rs` >= 1
    - **Discriminant exhaustiveness gate (TIGHTENED — no fallback arm allowed):** `grep -E '_\s*=>' src/error/mod.rs` against the discriminant fn body returns 0 lines. Practical command: `awk '/pub fn discriminant/,/^    }$/' src/error/mod.rs | grep -cE '_\s*=>'` == 0
    - `cargo test -p libxc_rs --lib error::discriminant_tests::discriminant_all_variants_unique` exits 0
    - `grep -c 'auxiliaries\.iter_mut' src/functional/config.rs` >= 4
    - `grep -c 'aux\.set_density_threshold' src/functional/config.rs` == 1
    - `grep -c 'aux\.set_zeta_threshold' src/functional/config.rs` == 1
    - `grep -c 'aux\.set_sigma_threshold' src/functional/config.rs` == 1
    - `grep -c 'aux\.set_tau_threshold' src/functional/config.rs` == 1
    - `cargo test -p libxc_rs --lib functional::config::tests::threshold_propagates_to_aux_density` exits 0
    - `cargo test -p libxc_rs --lib functional::config::tests::threshold_propagates_to_aux_zeta` exits 0
    - `cargo test -p libxc_rs --lib functional::config::tests::threshold_propagates_to_aux_sigma` exits 0
    - `cargo test -p libxc_rs --lib functional::config::tests::threshold_propagates_to_aux_tau` exits 0
    - `cargo test -p libxc_rs --lib compat::c_layout::tests::opaque_size_zero` exits 0
    - `cargo test -p libxc_rs --lib compat::c_layout::tests::repr_constants_match_libxc` exits 0
    - `cargo test -p libxc_rs --lib compat::errno::tests::errno_round_trip` exits 0
    - `cargo test -p libxc_rs --lib compat::errno::tests::cache_cstring_holds_649_pointers_stable` exits 0
    - `cargo test -p libxc_rs --lib compat::macros::tests::catch_panic_returns_errno` exits 0
    - `grep -c 'pub mod compat;' src/lib.rs` == 1
    - `grep -c 'pub use compat::' src/lib.rs` >= 1
    - `cargo build -p libxc_rs --release` exits 0
    - `cargo clippy -p libxc_rs --no-deps -- -D warnings` exits 0
  </acceptance_criteria>
  <done>
    Opaque types compile-asserted zero-sized; repr-i32 constants match libxc; thread-local errno round-trips; cache_cstring backed by HashMap<&'static str, Pin<Box<CString>>> with stability test for 649+ entries; extern_c_wrapper! catches panics and sets errno; discriminant() maps every 24 LibxcRsError variant to a unique negative int with NO catch-all arm; Pitfall 4 fix in functional/config.rs propagates thresholds to all auxiliaries; src/lib.rs has `pub mod compat;` declaration.
  </done>
</task>

<task id="06-02a-T2" type="auto">
  <name>Task 2: compat::raw_handle — FunctionalSlot enum + 5 lifecycle functions (alloc/init/end/free/get_info), with InvalidSpin handling and Pitfall 1 (re-init drops previous Functional)</name>
  <files>src/compat/raw_handle.rs</files>
  <read_first>
    - /home/user/Documents/workspace/libxc_rs/src/compat/raw_handle.rs (current placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/functional/lifecycle.rs (Functional::new signature)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 277-357 (Pattern 1)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 569-582 (Pitfall 1)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/functionals.c lines 224-510 (lifecycle reference impl)
  </read_first>
  <action>
    Implement FunctionalSlot + 5 lifecycle functions. The `as_initialized_const`/`_mut` accessors are consumed by 06-02a-T3 (threshold/ext_params setters), 06-02b (info/hybrid accessors), and 06-03 (evaluators).

    **Step 1 — `src/compat/raw_handle.rs`** (full file):

    ```rust
    //! Lifecycle (alloc/init/end/free/get_info) and the FunctionalSlot state machine.
    //!
    //! `xc_func_type*` is `Box<FunctionalSlot>::into_raw()` cast to opaque.
    //! All Box::into_raw / Box::from_raw / std::ptr::replace live here.
    //!
    //! Per CONTEXT D-A1-1 / D-A1-2 / D-A1-3.

    use crate::compat::c_layout::{xc_func_type, xc_func_info_type};
    use crate::compat::errno::{self, set_error};
    use crate::extern_c_wrapper;
    use crate::functional::Functional;
    use crate::model::{FunctionalId, Spin};
    use crate::LibxcRsError;

    /// Two-state slot: Empty (allocated but not initialized) or Initialized(Functional).
    /// Re-init replaces the inner Functional, dropping the old one (Pitfall 1).
    #[repr(C)]
    pub enum FunctionalSlot {
        Empty,
        Initialized(Functional),
    }

    impl FunctionalSlot {
        /// Read-only access. SAFETY: caller asserts `p` is valid + non-null
        /// (the wrapper macro NULL-checks before this is called).
        pub(crate) unsafe fn as_initialized_const<'a>(
            p: *const xc_func_type,
        ) -> Result<&'a Functional, LibxcRsError> {
            // SAFETY: caller's contract.
            let slot: &FunctionalSlot = unsafe { &*(p as *const FunctionalSlot) };
            match slot {
                FunctionalSlot::Initialized(f) => Ok(f),
                FunctionalSlot::Empty => Err(LibxcRsError::UninitializedHandle),
            }
        }

        /// Mutable access. Same safety contract.
        pub(crate) unsafe fn as_initialized_mut<'a>(
            p: *mut xc_func_type,
        ) -> Result<&'a mut Functional, LibxcRsError> {
            let slot: &mut FunctionalSlot = unsafe { &mut *(p as *mut FunctionalSlot) };
            match slot {
                FunctionalSlot::Initialized(f) => Ok(f),
                FunctionalSlot::Empty => Err(LibxcRsError::UninitializedHandle),
            }
        }
    }

    // === Lifecycle ===

    /// `xc_func_type *xc_func_alloc();` — allocates an empty slot.
    /// Caller must call `xc_func_free` to release.
    #[unsafe(no_mangle)]
    pub extern "C" fn xc_func_alloc() -> *mut xc_func_type {
        Box::into_raw(Box::new(FunctionalSlot::Empty)) as *mut xc_func_type
    }

    /// `int xc_func_init(xc_func_type *p, int functional, int nspin);`
    /// Initializes the slot; re-init replaces the prior Functional via std::ptr::replace.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_init(p: *mut xc_func_type, functional: i32, nspin: i32) -> i32 {
        extern_c_wrapper!(p, "xc_func_init", {
            if functional < 0 || functional > u16::MAX as i32 {
                return Err(LibxcRsError::UnknownFunctionalId(0));
            }
            let id = FunctionalId::from_raw(functional as u16)?;
            let spin = match nspin {
                1 => Spin::Unpolarized,
                2 => Spin::Polarized,
                other => return Err(LibxcRsError::InvalidSpin(other)),
            };
            let f = Functional::new(id, spin)?;
            // SAFETY: p is non-null (wrapper macro). std::ptr::replace drops the
            // previous slot value, preventing leaks on re-init (Pitfall 1).
            unsafe {
                let _ = std::ptr::replace(p as *mut FunctionalSlot, FunctionalSlot::Initialized(f));
            }
            Ok(0)
        })
    }

    /// `int xc_func_end(xc_func_type *p);` — resets to Empty, dropping the inner Functional.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_end(p: *mut xc_func_type) -> i32 {
        extern_c_wrapper!(p, "xc_func_end", {
            unsafe {
                let _ = std::ptr::replace(p as *mut FunctionalSlot, FunctionalSlot::Empty);
            }
            Ok(0)
        })
    }

    /// `void xc_func_free(xc_func_type *p);` — frees the Box.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_free(p: *mut xc_func_type) {
        if p.is_null() { return; }
        // SAFETY: p obtained from xc_func_alloc (caller contract).
        unsafe { drop(Box::from_raw(p as *mut FunctionalSlot)); }
    }

    /// `const xc_func_info_type *xc_func_get_info(const xc_func_type *p);`
    /// Returns &'static FunctionalMeta cast to *const xc_func_info_type, or NULL on Empty / panic.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_get_info(p: *const xc_func_type) -> *const xc_func_info_type {
        if p.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_func_get_info: null handle");
            return std::ptr::null();
        }
        let result = std::panic::catch_unwind(|| {
            unsafe { FunctionalSlot::as_initialized_const(p) }
                .ok()
                .map(|f| f.meta() as *const crate::meta::FunctionalMeta as *const xc_func_info_type)
        });
        match result {
            Ok(Some(info)) => info,
            Ok(None) => {
                set_error(errno::LIBXC_RS_UNINITIALIZED_HANDLE,
                    "xc_func_get_info: handle uninitialized");
                std::ptr::null()
            }
            Err(_) => {
                set_error(errno::LIBXC_RS_PANIC, "xc_func_get_info: panic");
                std::ptr::null()
            }
        }
    }

    #[cfg(test)] mod tests {
        use super::*;
        use crate::compat::errno::xc_rs_last_error_code;

        #[test]
        fn lifecycle_round_trip() {
            unsafe {
                let p = xc_func_alloc();
                assert!(!p.is_null());
                let rc = xc_func_init(p, 1, 1);  // lda_x unpolarized
                assert_eq!(rc, 0, "init failed: code={}", xc_rs_last_error_code());
                assert_eq!(xc_func_end(p), 0);
                xc_func_free(p);
            }
        }

        #[test]
        fn reinit_drops_previous() {
            unsafe {
                let p = xc_func_alloc();
                assert_eq!(xc_func_init(p, 1, 1), 0);
                assert_eq!(xc_func_init(p, 2, 1), 0);  // re-init: previous lda_x dropped
                let info = xc_func_get_info(p);
                assert!(!info.is_null());
                xc_func_end(p);
                xc_func_free(p);
            }
        }

        #[test]
        fn null_handle_returns_null_handle_errno() {
            unsafe {
                let rc = xc_func_init(std::ptr::null_mut(), 1, 1);
                assert_eq!(rc, errno::LIBXC_RS_NULL_HANDLE);
                assert_eq!(xc_rs_last_error_code(), errno::LIBXC_RS_NULL_HANDLE);
            }
        }

        #[test]
        fn invalid_spin_returns_invalid_spin_errno() {
            unsafe {
                let p = xc_func_alloc();
                let rc = xc_func_init(p, 1, 7);  // nspin=7 invalid
                assert_eq!(rc, errno::LIBXC_RS_INVALID_SPIN,
                    "expected LIBXC_RS_INVALID_SPIN ({}); got {}",
                    errno::LIBXC_RS_INVALID_SPIN, rc);
                assert_eq!(xc_rs_last_error_code(), errno::LIBXC_RS_INVALID_SPIN);
                let msg = xc_rs_last_error_message();
                let s = std::ffi::CStr::from_ptr(msg).to_string_lossy();
                assert!(s.contains("7"), "InvalidSpin message must mention the bad value: {s}");
                xc_func_free(p);
            }
        }

        #[test]
        fn uninitialized_handle_get_info_returns_null() {
            unsafe {
                let p = xc_func_alloc();
                let info = xc_func_get_info(p);
                assert!(info.is_null());
                xc_func_free(p);
            }
        }
    }
    ```

    **Step 2 — verify:** `cargo test -p libxc_rs --lib compat::raw_handle`. Commit: `feat(06-02a): compat::raw_handle lifecycle (alloc/init/end/free/get_info) + InvalidSpin handling + Pitfall 1`.

    Do NOT add threshold/ext_params extern Cs (T3). Do NOT touch info/hybrid/library accessors (06-02b).
  </action>
  <verify>
    <automated>cargo test -p libxc_rs --lib compat::raw_handle 2>&1 | tail -20</automated>
  </verify>
  <acceptance_criteria>
    - `grep -c 'pub enum FunctionalSlot' src/compat/raw_handle.rs` == 1
    - `grep -c 'extern "C" fn xc_func_alloc' src/compat/raw_handle.rs` == 1
    - `grep -c 'extern "C" fn xc_func_init' src/compat/raw_handle.rs` == 1
    - `grep -c 'extern "C" fn xc_func_end' src/compat/raw_handle.rs` == 1
    - `grep -c 'extern "C" fn xc_func_free' src/compat/raw_handle.rs` == 1
    - `grep -c 'extern "C" fn xc_func_get_info' src/compat/raw_handle.rs` == 1
    - `grep -c 'std::ptr::replace' src/compat/raw_handle.rs` >= 2
    - `grep -c 'LibxcRsError::InvalidSpin' src/compat/raw_handle.rs` >= 1
    - `cargo test -p libxc_rs --lib compat::raw_handle::tests::lifecycle_round_trip` exits 0
    - `cargo test -p libxc_rs --lib compat::raw_handle::tests::reinit_drops_previous` exits 0
    - `cargo test -p libxc_rs --lib compat::raw_handle::tests::null_handle_returns_null_handle_errno` exits 0
    - `cargo test -p libxc_rs --lib compat::raw_handle::tests::invalid_spin_returns_invalid_spin_errno` exits 0
    - `cargo build -p libxc_rs --release` exits 0
    - `cargo clippy -p libxc_rs --no-deps -- -D warnings` exits 0
  </acceptance_criteria>
  <done>
    FunctionalSlot Empty/Initialized state machine; 5 lifecycle functions exported; xc_func_init returns InvalidSpin errno on out-of-range nspin; re-init drops previous Functional via std::ptr::replace.
  </done>
</task>

<task id="06-02a-T3" type="auto">
  <name>Task 3: compat::legacy_eval (partial) — 4 threshold setters + 5 ext_params setters/getters with Pitfall 10 (XC_EXT_PARAMS_DEFAULT) substitution</name>
  <files>src/compat/legacy_eval.rs</files>
  <read_first>
    - /home/user/Documents/workspace/libxc_rs/src/compat/legacy_eval.rs (current placeholder — overwrite; 06-03 will EXTEND this with 35 evaluate fns)
    - /home/user/Documents/workspace/libxc_rs/src/functional/config.rs (post-T1 state — threshold setters now propagate to aux per Pitfall 4 fix)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 660-672 (Pitfall 10)
  </read_first>
  <action>
    Implement the 4 threshold setters and 5 ext_params setters/getters. All take `*xc_func_type` so use `extern_c_wrapper!` form 1.

    Write `src/compat/legacy_eval.rs`:

    ```rust
    //! C-ABI threshold setters and ext_params setters/getters.
    //!
    //! Plan 06-03 EXTENDS this file with the 35 evaluate functions
    //! (12 LDA + 12 GGA + 11 MGGA). This task covers the 9 setters/getters only.

    use crate::compat::c_layout::{xc_func_type, LIBXC_EXT_PARAMS_DEFAULT};
    use crate::compat::errno::{self, set_error};
    use crate::compat::raw_handle::FunctionalSlot;
    use crate::extern_c_wrapper;
    use std::ffi::{c_char, CStr};

    // === 4 threshold setters — each forwards to the Phase-5 setter (which now
    //     walks auxiliaries per Pitfall 4 fix in 06-02a-T1 Step 5).

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_dens_threshold(p: *mut xc_func_type, t: f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_dens_threshold", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            f.set_density_threshold(t);
            Ok(0)
        })
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_zeta_threshold(p: *mut xc_func_type, t: f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_zeta_threshold", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            f.set_zeta_threshold(t);
            Ok(0)
        })
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_sigma_threshold(p: *mut xc_func_type, t: f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_sigma_threshold", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            f.set_sigma_threshold(t);
            Ok(0)
        })
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_tau_threshold(p: *mut xc_func_type, t: f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_tau_threshold", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            f.set_tau_threshold(t);
            Ok(0)
        })
    }

    // === 5 ext_params functions ===

    /// `int xc_func_set_ext_params(xc_func_type *p, const double *ext_params);`
    /// Pitfall 10: substitute LIBXC_EXT_PARAMS_DEFAULT (-999998888.0) with per-spec default.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_ext_params(p: *mut xc_func_type, ext_params: *const f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_ext_params", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            let n = f.meta().ext_params.len();
            if n == 0 {
                return Ok(0);  // no ext_params on this functional; nothing to do
            }
            if ext_params.is_null() {
                return Err(crate::LibxcRsError::ExtParamCountMismatch {
                    id: f.meta().id, expected: n, actual: 0,
                });
            }
            // SAFETY: caller contract — buffer sized for `meta().ext_params.len()`.
            let raw_slice = unsafe { std::slice::from_raw_parts(ext_params, n) };
            // Pitfall 10 substitution.
            let mut substituted: Vec<f64> = Vec::with_capacity(n);
            for (i, &v) in raw_slice.iter().enumerate() {
                if v == LIBXC_EXT_PARAMS_DEFAULT {
                    substituted.push(f.meta().ext_params[i].default_value);
                } else {
                    substituted.push(v);
                }
            }
            f.set_ext_params(&substituted)?;
            Ok(0)
        })
    }

    /// `int xc_func_get_ext_params(const xc_func_type *p, double *ext_params);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_get_ext_params(p: *const xc_func_type, ext_params: *mut f64) -> i32 {
        extern_c_wrapper!(p, "xc_func_get_ext_params", {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            let n = f.meta().ext_params.len();
            if n == 0 {
                return Ok(0);
            }
            if ext_params.is_null() {
                return Err(crate::LibxcRsError::ExtParamCountMismatch {
                    id: f.meta().id, expected: n, actual: 0,
                });
            }
            // SAFETY: caller contract.
            let slice = unsafe { std::slice::from_raw_parts_mut(ext_params, n) };
            if let Some(vals) = f.ext_params() {
                slice.copy_from_slice(vals);
            } else {
                slice.fill(f64::NAN);
            }
            Ok(0)
        })
    }

    /// `int xc_func_set_ext_params_name(xc_func_type *p, const char *name, double par);`
    /// Pitfall 10 substitution applies at single-name level too.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_set_ext_params_name(
        p: *mut xc_func_type, name: *const c_char, par: f64,
    ) -> i32 {
        extern_c_wrapper!(p, "xc_func_set_ext_params_name", {
            let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
            if name.is_null() {
                return Err(crate::LibxcRsError::UnknownExtParamName {
                    id: f.meta().id, name: "<null>".to_string(),
                });
            }
            // SAFETY: name is non-null; caller contract = valid C string.
            let s = unsafe { CStr::from_ptr(name) }.to_str()
                .map_err(|_| crate::LibxcRsError::UnknownExtParamName {
                    id: f.meta().id, name: "<non-utf8>".to_string(),
                })?;
            // Pitfall 10 substitution at single-name level.
            let val = if par == LIBXC_EXT_PARAMS_DEFAULT {
                let idx = f.meta().ext_params.iter().position(|spec| spec.name == s)
                    .ok_or_else(|| crate::LibxcRsError::UnknownExtParamName {
                        id: f.meta().id, name: s.to_string(),
                    })?;
                f.meta().ext_params[idx].default_value
            } else { par };
            f.set_ext_param(s, val)?;
            Ok(0)
        })
    }

    /// `double xc_func_get_ext_params_name(const xc_func_type *p, const char *name);`
    /// Returns NaN on error (errno set); double-returning fns use hand-rolled catch_unwind.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_get_ext_params_name(
        p: *const xc_func_type, name: *const c_char,
    ) -> f64 {
        if p.is_null() || name.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_func_get_ext_params_name: null pointer");
            return f64::NAN;
        }
        let result = std::panic::catch_unwind(|| -> Result<f64, crate::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            let s = unsafe { CStr::from_ptr(name) }.to_str()
                .map_err(|_| crate::LibxcRsError::UnknownExtParamName {
                    id: f.meta().id, name: "<non-utf8>".to_string(),
                })?;
            f.ext_param(s)
        });
        match result {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => { set_error(e.discriminant(), &e.to_string()); f64::NAN }
            Err(_)     => { set_error(errno::LIBXC_RS_PANIC, "xc_func_get_ext_params_name: panic"); f64::NAN }
        }
    }

    /// `double xc_func_get_ext_params_value(const xc_func_type *p, int number);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_get_ext_params_value(p: *const xc_func_type, number: i32) -> f64 {
        if p.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_func_get_ext_params_value: null pointer");
            return f64::NAN;
        }
        let result = std::panic::catch_unwind(|| -> Result<f64, crate::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            f.ext_param_by_index(number as usize)
        });
        match result {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => { set_error(e.discriminant(), &e.to_string()); f64::NAN }
            Err(_)     => { set_error(errno::LIBXC_RS_PANIC, "xc_func_get_ext_params_value: panic"); f64::NAN }
        }
    }

    #[cfg(test)] mod tests {
        use super::*;
        use crate::compat::raw_handle::*;

        /// Pitfall 10: passing LIBXC_EXT_PARAMS_DEFAULT for every parameter must
        /// substitute the per-spec default values.
        #[test]
        fn ext_params_default_marker_substitution() {
            // Pick the first registered functional that has at least one ext_param.
            let target_id = crate::registry::all_functional_ids()
                .find(|fid| {
                    crate::registry::lookup_by_id(fid.raw())
                        .map(|m| !m.ext_params.is_empty()).unwrap_or(false)
                })
                .expect("at least one functional has ext_params");
            unsafe {
                let p = xc_func_alloc();
                assert_eq!(xc_func_init(p, target_id.raw() as i32, 1), 0);
                let meta = crate::registry::lookup_by_id(target_id.raw()).unwrap();
                let n = meta.ext_params.len();
                let vals: Vec<f64> = vec![LIBXC_EXT_PARAMS_DEFAULT; n];
                assert_eq!(xc_func_set_ext_params(p, vals.as_ptr()), 0);
                let mut readback = vec![0.0; n];
                assert_eq!(xc_func_get_ext_params(p, readback.as_mut_ptr()), 0);
                for (i, v) in readback.iter().enumerate() {
                    assert_eq!(*v, meta.ext_params[i].default_value,
                        "param {i} default not substituted");
                }
                xc_func_end(p);
                xc_func_free(p);
            }
        }

        /// Threshold setter wired via compat layer must reach auxiliaries
        /// (covers both the Pitfall 4 fix and the wrapper plumbing in one test).
        #[test]
        fn xc_func_set_dens_threshold_propagates_to_aux_b3lyp() {
            unsafe {
                let p = xc_func_alloc();
                let id = crate::registry::lookup_by_name("hyb_gga_xc_b3lyp").unwrap().raw() as i32;
                assert_eq!(xc_func_init(p, id, 1), 0);
                assert_eq!(xc_func_set_dens_threshold(p, 1e-12), 0);
                let f = FunctionalSlot::as_initialized_const(p).unwrap();
                assert_eq!(f.thresholds().density, 1e-12);
                for aux in f.auxiliary_functionals() {
                    assert_eq!(aux.thresholds().density, 1e-12,
                        "aux {} did not receive threshold via FFI path", aux.meta().name);
                }
                xc_func_end(p);
                xc_func_free(p);
            }
        }
    }
    ```

    Verify: `cargo test -p libxc_rs --lib compat::legacy_eval`. Commit: `feat(06-02a): compat threshold + ext_params setters with Pitfall 10 substitution`.

    Do NOT add the 35 evaluate functions in this task — they live in 06-03. The file's test module name `tests` will need to be renamed `setter_tests` by 06-03 if it adds `lda_evaluate_tests`/`gga_evaluate_tests`/`mgga_evaluate_tests` separately; that's an executor-side detail.
  </action>
  <verify>
    <automated>cargo test -p libxc_rs --lib compat::legacy_eval 2>&1 | tail -20</automated>
  </verify>
  <acceptance_criteria>
    - `grep -c 'extern "C" fn xc_func_set_dens_threshold' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_set_zeta_threshold' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_set_sigma_threshold' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_set_tau_threshold' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_set_ext_params\b' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_get_ext_params\b' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_set_ext_params_name' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_get_ext_params_name' src/compat/legacy_eval.rs` == 1
    - `grep -c 'extern "C" fn xc_func_get_ext_params_value' src/compat/legacy_eval.rs` == 1
    - `grep -c 'LIBXC_EXT_PARAMS_DEFAULT' src/compat/legacy_eval.rs` >= 2
    - `cargo test -p libxc_rs --lib compat::legacy_eval::tests::ext_params_default_marker_substitution` exits 0
    - `cargo test -p libxc_rs --lib compat::legacy_eval::tests::xc_func_set_dens_threshold_propagates_to_aux_b3lyp` exits 0
    - `cargo build -p libxc_rs --release` exits 0
    - `cargo clippy -p libxc_rs --no-deps -- -D warnings` exits 0
  </acceptance_criteria>
  <done>
    4 threshold + 5 ext_params extern Cs exported; Pitfall 10 substitution applied in xc_func_set_ext_params and xc_func_set_ext_params_name; B3LYP threshold propagates to auxiliaries through the FFI path.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| C caller → compat/* | Untrusted: `*mut xc_func_type` may be NULL, dangling, or aliased; `int functional` may be out-of-range; `double *` buffers may be incorrectly sized. |
| compat/* → functional/* | Internal Rust call — Phase 5 surface is trusted. |
| Phase-5 setter → auxiliaries | New trust boundary introduced by Pitfall 4 fix: mutable iteration over `auxiliaries: Vec<Functional>` must not violate borrow invariants (depth-first recursion via per-aux `set_*_threshold` is safe). |
| compat/* (errno) → C caller | Returned `*const c_char` lifetime is "until next call on this thread" — documented in include/xc.h. |
| compat/* (cache_cstring) → C caller | HashMap-backed cache; pointers stable across rehash because Box's heap allocation does not move. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-06-01 | Tampering | NULL `*mut xc_func_type` to any extern C function | mitigate | `extern_c_wrapper!` form 1 NULL-checks at every entry. Test `null_handle_returns_null_handle_errno` enforces. |
| T-06-02 | Tampering | Opaque struct field access from C (e.g. `p->info`) | accept | Header forward-declares only; any field access fails to compile. |
| T-06-03 | DoS | Panic propagating across FFI (UB in Rust 1.33+) | mitigate | `catch_unwind` at every entry via `extern_c_wrapper!`. Test `catch_panic_returns_errno` enforces. |
| T-06-04 | Tampering / Information Disclosure | Re-init leaking previous Functional (Pitfall 1) | mitigate | `std::ptr::replace` in xc_func_init / xc_func_end drops the previous slot. Test `reinit_drops_previous` enforces. |
| T-06-05 | DoS | XC_EXT_PARAMS_DEFAULT silently corrupting numerical results (Pitfall 10) | mitigate | Substitution applied in `xc_func_set_ext_params` and `xc_func_set_ext_params_name` before forwarding. Test `ext_params_default_marker_substitution` enforces. |
| T-06-06 | Tampering | Threshold setter on hybrid functional not propagating to auxiliaries (Pitfall 4) | mitigate | Phase-5 fix walks `self.auxiliaries.iter_mut()`. Tests `threshold_propagates_to_aux_*` enforce. |
| T-06-07 | Tampering | Out-of-range `nspin` to xc_func_init | mitigate | Returns `LibxcRsError::InvalidSpin(nspin)` → LIBXC_RS_INVALID_SPIN errno. Test `invalid_spin_returns_invalid_spin_errno` enforces. |
| T-06-08 | DoS | cache_cstring corruption when 649+ names are inserted | mitigate | HashMap-keyed cache with `Pin<Box<CString>>` values; Box heap address stable across rehash. Test `cache_cstring_holds_649_pointers_stable` enforces. |
| T-06-09 | Tampering | Concurrent mutation of shared `*mut xc_func_type` | accept | CONTEXT D-A1-3: single-threaded per handle. Documented in include/xc.h. |
| T-06-10 | Tampering | Catch-all `_ => -N` arm masking a forgotten variant in discriminant() | mitigate | Exhaustive match enforced by acceptance gate `awk '/pub fn discriminant/,/^    }$/' | grep -cE '_\s*=>'` == 0; uniqueness test enforces 24-distinct codes. |
</threat_model>

<verification>
After all three tasks complete, run from repo root:

```bash
cargo test -p libxc_rs --lib functional::config:: compat:: error::
cargo build -p libxc_rs --release
cargo clippy -p libxc_rs --no-deps -- -D warnings

# Symbol export count gate (partial — full 85 reached in 06-03):
nm target/release/liblibxc_rs.so 2>/dev/null | grep -c 'T xc_'
# Expect ≥ 16 (5 lifecycle + 4 thresholds + 5 ext_params + 2 errno = 16)

# Pitfall fixes:
grep -c 'auxiliaries\.iter_mut' src/functional/config.rs   # ≥ 4
grep -c 'LIBXC_EXT_PARAMS_DEFAULT' src/compat/legacy_eval.rs  # ≥ 2

# Discriminant exhaustiveness — TIGHTENED gate:
awk '/pub fn discriminant/,/^    }$/' src/error/mod.rs | grep -cE '_\s*=>'   # == 0

# pub mod compat; wired:
grep -c 'pub mod compat;' src/lib.rs   # == 1
```

All commands exit 0.
</verification>

<success_criteria>
- COMPAT-02: opaque types compile-asserted zero-sized; `Family::Lda as i32 == 1` etc. assertions pass.
- COMPAT-03 (this slice): all `unsafe` introduced in this plan lives under `src/compat/*`. Existing Phase-5 unsafe (kernel/launch.rs, kernel/buffer.rs) unchanged. `src/api/*` remains zero-unsafe.
- COMPAT-01 (lifecycle + setter slice): 5 lifecycle + 4 threshold + 5 ext_params + 2 errno = 16 extern Cs exported. Discovery / info / library / hybrid / removed / AK13 (06-02b) and 35 evaluators (06-03) follow.
- Pitfall 4: B3LYP unit test passes.
- Pitfall 10: ext_params default-marker substitution unit test passes.
- Pitfall 1: re-init drops previous Functional via `std::ptr::replace`; unit test passes.
- InvalidSpin: out-of-range nspin returns LIBXC_RS_INVALID_SPIN; unit test passes.
- Discriminant exhaustiveness: 24 variants → 24 unique negative codes, no `_ =>` arm.
- cache_cstring: 649+ pointer stability test passes.
- All extern "C" symbols use `#[unsafe(no_mangle)]` (Edition 2024 spelling).
</success_criteria>

<output>
After completion, create `.planning/phases/06-public-api-and-c-compatibility/06-02a-SUMMARY.md` documenting:
- File-by-file summary: c_layout, errno, macros, raw_handle, legacy_eval (partial)
- Phase-5 modification: 4 threshold setters in src/functional/config.rs (Pitfall 4 fix). The single Phase-5 surface change in Phase 6.
- The 16-function inventory exported in this plan (5 lifecycle + 4 thresholds + 5 ext_params + 2 errno)
- Errno discriminant table — final 25 codes (LIBXC_RS_OK + 24 negatives), 1:1 with LibxcRsError variants. Cross-plan contract: `include/xc.h` LIBXC_RS_* block (06-03 task) must mirror this exactly.
- HashMap-keyed cache_cstring — storage: thread_local RefCell<HashMap<&'static str, Pin<Box<CString>>>>. Stability guarantee: Box heap allocation stable across rehash; cached pointer never moves.
- InvalidSpin(i32) consumer in xc_func_init for nspin ∉ {1, 2}.
- Test counts (lifecycle, errno, panic, repr-constants, threshold-propagation, ext_params-default, cache_cstring stability, discriminant uniqueness)
- Symbol-count smoke: `nm | grep -c 'T xc_'` ≥ 16
</output>
</content>
</invoke>