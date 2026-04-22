---
phase: 01-foundation-and-registry
plan: 01
subsystem: domain-model
tags: [rust, bitflags, thiserror, enums, newtypes, dimensions]

requires:
  - phase: none
    provides: first plan in project
provides:
  - All domain enums (Family, Kind, Spin, DerivativeOrder, HybridType, HybridTermKind, Dimensionality)
  - FunctionalId newtype with Display impl
  - FunctionalFlags bitflags with all 13 flags matching libxc xc.h
  - Thresholds struct with default values
  - FunctionalMeta, Reference, ExtParamSpec, HybridTerm metadata structs
  - LibxcRsError enum with all 12 variants
  - Dimensions struct with lda/gga/mgga constructors and total_output_components
  - Public re-exports in crate root
affects: [01-02, 01-03, all-future-plans]

tech-stack:
  added: [bitflags 2.10, thiserror 2.0, bytemuck 1.25]
  patterns: [repr-u8 enums, pub(crate) newtype constructors, static lifetime metadata, zeroed struct initialization for dimensions]

key-files:
  created:
    - src/lib.rs
    - src/model/mod.rs
    - src/meta/mod.rs
    - src/error/mod.rs
    - src/dims/mod.rs
    - Cargo.toml
  modified: []

key-decisions:
  - "Added Display impl for FunctionalId to satisfy thiserror format strings in error variants"
  - "Used unsafe zeroed() for Dimensions initialization since all fields are integer types"
  - "Corrected total_output_components to 767 for polarized MGGA (plan incorrectly stated 477, which is only the MGGA-added order-4 fields from libxc util.c comment)"
  - "Removed cubecl dependency from Cargo.toml for this plan since only domain types are needed"

patterns-established:
  - "Module structure: src/{module}/mod.rs with #[cfg(test)] mod tests inline"
  - "Domain enums use #[repr(u8/u32)] with explicit discriminant values matching libxc C defines"
  - "Dimension values transcribed directly from libxc util.c using same arithmetic expressions"

requirements-completed: [DOM-01, DOM-02, DOM-03, DOM-04, DOM-05, ERR-01, ERR-03]

duration: 6min
completed: 2026-04-09
---

# Phase 01 Plan 01: Domain Types and Foundation Summary

**All domain enums, FunctionalFlags bitflags, Thresholds, error hierarchy, and dimension computation matching libxc util.c through 4th-order derivatives**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-08T23:42:44Z
- **Completed:** 2026-04-08T23:48:59Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments
- Complete domain type system with 7 enums, 1 newtype, and bitflags matching libxc C API values
- Full error hierarchy with 12 typed variants using thiserror v2, verified Send + Sync
- Dimensions struct computing correct array sizes for all family/spin combinations including 4th-order MGGA polarized (767 total output components)
- 19 unit tests covering repr values, flag positions, threshold defaults, error formatting, and dimension correctness

## Task Commits

Each task was committed atomically:

1. **Task 1: Create library crate root and domain model types** - `fae422f` (feat)
2. **Task 2: Create error module with complete LibxcRsError enum** - `3874b09` (feat)
3. **Task 3: Create Dimensions struct with family constructors** - `22927a2` (feat)

## Files Created/Modified
- `Cargo.toml` - Crate manifest with bitflags, thiserror, bytemuck dependencies
- `src/lib.rs` - Crate root with module declarations and public re-exports
- `src/model/mod.rs` - All domain enums, FunctionalId newtype, FunctionalFlags, Thresholds (7 tests)
- `src/meta/mod.rs` - FunctionalMeta, Reference, ExtParamSpec, HybridTerm structs
- `src/error/mod.rs` - LibxcRsError with 12 variants and 4 tests
- `src/dims/mod.rs` - Dimensions struct with lda/gga/mgga constructors and 8 tests

## Decisions Made
- Added `Display` impl for `FunctionalId` -- required by thiserror format strings in error variants that reference `{id}`. Not in plan but necessary for compilation (Rule 3 auto-fix).
- Used `unsafe { std::mem::zeroed() }` for Dimensions initialization -- all fields are u8/u16 integer types where zero is valid. Avoids writing out 80+ field initializations.
- Corrected polarized MGGA total from plan's stated 477 to the actual 767 -- the C code comment "in total: 477" refers only to MGGA-added order-4 cross-terms, not the full total across all orders.
- Removed cubecl from Cargo.toml dependencies since this plan only defines domain types; cubecl will be added when needed in Phase 2.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added Display impl for FunctionalId**
- **Found during:** Task 1 (domain model types)
- **Issue:** thiserror `#[error("... {id} ...")]` format strings require `Display` on `FunctionalId`, but the design doc only specifies `Debug` derive
- **Fix:** Added `impl std::fmt::Display for FunctionalId` that formats the inner u16
- **Files modified:** src/model/mod.rs
- **Verification:** cargo build succeeds, error format tests pass
- **Committed in:** fae422f (Task 1 commit)

**2. [Rule 1 - Bug] Corrected polarized MGGA total_output_components expected value**
- **Found during:** Task 3 (Dimensions struct)
- **Issue:** Plan stated total_output_components should be 477 for polarized MGGA, but summing all dimension values from libxc util.c yields 767. The "477" in the C code comment refers only to MGGA-added order-4 cross-terms.
- **Fix:** Test verifies correct total of 767; added spot-check tests for individual order-4 values
- **Files modified:** src/dims/mod.rs
- **Verification:** All dimension tests pass; values cross-referenced against libxc util.c
- **Committed in:** 22927a2 (Task 3 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
None beyond the deviations documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All domain types available for import by Plan 02 (static registry) and Plan 03 (input/output bundles)
- FunctionalId::from_raw() and from_name() are placeholder stubs returning errors -- will be wired to registry in Plan 02
- Cargo.toml ready for additional dependencies as needed

## Self-Check: PASSED

All 6 created files verified present. All 3 task commits verified in git log.

---
*Phase: 01-foundation-and-registry*
*Completed: 2026-04-09*
