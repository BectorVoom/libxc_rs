---
phase: 01-foundation-and-registry
plan: 02
subsystem: registry
tags: [code-generation, xtask, registry, sparse-array, binary-search]

requires:
  - phase: 01-foundation-and-registry/01
    provides: "Domain model types (FunctionalId, Family, Kind, FunctionalMeta, LibxcRsError)"
provides:
  - "649 FunctionalMeta const entries generated from C headers"
  - "O(1) ID lookup via sparse array REGISTRY_BY_ID"
  - "O(log n) name lookup via sorted REGISTRY_BY_NAME with binary search"
  - "Removed/aliased ID handling (1 truly removed, 24 name aliases)"
  - "FunctionalId::from_raw/from_name wired to registry"
  - "Version and reference string functions"
affects: [phase-02, phase-03, phase-04, phase-05]

tech-stack:
  added: [regex, anyhow (xtask only)]
  patterns: [xtask-code-generation, sparse-array-registry, binary-search-name-lookup]

key-files:
  created:
    - xtask/Cargo.toml
    - xtask/src/main.rs
    - src/meta/generated.rs
    - src/registry/mod.rs
    - src/registry/by_id.rs
    - src/registry/by_name.rs
    - src/registry/removed.rs
  modified:
    - Cargo.toml
    - src/lib.rs
    - src/meta/mod.rs
    - src/model/mod.rs

key-decisions:
  - "Only ID 104 is truly removed; all other 'removed' IDs were reassigned in xc_funcs.h"
  - "Separated source root from output root in xtask for git worktree compatibility"
  - "Name aliases use case-insensitive matching for backward compatibility"

patterns-established:
  - "Xtask code generator pattern: parse vendored C headers, generate Rust source"
  - "Sparse array registry: Option<&'static FunctionalMeta> indexed by ID for O(1)"
  - "Sorted name table with binary_search_by_key for O(log n) name lookup"

requirements-completed: [DOM-02, REG-01, REG-02, REG-03, REG-04, REG-05, ERR-02]

duration: 8min
completed: 2026-04-09
---

# Phase 01 Plan 02: Xtask Code Generator and Static Registry Summary

**Xtask generator parsing xc_funcs.h/xc_funcs_removed.h producing 649 FunctionalMeta entries with O(1) ID lookup, O(log n) name search, and removed/alias handling**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-08T23:53:20Z
- **Completed:** 2026-04-09T00:01:23Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments
- Built xtask code generator that parses vendored C headers and produces all registry source files
- Generated 649 FunctionalMeta const entries with correct family/kind classification from name prefixes
- Created sparse array (1024 slots) for O(1) ID lookup and sorted name table for O(log n) binary search
- Wired FunctionalId::from_raw/from_name to registry with full error handling for removed/unknown IDs
- All 38 library tests pass with zero clippy warnings

## Task Commits

Each task was committed atomically:

1. **Task 1: Build xtask code generator** - `a56dd15` (feat)
2. **Task 2: Create registry lookup API** - `b0cf1b4` (feat)
3. **Task 3: Wire FunctionalId to registry** - `1b37fc1` (feat)

## Files Created/Modified
- `xtask/Cargo.toml` - Xtask workspace member with regex and anyhow dependencies
- `xtask/src/main.rs` - Code generator parsing xc_funcs.h and xc_funcs_removed.h
- `src/meta/generated.rs` - 649 FunctionalMeta const entries (auto-generated)
- `src/registry/mod.rs` - Public lookup API: lookup_by_id, lookup_by_name, version functions
- `src/registry/by_id.rs` - Sparse array [Option<&FunctionalMeta>; 1024] (auto-generated)
- `src/registry/by_name.rs` - Sorted name table for binary search (auto-generated)
- `src/registry/removed.rs` - 1 truly removed ID and 24 name aliases (auto-generated)
- `Cargo.toml` - Added xtask to workspace members
- `src/lib.rs` - Added pub mod registry and re-exports
- `src/meta/mod.rs` - Added pub(crate) mod generated
- `src/model/mod.rs` - Wired FunctionalId methods to registry

## Decisions Made
- Only ID 104 (XC_GGA_X_HERMAN) is truly removed from the active registry. All other 27 "removed" IDs in xc_funcs_removed.h were reassigned to different functionals in xc_funcs.h (e.g., ID 167 was XC_GGA_XC_B97 but is now XC_GGA_XC_KT1). Adding these to REMOVED_IDS would break lookups for the reassigned functionals.
- Separated source root (for finding libxc-master/) from output root (for writing generated files) in the xtask generator to support git worktree setups.
- Name alias lookup uses eq_ignore_ascii_case for case-insensitive backward compatibility (e.g., XC_LDA_C_vBH maps to ID 17).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed removed ID handling for reassigned IDs**
- **Found during:** Task 2 (registry lookup API)
- **Issue:** Plan assumed all 28 "removed" IDs should go in REMOVED_IDS, but 27 of them were reassigned in xc_funcs.h. lookup_by_id checked REMOVED_IDS first, causing valid lookups (e.g., ID 167 = XC_GGA_XC_KT1) to return RemovedFunctionalId error.
- **Fix:** Modified xtask generator to only add IDs to REMOVED_IDS if they do not exist in the active xc_funcs.h. Reassigned IDs are simply active with their new names.
- **Files modified:** xtask/src/main.rs, src/registry/removed.rs
- **Verification:** test_registry_completeness passes (all 649 IDs resolve correctly)
- **Committed in:** b0cf1b4 (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed xtask output path for git worktree**
- **Found during:** Task 1 (xtask code generator)
- **Issue:** find_workspace_root() resolved to main repo root (containing libxc-master/) but generated files needed to be written to the worktree's src/ directory. First run wrote generated.rs to wrong location and failed on missing src/registry/ in main repo.
- **Fix:** Split into find_source_root() (for reading headers) and find_output_root() (for writing generated files).
- **Files modified:** xtask/src/main.rs
- **Verification:** Generator writes all files to correct worktree paths
- **Committed in:** a56dd15 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes were essential for correctness. No scope creep.

## Issues Encountered
None beyond the auto-fixed deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Registry is complete with all 649 functionals accessible via ID and name lookup
- FunctionalId is fully wired to registry with from_raw(), from_name(), name(), family(), meta()
- Ready for Plan 03 (dimension calculations) which depends on registry lookups
- All subsequent phases can use lookup_by_id/lookup_by_name for functional metadata access

---
*Phase: 01-foundation-and-registry*
*Completed: 2026-04-09*
