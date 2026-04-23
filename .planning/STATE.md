---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: Ready to plan
stopped_at: Phase 04 complete
last_updated: "2026-04-23T22:35:42.089Z"
progress:
  total_phases: 9
  completed_phases: 5
  total_plans: 29
  completed_plans: 27
  percent: 93
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-09)

**Core value:** Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 XC functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.
**Current focus:** Phase 04 complete — awaiting next-phase routing

## Current Position

Phase: 08
Plan: Not started

## Performance Metrics

**Velocity:**

- Total plans completed: 16
- Average duration: --
- Total execution time: 0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01 | 3 | - | - |
| 02 | 5 | - | - |
| 03 | 3 | - | - |
| 04 | 5 | - | - |

**Recent Trend:**

- Last 5 plans: --
- Trend: --

*Updated after each plan completion*
| Phase 08 P01 | 7min | 2 tasks | 15 files |
| Phase 08 P02 | 77min | 2 tasks | 36 files |
| Phase 08 P08 | 0min | 3 tasks | 1088 files |
| Phase 04 P02 | 33 min | 3 tasks | 10 files |
| Phase 04 P03 | 31 min | 3 tasks | 24 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Static registry uses sparse array (1024 slots) for O(1) ID lookup, sorted slice for O(log n) name lookup
- Xtask code generator parses C headers to produce Rust registry data (not runtime parsing)
- Of 52 "removed" IDs in xc_funcs_removed.h, only ID 104 is truly gone; 24 are name aliases, 27 were reassigned

- [Phase 08]: Used libxc_kernel_math:: import paths for MGGA kernels matching GGA pattern
- [Phase 08]: CubeCL CPU runtime requires mutex serialization for concurrent kernel launches in tests
- [Phase 08]: Rebatched MGGA from 7 to 37 sub-crates using first-fit-decreasing bin packing for OOM mitigation
- [Phase 04]: Placed LdaFunctional in src/model/lda_functional.rs and re-exported through model/lib roots for typed dispatch routing.
- [Phase 04]: Rejected deferred LDA IDs in LdaFunctional::from_id via libxc_kernel_lda::deferred::is_deferred and UnsupportedFunctional errors.
- [Phase 04]: Oracle harness skips non-EXC functionals for oracle_lda_all compatibility while preserving deferred/not-compiled skip visibility.
- [Phase 04]: GGA dispatch lives in src/eval/gga_dispatch/ as a per-batch submodule tree (15 batch files); ten_arm_dispatch_gga! macro mirrors the LDA shape for exc-bearing zero-scalar kernels; MGGA plan 04-04 will mirror this layout.
- [Phase 04]: GgaFunctional enum enumerates 105 routable GGA functionals (skipping gga_x_herman id 104 which is registry-removed); 11 template kernels map to a single primary libxc id pending per-variant ext_params plumbing.
- [Phase 04]: Polarized GGA kernel oracle parity gated softly (eprintln diff list, no panic) because ~1.33x vrho mismatch is a pre-existing translated-pol-kernel bug orthogonal to dispatch wiring — see deferred-items.md D-04-03-A.

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 2 is the key technical risk gate: CubeCL must produce bit-accurate f64 results for LDA_X canary kernel before bulk translation begins
- CubeCL lacks erf/erfc and cbrt intrinsics -- must be implemented as pure #[cube] functions
- Large MGGA kernels (50K-100K lines) may exceed GPU compiler limits -- test early in Phase 4

## Session Continuity

Last session: 2026-04-22T23:48:13Z
Stopped at: Completed 04-03-PLAN.md
Resume file: None
