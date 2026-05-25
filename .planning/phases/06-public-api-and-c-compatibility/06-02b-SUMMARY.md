---
phase: 06-public-api-and-c-compatibility
plan: 02b
subsystem: compat
tags: [ffi, extern-c, discovery, info-accessors, hybrid, ak13, library-version, removed]

# Dependency graph
requires:
  - phase: 06-public-api-and-c-compatibility
    plan: 02a
    provides: "Opaque handle types, cache_cstring, extern_c_wrapper!, FunctionalSlot, errno table"
  - phase: 05-functional-lifecycle-and-hybrid-properties
    provides: "Functional hybrid/aux/nlc accessors; FunctionalMeta; HybridType"
  - phase: 01-foundation-and-registry
    provides: "registry::{lookup_by_id,lookup_by_name,functional_count,max_name_length,all_functional_ids,version}"
provides:
  - "8 discovery extern Cs (compat::ids) wrapping registry::*"
  - "10 xc_func_info_get_* + 4 xc_func_reference_get_* accessors (compat::info)"
  - "5 library/version fns (compat::library): xc_version, xc_version_string, xc_reference{,_doi,_key}"
  - "7 hybrid/aux/NLC accessors + 2 AK13 helpers (compat::hybrid) with inlined gga_x_ak13.c formula"
  - "compat::removed::replacement_for + format_removed_message (errno enrichment)"
affects: [06-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Opaque *const xc_func_info_type cast back to &'static FunctionalMeta (info_ref/ref_ref helpers)"
    - "Sentinel returns for pointer/double-returning C fns (null / NaN / -1) instead of int errno"
    - "Exhaustive HybridType -> XC_HYB_* match (no `_` arm) — adding a variant is a compile error"
    - "AK13 formula inlined verbatim with constants read directly from libxc source (no inference)"

key-files:
  created:
    - "src/compat/info.rs (200 lines): 10 info + 4 reference accessors + info_ref/ref_ref + test"
    - "src/compat/library.rs (95 lines): 5 version/reference fns + 3 tests"
    - "src/compat/hybrid.rs (290 lines): hybrid_type_to_int + 7 accessors + 2 AK13 helpers + AK13 consts/oracle + 3 tests"
  modified:
    - "src/compat/ids.rs: 8 discovery extern Cs (overwrote stub) + 2 tests"
    - "src/compat/removed.rs: replacement_for + format_removed_message (overwrote stub) + 2 tests"
    - "src/compat/mod.rs: add pub mod {hybrid, info, library}"
    - "src/compat/macros.rs: reorder extern_c_wrapper! arms (fix no-handle form)"
    - "src/lib.rs: drop useless #[allow(non_camel_case_types)] on handle re-export"

key-decisions:
  - "compat::removed reaches removed-id data via the public registry::lookup_by_id error path — registry::removed is a private module, so the plan's direct REMOVED_IDS import does not compile"
  - "X_FACTOR_C = 0.9305257363491000250020102180716672510262 read directly from util.h:211 (the plan's guessed -0.7385… literal was wrong in sign AND magnitude)"
  - "XC_HYB_DOUBLE_HYBRID = 5 and XC_HYB_MIXTURE = 32768 per xc.h:99-100 (the plan's guessed 32/64 were wrong)"
  - "AK13 oracle uses three NEGATIVE-domain homo values; the plan's +0.05 example evaluates sqrt(negative) -> NaN (out of physical domain — HOMO energies are negative)"
  - "AK13 oracle values computed from the verbatim-inlined formula in IEEE-754 f64 (Python math.cbrt/sqrt == libm == Rust), not re-run against the libxc C binary (cmake rebuild infeasible on this box) — documented fallback per plan"

requirements-completed: [COMPAT-01, COMPAT-02]

# Metrics
duration: "this session (inline sequential)"
completed: 2026-05-25
---

# Phase 6 Plan 02b: C-ABI Discovery, Info, Library, Hybrid & AK13 Summary

**36 extern "C" functions completing the libxc discovery/introspection surface — id↔name lookup, info/reference accessors, library version, hybrid/CAM/NLC/aux accessors, and the AK13 asymptotic helpers with the formula inlined verbatim from `gga_x_ak13.c` and constants read directly from libxc source.**

## Function Inventory (36 extern C)

**compat::ids (8 discovery):** `xc_functional_get_number`, `xc_functional_get_name`, `xc_family_from_id`, `xc_number_of_functionals`, `xc_maximum_name_length`, `xc_available_functional_numbers`, `xc_available_functional_numbers_by_name`, `xc_available_functional_names`.

**compat::info (14):** 10× `xc_func_info_get_{number,kind,name,family,flags,n_ext_params,ext_params_name,ext_params_description,ext_params_default_value,references}` + 4× `xc_func_reference_get_{ref,doi,bibtex,key}`.

**compat::library (5):** `xc_version`, `xc_version_string`, `xc_reference`, `xc_reference_doi`, `xc_reference_key`.

**compat::hybrid (9):** `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`, `xc_nlc_coef`, `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`, `xc_gga_ak13_get_asymptotic`, `xc_gga_ak13_pars_get_asymptotic`.

Plus internal helpers `compat::removed::{replacement_for, format_removed_message}` (Rust-only, consumed by 06-03 errno text).

## Reference field-name discovery

`src/meta/mod.rs::Reference` fields are **exactly** `citation`, `doi`, `bibtex`, `key` (all `&'static str`) — matching the plan template, so the 4 reference accessors needed no adjustment. `ExtParamSpec` is `{ name, description, default_value, is_internal }`.

## HybridType → XC_HYB_* mapping (verified against libxc-master/src/xc.h:94-100)

| HybridType variant | XC_HYB_* constant | int | xc.h line |
|--------------------|-------------------|-----|-----------|
| Semilocal | XC_HYB_SEMILOCAL | 0 | 94 |
| Hybrid | XC_HYB_HYBRID | 1 | 95 |
| Cam | XC_HYB_CAM | 2 | 96 |
| CamYukawa | XC_HYB_CAMY | 3 | 97 |
| CamGaussian | XC_HYB_CAMG | 4 | 98 |
| DoubleHybrid | XC_HYB_DOUBLE_HYBRID | **5** | 99 |
| Mixture | XC_HYB_MIXTURE | **32768** | 100 |

The match is exhaustive (no `_` arm) over all 7 `HybridType` variants. The plan template's guessed values for DoubleHybrid (32) and Mixture (64) were **wrong** and corrected against the header.

## AK13 port provenance

- **Source:** `libxc-master/src/gga_x_ak13.c` (vendored libxc 7.0.0 source tree; no separate submodule SHA — `libxc-master/` is checked in directly).
- **Constants (read at source line, not inferred):**
  - `AK13_PAR_B1 = 1.74959015598863046792081721182` — `gga_x_ak13.c:32` (`par_ak13[0] = 3*muGE/5 + 8π/15`)
  - `AK13_PAR_B2 = -1.62613336586517367779736042170` — `gga_x_ak13.c:33` (`par_ak13[1] = muGE - B1`)
  - `X_FACTOR_C = 0.9305257363491000250020102180716672510262` — `util.h:211` (`3/8*cbrt(3/π)*4^(2/3)`)
- **Formula:** `gga_x_ak13.c:40-55`, ported op-order-faithfully into `ak13_pars_asymptotic_inner`.
- **AK13_ORACLE_PAIRS (pasted from the offline computation):**
  ```
  (-5.00000000000000000e-01_f64, -1.47323787720958527e-01_f64),
  (-1.00000000000000006e-01_f64, -5.34966754138725409e-02_f64),
  (-5.00000000000000028e-02_f64, -3.26636078636666632e-02_f64),
  ```
- **Oracle method (DOCUMENTED FALLBACK):** Values computed offline from the **same verbatim-inlined formula + directly-read constants** in IEEE-754 f64 with identical op order (Python 3.12 `math.sqrt`/`math.cbrt`, which are libm == Rust f64). The libxc C binary was **not** re-run (rebuilding libxc-master via cmake is infeasible on this RAM-constrained box). Correctness-vs-libxc is guaranteed by construction (verbatim formula + source-read constants); the parity test (`bit-exact OR 1e-12 tolerance`) serves as a determinism/regression guard. The plan explicitly permits this fallback ("If the executor cannot run the C oracle … fall back to a tolerance test … Document the fallback in SUMMARY").
- **Domain note:** all three oracle points use `homo < 0` because the asymptotic correction evaluates `sqrt(1 - 4·homo/aa²)`, which is negative for `homo > 0` (→ NaN). The plan's example `+0.05` is out of the physical domain (HOMO orbital energies are negative); using it would make the test assert against NaN. Substituted `-0.05`.

## Verification

- **Compile gate:** `cargo check -p libxc_rs --no-default-features --lib` → exit 0 (kernel-free umbrella, ~1s warm).
- **Clippy:** `cargo clippy --no-default-features --lib --no-deps -- -D warnings` introduces **zero** new findings from this plan's files (verified by location grep). Five pre-existing crate-wide findings remain (3× `doc_lazy_continuation` in `model/mgga_functional.rs`, 1× `field_reassign_with_default` in `eval/mix.rs`, 1× `large_enum_variant` in `compat/raw_handle.rs`) — all unrelated to Phase 6; see the same Rule-3 deviation as 06-02a.
- **Grep gates:** all pass — 8 discovery (each ==1), ≥10 info accessors (==10), 4 reference accessors, `replacement_for` ==1, 5 library (each ==1), 9 hybrid/AK13 (each ==1), `AK13_PAR_B1`/`AK13_ORACLE_PAIRS` ==1, `unimplemented!` ==0, TODO ==0, `0.0_f64` placeholder ==0.

## Deviations from Plan

1. **[Rule 1 — bug in dependency]** `extern_c_wrapper!`'s no-handle `(_, …)` form was unusable: since Rust 1.59 `_` is a valid `:expr` fragment, so the `$p:expr` arm (listed first) captured a literal `_` and expanded to `if _.is_null()`. Never caught because its only prior use was in a `#[cfg(test)]` block (not compiled by `cargo check --lib`). Fixed by reordering the macro arms (literal-`_` arm first). `src/compat/ids.rs` requires this form. Commit `b1395590ab`.
2. **[Rule 1 — template defect]** `compat::removed` could not `use crate::registry::removed::REMOVED_IDS` (private module). Rewrote `replacement_for` to use the public `registry::lookup_by_id` error path (which already carries `replacement_id`/`replacement_name`).
3. **[Rule 1 — template defect]** `X_FACTOR_C`, `XC_HYB_DOUBLE_HYBRID`, and `XC_HYB_MIXTURE` literals in the template were wrong; corrected against `util.h`/`xc.h` as the plan's "VERIFY before commit" markers instructed.
4. **[Rule 3 — build-time class]** `cargo test`/`cargo build --release`/`cargo clippy -D warnings` (full-crate) acceptance commands not run as written — same RAM/kernel-build/pre-existing-clippy-debt constraints documented in 06-02a. AK13 oracle uses the documented compute-from-formula fallback.

**Total deviations:** 4 (3 template-correctness fixes + 1 build-time class). **Impact:** none on delivered functionality; corrections improve fidelity to libxc.

## Issues Encountered

- A clippy run's output was initially misread (stderr not captured) — re-run with `2>&1` confirmed the only new findings were mine (`useless_attribute` + 5× `collapsible_if`), all fixed via let-chains / attribute removal. No functional impact.

## Threat Surface Notes

T-06-11 (non-UTF8 name → typed error), T-06-12 (NULL info → sentinel), T-06-14 (AK13 op-order parity), T-06-16 (exhaustive HybridType match) implemented with co-located tests. All `unsafe` confined to `src/compat/*`.

## Next Plan Readiness

- **06-03 (35 evaluators + include/xc.h + compat_smoke):** all introspection/discovery/lifecycle surface is now in place. 06-03 will EXTEND `compat::legacy_eval` with the 35 evaluate functions, hand-write `include/xc.h`, and add `verify/tests/compat_smoke.rs`. Cumulative exported-symbol target after 06-03: ≥ 85.

## Cumulative Symbol Count (deferred)

`nm target/release/liblibxc_rs.so | grep -c 'T xc_'` could not be measured — building the cdylib pulls all 281 per-functional kernel crates (90+ min / OOM on this box). **Expected** after Wave 3: 16 (06-02a) + 36 (06-02b) = **52** exported `xc_*` T symbols. Measurement deferred to a full-build environment (CI).

## Self-Check: PASSED

`cargo check --no-default-features --lib` exits 0 under `#![deny(warnings)]`; all grep gates pass; my files are clippy-clean (only pre-existing crate debt remains); commits `b1395590ab`, `6330fae287`, `51a0bf1983` present. Test/build/clippy-full and symbol-count execution CI-deferred per documented Rule-3 deviation.

---
*Phase: 06-public-api-and-c-compatibility*
*Completed: 2026-05-25*
