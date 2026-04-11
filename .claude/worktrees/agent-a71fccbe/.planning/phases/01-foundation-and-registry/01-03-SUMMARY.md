---
phase: 01-foundation-and-registry
plan: 03
subsystem: testing
tags: [libxc, ffi, bindgen, cmake, oracle, verification, clippy]

# Dependency graph
requires:
  - phase: 01-foundation-and-registry/01-01
    provides: Domain types (FunctionalId, Family, Spin, Dimensions)
  - phase: 01-foundation-and-registry/01-02
    provides: Registry with lookup_by_id, lookup_by_name, xtask generator
provides:
  - verify/ crate with cmake build of vendored libxc 7.0.0
  - bindgen FFI bindings to C libxc
  - oracle_lda_exc helper for calling C libxc LDA evaluation
  - LDA_X oracle smoke tests (unpolarized + polarized)
  - Clean workspace build with all quality gates passing
affects: [phase-02, phase-03, phase-04, verification]

# Tech tracking
tech-stack:
  added: [bindgen 0.72.1, cmake 0.1.58, anyhow 1.0.100, approx 0.5.1]
  patterns: [oracle FFI wrapper pattern, cmake vendored C build, assert_relative_eq with 1e-12 tolerance]

key-files:
  created: [verify/Cargo.toml, verify/build.rs, verify/src/lib.rs, verify/src/oracle_ffi.rs, verify/tests/lda_x_oracle.rs]
  modified: [Cargo.toml, xtask/src/main.rs, src/dims/mod.rs]

key-decisions:
  - "Used cmake crate to build vendored libxc-master from source rather than linking against system libxc"
  - "oracle_lda_exc returns Result<Vec<f64>> using anyhow for ergonomic error handling in test harness"
  - "Replaced unsafe mem::zeroed with Default derive for Dimensions struct to satisfy BUILD-04"

patterns-established:
  - "Oracle verification pattern: oracle_lda_exc(func_id, spin, rho) -> Result<Vec<f64>> wrapping C libxc calls"
  - "Float comparison in tests: approx::assert_relative_eq! with max_relative = 1e-12"
  - "verify/ crate is the only FFI consumer; src/ has zero unsafe and zero C FFI"

requirements-completed: [VERIFY-01, BUILD-01, BUILD-02, BUILD-03, BUILD-04, BUILD-05]

# Metrics
duration: 5min
completed: 2026-04-09
---

# Phase 01 Plan 03: Oracle Verification Harness and Build Quality Summary

**LDA_X oracle smoke test calling C libxc 7.0.0 via bindgen FFI with 1e-12 tolerance verification and clean workspace build quality gates**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-09T05:56:01Z
- **Completed:** 2026-04-09T06:01:02Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Created verify/ crate with cmake build of vendored libxc 7.0.0 and bindgen FFI bindings
- Implemented oracle_lda_exc helper wrapping C xc_func_alloc/init/lda_exc/end/free lifecycle
- LDA_X smoke tests pass for both unpolarized and polarized modes with analytical value verification
- Full workspace: zero build warnings, 40 tests passing, zero clippy warnings, no unsafe in src/, no C FFI in src/

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend verification harness with LDA_X oracle smoke test** - `632aee6` (feat)
2. **Task 2: Final build quality validation** - `d17e6d4` (fix)

## Files Created/Modified
- `verify/Cargo.toml` - Verify crate with anyhow, approx, bindgen, cmake deps
- `verify/build.rs` - CMake build of vendored libxc + bindgen FFI generation
- `verify/src/lib.rs` - oracle_lda_exc helper function
- `verify/src/oracle_ffi.rs` - Raw bindgen FFI bindings include
- `verify/tests/lda_x_oracle.rs` - Unpolarized and polarized LDA_X smoke tests
- `Cargo.toml` - Added verify/ to workspace members
- `xtask/src/main.rs` - Fixed clippy warnings (manual strip_prefix, useless format)
- `src/dims/mod.rs` - Replaced unsafe mem::zeroed with Default derive

## Decisions Made
- Used cmake crate to build vendored libxc-master from source with CMAKE_POLICY_VERSION_MINIMUM=3.5 for newer cmake compatibility
- oracle_lda_exc returns Result<Vec<f64>> using anyhow for ergonomic error handling in test harness
- Replaced unsafe mem::zeroed with Default derive for Dimensions struct to satisfy BUILD-04 (no unsafe in src/)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] CMake policy version compatibility**
- **Found during:** Task 1
- **Issue:** CMake >= 3.35 removed compatibility with cmake_minimum_required < 3.5, causing libxc build failure
- **Fix:** Added CMAKE_POLICY_VERSION_MINIMUM=3.5 define to cmake Config
- **Files modified:** verify/build.rs
- **Verification:** Build completes successfully
- **Committed in:** 632aee6 (Task 1 commit)

**2. [Rule 1 - Bug] Clippy warnings in xtask and verify code**
- **Found during:** Task 2
- **Issue:** 13 clippy errors: manual strip_prefix (9), identical blocks (2), useless format (2), plus doc overindent and manual modulo
- **Fix:** Used strip_prefix method, removed useless format!, fixed doc indent, used is_multiple_of
- **Files modified:** xtask/src/main.rs, verify/src/lib.rs
- **Committed in:** d17e6d4 (Task 2 commit)

**3. [Rule 2 - Missing Critical] Unsafe code in production src/**
- **Found during:** Task 2
- **Issue:** src/dims/mod.rs used unsafe { std::mem::zeroed() } violating BUILD-04
- **Fix:** Added Default derive to Dimensions, replaced unsafe with Self::default()
- **Files modified:** src/dims/mod.rs
- **Committed in:** d17e6d4 (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (1 blocking, 1 bug, 1 missing critical)
**Impact on plan:** All auto-fixes necessary for correctness and build quality. No scope creep.

## Issues Encountered
- libxc-master directory is untracked and not present in git worktrees; symlinked from main repo to worktree for build

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Oracle verification pattern established and ready for extension to all functional families
- verify/ crate can be extended with GGA, MGGA oracle helpers following same pattern
- All Phase 01 quality gates met; ready for Phase 02 (I/O bundles and CubeCL substrate)

---
## Self-Check: PASSED

All 5 created files verified present. Both task commits (632aee6, d17e6d4) verified in git log. SUMMARY.md exists.

---
*Phase: 01-foundation-and-registry*
*Completed: 2026-04-09*
