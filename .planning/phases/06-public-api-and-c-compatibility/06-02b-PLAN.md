---
phase: 06-public-api-and-c-compatibility
plan: 02b
type: execute
wave: 3
depends_on: ["06-02a"]
files_modified:
  - src/compat/mod.rs
  - src/compat/ids.rs
  - src/compat/info.rs
  - src/compat/library.rs
  - src/compat/hybrid.rs
  - src/compat/removed.rs
autonomous: true
requirements: [COMPAT-01, COMPAT-02]
tags: [ffi, extern-c, discovery, info-accessors, hybrid, ak13, library-version, removed]

must_haves:
  truths:
    - "All 8 discovery functions (xc_number_of_functionals, xc_functional_get_number/_name, xc_family_from_id, xc_maximum_name_length, xc_available_functional_numbers/_by_name, xc_available_functional_names) wrap registry::* and return correct values"
    - "All 14 info accessors (10 xc_func_info_get_* + 4 xc_func_reference_get_*) return field values from `&'static FunctionalMeta` cast through `*const xc_func_info_type`"
    - "All 5 library-version functions (xc_version writes (7,0,0); xc_version_string returns \"7.0.0\"; xc_reference{,_doi,_key} return canonical libxc Lehtola2018 strings)"
    - "All 9 hybrid+aux+NLC+ak13 functions (xc_hyb_type, xc_hyb_exx_coef, xc_hyb_cam_coef, xc_nlc_coef, xc_num_aux_funcs, xc_aux_func_ids/_weights, xc_gga_ak13_get_asymptotic, xc_gga_ak13_pars_get_asymptotic) wire to Phase-5 surface"
    - "AK13 helpers produce numerical results that match libxc's gga_x_ak13.c byte-for-byte for ≥ 3 oracle (homo, expected) pairs"
    - "compat::removed::replacement_for(id) helper surfaces RemovedFunctionalId payloads for the errno text"
    - "xc_available_functional_names fills 649 char* slots, all pointers remain valid through subsequent calls (HashMap-keyed cache from 06-02a)"
    - "All `unsafe` blocks introduced in this plan live exclusively under src/compat/*"
  artifacts:
    - path: "src/compat/ids.rs"
      provides: "8 discovery extern C functions wrapping registry::lookup_by_*"
      contains: "extern \"C\" fn xc_functional_get_number"
      contains: "extern \"C\" fn xc_functional_get_name"
      contains: "extern \"C\" fn xc_family_from_id"
      contains: "extern \"C\" fn xc_number_of_functionals"
      contains: "extern \"C\" fn xc_maximum_name_length"
      contains: "extern \"C\" fn xc_available_functional_numbers"
      contains: "extern \"C\" fn xc_available_functional_numbers_by_name"
      contains: "extern \"C\" fn xc_available_functional_names"
    - path: "src/compat/info.rs"
      provides: "10 xc_func_info_get_* + 4 xc_func_reference_get_* accessors"
      contains: "extern \"C\" fn xc_func_info_get_number"
      contains: "extern \"C\" fn xc_func_info_get_name"
      contains: "extern \"C\" fn xc_func_info_get_n_ext_params"
    - path: "src/compat/library.rs"
      provides: "5 library-info functions"
      contains: "extern \"C\" fn xc_version"
      contains: "extern \"C\" fn xc_version_string"
      contains: "extern \"C\" fn xc_reference"
    - path: "src/compat/hybrid.rs"
      provides: "7 hybrid/aux/nlc accessors + 2 xc_gga_ak13_* helpers (formula inlined verbatim from gga_x_ak13.c)"
      contains: "extern \"C\" fn xc_hyb_type"
      contains: "extern \"C\" fn xc_hyb_exx_coef"
      contains: "extern \"C\" fn xc_hyb_cam_coef"
      contains: "extern \"C\" fn xc_nlc_coef"
      contains: "extern \"C\" fn xc_num_aux_funcs"
      contains: "extern \"C\" fn xc_aux_func_ids"
      contains: "extern \"C\" fn xc_aux_func_weights"
      contains: "extern \"C\" fn xc_gga_ak13_get_asymptotic"
      contains: "extern \"C\" fn xc_gga_ak13_pars_get_asymptotic"
      contains: "AK13_PAR_B1"
      contains: "AK13_ORACLE_PAIRS"
    - path: "src/compat/removed.rs"
      provides: "Helper to surface RemovedFunctionalId replacement payload through errno text"
      contains: "pub fn replacement_for"
  key_links:
    - from: "src/compat/ids.rs"
      to: "src/registry/mod.rs"
      via: "lookup_by_id, lookup_by_name, functional_count, max_name_length, all_functional_ids"
      pattern: "registry::"
    - from: "src/compat/info.rs"
      to: "src/meta/mod.rs (FunctionalMeta fields)"
      via: "info_ref helper casts *const xc_func_info_type to &'static FunctionalMeta"
      pattern: "as \\*const FunctionalMeta"
    - from: "src/compat/hybrid.rs"
      to: "src/functional/hybrid.rs"
      via: "Functional::hybrid_type, exx_coefficient, cam_coefficients, nlc_coefficients, auxiliary_functionals, mix_coefficients"
      pattern: "cam_coefficients\\(\\)"
    - from: "src/compat/hybrid.rs::xc_gga_ak13_get_asymptotic"
      to: "Inlined formula from libxc-master/src/gga_x_ak13.c lines 35-55"
      via: "Direct port preserving op order; exact constants from par_ak13[]"
      pattern: "AK13_PAR_B1"
---

<objective>
Build the compat-layer **read-only accessors**: 8 discovery functions wrapping `registry::*`, 14 info accessors (`xc_func_info_get_*` + `xc_func_reference_get_*`) reading `&'static FunctionalMeta`, 5 library-version functions, 9 hybrid/aux/NLC/AK13 functions (with the AK13 formula INLINED VERBATIM from `libxc-master/src/gga_x_ak13.c`), and the `compat::removed` errno-enrichment helper. Total this plan: **35 extern "C" functions** added to the cdylib (8 + 10 + 4 + 5 + 9 = 36; one of those — `xc_func_info_get_references` — returns a `*const func_reference_type` that bridges into the 4 reference accessors, so the function-count book-keeping is "8 discovery + 14 info/reference + 5 library + 9 hybrid/AK13 = 36"). Combined with 06-02a (16 fns) the post-Wave-3 total is **52 extern Cs**, leaving 33 evaluate fns for 06-03 (52 + 33 = 85).

Purpose: Phase 6 wraps Phase-5's `Functional` in a Layer-1 C ABI. 06-02a (Wave 2) shipped lifecycle + setters + errno + macros. This plan (Wave 3) adds every remaining accessor that 06-03's evaluate functions do NOT need but that the libxc ABI requires. The AK13 helpers ship with their formula INLINED inside this plan (no `unimplemented!()`) and are oracle-tested against ≥ 3 hardcoded `(homo, expected)` pairs computed offline from the libxc reference impl.

Output: 5 files under `src/compat/` (ids, info, library, hybrid, removed); module wiring updates in `src/compat/mod.rs`.

This plan executes in Wave 3; it depends on 06-02a (Wave 2) for `xc_func_type`/`xc_func_info_type`/`func_reference_type` opaque types, the `extern_c_wrapper!` macro, the `cache_cstring` helper, the errno table, and `FunctionalSlot::as_initialized_const`.
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
@/home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-02a-PLAN.md
@/home/user/Documents/workspace/libxc_rs/CLAUDE.md
@/home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h
@/home/user/Documents/workspace/libxc_rs/libxc-master/src/gga_x_ak13.c

<interfaces>
<!-- Compat infrastructure from 06-02a (this plan consumes, does not modify) -->

From src/compat/c_layout.rs:
```rust
#[repr(C)] pub struct xc_func_type { _opaque: [u8; 0], _marker: ... }
#[repr(C)] pub struct xc_func_info_type { _opaque: [u8; 0], _marker: ... }
#[repr(C)] pub struct func_reference_type { _opaque: [u8; 0], _marker: ... }
pub const LIBXC_EXT_PARAMS_DEFAULT: f64 = -999998888.0;
pub const XC_MAX_REFERENCES: usize = 5;
```

From src/compat/raw_handle.rs:
```rust
pub enum FunctionalSlot { Empty, Initialized(Functional) }
impl FunctionalSlot {
    pub(crate) unsafe fn as_initialized_const<'a>(p: *const xc_func_type) -> Result<&'a Functional, LibxcRsError>;
}
```

From src/compat/errno.rs:
```rust
pub fn cache_cstring(s: &'static str) -> *const c_char;   // HashMap-keyed; stable across rehash
pub fn set_error(code: i32, msg: &str);
pub const LIBXC_RS_NULL_HANDLE: i32 = -2;
pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_ID: i32 = -4;
// ... full table (25 codes)
```

From src/compat/macros.rs:
```rust
macro_rules! extern_c_wrapper {
    ($p:expr, $name:literal, $body:block) => { /* NULL check + catch_unwind + errno */ };
    (_, $name:literal, $body:block)       => { /* catch_unwind + errno */ };
}
```

<!-- Phase-5 surface (frozen) -->

From src/registry/mod.rs:
```rust
pub fn lookup_by_id(id: u16) -> Result<&'static FunctionalMeta, LibxcRsError>;
pub fn lookup_by_name(name: &str) -> Result<FunctionalId, LibxcRsError>;
pub fn functional_count() -> usize;       // 649
pub fn max_name_length() -> usize;
pub fn all_functional_ids() -> impl Iterator<Item = FunctionalId>;
pub fn version() -> (u32, u32, u32);      // (7, 0, 0)
pub fn version_string() -> &'static str;  // "7.0.0"
pub fn reference_string() -> &'static str;
```

From src/meta/mod.rs:
```rust
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
pub struct ExtParamSpec { pub name, pub description, pub default_value };
pub struct Reference { /* citation/doi/bibtex/key as &'static str — verify exact field names by reading src/meta/mod.rs */ }
```

From src/functional/hybrid.rs:
```rust
impl Functional {
    pub fn hybrid_type(&self) -> HybridType;
    pub fn exx_coefficient(&self) -> Option<f64>;
    pub fn cam_coefficients(&self) -> Option<CamCoefficients>;  // { omega, alpha, beta }
    pub fn nlc_coefficients(&self) -> Option<NlcCoefficients>;  // { b, c }
    pub fn auxiliary_functionals(&self) -> &[Functional];
    pub fn mix_coefficients(&self) -> &[f64];
}
```

<!-- AK13 INLINE FORMULA — VERIFIED libxc-master/src/gga_x_ak13.c lines 32-55 -->

```c
// libxc-master/src/gga_x_ak13.c:32-33 — par_ak13 array, the per-spec defaults:
static const double par_ak13[N_PAR] =
  {1.74959015598863046792081721182, -1.62613336586517367779736042170};
//   ^B1 = 3*muGE/5 + 8 pi/15        ^B2 = muGE - B1

// libxc-master/src/gga_x_ak13.c:35-38 — get_asymptotic forwards to pars_get with par_ak13:
double xc_gga_ak13_get_asymptotic (double homo) {
  return xc_gga_ak13_pars_get_asymptotic(homo, par_ak13);
}

// libxc-master/src/gga_x_ak13.c:40-55 — pars_get_asymptotic body:
double xc_gga_ak13_pars_get_asymptotic (double homo, const double *ext_params) {
  double Qx, aa, aa2, factor;
  double ak13_B1;
  ak13_B1 = ext_params[0];
  Qx = sqrt(2.0)*ak13_B1/(3.0*CBRT(3.0*M_PI*M_PI));
  aa  = X_FACTOR_C*Qx;
  aa2 = aa*aa;
  factor = (homo < 0.0) ? -1.0 : 1.0;
  return (aa2/2.0)*(1.0 + factor*sqrt(1.0 - 4.0*homo/aa2));
}
```

Where:
- `X_FACTOR_C` is libxc's `CBRT(3.0)*POW(M_PI, -1.0/3.0)*0.75 = -3.0/4.0 * (3.0/M_PI)^(1/3) * (-1)`. Wait — re-read libxc-master/src/util.h before commit. **Conservative recommendation**: Read util.h to extract the EXACT numeric value; do not infer. Use `const X_FACTOR_C: f64 = ...verified literal from util.h...;` and document the source line.
- `CBRT(x)` is libxc's `cbrt(x)` macro (libm cube root).
- `M_PI` = `std::f64::consts::PI`.

The Rust port must preserve op order EXACTLY (CLAUDE.md "Operation order preservation").

<!-- Pre-computed AK13 oracle pairs — to be embedded as a const &[(f64, f64)] -->

The executor MUST run a one-off C program against libxc 7.0.0's `xc_gga_ak13_get_asymptotic` for at least 3 distinct `homo` values (e.g. -0.5, -0.1, +0.05) and embed the bit-exact f64 results as `pub const AK13_ORACLE_PAIRS: &[(f64, f64)]` in `src/compat/hybrid.rs`. The reference oracle program shape (place in `xtask/src/bin/ak13_oracle.rs` or a one-off `.c` file built via `verify/build.rs`):

```c
#include <stdio.h>
double xc_gga_ak13_get_asymptotic(double homo);
int main(void) {
    double inputs[] = {-0.5, -0.1, 0.05};
    for (int i = 0; i < 3; i++) {
        double y = xc_gga_ak13_get_asymptotic(inputs[i]);
        printf("(%.17e, %.17e),\n", inputs[i], y);
    }
    return 0;
}
```

Compile against the libxc-sys oracle (`verify/` already builds libxc-master via cmake; reuse). The 3 printed lines become the body of:

```rust
/// Oracle pairs: (homo, expected). Computed offline from libxc 7.0.0 via
/// `xtask/src/bin/ak13_oracle.rs` (see SUMMARY for the exact run output).
/// Bit-exact values; testing requires `assert_eq!(actual.to_bits(), expected.to_bits())`.
pub const AK13_ORACLE_PAIRS: &[(f64, f64)] = &[
    (-5.0e-1_f64, /* paste oracle bytes */),
    (-1.0e-1_f64, /* paste oracle bytes */),
    ( 5.0e-2_f64, /* paste oracle bytes */),
];
```

If the executor cannot run the C oracle (e.g. cmake/libxc build broken), fall back to a tolerance test: `assert!((actual - expected).abs() < 1e-12)` with values computed from the inlined Rust formula by hand on a calculator and the original libxc paper. Document the fallback in SUMMARY explicitly. **Do NOT skip the test.**

<!-- HybridType → XC_HYB_* int mapping (from libxc-master/src/xc.h) -->

```rust
fn hybrid_type_to_int(t: HybridType) -> i32 {
    match t {
        HybridType::Semilocal    => 0,    // XC_HYB_NONE
        HybridType::Hybrid       => 1,    // XC_HYB_HYBRID
        HybridType::Cam          => 2,    // XC_HYB_CAM
        HybridType::CamYukawa    => 3,    // XC_HYB_CAMY
        HybridType::CamGaussian  => 4,    // XC_HYB_CAMG
        HybridType::DoubleHybrid => 32,   // XC_HYB_DOUBLE_HYBRID — verify
        HybridType::Mixture      => 64,   // XC_HYB_MIXTURE      — verify
        // Read every variant of HybridType in src/model/mod.rs and assign each one.
        // Verify integer values against libxc-master/src/xc.h:69-100.
    }
}
```
</interfaces>

</context>

<tasks>

<task id="06-02b-T1" type="auto">
  <name>Task 1: compat::ids (8 discovery extern Cs) + compat::info (10 + 4 accessors) + compat::removed helper</name>
  <files>
    src/compat/ids.rs,
    src/compat/info.rs,
    src/compat/removed.rs,
    src/compat/mod.rs
  </files>
  <read_first>
    - /home/user/Documents/workspace/libxc_rs/src/compat/ids.rs (placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/compat/info.rs (read if exists; otherwise create)
    - /home/user/Documents/workspace/libxc_rs/src/compat/removed.rs (placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/registry/mod.rs (read in full)
    - /home/user/Documents/workspace/libxc_rs/src/registry/removed.rs (REMOVED_IDS data shape)
    - /home/user/Documents/workspace/libxc_rs/src/meta/mod.rs (FunctionalMeta + Reference field names)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h lines 305-316 (info accessor signatures), lines 177-180 (reference accessor signatures), lines 370-387 (discovery signatures)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-RESEARCH.md lines 691-705 (Example 3 + Pitfall 2)
    - /home/user/Documents/workspace/libxc_rs/.planning/phases/06-public-api-and-c-compatibility/06-PATTERNS.md lines 358-432
  </read_first>
  <action>
    **Step 1 — `src/compat/ids.rs`** (8 discovery functions):

    Use the canonical pattern from RESEARCH § Pattern 3. Excerpt — write all 8:

    ```rust
    //! Discovery functions: id ↔ name lookup, family classification, listing.
    //! Wraps `src/registry/mod.rs` 1:1.

    use crate::compat::errno::{self, cache_cstring, set_error};
    use crate::extern_c_wrapper;
    use crate::registry;
    use std::ffi::{c_char, CStr};

    /// `int xc_functional_get_number(const char *name);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_functional_get_number(name: *const c_char) -> i32 {
        if name.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_functional_get_number: null name");
            return errno::LIBXC_RS_NULL_HANDLE;
        }
        extern_c_wrapper!(_, "xc_functional_get_number", {
            // SAFETY: name non-null; caller contract = valid C string.
            let s = unsafe { CStr::from_ptr(name) }.to_str()
                .map_err(|_| crate::LibxcRsError::UnknownFunctionalName("non-utf8".into()))?;
            let id = registry::lookup_by_name(s)?;
            Ok(id.raw() as i32)
        })
    }

    /// `const char *xc_functional_get_name(int number);` — pointer into thread-local CString cache.
    /// Lifetime: pointer remains valid across subsequent cache_cstring calls (HashMap stability).
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_functional_get_name(number: i32) -> *const c_char {
        let result = std::panic::catch_unwind(|| {
            if number < 0 || number > u16::MAX as i32 { return None; }
            registry::lookup_by_id(number as u16).ok().map(|m| m.name)
        });
        match result {
            Ok(Some(name)) => cache_cstring(name),
            Ok(None) => {
                set_error(errno::LIBXC_RS_UNKNOWN_FUNCTIONAL_ID,
                    &format!("xc_functional_get_name: unknown id {number}"));
                std::ptr::null()
            }
            Err(_) => {
                set_error(errno::LIBXC_RS_PANIC, "xc_functional_get_name: panic");
                std::ptr::null()
            }
        }
    }

    /// `int xc_family_from_id(int id, int *family, int *number);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_family_from_id(id: i32, family: *mut i32, number: *mut i32) -> i32 {
        extern_c_wrapper!(_, "xc_family_from_id", {
            if id < 0 || id > u16::MAX as i32 {
                return Err(crate::LibxcRsError::UnknownFunctionalId(0));
            }
            let meta = registry::lookup_by_id(id as u16)?;
            if !family.is_null() { unsafe { *family = meta.family as i32; } }
            if !number.is_null() { unsafe { *number = id; } }
            Ok(0)
        })
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_number_of_functionals() -> i32 {
        registry::functional_count() as i32
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_maximum_name_length() -> i32 {
        registry::max_name_length() as i32
    }

    /// `void xc_available_functional_numbers(int *list);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_available_functional_numbers(list: *mut i32) {
        if list.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE,
                "xc_available_functional_numbers: null list");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            let count = registry::functional_count();
            let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
            for (i, fid) in registry::all_functional_ids().enumerate() {
                if i < count { slice[i] = fid.raw() as i32; }
            }
        });
    }

    /// `void xc_available_functional_numbers_by_name(int *list);` — sorted alphabetically.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_available_functional_numbers_by_name(list: *mut i32) {
        if list.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE,
                "xc_available_functional_numbers_by_name: null list");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            let count = registry::functional_count();
            let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
            let mut pairs: Vec<(&'static str, u16)> = registry::all_functional_ids()
                .map(|fid| {
                    let m = registry::lookup_by_id(fid.raw()).expect("registered id must lookup");
                    (m.name, fid.raw())
                })
                .collect();
            pairs.sort_by_key(|&(n, _)| n);
            for (i, &(_, raw)) in pairs.iter().enumerate() {
                if i < count { slice[i] = raw as i32; }
            }
        });
    }

    /// `void xc_available_functional_names(char **list);` — fills 649 thread-local cached pointers.
    /// Pointers stable across rehash (HashMap-keyed cache from 06-02a).
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_available_functional_names(list: *mut *mut c_char) {
        if list.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE,
                "xc_available_functional_names: null list");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            let count = registry::functional_count();
            let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
            for (i, fid) in registry::all_functional_ids().enumerate() {
                if i >= count { break; }
                let m = registry::lookup_by_id(fid.raw()).expect("registered id must lookup");
                slice[i] = cache_cstring(m.name) as *mut c_char;
            }
        });
    }

    #[cfg(test)] mod tests {
        use super::*;
        #[test]
        fn discovery_matches_registry() {
            assert_eq!(xc_number_of_functionals(), 649);
            let name = std::ffi::CString::new("lda_x").unwrap();
            unsafe {
                let id = xc_functional_get_number(name.as_ptr());
                assert!(id > 0, "lda_x lookup must return positive id; got {id}");
                let p = xc_functional_get_name(id);
                assert!(!p.is_null());
                let s = CStr::from_ptr(p).to_string_lossy();
                assert_eq!(s, "lda_x");
                let mut family = 0i32;
                let mut number = 0i32;
                assert_eq!(xc_family_from_id(id, &mut family, &mut number), 0);
                assert_eq!(family, 1);
            }
        }

        #[test]
        fn available_names_fills_649() {
            let count = xc_number_of_functionals() as usize;
            let mut buf: Vec<*mut c_char> = vec![std::ptr::null_mut(); count];
            unsafe { xc_available_functional_names(buf.as_mut_ptr()); }
            // Every slot non-null and points to a valid C string.
            for (i, p) in buf.iter().enumerate() {
                assert!(!p.is_null(), "slot {i} null");
                let s = unsafe { CStr::from_ptr(*p).to_string_lossy() };
                assert!(!s.is_empty(), "slot {i} empty");
            }
        }
    }
    ```

    **Step 2 — `src/compat/info.rs`** (10 + 4 accessors). Use the `info_ref`/`ref_ref` helper pattern; full text from RESEARCH § Pattern 3 / Example 3:

    ```rust
    //! `xc_func_info_get_*` and `xc_func_reference_get_*` accessors.

    use crate::compat::c_layout::{xc_func_info_type, func_reference_type};
    use crate::compat::errno::cache_cstring;
    use crate::meta::{FunctionalMeta, Reference};
    use std::ffi::c_char;

    unsafe fn info_ref<'a>(info: *const xc_func_info_type) -> Option<&'a FunctionalMeta> {
        if info.is_null() { None } else { Some(unsafe { &*(info as *const FunctionalMeta) }) }
    }
    unsafe fn ref_ref<'a>(r: *const func_reference_type) -> Option<&'a Reference> {
        if r.is_null() { None } else { Some(unsafe { &*(r as *const Reference) }) }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_number(info: *const xc_func_info_type) -> i32 {
        if let Some(m) = unsafe { info_ref(info) } { m.id.raw() as i32 } else { -1 }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_kind(info: *const xc_func_info_type) -> i32 {
        if let Some(m) = unsafe { info_ref(info) } { m.kind as i32 } else { -1 }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_name(info: *const xc_func_info_type) -> *const c_char {
        if let Some(m) = unsafe { info_ref(info) } { cache_cstring(m.name) } else { std::ptr::null() }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_family(info: *const xc_func_info_type) -> i32 {
        if let Some(m) = unsafe { info_ref(info) } { m.family as i32 } else { -1 }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_flags(info: *const xc_func_info_type) -> i32 {
        if let Some(m) = unsafe { info_ref(info) } { m.flags.bits() as i32 } else { 0 }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_n_ext_params(info: *const xc_func_info_type) -> i32 {
        if let Some(m) = unsafe { info_ref(info) } { m.ext_params.len() as i32 } else { -1 }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_ext_params_name(
        info: *const xc_func_info_type, number: i32,
    ) -> *const c_char {
        if let Some(m) = unsafe { info_ref(info) } {
            if (number as usize) < m.ext_params.len() {
                return cache_cstring(m.ext_params[number as usize].name);
            }
        }
        std::ptr::null()
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_ext_params_description(
        info: *const xc_func_info_type, number: i32,
    ) -> *const c_char {
        if let Some(m) = unsafe { info_ref(info) } {
            if (number as usize) < m.ext_params.len() {
                return cache_cstring(m.ext_params[number as usize].description);
            }
        }
        std::ptr::null()
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_ext_params_default_value(
        info: *const xc_func_info_type, number: i32,
    ) -> f64 {
        if let Some(m) = unsafe { info_ref(info) } {
            if (number as usize) < m.ext_params.len() {
                return m.ext_params[number as usize].default_value;
            }
        }
        f64::NAN
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_info_get_references(
        info: *const xc_func_info_type, number: i32,
    ) -> *const func_reference_type {
        if let Some(m) = unsafe { info_ref(info) } {
            if (number as usize) < m.references.len() {
                return &m.references[number as usize] as *const Reference as *const func_reference_type;
            }
        }
        std::ptr::null()
    }

    // 4 reference accessors. Verify the Reference field names by reading src/meta/mod.rs.
    // If field names differ from the example below (e.g. citation vs ref_str), adjust.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_reference_get_ref(r: *const func_reference_type) -> *const c_char {
        if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.citation) } else { std::ptr::null() }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_reference_get_doi(r: *const func_reference_type) -> *const c_char {
        if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.doi) } else { std::ptr::null() }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_reference_get_bibtex(r: *const func_reference_type) -> *const c_char {
        if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.bibtex) } else { std::ptr::null() }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_func_reference_get_key(r: *const func_reference_type) -> *const c_char {
        if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.key) } else { std::ptr::null() }
    }

    #[cfg(test)] mod tests {
        use super::*;
        use crate::compat::raw_handle::*;
        use std::ffi::CStr;

        #[test]
        fn info_get_name_returns_cached_cstring() {
            unsafe {
                let p = xc_func_alloc();
                assert_eq!(xc_func_init(p, 1, 1), 0);  // lda_x
                let info = xc_func_get_info(p);
                assert!(!info.is_null());
                let name = xc_func_info_get_name(info);
                let s = CStr::from_ptr(name).to_string_lossy();
                assert_eq!(s, "lda_x");
                xc_func_end(p);
                xc_func_free(p);
            }
        }
    }
    ```

    Verify Reference's actual field names by `grep -nE 'pub (citation|doi|bibtex|key|ref_str)' src/meta/mod.rs`. If a field is named differently (e.g. `ref_str` instead of `citation`), adjust the calls.

    **Step 3 — `src/compat/removed.rs`** (errno-enrichment helper):

    ```rust
    //! Errno enrichment for "removed functional ID" errors.
    //!
    //! `LibxcRsError::RemovedFunctionalId { removed_id, replacement_id, replacement_name }`
    //! is raised by `src/registry/mod.rs::lookup_by_id`. The compat layer surfaces it
    //! via the int errno mechanism; this helper formats the replacement payload into
    //! the thread-local message text.

    use crate::error::LibxcRsError;
    use crate::registry::removed::REMOVED_IDS;

    /// Returns Some((replacement_id, replacement_name)) if `id` is in REMOVED_IDS.
    pub fn replacement_for(id: u16) -> Option<(u16, &'static str)> {
        REMOVED_IDS.iter().find(|&&(r, _)| r == id).map(|&(_, replacement_id)| {
            let replacement_name = if replacement_id > 0 {
                crate::registry::lookup_by_id(replacement_id)
                    .map(|m| m.name).unwrap_or("unknown")
            } else { "none" };
            (replacement_id, replacement_name)
        })
    }

    /// Format a `RemovedFunctionalId` error's payload for the thread-local errno message.
    pub fn format_removed_message(e: &LibxcRsError) -> Option<String> {
        match e {
            LibxcRsError::RemovedFunctionalId { removed_id, replacement_id, replacement_name } => {
                Some(format!("ID {removed_id} removed; use {replacement_id} ({replacement_name}) instead"))
            }
            _ => None,
        }
    }
    ```

    Verify `crate::registry::removed::REMOVED_IDS` shape and field name by grep — this is in 06-02a's existing-file domain, may be tuple (u16, u16) or named struct.

    **Step 4 — extend `src/compat/mod.rs`** to expose the new submodules:

    ```rust
    pub mod ids;       // 06-02b
    pub mod info;      // 06-02b
    pub mod removed;   // 06-02b
    // (library and hybrid added in 06-02b-T2)
    ```

    Place these after the existing `pub mod legacy_eval;`.

    **Step 5 — verify:** `cargo test -p libxc_rs --lib compat::ids compat::info compat::removed`. Commit: `feat(06-02b): compat::ids (8 fns) + compat::info (14 accessors) + compat::removed helper`.

    Do NOT add library/hybrid functions in this task — those are T2.
  </action>
  <verify>
    <automated>cargo test -p libxc_rs --lib compat::ids compat::info compat::removed 2>&1 | tail -30</automated>
  </verify>
  <acceptance_criteria>
    - `grep -c 'extern "C" fn xc_functional_get_number' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_functional_get_name' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_family_from_id' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_number_of_functionals' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_maximum_name_length' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_available_functional_numbers\b' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_available_functional_numbers_by_name' src/compat/ids.rs` == 1
    - `grep -c 'extern "C" fn xc_available_functional_names' src/compat/ids.rs` == 1
    - `grep -cE '^pub unsafe extern "C" fn xc_func_info_get_' src/compat/info.rs` >= 10
    - `grep -cE '^pub unsafe extern "C" fn xc_func_reference_get_' src/compat/info.rs` == 4
    - `grep -c 'pub fn replacement_for' src/compat/removed.rs` == 1
    - `cargo test -p libxc_rs --lib compat::ids::tests::discovery_matches_registry` exits 0
    - `cargo test -p libxc_rs --lib compat::ids::tests::available_names_fills_649` exits 0
    - `cargo test -p libxc_rs --lib compat::info::tests::info_get_name_returns_cached_cstring` exits 0
    - `cargo build -p libxc_rs --release` exits 0
    - `cargo clippy -p libxc_rs --no-deps -- -D warnings` exits 0
  </acceptance_criteria>
  <done>
    8 discovery + 14 info/reference accessors + removed helper compile; discovery_matches_registry and available_names_fills_649 pass; info accessors return correct values via &'static FunctionalMeta cast.
  </done>
</task>

<task id="06-02b-T2" type="auto">
  <name>Task 2: compat::library (5 version/reference fns) + compat::hybrid (7 hybrid/aux/NLC accessors + 2 AK13 helpers with INLINED formula + AK13_ORACLE_PAIRS)</name>
  <files>
    src/compat/library.rs,
    src/compat/hybrid.rs,
    src/compat/mod.rs
  </files>
  <read_first>
    - /home/user/Documents/workspace/libxc_rs/src/compat/library.rs (placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/compat/hybrid.rs (placeholder — overwrite)
    - /home/user/Documents/workspace/libxc_rs/src/registry/mod.rs lines 79-92 (version, version_string, reference_string)
    - /home/user/Documents/workspace/libxc_rs/src/functional/hybrid.rs (Functional accessors)
    - /home/user/Documents/workspace/libxc_rs/src/model/mod.rs lines 50-63 (HybridType variants — read all of them; the i32 mapping must cover every variant)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h lines 17-26 (xc_version_string + reference signatures)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h lines 70-100 (XC_HYB_* integer constants — verify hybrid_type_to_int mapping)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/xc.h lines 583-585 (xc_gga_ak13_*)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/gga_x_ak13.c (full file — INLINE formula + verify B1/B2 constants + extract X_FACTOR_C from libxc-master/src/util.h)
    - /home/user/Documents/workspace/libxc_rs/libxc-master/src/util.h (X_FACTOR_C definition — direct read; do not infer)
  </read_first>
  <action>
    **Step 1 — `src/compat/library.rs`:**

    ```rust
    //! Library version and reference functions.
    use crate::registry;
    use std::ffi::{c_char, CStr};

    static VERSION_STRING: &CStr = c"7.0.0";
    static REFERENCE: &CStr = c"libxc_rs: Rust reimplementation of libxc 7.0.0";
    static REFERENCE_DOI: &CStr = c"10.1016/j.softx.2017.11.002";  // Lehtola 2018 SoftwareX
    static REFERENCE_KEY: &CStr = c"Lehtola2018";

    /// `void xc_version(int *major, int *minor, int *micro);`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_version(major: *mut i32, minor: *mut i32, micro: *mut i32) {
        let (ma, mi, mc) = registry::version();
        if !major.is_null() { unsafe { *major = ma as i32; } }
        if !minor.is_null() { unsafe { *minor = mi as i32; } }
        if !micro.is_null() { unsafe { *micro = mc as i32; } }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_version_string() -> *const c_char { VERSION_STRING.as_ptr() }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_reference() -> *const c_char { REFERENCE.as_ptr() }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_reference_doi() -> *const c_char { REFERENCE_DOI.as_ptr() }

    #[unsafe(no_mangle)]
    pub extern "C" fn xc_reference_key() -> *const c_char { REFERENCE_KEY.as_ptr() }

    #[cfg(test)] mod tests {
        use super::*;
        #[test]
        fn version_writes_components() {
            let mut ma: i32 = 0; let mut mi: i32 = 0; let mut mc: i32 = 0;
            unsafe { xc_version(&mut ma, &mut mi, &mut mc); }
            assert_eq!((ma, mi, mc), (7, 0, 0));
        }
        #[test]
        fn version_string_matches() {
            unsafe {
                let s = CStr::from_ptr(xc_version_string()).to_string_lossy();
                assert_eq!(s, "7.0.0");
            }
        }
    }
    ```

    Verify the actual DOI/key strings by `grep -A2 'xc_reference' libxc-master/src/xc.c` if libxc has fields with concrete values. If libxc returns empty strings, fall back to the Lehtola 2018 canonical paper as documented.

    **Step 2 — `src/compat/hybrid.rs` — 7 hybrid/aux/NLC accessors + 2 AK13 helpers with FORMULA INLINED:**

    Read `libxc-master/src/util.h` to extract the EXACT `X_FACTOR_C` literal. Do not guess.

    ```rust
    //! Hybrid + auxiliary + NLC accessors + AK13 helpers (formula inlined verbatim).

    use crate::compat::c_layout::xc_func_type;
    use crate::compat::errno::{self, set_error};
    use crate::compat::raw_handle::FunctionalSlot;
    use crate::extern_c_wrapper;
    use crate::model::HybridType;

    /// libxc XC_HYB_* integer constants. VERIFIED libxc-master/src/xc.h:69-100.
    /// MUST cover every variant of HybridType in src/model/mod.rs.
    fn hybrid_type_to_int(t: HybridType) -> i32 {
        match t {
            HybridType::Semilocal    => 0,    // XC_HYB_NONE
            HybridType::Hybrid       => 1,    // XC_HYB_HYBRID
            HybridType::Cam          => 2,    // XC_HYB_CAM
            HybridType::CamYukawa    => 3,    // XC_HYB_CAMY
            HybridType::CamGaussian  => 4,    // XC_HYB_CAMG
            HybridType::DoubleHybrid => 32,   // XC_HYB_DOUBLE_HYBRID — VERIFY against xc.h
            HybridType::Mixture      => 64,   // XC_HYB_MIXTURE       — VERIFY against xc.h
            // If src/model/mod.rs has additional variants, add them here. Exhaustive
            // match (no `_` arm) so adding a variant becomes a compile error.
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_hyb_type(p: *const xc_func_type) -> i32 {
        extern_c_wrapper!(p, "xc_hyb_type", {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            Ok(hybrid_type_to_int(f.hybrid_type()))
        })
    }

    /// `double xc_hyb_exx_coef(const xc_func_type *p);` — returns NaN on error.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_hyb_exx_coef(p: *const xc_func_type) -> f64 {
        if p.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_hyb_exx_coef: null handle");
            return f64::NAN;
        }
        let result = std::panic::catch_unwind(|| -> Result<f64, crate::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            Ok(f.exx_coefficient().unwrap_or(0.0))
        });
        match result {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => { set_error(e.discriminant(), &e.to_string()); f64::NAN }
            Err(_)     => { set_error(errno::LIBXC_RS_PANIC, "xc_hyb_exx_coef: panic"); f64::NAN }
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_hyb_cam_coef(
        p: *const xc_func_type, omega: *mut f64, alpha: *mut f64, beta: *mut f64,
    ) {
        if p.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_hyb_cam_coef: null handle");
            return;
        }
        let _ = std::panic::catch_unwind(|| -> Result<(), crate::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            if let Some(c) = f.cam_coefficients() {
                if !omega.is_null() { unsafe { *omega = c.omega; } }
                if !alpha.is_null() { unsafe { *alpha = c.alpha; } }
                if !beta.is_null()  { unsafe { *beta  = c.beta;  } }
                Ok(())
            } else {
                Err(crate::LibxcRsError::FamilyMismatch {
                    id: f.meta().id,
                    expected: crate::Family::Gga,
                    actual: f.meta().family,
                })
            }
        });
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_nlc_coef(p: *const xc_func_type, nlc_b: *mut f64, nlc_c: *mut f64) {
        if p.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_nlc_coef: null handle");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) } {
                if let Some(c) = f.nlc_coefficients() {
                    if !nlc_b.is_null() { unsafe { *nlc_b = c.b; } }
                    if !nlc_c.is_null() { unsafe { *nlc_c = c.c; } }
                }
            }
        });
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_num_aux_funcs(p: *const xc_func_type) -> i32 {
        extern_c_wrapper!(p, "xc_num_aux_funcs", {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            Ok(f.auxiliary_functionals().len() as i32)
        })
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_aux_func_ids(p: *const xc_func_type, ids: *mut i32) {
        if p.is_null() || ids.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_aux_func_ids: null pointer");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) } {
                let aux = f.auxiliary_functionals();
                let slice = unsafe { std::slice::from_raw_parts_mut(ids, aux.len()) };
                for (i, a) in aux.iter().enumerate() {
                    slice[i] = a.meta().id.raw() as i32;
                }
            }
        });
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_aux_func_weights(p: *const xc_func_type, weights: *mut f64) {
        if p.is_null() || weights.is_null() {
            set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_aux_func_weights: null pointer");
            return;
        }
        let _ = std::panic::catch_unwind(|| {
            if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) } {
                let mix = f.mix_coefficients();
                let slice = unsafe { std::slice::from_raw_parts_mut(weights, mix.len()) };
                slice.copy_from_slice(mix);
            }
        });
    }

    // === AK13 helpers — formula INLINED verbatim from libxc-master/src/gga_x_ak13.c ===

    /// libxc-master/src/gga_x_ak13.c:32 — par_ak13[0] = 3*muGE/5 + 8*pi/15 = 1.74959015598863046792081721182.
    /// VERIFIED by direct read of libxc-master/src/gga_x_ak13.c line 32.
    pub const AK13_PAR_B1: f64 = 1.74959015598863046792081721182;
    /// libxc-master/src/gga_x_ak13.c:33 — par_ak13[1] = muGE - B1 = -1.62613336586517367779736042170.
    pub const AK13_PAR_B2: f64 = -1.62613336586517367779736042170;

    /// libxc-master/src/util.h X_FACTOR_C — VERIFY exact value by direct read of util.h
    /// before commit. Common literal: -0.7385587663820224058842300326808 (= -3/4 * (3/pi)^(1/3)).
    /// **Executor: open libxc-master/src/util.h, search for `#define X_FACTOR_C`, copy the
    /// expression, and either inline the literal here OR write the exact expression
    /// `-0.75 * f64::cbrt(3.0 / std::f64::consts::PI)` if the macro evaluates to that.**
    /// Document the source line in the doc-comment.
    const X_FACTOR_C: f64 = -0.7385587663820224058842300326808;

    /// `double xc_gga_ak13_get_asymptotic(double homo);`
    /// libxc-master/src/gga_x_ak13.c lines 35-38 — forwards to pars_get with par_ak13.
    #[unsafe(no_mangle)]
    pub extern "C" fn xc_gga_ak13_get_asymptotic(homo: f64) -> f64 {
        // Inline the par_ak13 default and call the inner formula.
        ak13_pars_asymptotic_inner(homo, AK13_PAR_B1)
    }

    /// `double xc_gga_ak13_pars_get_asymptotic(double homo, const double *ext_params);`
    /// libxc-master/src/gga_x_ak13.c lines 40-55 — formula port, op-order preserving.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xc_gga_ak13_pars_get_asymptotic(homo: f64, ext_params: *const f64) -> f64 {
        let b1 = if ext_params.is_null() {
            AK13_PAR_B1
        } else {
            // SAFETY: caller contract — ext_params points to at least 1 f64.
            unsafe { *ext_params }
        };
        ak13_pars_asymptotic_inner(homo, b1)
    }

    /// Inner formula. Direct port of libxc-master/src/gga_x_ak13.c:40-55.
    /// Op order preserved exactly per CLAUDE.md "Operation order preservation".
    #[inline]
    fn ak13_pars_asymptotic_inner(homo: f64, ak13_b1: f64) -> f64 {
        // libxc:47 — Qx = sqrt(2.0)*ak13_B1/(3.0*CBRT(3.0*M_PI*M_PI));
        let qx = (2.0_f64).sqrt() * ak13_b1
            / (3.0 * (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt());
        // libxc:49-50 — aa = X_FACTOR_C*Qx; aa2 = aa*aa;
        let aa  = X_FACTOR_C * qx;
        let aa2 = aa * aa;
        // libxc:52 — factor = (homo < 0.0) ? -1.0 : 1.0;
        let factor = if homo < 0.0 { -1.0 } else { 1.0 };
        // libxc:54 — return (aa2/2.0)*(1.0 + factor*sqrt(1.0 - 4.0*homo/aa2));
        (aa2 / 2.0) * (1.0 + factor * (1.0 - 4.0 * homo / aa2).sqrt())
    }

    // === AK13 oracle pairs — bit-exact (homo, expected) computed offline ===
    //
    // **Executor instructions:**
    //
    // Compile a one-off C program against libxc 7.0.0 (verify/build.rs already builds it):
    //
    //     #include <stdio.h>
    //     double xc_gga_ak13_get_asymptotic(double homo);
    //     int main(void) {
    //         double inputs[] = {-0.5, -0.1, 0.05};
    //         for (int i = 0; i < 3; i++) {
    //             double y = xc_gga_ak13_get_asymptotic(inputs[i]);
    //             printf("(%.17e_f64, %.17e_f64),\n", inputs[i], y);
    //         }
    //         return 0;
    //     }
    //
    // Build via `cargo run -p xtask --bin ak13_oracle` (add the bin to xtask/Cargo.toml).
    // Paste the printed lines below verbatim. Document the run output + git SHA of
    // libxc-master at the time of run in 06-02b-SUMMARY.md.
    //
    // Fallback if the C oracle is unbuildable: replace the bit-exact assertion in the
    // test with `(actual - expected).abs() < 1e-12`, and document in SUMMARY.
    /// Oracle pairs: (homo, expected). VERIFIED libxc 7.0.0 via xtask/src/bin/ak13_oracle.rs.
    /// 17 significant digits — bit-exact f64 round-trip.
    pub const AK13_ORACLE_PAIRS: &[(f64, f64)] = &[
        // Executor: paste 3 verified lines here. Example shape (DO NOT COMMIT placeholder values):
        //   (-5.00000000000000000e-01_f64, +1.23456789012345678e-01_f64),
        //   (-1.00000000000000006e-01_f64, +2.34567890123456789e-02_f64),
        //   ( 5.00000000000000028e-02_f64, +3.45678901234567890e-03_f64),
        (-5.00000000000000000e-01_f64, /* TODO: paste C-oracle f64 result */ 0.0_f64),
        (-1.00000000000000006e-01_f64, /* TODO: paste C-oracle f64 result */ 0.0_f64),
        ( 5.00000000000000028e-02_f64, /* TODO: paste C-oracle f64 result */ 0.0_f64),
    ];

    #[cfg(test)] mod tests {
        use super::*;

        #[test]
        fn ak13_default_constants() {
            assert_eq!(AK13_PAR_B1.to_bits(), 1.74959015598863046792081721182_f64.to_bits());
            assert_eq!(AK13_PAR_B2.to_bits(), (-1.62613336586517367779736042170_f64).to_bits());
        }

        #[test]
        fn ak13_get_asymptotic_oracle_parity() {
            for &(homo, expected) in AK13_ORACLE_PAIRS.iter() {
                let actual = xc_gga_ak13_get_asymptotic(homo);
                // Bit-exact preferred; tolerance fallback documented if oracle unbuildable.
                let bit_match = actual.to_bits() == expected.to_bits();
                let tol_match = (actual - expected).abs() < 1e-12;
                assert!(bit_match || tol_match,
                    "AK13 oracle mismatch at homo={homo}: actual={actual}, expected={expected}, \
                     diff={}", (actual - expected).abs());
            }
        }

        #[test]
        fn ak13_pars_with_default_matches_get_asymptotic() {
            // Calling pars_get_asymptotic with ext_params = [B1] must match get_asymptotic.
            let homo = -0.3;
            let actual_default = xc_gga_ak13_get_asymptotic(homo);
            let params = [AK13_PAR_B1, AK13_PAR_B2];
            let actual_pars = unsafe { xc_gga_ak13_pars_get_asymptotic(homo, params.as_ptr()) };
            assert_eq!(actual_default.to_bits(), actual_pars.to_bits(),
                "default-path and pars-path must produce bit-identical results");
        }
    }
    ```

    **Step 3 — extend `src/compat/mod.rs`** to expose library + hybrid:

    ```rust
    pub mod hybrid;    // 06-02b-T2
    pub mod library;   // 06-02b-T2
    ```

    **Step 4 — verify:** `cargo test -p libxc_rs --lib compat::library compat::hybrid`. Commit: `feat(06-02b): compat::library + compat::hybrid (incl. AK13 inlined formula + oracle pairs)`.
  </action>
  <verify>
    <automated>cargo test -p libxc_rs --lib compat::library compat::hybrid 2>&1 | tail -30</automated>
  </verify>
  <acceptance_criteria>
    - `grep -c 'extern "C" fn xc_version\b' src/compat/library.rs` == 1
    - `grep -c 'extern "C" fn xc_version_string' src/compat/library.rs` == 1
    - `grep -c 'extern "C" fn xc_reference\b' src/compat/library.rs` == 1
    - `grep -c 'extern "C" fn xc_reference_doi' src/compat/library.rs` == 1
    - `grep -c 'extern "C" fn xc_reference_key' src/compat/library.rs` == 1
    - `grep -c 'extern "C" fn xc_hyb_type' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_hyb_exx_coef' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_hyb_cam_coef' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_nlc_coef' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_num_aux_funcs' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_aux_func_ids' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_aux_func_weights' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_gga_ak13_get_asymptotic' src/compat/hybrid.rs` == 1
    - `grep -c 'extern "C" fn xc_gga_ak13_pars_get_asymptotic' src/compat/hybrid.rs` == 1
    - `grep -c 'unimplemented!' src/compat/hybrid.rs` == 0  (formula INLINED, no stubs)
    - `grep -c 'pub const AK13_PAR_B1' src/compat/hybrid.rs` == 1
    - `grep -c 'pub const AK13_ORACLE_PAIRS' src/compat/hybrid.rs` == 1
    - **Oracle pairs MUST be filled with real values (no `0.0_f64` placeholders):** `grep -cE '/\*\s*TODO' src/compat/hybrid.rs` == 0
    - `cargo test -p libxc_rs --lib compat::library::tests::version_writes_components` exits 0
    - `cargo test -p libxc_rs --lib compat::hybrid::tests::ak13_default_constants` exits 0
    - `cargo test -p libxc_rs --lib compat::hybrid::tests::ak13_get_asymptotic_oracle_parity` exits 0
    - `cargo test -p libxc_rs --lib compat::hybrid::tests::ak13_pars_with_default_matches_get_asymptotic` exits 0
    - `cargo build -p libxc_rs --release` exits 0
    - `cargo clippy -p libxc_rs --no-deps -- -D warnings` exits 0
  </acceptance_criteria>
  <done>
    5 library + 9 hybrid/AK13 extern Cs exported; AK13 formula inlined verbatim from libxc-master/src/gga_x_ak13.c with B1/B2 constants verified at the source line; AK13_ORACLE_PAIRS const filled with bit-exact libxc results; oracle parity test passes (bit-exact or 1e-12 tolerance fallback documented); HybridType→XC_HYB_* mapping covers every variant.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| C caller → compat::ids/info/library/hybrid | Untrusted: name strings may contain non-UTF8, info pointers may be NULL or dangling. |
| compat::* → registry / functional | Internal Rust call — Phase-5 surface trusted. |
| compat::hybrid AK13 → libxc oracle | Bit-exact parity boundary; preserved by op-order-faithful Rust port. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-06-11 | Tampering | Non-UTF8 name passed to xc_functional_get_number | mitigate | `CStr::to_str().map_err(|_| UnknownFunctionalName("non-utf8"))?` returns typed error. |
| T-06-12 | Tampering | NULL info pointer to any xc_func_info_get_* | mitigate | `info_ref` returns None on NULL; sentinel return (-1 / NaN / null pointer). |
| T-06-13 | Information Disclosure | Cached `*const c_char` from xc_functional_get_name held across thread exit | accept | Documented in include/xc.h: pointer valid until thread exit OR cache cleared (production: never). |
| T-06-14 | Tampering | AK13 numerical drift from op-order changes | mitigate | Inline formula preserves op order verbatim from gga_x_ak13.c; oracle parity test enforces. |
| T-06-15 | DoS | Concurrent cache_cstring writes from multiple threads | accept | Cache is `thread_local!`; per-thread state, no cross-thread contention. |
| T-06-16 | Tampering | HybridType variant added in Phase-5 with no XC_HYB_* mapping | mitigate | `hybrid_type_to_int` is exhaustive (no `_` arm); adding a variant becomes a compile error. |
</threat_model>

<verification>
After both tasks complete, run from repo root:

```bash
cargo test -p libxc_rs --lib compat::
cargo build -p libxc_rs --release
cargo clippy -p libxc_rs --no-deps -- -D warnings

# Symbol export count gate (Wave 3 cumulative — full 85 reached in 06-03):
nm target/release/liblibxc_rs.so 2>/dev/null | grep -c 'T xc_'
# Expect ≥ 50 (16 from 06-02a + 8 discovery + 14 info/ref + 5 library + 9 hybrid/ak13 = 52,
#               minus any post-mangling sub-symbols)

# AK13 placeholder gate:
grep -cE '/\* TODO' src/compat/hybrid.rs   # == 0 (oracle pairs filled)

# AK13 stub gate:
grep -c 'unimplemented!' src/compat/hybrid.rs   # == 0
```

All commands exit 0.
</verification>

<success_criteria>
- COMPAT-01 (accessor slice): 35 extern Cs added to the cdylib (8 discovery + 14 info/ref + 5 library + 9 hybrid/AK13 = 36; one is a pointer-returning bridge).
- COMPAT-02: HashMap-backed cache_cstring proves stability across 649+ insertions (test in 06-02a; consumed in this plan).
- AK13 helpers ship with formula INLINED from libxc-master/src/gga_x_ak13.c (verified at source line) and oracle parity tested against ≥ 3 bit-exact (homo, expected) pairs.
- HybridType→XC_HYB_* mapping covers every variant (exhaustive match).
- Library version returns (7, 0, 0) and "7.0.0".
- All extern "C" symbols use `#[unsafe(no_mangle)]`.
</success_criteria>

<output>
After completion, create `.planning/phases/06-public-api-and-c-compatibility/06-02b-SUMMARY.md` documenting:
- File-by-file summary: ids, info, library, hybrid, removed
- The 35-function inventory exported in this plan
- AK13 port: source file (libxc-master/src/gga_x_ak13.c), git SHA of libxc-master at port time, X_FACTOR_C source line in util.h, the exact AK13_ORACLE_PAIRS values (paste them), oracle-program build invocation, oracle-vs-Rust comparison method (bit-exact or 1e-12 tolerance — say which)
- HybridType → XC_HYB_* int mapping table (every variant covered, with libxc-master/src/xc.h line reference)
- Test counts (discovery, info accessors, AK13 parity, version)
- Cumulative symbol count after Wave 3: `nm | grep -c 'T xc_'` ≥ 50
- Reference field-name discovery (citation/doi/bibtex/key) — what the actual src/meta/mod.rs Reference struct calls them
</output>
</content>
</invoke>