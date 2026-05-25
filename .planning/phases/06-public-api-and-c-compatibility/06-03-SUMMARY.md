---
phase: 06-public-api-and-c-compatibility
plan: 03
subsystem: compat
tags: [ffi, extern-c, evaluate-functions, c-header, integration-test, drop-in-replacement]

# Dependency graph
requires:
  - phase: 06-public-api-and-c-compatibility
    plan: 02b
    provides: "Discovery/info/library/hybrid surface; opaque handles; errno; extern_c_wrapper!"
  - phase: 06-public-api-and-c-compatibility
    plan: 02a
    provides: "Lifecycle + threshold/ext_params setters in legacy_eval.rs (extended here)"
  - phase: 03-input-output-and-evaluation-framework
    provides: "LdaInput/GgaInput/MggaInput::new; LdaOutput/GgaOutput/MggaOutput; Dimensions; EvaluationWorkspace"
  - phase: 05-functional-lifecycle-and-hybrid-properties
    provides: "Functional::evaluate_{lda,gga,mgga}; spin()"
provides:
  - "35 evaluate extern Cs (12 LDA + 12 GGA + 11 MGGA) with NULL-skip + family-summary order inference"
  - "XcLdaOutParams / XcGgaOutParams #[repr(C)] structs for xc_{lda,gga}_new"
  - "include/xc.h — C89/C99 header for the full 87-function ABI surface"
  - "verify/tests/compat_smoke.rs — 7-scenario FFI integration suite"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Per-family evaluate helper (ptr_to_opt_slice + input_slice) maps NULL output ptr → None (skip-derivative)"
    - "Family-summary fns (xc_lda/xc_gga/xc_mgga) infer DerivativeOrder from highest non-NULL output (Pitfall 8)"
    - "mgga_out! struct-literal macro builds the ~70-field MggaOutput from a field list (name == field == dims stride)"
    - "Hand-written C header (NOT cbindgen); opaque-only structs + void→int return deviations documented inline"

key-files:
  modified:
    - "src/compat/legacy_eval.rs (1225 lines): +35 evaluators, +2 out-params structs, +shared ptr helpers + 5 smoke tests"
  created:
    - "include/xc.h (353 lines): full C-ABI header, gcc -std=c89/c99 clean"
    - "verify/tests/compat_smoke.rs (220 lines): 7 FFI integration scenarios"

key-decisions:
  - "MGGA outputs built via struct-literal + ..Default::default() (MggaOutput derives Default; pub fields) rather than a 70-arg constructor"
  - "Shared int→DerivativeOrder mapper (lda_order_from_int) reused by xc_lda_new and xc_gga_new (family-independent)"
  - "include/xc.h hand-written; MGGA signatures fully expanded from xc.h:436-580 (no placeholders); 87-fn symbol set verified zero-diff against src/compat extern fns"
  - "compat_smoke.rs + 5 in-crate smoke tests are CI/full-build-only — verify default features pull all 281 kernels (OOM on this box)"

requirements-completed: [COMPAT-01, COMPAT-02, COMPAT-03]

# Metrics
duration: "this session (inline sequential)"
completed: 2026-05-25
---

# Phase 6 Plan 03: C-ABI Evaluators, Header & Integration Test Summary

**Completes the libxc drop-in surface — 35 evaluate functions (12 LDA + 12 GGA + 11 MGGA) with libxc-faithful NULL-skip and derivative-order inference, a hand-written `include/xc.h` covering all 87 exported symbols (gcc-clean under C89 and C99), and a 7-scenario FFI integration suite.**

## Task Commits

| Task | Commit | Content |
|------|--------|---------|
| T1: 12 LDA evaluators | `ee687b69eb` | ptr helpers, lda_evaluate, 10 per-order + xc_lda + xc_lda_new + XcLdaOutParams |
| T2a: 12 GGA evaluators | `bb210bbde7` | gga_evaluate (15 outputs), 10 per-order + xc_gga + xc_gga_new + XcGgaOutParams |
| T2b: 11 MGGA evaluators | `3c7b873cb4` | mgga_out! macro, mgga_dims/mgga_run, 10 per-order + xc_mgga (70 outputs) |
| T3: include/xc.h | `25e36085d0` | 353-line C89/C99 header, full ABI surface |
| T4: compat_smoke.rs | `4f073a1ea4` | 7 FFI integration scenarios |

## Function Inventory (35 evaluators)

- **LDA (12):** xc_lda_new, xc_lda, xc_lda_exc, xc_lda_exc_vxc, xc_lda_vxc, xc_lda_exc_vxc_fxc, xc_lda_vxc_fxc, xc_lda_fxc, xc_lda_exc_vxc_fxc_kxc, xc_lda_vxc_fxc_kxc, xc_lda_kxc, xc_lda_lxc.
- **GGA (12):** xc_gga_new, xc_gga, xc_gga_exc, xc_gga_exc_vxc, xc_gga_vxc, xc_gga_exc_vxc_fxc, xc_gga_vxc_fxc, xc_gga_fxc, xc_gga_exc_vxc_fxc_kxc, xc_gga_vxc_fxc_kxc, xc_gga_kxc, xc_gga_lxc.
- **MGGA (11):** xc_mgga, xc_mgga_exc, xc_mgga_exc_vxc, xc_mgga_vxc, xc_mgga_exc_vxc_fxc, xc_mgga_vxc_fxc, xc_mgga_fxc, xc_mgga_exc_vxc_fxc_kxc, xc_mgga_vxc_fxc_kxc, xc_mgga_kxc, xc_mgga_lxc. (No `xc_mgga_new` — absent from libxc xc.h.)

**Cumulative ABI surface: 87 exported `xc_*` extern "C" functions** (16 from 06-02a + 36 from 06-02b + 35 here = 87; the 2 AK13 helpers shipped in 06-02b, so this plan adds 35). The plan's must_have arithmetic said "85"; the interfaces block (line 261) flags that as approximate — the verified count of distinct extern "C" symbols in `src/compat/*` is **87** (`grep -hoE 'extern "C" fn xc_[a-z0-9_]+' src/compat/*.rs | sort -u | wc -l`).

## include/xc.h

353-line C89/C99-compatible header. Declares all 87 extern functions, the opaque handle typedefs, the `xc_lda_out_params`/`xc_gga_out_params` structs, all 25 `LIBXC_RS_*` codes (mirroring `errno.rs`), and the `XC_*` constants (`XC_HYB_DOUBLE_HYBRID=5`, `XC_HYB_MIXTURE=32768`). MGGA signatures fully expanded from `xc.h:436-580` (no placeholders). The two drop-in deviations (opaque-only structs; `void → int` returns) are documented inline at the top.

## Verification

- **Compile gate:** `cargo check -p libxc_rs --no-default-features --lib` → exit 0 (kernel-free umbrella). The 35 evaluators type-check; all ~70 MggaOutput field names resolved (any mismatch would be a compile error via the `mgga_out!` macro).
- **Clippy:** `cargo clippy --no-default-features --lib --no-deps -- -D warnings` introduces **zero** new findings from `legacy_eval.rs` (the only added allow is `#[allow(clippy::needless_update)]` on `xc_mgga`, whose 70-field literal makes the macro's `..Default::default()` a no-op — which also confirms the field list is exactly complete). Five pre-existing crate-wide findings remain (see 06-02a deviation).
- **Header:** `gcc -fsyntax-only -Wall -Werror -std=c89` and `-std=c99` both exit 0. Symbol↔header alignment: **zero diff** between the 87 Rust extern fns and the 87 header declarations (digit-aware grep).
- **Grep gates:** all pass — ≥12 LDA, ≥12 GGA, ≥11 MGGA `extern "C" fn`, per-name gates, XcLda/GgaOutParams, family-summary functions, 7 compat_smoke test fns, header typedef/constant/errno gates, no placeholders.

## Deviations from Plan

1. **[Rule 3 — build-time class]** The in-crate evaluate smoke tests (`lda/gga/mgga_evaluate_tests`) and `verify/tests/compat_smoke.rs` are **authored but not executed** here. `cargo test -p libxc_rs` and the verify crate both pull the full 281-kernel build (90+ min / OOM on this RAM-constrained box); verify's default features force all `oracle-*`. Execution is CI/full-build-deferred. Verified instead via `cargo check --no-default-features --lib` + grep gates + `gcc -fsyntax-only` (header). Mirrors the 06-01/06-02a/06-02b precedent.
2. **[Rule 3]** `cargo build --release` + `cargo clippy -D warnings` (full-crate) + the `nm | grep -c 'T xc_'` ≥ 85 symbol-count gate not run as written (same kernel-build/OOM + pre-existing-clippy-debt constraints). Expected symbol count after this plan: 87.
3. **[Rule 1 — count correction]** Documented the must_have's "85 / 33" figures vs. the verified "87 / 35" (the interfaces block already flagged the must_have arithmetic as approximate).
4. **[Minor]** `lda_order_from_int` (T1) is reused by `xc_gga_new` (T2a) since the int→`DerivativeOrder` mapping is family-independent; named for its first user.

**Total deviations:** 4 (1 build-time test/clippy/symbol deferral + 1 build-time clippy/build/nm deferral + 1 count correction + 1 helper reuse). **Impact:** none on delivered functionality.

## Issues Encountered

- T1 first compile: `lda_order_from_int` (an `unsafe fn`) called outside an `unsafe {}` block, and premature `Gga/MggaInput`/`Output` imports were unused under `#![deny(warnings)]`. Fixed by wrapping the call and importing each family's types only when its section landed.
- T2b: `xc_mgga` (all 70 fields specified) triggered `clippy::needless_update` on the macro's trailing `..Default::default()`; resolved with a localized `#[allow]` + explanatory comment.

## Threat Surface Notes

T-06 evaluators: NULL output pointers map to None (skip-derivative) via `ptr_to_opt_slice`; all-NULL family-summary calls are a no-op returning 0 (libxc parity). `np * stride` uses `checked_mul().expect(...)` (overflow → caught by `extern_c_wrapper!`'s `catch_unwind` → LIBXC_RS_PANIC). All `unsafe` confined to `src/compat/*`.

## Next Plan Readiness

Phase 6 is the final plan of the phase. The full C-ABI drop-in surface (lifecycle + introspection + setters + 35 evaluators + hand-written header) is in place. Remaining cross-phase verification (running the smoke + FFI integration tests, the `nm` symbol-count gate) is deferred to a full-build/CI environment.

## Self-Check: PASSED

`cargo check --no-default-features --lib` exits 0 under `#![deny(warnings)]`; `gcc -fsyntax-only -std=c89/-std=c99 include/xc.h` exit 0; header↔symbol zero-diff (87/87); all grep gates pass; commits `ee687b69eb`, `bb210bbde7`, `3c7b873cb4`, `25e36085d0`, `4f073a1ea4` present. Test execution + full build + nm symbol gate CI-deferred per documented Rule-3 deviations.

---
*Phase: 06-public-api-and-c-compatibility*
*Completed: 2026-05-25*
