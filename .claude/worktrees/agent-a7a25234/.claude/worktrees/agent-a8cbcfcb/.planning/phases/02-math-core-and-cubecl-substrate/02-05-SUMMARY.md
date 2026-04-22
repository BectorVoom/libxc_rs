---
phase: 02-math-core-and-cubecl-substrate
plan: 05
subsystem: kernel-lda-x-stress
tags: [cubecl, lda-x, stress-test, edge-cases, threshold, alpha, polarization]

# Dependency graph
requires:
  - phase: 02-math-core-and-cubecl-substrate
    plan: 03
    provides: "10 LDA_X kernel functions, oracle comparison infrastructure"
provides:
  - "10 stress tests covering threshold boundary, non-default alpha, extreme density, asymmetric spins, symmetric-vs-unpol, and large batch"
  - "oracle_lda_*_with_opts helpers for ext_params/dens_threshold configuration"
affects: [04-kernel-translation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Alpha linearity verification: zk(alpha) = alpha * zk(1.0) for all derivatives"
    - "Threshold boundary testing: densities at 0.5x, 0.9x, 1.0x, 1.1x, 2.0x threshold"
    - "oracle_lda_*_with_opts pattern for configurable oracle evaluation"

key-files:
  created:
    - verify/tests/lda_x_stress.rs
  modified:
    - verify/src/lib.rs

key-decisions:
  - "Alpha testing via linearity rather than C oracle ext_params because LDA_X (ID=1) does not expose alpha as an ext_param in C libxc"
  - "CubeCL branchless select means sub-threshold densities may not produce exact zero but match C oracle behavior"

patterns-established:
  - "Stress test pattern: threshold boundary, parameter scaling, extreme regimes, asymmetric spins, self-consistency, large batch"

requirements-completed: [KERN-02]

# Metrics
duration: 4min
completed: 2026-04-09
---

# Phase 02 Plan 05: LDA_X Stress Tests Summary

**10 edge-case stress tests proving LDA_X robustness at threshold boundaries, non-default alpha scaling, extreme densities (1e-14 to 1e8), highly asymmetric spins (0.999/0.001), and 10000-point multi-workgroup dispatch**

## Performance

- **Duration:** 4 min
- **Started:** 2026-04-09T08:31:54Z
- **Completed:** 2026-04-09T08:36:14Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments

- 10 stress tests in 719 lines covering 7 distinct edge-case categories
- Threshold boundary: densities at [0.5e-15, 0.9e-15, 1.0e-15, 1.1e-15, 2.0e-15, 1e-14] with derivative-order validation
- Non-default alpha: scaling factor linearity verified for alpha in [0.0, 0.25, 0.5, 1.0, 1.5, 2.0] with all 5 derivative orders
- Extreme density: 12 values spanning 22 orders of magnitude with no NaN/Inf and <= 1e-12 oracle match
- Asymmetric spins: 10 highly polarized pairs including (0.999, 0.001) and (1e-8, 1.0), all within 1e-12 of oracle
- Symmetric pol == unpol: self-consistency at 1e-14 for rho in [0.1, 1.0, 10.0, 100.0]
- Large batch: 10000 points across 40 workgroups, verifying partial last workgroup (16/256 threads active)
- Added oracle helper functions with configurable ext_params and dens_threshold

## Task Commits

Each task was committed atomically:

1. **Task 1: LDA_X edge-case and stress tests** - `fe0f651` (test)

## Files Created/Modified

- `verify/tests/lda_x_stress.rs` - 10 stress tests, 719 lines covering all edge-case categories
- `verify/src/lib.rs` - Added `OracleOptions`, `oracle_lda_exc_with_opts`, `oracle_lda_all_with_opts` helpers

## Decisions Made

- **Alpha testing via linearity**: C libxc LDA_X (ID=1) hardcodes alpha=1.0 with no ext_param interface. Verified Rust kernel's alpha parameter via self-consistency: zk(alpha) = alpha * zk(1.0). The alpha=1.0 baseline is validated against oracle in lda_x_oracle.rs.
- **Threshold boundary tolerance**: CubeCL's branchless `select` means sub-threshold densities may produce non-zero but extremely small values. Tests verify these match the C oracle's behavior rather than asserting exact zero.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] libxc-master symlink needed in worktree**
- **Found during:** Task 1
- **Issue:** Vendored C libxc source not present in worktree (not tracked by git)
- **Fix:** Created symlink to main repo's libxc-master/
- **Files modified:** None (symlink only)
- **Committed in:** N/A

**2. [Rule 1 - Bug] C oracle does not support ext_params on LDA_X ID=1**
- **Found during:** Task 1
- **Issue:** Plan suggested using `xc_func_set_ext_params` for non-default alpha on LDA_X, but C libxc's LDA_X (ID=1) has 0 ext_params (alpha is hardcoded to 1.0). Calling set_ext_params causes assertion failure.
- **Fix:** Changed non-default alpha tests to verify linearity property (zk(alpha) = alpha * zk(1)) instead of oracle comparison. This is mathematically rigorous since the alpha=1.0 case is validated against oracle in lda_x_oracle.rs.
- **Files modified:** verify/tests/lda_x_stress.rs
- **Committed in:** fe0f651

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** No scope change. Alpha tests use linearity property instead of direct oracle comparison, which is equally rigorous.

## Issues Encountered

None beyond the deviations documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- LDA_X canary kernel is thoroughly stress-tested across all edge cases
- Pattern validated for robustness before Phase 4 scales to 270 kernels
- Threshold boundary, alpha parameter handling, extreme regimes, and spin asymmetry all proven correct

## Self-Check: PASSED

All created files verified present. Task commit fe0f651 verified in git log.

---
*Phase: 02-math-core-and-cubecl-substrate*
*Completed: 2026-04-09*
