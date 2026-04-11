---
phase: 03-input-output-and-evaluation-framework
plan: 01
subsystem: api
tags: [input-validation, output-bundles, bitflags, buffer-safety, type-safe-io]

# Dependency graph
requires:
  - phase: 01-foundation-and-registry
    provides: "Dimensions struct with lda/gga/mgga constructors, Spin enum, DerivativeOrder enum, LibxcRsError variants"
provides:
  - "LdaInput, GgaInput, MggaInput with construction-time buffer validation"
  - "LdaOutput, GgaOutput, MggaOutput with Option<&mut [f64]> fields"
  - "OutputMask bitflags with cumulative from_order"
affects: [dispatch-layer, evaluation-orchestration, c-compatibility-layer]

# Tech tracking
tech-stack:
  added: []
  patterns: [construction-time-validation, option-output-fields, default-then-validate]

key-files:
  created:
    - src/input/mod.rs
    - src/output/mod.rs
    - src/output/mask.rs
  modified:
    - src/lib.rs

key-decisions:
  - "MggaOutput uses Default + validate pattern instead of 70-arg constructor for ergonomic struct-literal construction"
  - "OutputMask uses bitflags u8 with cumulative from_order matching libxc semantics"

patterns-established:
  - "Construction-time validation: input bundles validate buffer sizes in new(), making subsequent evaluation infallible"
  - "Default-then-validate: MggaOutput uses #[derive(Default)] with pub fields and separate validate() method"
  - "validate_output_field helper: shared validation for Option<&mut [f64]> fields"

requirements-completed: [IO-01, IO-02, IO-03, IO-04, IO-05]

# Metrics
duration: 5min
completed: 2026-04-09
---

# Phase 03 Plan 01: Input/Output Bundles Summary

**Type-safe LDA/GGA/MGGA input and output bundles with construction-time buffer validation, Option output fields, and cumulative OutputMask bitflags**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-09T11:23:13Z
- **Completed:** 2026-04-09T11:28:33Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Input bundles (LdaInput, GgaInput, MggaInput) reject wrong-sized buffers at construction via Dimensions validation
- Output bundles (LdaOutput with 5, GgaOutput with 15, MggaOutput with 70 Option fields) support selective derivative computation
- OutputMask::from_order is cumulative (Vxc includes Exc) matching libxc semantics
- 38 total tests covering all families, both spin modes, success and error paths

## Task Commits

Each task was committed atomically:

1. **Task 1: Input bundle types with construction-time validation** - `510425d` (feat)
2. **Task 2: Output bundle types with OutputMask bitflags** - `f06b818` (feat)

## Files Created/Modified
- `src/input/mod.rs` - LdaInput, GgaInput, MggaInput structs with new() validation and getters
- `src/output/mod.rs` - LdaOutput, GgaOutput, MggaOutput with Option<&mut [f64]> fields and validation
- `src/output/mask.rs` - OutputMask bitflags with cumulative from_order()
- `src/lib.rs` - Added input/output module declarations and re-exports

## Decisions Made
- MggaOutput uses Default + validate() pattern instead of a 70-argument constructor, enabling ergonomic struct-literal construction with `..Default::default()`
- OutputMask uses u8 bitflags (5 bits needed) matching the 5 derivative orders
- GgaOutput has explicit 15-arg constructor since that is manageable; LdaOutput has 5-arg constructor

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Input and output bundles are ready for the dispatch layer (Plan 02) to wire up
- OutputMask is ready for evaluation orchestration to select derivative levels
- All types re-exported from crate root for public API

## Self-Check: PASSED

---
*Phase: 03-input-output-and-evaluation-framework*
*Completed: 2026-04-09*
