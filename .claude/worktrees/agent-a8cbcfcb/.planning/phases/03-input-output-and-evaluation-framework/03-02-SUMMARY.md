---
phase: 03-input-output-and-evaluation-framework
plan: 02
subsystem: eval
tags: [cubecl, dispatch, lda, kernel-launch, build-04]

# Dependency graph
requires:
  - phase: 03-01
    provides: LdaInput, LdaOutput, GgaOutput, MggaOutput bundles with validation
  - phase: 02
    provides: LDA_X CubeCL kernel functions (10 variants) and launch infrastructure
provides:
  - Safe kernel launch wrappers (BufArg abstraction) encapsulating all unsafe CubeCL calls
  - dispatch_lda function routing (order, spin) to correct kernel through safe wrappers
  - BUILD-04 compliant architecture with zero unsafe in eval/ module
affects: [03-03, phase-04, phase-05]

# Tech tracking
tech-stack:
  added: []
  patterns: [BufArg handle+size wrapper for safe CubeCL dispatch, match-based kernel routing]

key-files:
  created:
    - src/kernel/lda/launch_lda_x.rs
    - src/eval/dispatch.rs
    - src/eval/mod.rs
  modified:
    - src/kernel/lda/mod.rs
    - src/lib.rs

key-decisions:
  - "BufArg abstraction over raw ArrayArg to keep all unsafe confined to kernel/"
  - "Dummy buffer pattern for None output fields rather than conditional kernel signatures"

patterns-established:
  - "BufArg: handle+size wrapper enabling safe dispatch without unsafe at call site"
  - "Kernel launch wrappers: one safe pub fn per kernel variant in kernel/family/launch_*.rs"
  - "Dispatch: match on (DerivativeOrder, Spin) tuple for exhaustive kernel routing"

requirements-completed: [EVAL-01, EVAL-04]

# Metrics
duration: 9min
completed: 2026-04-09
---

# Phase 03 Plan 02: LDA Dispatch Layer Summary

**Match-based dispatch routing all 10 LDA_X (order, spin) combinations through safe BufArg wrappers with zero unsafe in eval/**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-09T11:31:04Z
- **Completed:** 2026-04-09T11:39:57Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- 10 safe kernel launch wrappers in src/kernel/lda/launch_lda_x.rs encapsulating all unsafe CubeCL launch_unchecked and ArrayArg::from_raw_parts calls
- dispatch_lda function in src/eval/dispatch.rs routing all derivative orders (Exc through Lxc) and both spin modes to correct kernel with zero unsafe code
- BufArg abstraction that packages CubeCL Handle + element count, enabling safe callers to pass buffer arguments without touching unsafe ArrayArg construction
- Output buffer zeroing before kernel launch (T-03-04 mitigation) and dummy buffer allocation for None output fields

## Task Commits

Each task was committed atomically:

1. **Task 1: Safe kernel launch wrappers for LDA_X** - `8370277` (feat)
2. **Task 2 RED: Failing tests for LDA dispatch** - `8e4e43c` (test)
3. **Task 2 GREEN: Implement dispatch_lda** - `7a5d52e` (feat)

## Files Created/Modified
- `src/kernel/lda/launch_lda_x.rs` - 10 safe wrapper functions + BufArg type, encapsulating all unsafe kernel launches
- `src/eval/dispatch.rs` - dispatch_lda function with match-based routing, buffer management, result readback
- `src/eval/mod.rs` - Module re-exports for dispatch_lda
- `src/kernel/lda/mod.rs` - Added launch_lda_x module declaration
- `src/lib.rs` - Added eval module and dispatch_lda re-export

## Decisions Made
- **BufArg over ArrayArg parameters:** Changed safe wrapper signatures from taking ArrayArg (which requires unsafe from_raw_parts at call site) to taking BufArg (handle+size), moving ArrayArg construction inside the unsafe block. This achieves true zero-unsafe in eval/dispatch.rs.
- **Dummy buffer for None outputs:** When output.vrho is None but order >= Vxc, allocate a CubeCL buffer the kernel writes to but never read back. This avoids conditional kernel signatures while supporting selective output.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] BufArg abstraction to achieve zero-unsafe dispatch**
- **Found during:** Task 2 (dispatch implementation)
- **Issue:** Plan specified wrappers taking ArrayArg parameters, but ArrayArg::from_raw_parts is unsafe, meaning dispatch.rs would need unsafe blocks to construct ArrayArgs
- **Fix:** Changed wrapper signatures to take BufArg (handle+size) and construct ArrayArg inside the existing unsafe block in launch_lda_x.rs
- **Files modified:** src/kernel/lda/launch_lda_x.rs, src/eval/dispatch.rs
- **Verification:** grep -rn "unsafe" src/eval/ returns nothing
- **Committed in:** 7a5d52e

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Essential fix to achieve the BUILD-04 zero-unsafe requirement in eval/. No scope creep.

## Issues Encountered
None beyond the BufArg deviation documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- dispatch_lda is ready for use by the evaluation orchestration layer (Plan 03)
- Safe wrapper pattern established for future GGA/MGGA kernel families
- BufArg pattern can be extended to GPU backends when feature-gated

---
*Phase: 03-input-output-and-evaluation-framework*
*Completed: 2026-04-09*
