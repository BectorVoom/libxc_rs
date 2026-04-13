---
phase: 08-rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat
plan: 02
subsystem: kernel-codegen
tags: [mgga, cubecl, oracle, verification, code-generation]

# Dependency graph
requires:
  - phase: 08-rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat
    provides: translate_mgga.py MGGA translator, kernel-mgga-1 sub-crate with mgga_xc_lp90
provides:
  - 3 additional compiled MGGA functionals (mgga_k_gea2, mgga_x_lta, mgga_c_b88)
  - Oracle comparison tests proving numerical equivalence for all 4 MGGA kernels
  - CubeCL concurrency workaround pattern for kernel tests
affects: [08-03, 08-04, MGGA batch translation, kernel verification infrastructure]

# Tech tracking
tech-stack:
  added: [libxc_rs-verify (dev-dependency)]
  patterns: [MGGA oracle comparison via CubeCL CPU launch + libxc FFI oracle, mutex-serialized CubeCL tests]

key-files:
  created:
    - crates/kernel-mgga-1/src/mgga_k_gea2/mod.rs
    - crates/kernel-mgga-1/src/mgga_k_gea2/exc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_k_gea2/exc_pol.rs
    - crates/kernel-mgga-1/src/mgga_x_lta/mod.rs
    - crates/kernel-mgga-1/src/mgga_x_lta/exc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_x_lta/exc_pol.rs
    - crates/kernel-mgga-1/src/mgga_c_b88/mod.rs
    - crates/kernel-mgga-1/src/mgga_c_b88/exc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_c_b88/exc_pol.rs
  modified:
    - crates/kernel-mgga-1/src/lib.rs
    - tests/oracle_mgga.rs
    - Cargo.toml

key-decisions:
  - "Added global Mutex to serialize CubeCL kernel launches in test suite to prevent buffer corruption from CubeCL CPU runtime concurrency bug"
  - "Used default ext_params for mgga_x_lta (ltafrac=1.0) matching libxc defaults"
  - "Added verify crate as dev-dependency to workspace root for oracle FFI access"

patterns-established:
  - "CubeCL oracle test pattern: oracle_mgga_all() -> CubeCL CPU launch -> relative error comparison at 1e-12 threshold"
  - "Global CUBECL_LOCK mutex required for all CubeCL test files to prevent concurrent kernel launch corruption"

requirements-completed: [KERN-05, KERN-06, VERIFY-03]

# Metrics
duration: 77min
completed: 2026-04-13
---

# Phase 08 Plan 02: Representative MGGA Functionals and Oracle Tests Summary

**3 additional MGGA functionals (K/X/C families) compiled and all 4 pass oracle comparison at sub-1e-15 relative error**

## Performance

- **Duration:** 77 min
- **Started:** 2026-04-13T11:56:16Z
- **Completed:** 2026-04-13T13:13:25Z
- **Tasks:** 2
- **Files modified:** 36

## Accomplishments
- Translated 3 representative MGGA functionals covering all family patterns: kinetic (mgga_k_gea2), exchange (mgga_x_lta), correlation (mgga_c_b88)
- All 4 MGGA functionals (including lp90 from Plan 01) produce 10 kernel files each (5 derivative levels x 2 spin modes) totaling 30 new kernel files
- Wrote 5 oracle comparison tests proving numerical equivalence: max relative error 8.69e-16 (well below 1e-12 threshold)
- Discovered and mitigated CubeCL CPU runtime concurrency bug with global mutex serialization

## Task Commits

Each task was committed atomically:

1. **Task 1: Translate representative functionals** - `6aa7493` (feat)
2. **Task 2: Write oracle comparison tests** - `58ff55a` (feat)

## Files Created/Modified
- `crates/kernel-mgga-1/src/mgga_k_gea2/` - 10 kernel files + mod.rs for kinetic functional (7,542 lines C)
- `crates/kernel-mgga-1/src/mgga_x_lta/` - 10 kernel files + mod.rs for exchange functional (7,616 lines C)
- `crates/kernel-mgga-1/src/mgga_c_b88/` - 10 kernel files + mod.rs for correlation functional (21,391 lines C)
- `crates/kernel-mgga-1/src/lib.rs` - Added 3 new module declarations
- `tests/oracle_mgga.rs` - 5 oracle comparison tests replacing placeholder
- `Cargo.toml` - Added verify and kernel-mgga-1 to dev-dependencies

## Decisions Made
- Used global `std::sync::Mutex` to serialize CubeCL kernel launches because the CubeCL CPU runtime has shared global state that corrupts output buffers when multiple kernels launch concurrently from different test threads
- Passed `ltafrac=1.0` as default ext_param for mgga_x_lta matching libxc's default initialization
- Used `libxc_rs-verify` (not `verify`) as the dev-dependency name since the crate's package name differs from the directory name

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] CubeCL CPU runtime concurrency buffer corruption**
- **Found during:** Task 2 (oracle tests)
- **Issue:** When 5 oracle tests ran in parallel (default test harness behavior), CubeCL CPU runtime produced wrong results -- buffer writes from concurrent kernels leaked across tests
- **Fix:** Added `static CUBECL_LOCK: Mutex<()>` and acquire lock at start of each test
- **Files modified:** tests/oracle_mgga.rs
- **Verification:** All 5 tests pass with both parallel and sequential execution
- **Committed in:** 58ff55a (Task 2 commit)

**2. [Rule 3 - Blocking] cmake not installed for verify crate build**
- **Found during:** Task 2 (oracle tests compilation)
- **Issue:** verify crate's build.rs requires cmake to compile vendored libxc-master; cmake was not installed
- **Fix:** `sudo apt-get install cmake`
- **Files modified:** None (system package)
- **Verification:** verify crate builds successfully

**3. [Rule 3 - Blocking] Wrong crate name for verify dev-dependency**
- **Found during:** Task 2 (oracle tests compilation)
- **Issue:** Used `verify = { path = "verify" }` but crate package name is `libxc_rs-verify`
- **Fix:** Changed to `libxc_rs-verify = { path = "verify" }`
- **Files modified:** Cargo.toml
- **Verification:** cargo resolves dependency correctly

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All auto-fixes necessary for correctness and compilation. No scope creep.

## Issues Encountered
- OOM kills when compiling GGA crates in parallel during test build -- mitigated by using `-j 2` for cargo test invocations
- CubeCL CPU runtime produces non-deterministic buffer corruption under concurrent kernel launches -- systematic issue affecting all CubeCL test suites, not just MGGA

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- translate_mgga.py validated across all 4 MGGA family patterns (XC, K, X, C)
- Oracle test infrastructure proven and ready for batch functional validation
- CubeCL concurrency workaround documented for future test files
- Ready for Plan 03: batch translation of remaining MGGA functionals

## Self-Check: PASSED

All key files verified present. Both task commits (6aa7493, 58ff55a) verified in git log.

---
*Phase: 08-rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat*
*Completed: 2026-04-13*
