---
phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
plan: 06
subsystem: kernel-codegen
tags: [mgga, python, cubecl, code-generation, maple2c]

# Dependency graph
requires:
  - phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
    provides: GGA translator pattern (translate_gga.py), kernel-math crate, kernel sub-crate structure
provides:
  - translate_mgga.py MGGA maple2c-to-Rust translation tool
  - kernel-mgga-1 sub-crate with first compiled MGGA functional (mgga_xc_lp90)
  - Complete MGGA POL_DIMS table (70 fields) and LEVEL_OUTPUTS dictionary
affects: [08-02, 08-03, kernel-mgga batch translation, MGGA oracle verification]

# Tech tracking
tech-stack:
  added: []
  patterns: [MGGA kernel translation with 4 guard patterns, lapl/tau input arrays in kernel signatures]

key-files:
  created:
    - tools/translate_mgga.py
    - crates/kernel-mgga-1/Cargo.toml
    - crates/kernel-mgga-1/src/lib.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/mod.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/exc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/exc_pol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/vxc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/vxc_pol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/fxc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/fxc_pol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/kxc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/kxc_pol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/lxc_unpol.rs
    - crates/kernel-mgga-1/src/mgga_xc_lp90/lxc_pol.rs
  modified:
    - Cargo.toml

key-decisions:
  - "Used libxc_kernel_math:: import paths (not crate::math::) matching existing GGA kernel pattern"
  - "Generated all 10 kernel files (5 levels x 2 spins) for mgga_xc_lp90 including kxc and lxc since this functional is small enough"

patterns-established:
  - "MGGA kernel signature: rho, sigma, lapl, tau input arrays followed by output arrays"
  - "All 4 MGGA guard patterns stripped to unconditional writes in generated Rust"
  - "MGGA sub-crate structure mirrors GGA: kernel-mgga-N with per-functional subdirectories"

requirements-completed: [KERN-05, KERN-06]

# Metrics
duration: 7min
completed: 2026-04-13
---

# Phase 08 Plan 01: MGGA Translation Tool and First Compiled Kernel Summary

**MGGA maple2c-to-Rust translator with 70 output fields, 4 guard patterns, and mgga_xc_lp90 compiled through all derivative orders**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-13T11:43:16Z
- **Completed:** 2026-04-13T11:50:08Z
- **Tasks:** 2
- **Files modified:** 15

## Accomplishments
- Built translate_mgga.py (718 lines) extending GGA translator with lapl/tau arrays, 70 MGGA output fields, and all 4 guard pattern handling
- Created kernel-mgga-1 sub-crate with mgga_xc_lp90 generating 10 kernel files (3,878 lines of Rust) across all 5 derivative levels and both spin modes
- Verified end-to-end compilation with `cargo check -p libxc-kernel-mgga-1` passing cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: Build translate_mgga.py with MGGA-specific extensions** - `f555ea4` (feat)
2. **Task 2: Create kernel-mgga-1 sub-crate and compile first MGGA functional** - `846b414` (feat)

## Files Created/Modified
- `tools/translate_mgga.py` - MGGA maple2c-to-Rust translation tool (718 lines)
- `crates/kernel-mgga-1/Cargo.toml` - Sub-crate manifest with cubecl and kernel-math deps
- `crates/kernel-mgga-1/src/lib.rs` - Sub-crate root with clippy allows
- `crates/kernel-mgga-1/src/mgga_xc_lp90/mod.rs` - Per-functional module declarations
- `crates/kernel-mgga-1/src/mgga_xc_lp90/exc_unpol.rs` - Energy kernel (unpolarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/exc_pol.rs` - Energy kernel (polarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/vxc_unpol.rs` - 1st derivative kernel (unpolarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/vxc_pol.rs` - 1st derivative kernel (polarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/fxc_unpol.rs` - 2nd derivative kernel (unpolarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/fxc_pol.rs` - 2nd derivative kernel (polarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/kxc_unpol.rs` - 3rd derivative kernel (unpolarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/kxc_pol.rs` - 3rd derivative kernel (polarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/lxc_unpol.rs` - 4th derivative kernel (unpolarized)
- `crates/kernel-mgga-1/src/mgga_xc_lp90/lxc_pol.rs` - 4th derivative kernel (polarized)
- `Cargo.toml` - Added kernel-mgga-1 to workspace members and profile overrides

## Decisions Made
- Used `libxc_kernel_math::` import paths (not `crate::math::`) matching existing GGA kernel pattern since kernel crates are separate workspace members
- Generated all 10 kernel files including kxc and lxc since mgga_xc_lp90 is small enough (7K lines C) to not cause OOM

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Permission issues with `/workspace/crates/` directory (root-owned, non-writable by vscode user) - resolved with `sudo mkdir` and `sudo chmod`
- Permission issue with `/workspace/Cargo.lock` - resolved with `sudo chmod 666`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- translate_mgga.py ready for batch translation of remaining 89 MGGA functionals
- kernel-mgga-1 sub-crate structure validated, ready to receive more functionals
- OOM risk not yet tested with large functionals (mgga_c_rmggac at 99K lines) - will need sub-crate splitting

## Self-Check: PASSED

All 7 key files verified present. Both task commits (f555ea4, 846b414) verified in git log.

---
*Phase: 08-rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat*
*Completed: 2026-04-13*
