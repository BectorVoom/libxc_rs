---
phase: 04-bulk-kernel-translation
plan: 01
subsystem: math, testing, kernel-scaffolding
tags: [cubecl, power-functions, oracle-ffi, gga, mgga, verification]

# Dependency graph
requires:
  - phase: 02-cubecl-kernel-substrate
    provides: "#[cube] function pattern, safe_cbrt, pow_1_3 through pow_5_3"
  - phase: 03-evaluation-orchestration
    provides: "dispatch_lda, LdaInput/Output, kernel launch infrastructure"
provides:
  - "pow_3_2, pow_1_4, pow_7_3, pow_2, pow_3 as #[cube] functions"
  - "oracle_gga_all and oracle_mgga_all FFI wrappers for C libxc"
  - "Batch oracle test infrastructure for all LDA/GGA/MGGA functionals"
  - "GGA and MGGA kernel module directories scaffolded"
  - "oracle_func_flags helper for querying functional capability flags"
affects: [04-02, 04-03, 04-04, 04-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Flag-guarded FFI calls: check FLAGS_HAVE_* before calling derivative-level functions"
    - "Null-pointer passthrough for unsupported derivative levels in oracle calls"
    - "Batch oracle test pattern: iterate all functional IDs, skip unsupported, collect failures"

key-files:
  created:
    - src/kernel/gga/mod.rs
    - src/kernel/mgga/mod.rs
    - verify/tests/lda_oracle.rs
    - verify/tests/gga_oracle.rs
    - verify/tests/mgga_oracle.rs
  modified:
    - src/math/powers.rs
    - src/kernel/mod.rs
    - verify/src/lib.rs

key-decisions:
  - "Flag-check before FFI: query functional flags before calling derivative functions to prevent libxc exit() on unsupported orders"
  - "Skip functionals without EXC in batch tests rather than maintaining exclusion lists"
  - "MGGA 4th-order dimensions taken directly from libxc util.c (total 477 components for polarized)"

patterns-established:
  - "Batch oracle test pattern: FunctionalTestCase struct with id/name, iterate with flag checks"
  - "GGA polarized dimensions: v4rhosigma3=20 (2*10), not 16"

requirements-completed: [KERN-07, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06, VERIFY-07]

# Metrics
duration: 17min
completed: 2026-04-09
---

# Phase 04 Plan 01: Infrastructure Preparation Summary

**Five missing power functions (#[cube]), GGA/MGGA oracle FFI wrappers with flag-guarded derivative calls, and batch oracle verification for all 469 functionals across 3 families**

## Performance

- **Duration:** 17 min
- **Started:** 2026-04-09T22:33:46Z
- **Completed:** 2026-04-09T22:50:36Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Added pow_3_2, pow_1_4, pow_7_3, pow_2, pow_3 as #[cube] functions with CubeCL kernel tests (11 total tests passing)
- Extended oracle harness with GgaOracleOutput (15 fields) and MggaOracleOutput (70 fields) structs plus full derivative wrappers
- Created batch oracle tests covering 67 LDA, 256 GGA, and 146 MGGA functionals in both spin modes
- Scaffolded src/kernel/gga/ and src/kernel/mgga/ module directories

## Task Commits

Each task was committed atomically:

1. **Task 1: Add missing power functions and GGA/MGGA module scaffolding** - `a7cb366` (feat)
2. **Task 2: Extend oracle harness for GGA/MGGA and create per-family batch test files** - `109d8aa` (feat)

## Files Created/Modified
- `src/math/powers.rs` - Added 5 new #[cube] power functions and their CubeCL kernel tests
- `src/kernel/mod.rs` - Added gga and mgga module declarations
- `src/kernel/gga/mod.rs` - GGA kernel module directory scaffold
- `src/kernel/mgga/mod.rs` - MGGA kernel module directory scaffold
- `verify/src/lib.rs` - GgaOracleOutput, MggaOracleOutput, oracle_gga_all, oracle_mgga_all, oracle_func_flags
- `verify/tests/lda_oracle.rs` - Batch LDA oracle test with 67 functional IDs
- `verify/tests/gga_oracle.rs` - Batch GGA oracle test with 256 functional IDs
- `verify/tests/mgga_oracle.rs` - Batch MGGA oracle test with 146 functional IDs

## Decisions Made
- **Flag-guarded FFI calls:** Added oracle_func_flags() helper and FLAGS_HAVE_* constants. Before calling exc_vxc_fxc_kxc, we pass null pointers for derivative levels the functional doesn't support, preventing libxc's exit(1) calls.
- **Functional skip pattern:** Tests skip functionals without EXC support (lda_xc_tih, gga_x_lb, gga_x_lbm, mgga_x_bj06, mgga_x_tb09, mgga_x_rpp09, mgga_x_2d_prhg07_prp10) rather than maintaining static exclusion lists.
- **Dimension values from C source:** All GGA/MGGA polarized dimension multipliers taken directly from libxc util.c internal_counters_set_gga/mgga functions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed GGA polarized v4rhosigma3 dimension**
- **Found during:** Task 2 (GGA oracle test)
- **Issue:** Plan specified d_v4rhosigma3=16, but libxc util.c defines it as 2*10=20. The undersized buffer caused heap corruption (free(): invalid size).
- **Fix:** Corrected to 20 per util.c
- **Files modified:** verify/src/lib.rs
- **Verification:** GGA oracle polarized test passes for all 254 supported functionals

**2. [Rule 1 - Bug] Fixed MGGA polarized 4th-order dimensions**
- **Found during:** Task 2 (MGGA oracle test)
- **Issue:** Multiple MGGA 4th-order dimension values were incorrect (v4rhosigma2lapl=18 should be 36, v4sigma3tau=18 should be 30, etc.)
- **Fix:** Replaced all values with exact computations from libxc util.c
- **Files modified:** verify/src/lib.rs

**3. [Rule 2 - Missing Critical] Added flag-guarded derivative FFI calls**
- **Found during:** Task 2 (functional exit on unsupported derivatives)
- **Issue:** Some functionals (e.g., mgga_c_b94) don't support lxc; libxc calls exit(1) when lxc output pointers are non-null without LXC flag support
- **Fix:** Added oracle_func_flags() helper, pass null pointers for unsupported derivative levels
- **Files modified:** verify/src/lib.rs
- **Verification:** All 469 functionals evaluated without process exit

---

**Total deviations:** 3 auto-fixed (2 bug fixes, 1 missing critical)
**Impact on plan:** All fixes necessary for correctness. No scope creep.

## Issues Encountered
- libxc-master directory not available in git worktree (not tracked by git). Resolved by creating symlink to main repo's libxc-master.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All math primitives for GGA/MGGA kernel translation are available
- Oracle comparison infrastructure ready for all 3 families
- GGA/MGGA kernel module directories scaffolded for Plan 02-05 translations
- Batch test files ready to activate per-functional comparison as kernels are translated

## Self-Check: PASSED

---
*Phase: 04-bulk-kernel-translation*
*Completed: 2026-04-09*
