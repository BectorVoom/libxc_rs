---
phase: 02-math-core-and-cubecl-substrate
verified: 2026-04-09T08:54:14Z
status: gaps_found
score: 4/5 must-haves verified
overrides_applied: 0
gaps:
  - truth: "erfc_approx matches libm::erfc to within 1e-15 relative error across [-6, 6]"
    status: partial
    reason: "Plan must_have requires 1e-15 relative error; actual test assertion is 1e-10 (100,000x looser). The doc comment on erfc_approx() incorrectly claims 1e-15 accuracy. The erfc sweep test only enforces err < 1e-10. The summary acknowledges the deviation at the region 3/4 polynomial boundary (~x=2.857). While 1e-10 is still well within the 10^-12 energy accuracy target, the erfc precision contract is not met as specified."
    artifacts:
      - path: "src/math/erf.rs"
        issue: "erfc test at line 348 asserts err < 1e-10, not 1e-15 as required by must_have. Doc comment at line 83 claims '1e-15' but the test does not enforce it."
    missing:
      - "Either tighten erfc_approx accuracy to 1e-15 at the region 3/4 boundary, OR update the plan must_have and doc comment to document the actual achieved precision of 1e-10, with explicit justification that this is sufficient for the energy accuracy target"
deferred:
  - truth: "All #[cube] math functions produce identical results on CubeCL CPU backend as on native Rust (cross-backend consistency) - MATH-10"
    addressed_in: "Phase 7"
    evidence: "Developer decision D-09 in 02-CONTEXT.md: 'Cross-backend consistency testing (MATH-10: CPU vs GPU producing identical results) is deferred to Phase 7 when GPU backends are available.' Phase 7 requirements include GPU-01 through GPU-07 and VERIFY-08 covering cross-backend verification."
---

# Phase 02: Math Core and CubeCL Substrate Verification Report

**Phase Goal:** All mathematical building blocks are implemented as #[cube] functions, validated against known values and libm references, and the CubeCL CPU backend produces bit-accurate f64 results for the LDA_X canary kernel
**Verified:** 2026-04-09T08:54:14Z
**Status:** gaps_found
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|---------|
| 1 | safe_cbrt(-8.0) returns -2.0 (not NaN) on CubeCL CPU backend | VERIFIED | tests/math_integration.rs, src/math/powers.rs tests pass; `#[cube] pub fn safe_cbrt` uses sign extraction pattern; 101 unit tests pass |
| 2 | erf and erfc approximations match libm values to within f64 precision across the full input domain | PARTIAL | erf meets 1e-13 (close to f64 precision); erfc only meets 1e-10 (plan must_have required 1e-15); doc comment claims 1e-15 but test only enforces 1e-10 |
| 3 | LDA_X kernel (both unpolarized and polarized) produces energy with relative error <= 10^-12 vs libxc oracle | VERIFIED | 12 oracle tests pass; actual max error ~6e-16 (far exceeds 1e-12 requirement); both spin modes verified |
| 4 | Kernel launch wrapper correctly handles buffer creation, CubeCount/CubeDim calculation, and backend selection | VERIFIED | src/kernel/launch.rs: cpu_client(), calculate_launch_config(), create_input_buffer(), create_zero_output_buffer(), read_output_buffer() all implemented and tested; 11 launch tests pass |
| 5 | MATH-10: All #[cube] math functions produce identical results on CubeCL CPU backend as on native Rust | DEFERRED | D-09 defers to Phase 7; CPU tests verify CubeCL CPU vs libm reference which is the relevant cross-check for Phase 2 |

**Score:** 4/5 truths verified (1 partial, 1 deferred not counted as gap)

### Deferred Items

Items not yet met but explicitly addressed in later milestone phases.

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | MATH-10: Cross-backend consistency (CPU vs GPU identical results) | Phase 7 | Developer decision D-09 in 02-CONTEXT.md: "deferred to Phase 7 when GPU backends are available." Phase 7 requirements include VERIFY-08: "GPU results match CPU results to within 10^-14" |

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `src/math/mod.rs` | Module declarations for 7 submodules | VERIFIED | All 7 submodules declared: constants, powers, piecewise, polynomials, erf, spin, dft_quantities |
| `src/math/constants.rs` | Mathematical constants from libxc util.h | VERIFIED | M_CBRT3=1.4422495703074084, X_FACTOR_C, RS_CONST all present |
| `src/math/powers.rs` | safe_cbrt, pow_1_3/2_3/4_3/5_3 with #[cube] | VERIFIED | 5 functions with #[cube] annotations; libm sweep test at 1000 points |
| `src/math/piecewise.rs` | piecewise3, piecewise5 using select() | VERIFIED | Both functions use branchless `select()` |
| `src/math/polynomials.rs` | poly_eval, rational_eval (Horner) | VERIFIED | poly_eval uses #[comptime] length; Horner evaluation implemented |
| `src/math/erf.rs` | erf_approx, erfc_approx with Cephes coefficients | PARTIAL | Functions exist with #[cube] and 30+ Cephes constants; erf achieves 1e-13, erfc only 1e-10 (not 1e-15) |
| `src/math/spin.rs` | compute_total, compute_zeta, spin_scaling, clamp_zeta | VERIFIED | All 4 functions present; to_total_zeta split per plan guidance |
| `src/math/dft_quantities.rs` | wigner_seitz_rs, reduced_gradient_s, tf_kinetic, dimensionless_alpha | VERIFIED | All 4 functions using pow_1_3/4_3/5_3 from powers module |
| `src/kernel/mod.rs` | Module declarations | VERIFIED | `pub mod launch` and `pub mod lda` present |
| `src/kernel/launch.rs` | CubeCL launch infrastructure | VERIFIED | cpu_client(), calculate_launch_config(), buffer management, identity kernel |
| `src/kernel/lda/mod.rs` | LDA kernel placeholder | VERIFIED | `pub mod lda_x` added |
| `src/kernel/lda/lda_x.rs` | 10 LDA_X kernel functions (min 400 lines) | VERIFIED | 1485 lines; all 10 functions present; maple2c variable names preserved |
| `verify/tests/lda_x_oracle.rs` | Oracle comparison tests | VERIFIED | 12 tests; exc unpol/pol, vxc/fxc/kxc/lxc; 1e-12 threshold; all pass |
| `verify/tests/lda_x_stress.rs` | Stress/edge-case tests (min 150 lines) | VERIFIED | 719 lines; 10 tests covering threshold, alpha, extreme density, asymmetric spins, large batch |
| `tests/math_integration.rs` | Integration tests for cross-module composition | VERIFIED | 6 tests; dft_quantities, spin, erf, pow chains verified through CubeCL |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| src/math/powers.rs | src/math/constants.rs | M_CBRT constants in tests | VERIFIED | Uses M_CBRT3 in test assertions |
| src/math/erf.rs | src/math/polynomials.rs | poly_eval for Horner | NOT_WIRED | erf uses inline Horner evaluation; poly_eval is separate; erf does not import polynomials.rs -- this is acceptable per plan which allowed inline expansion |
| src/math/dft_quantities.rs | src/math/powers.rs | pow_1_3 for density calculations | VERIFIED | `use super::powers::{pow_1_3, pow_4_3, pow_5_3}` in dft_quantities.rs |
| src/kernel/lda/lda_x.rs | src/math/powers.rs | pow_1_3 calls in kernel body | VERIFIED | `use crate::math::powers::pow_1_3` line 18 |
| src/kernel/lda/lda_x.rs | src/math/piecewise.rs | piecewise3 threshold guards | VERIFIED | `use crate::math::piecewise::piecewise3` line 17 |
| src/kernel/lda/lda_x.rs | src/math/constants.rs | M_CBRT3, M_CBRTPI constants | VERIFIED | `use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI}` line 16 |
| verify/tests/lda_x_oracle.rs | src/kernel/lda/lda_x.rs | Launches LDA_X kernel | VERIFIED | `use libxc_rs::kernel::lda::lda_x::*`; `lda_x_exc_unpol::launch_unchecked::<CpuRuntime>` |
| src/kernel/launch.rs | cubecl_cpu::CpuRuntime | cpu_client() | VERIFIED | `CpuRuntime::client(&device)` in cpu_client() |
| tests/math_integration.rs | src/kernel/launch.rs | Uses launch infrastructure | VERIFIED | `use libxc_rs::kernel::launch::{calculate_launch_config, cpu_client, create_input_buffer, ...}` |

**Note on erf -> polynomials key link:** The plan specified erf should use poly_eval from polynomials.rs. The actual implementation uses inline Horner expansion in erf.rs, which the summary documents as a decision: "Manually expand Horner evaluation inline for each region's polynomial (avoid Array overhead for small fixed-size coefficient sets)." This is functionally equivalent -- the Horner method is correct -- just not using the poly_eval abstraction. Not flagged as a gap since the behavior (correct Horner evaluation) is verified.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| src/kernel/lda/lda_x.rs | rho input | CubeCL Array<f64> from host slice | Yes - bytemuck cast_slice from test f64 array | FLOWING |
| verify/tests/lda_x_oracle.rs | c_zk | oracle_lda_exc() via C libxc FFI | Yes - real C libxc output | FLOWING |
| tests/math_integration.rs | rs values | wigner_seitz_rs via CubeCL kernel | Yes - computed from density input | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| All 129 tests pass | `cargo test --workspace -- --test-threads=1` | 101 unit + 6 integration + 12 oracle + 10 stress = 129 tests, all pass | PASS |
| Zero clippy warnings | `cargo clippy --workspace -- -D warnings` | No warnings, exit 0 | PASS |
| LDA_X oracle exc unpol at 1e-12 | Tests run; summary reports max ~6e-16 relative error | << 1e-12 requirement | PASS |
| erfc sweep at 1e-10 | Test asserts err < 1e-10 (not 1e-15 as plan required) | 1e-10 achieved, plan required 1e-15 | PARTIAL |
| 10000-point large batch | test_lda_x_large_batch: 40 workgroups, 10000 points, max err verified | PASS, oracle comparison holds | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|---------|
| MATH-01 | 02-01 | safe_cbrt handles negative values correctly | SATISFIED | Tests verify cbrt(-8)==-2; 101 unit tests pass |
| MATH-02 | 02-01 | pow_1_3, pow_2_3, pow_4_3, pow_5_3 as #[cube] | SATISFIED | All 5 functions present with #[cube] annotation |
| MATH-03 | 02-01 | piecewise3/5 as branch-free #[cube] select | SATISFIED | Both functions use CubeCL select() |
| MATH-04 | 02-01 | erf/erfc approximations accurate to f64 precision | PARTIAL | erf achieves 1e-13; erfc only achieves 1e-10 vs 1e-15 required; f64 precision is ~1e-15 |
| MATH-05 | 02-01 | All math constants defined as f64 const | SATISFIED | M_CBRT3, X_FACTOR_C, RS_CONST, KF_CONST and 16+ others verified |
| MATH-06 | 02-01 | Spin polarization transforms implemented | SATISFIED | compute_total, compute_zeta, spin_scaling, clamp_zeta all present |
| MATH-07 | 02-01 | DFT quantities implemented | SATISFIED | All 4 functions present and wired to powers module |
| MATH-08 | 02-01 | Polynomial evaluation via Horner's method | SATISFIED | poly_eval with #[comptime] length; rational_eval present |
| MATH-09 | 02-01, 02-04 | All math functions tested against known values and libm | SATISFIED | 101 unit tests + 6 integration tests; libm sweeps for cbrt, erf; composition chains verified |
| MATH-10 | 02-01 (deferred D-09) | Cross-backend consistency CPU vs GPU | DEFERRED | D-09 explicitly defers to Phase 7; no GPU backend in Phase 2 |
| KERN-01 | 02-02 | Kernel launch wrappers: backend, buffers, CubeCount/CubeDim | SATISFIED | src/kernel/launch.rs fully implements all requirements; 11 tests pass |
| KERN-02 | 02-03, 02-05 | LDA_X canary passes 10^-12 oracle comparison (both spin modes) | SATISFIED | 12 oracle tests + 10 stress tests pass; actual accuracy ~6e-16, far exceeds 1e-12 |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| src/math/erf.rs | 83 | Doc comment claims "Accuracy: relative error <= 1e-15 across [-6, 6]" but test at line 310 enforces 1e-13 for erf and line 348 enforces 1e-10 for erfc | WARNING | Misleading documentation; the function does not achieve its documented precision for erfc |

### Human Verification Required

None -- all verification was performed programmatically. The numerical accuracy (oracle comparison at 1e-12, erfc precision at 1e-10 vs 1e-15 target) is assessed through automated tests.

### Gaps Summary

**1 gap identified:**

**erfc_approx precision gap:** The plan must_have and success criteria require erfc to match libm to within 1e-15 relative error. The actual test assertion is 1e-10 (100,000x looser). The summary acknowledges this at the region 3/4 polynomial boundary (~x=2.857) introduces ~3e-11 relative error. While this is still well within the 10^-12 energy accuracy target (erfc errors only propagate indirectly), the specified precision contract is not met.

Options for resolution:
1. Fix erfc_approx to achieve 1e-15 by improving the region 3/4 polynomial fit or using a different coefficient set near the boundary
2. Update the plan must_have, doc comment, and ROADMAP SC-2 to document that erfc achieves 1e-10 precision, with explicit justification that this is sufficient for downstream energy accuracy

**1 deferred item (not a gap):**

**MATH-10:** Cross-backend consistency (CPU vs GPU) is deferred to Phase 7 by developer decision D-09. This is not a gap -- the developer explicitly made this call with justification (no GPU backend available in Phase 2). Phase 7 addresses it via VERIFY-08.

---

_Verified: 2026-04-09T08:54:14Z_
_Verifier: Claude (gsd-verifier)_
