---
phase: 09-reduce-kernel-build-time
plan: 02
subsystem: kernel-lda, kernel-gga, kernel-mgga, tools
tags: [incremental, batch-translation, build-optimization, kernel-restructure]
dependency_graph:
  requires: [incremental-lda-translator, incremental-gga-translator, incremental-mgga-translator]
  provides: [incremental-lda-kernels, incremental-gga-kernels, incremental-mgga-kernels]
  affects: [kernel-lda, kernel-gga-1, kernel-gga-2, kernel-gga-3, kernel-mgga-1 through kernel-mgga-37]
tech_stack:
  added: []
  patterns: [per-level-spin-file-split, incremental-delta-annotation, xc-integrate-translation]
key_files:
  created: []
  modified:
    - tools/translate_lda_v2.py
    - tools/translate_gga.py
    - crates/kernel-lda/src/lib.rs
    - crates/kernel-lda/src/**/*.rs
    - crates/kernel-gga-{1,2,3}/src/**/*.rs
    - crates/kernel-mgga-{1..37}/src/**/*.rs
decisions:
  - "Import paths fixed from crate::math to libxc_kernel_math in LDA and GGA translators"
  - "Added xc_integrate(func0/func1) -> xc_integrate_func0/func1 translation and implicit param_beta detection"
  - "Maintained 4 deferred LDA functionals and 6 deferred MGGA functionals due to CubeCL proc macro stack/OOM limits"
metrics:
  duration: 9361s
  completed: 2026-04-14T08:55:00Z
  tasks_completed: 7
  tasks_total: 7
  files_modified: 1500+
---

# Phase 09 Plan 02: Batch Re-translate All Kernel Crates Summary

All 239 kernel functionals (41 LDA, 106 GGA, 92 MGGA) re-translated with incremental derivative structure annotations using the translators built in Plan 01. LDA monolithic files split into per-(level, spin) subdirectories. Translator bugs fixed for import paths and xc_integrate function translation.

## Tasks Completed

| Task | Name | Commit | Key Changes |
|------|------|--------|-------------|
| 1 | Batch re-translate all LDA kernels | cefbbb18 | 41 functionals split into per-(level,spin) subdirs, import paths fixed |
| 2 | Verify all LDA kernels against oracle | 7f440fb9 | 3151 computation lines identical between old and new |
| 3 | Batch re-translate all GGA kernels | fae521c0 | 106 functionals re-translated, xc_integrate bug fixed |
| 4 | Verify all GGA kernels against oracle | e1433244 | All 3 sub-crates compile, computation identity confirmed |
| 5 | Batch re-translate all MGGA kernels | b65a99c1 | 92 functionals across 37 sub-crates |
| 6 | Verify all MGGA kernels against oracle | 84191334 | 86 compiled, 6 deferred, computation identity confirmed |
| 7 | Full workspace build and measurement | 33458eb6 | LDA+GGA: 12m32s total build time |

## Implementation Details

### LDA Re-translation (41 functionals)

Previously: 36 monolithic .rs files (all derivative orders in one file) + 4 already-split subdirs.
Now: All 41 functionals in per-(level, spin) subdirectories with incremental annotations.

- 37 functionals compiled (enabled in lib.rs)
- 4 deferred: lda_c_pk09 (kxc_pol 17.5K lines), lda_xc_ksdt (lxc_pol 14K), lda_c_pw_erf (lxc_pol 11K), lda_c_pmgb06 (lxc_pol 9.8K)
- Total: 187K lines across 451 files
- Build time: 2m10s (cargo check -j1)

### GGA Re-translation (106 functionals)

Previously: Already split into per-(level, spin) subdirectories across 3 sub-crates.
Now: Re-translated with incremental delta annotations.

- All 106 functionals compiled across 3 sub-crates (35 + 35 + 36)
- Total: 475K lines across 1166 files
- Build time: ~12m (3 crates, cargo check -j1)

### MGGA Re-translation (92 functionals)

Previously: Already split into per-(level, spin) subdirectories across 37 sub-crates.
Now: Re-translated with incremental delta annotations.

- 86 functionals compiled, 6 deferred (lxc_pol files 30K-58K lines)
- Total: 1.97M lines across 37 sub-crates
- Build time: MGGA crates OOM with 16GB RAM for large functionals (known limitation)

### Translator Bug Fixes

1. **Import path fix (LDA + GGA translators)**: `crate::math::*` changed to `libxc_kernel_math::*` in `translate_lda_v2.py` and `translate_gga.py`. The MGGA translator already used the correct path.

2. **xc_integrate translation (GGA translator)**: Added handling for `xc_integrate(func0, NULL, 0.0, x)` -> `xc_integrate_func0(x, param_beta)` in `translate_gga.py`. This pattern appears in `gga_x_fd_lb94` where the C source uses function pointer arguments. Also added implicit `param_beta` detection in `find_used_params()` when `xc_integrate(func` appears in compute lines.

## Build Time Measurements

| Crate | Functionals | Build Time (check -j1) | Status |
|-------|-------------|------------------------|--------|
| libxc-kernel-lda | 37 compiled, 4 deferred | 2m10s | OK |
| libxc-kernel-gga-1 | 35 | ~6m | OK |
| libxc-kernel-gga-2 | 35 | 5m06s | OK |
| libxc-kernel-gga-3 | 36 | ~6m | OK |
| LDA+GGA total | 143 | 12m32s | OK |
| libxc-kernel-mgga-* | 86 compiled, 6 deferred | OOM on large crates | Known limitation |

## Deviations from Plan

### Code Volume Reduction Not Achieved

**Found during:** Task 1
**Issue:** The plan expected 50-65% code volume reduction (3.7M -> 1.4M lines). The incremental translator from Plan 01 adds section-comment annotations marking shared preambles and per-level deltas, but does NOT actually reduce the code -- each function still contains all computation lines for its derivative level. The annotations mark boundaries for FUTURE extraction into helper functions.
**Impact:** Total code volume is essentially unchanged: LDA 182K->187K, GGA 469K->475K, MGGA 1.96M->1.97M (slight increase from per-file headers/imports).
**Actual benefit:** LDA monolithic files split into smaller per-(level,spin) files, reducing max single-file proc macro workload. GGA/MGGA already had this structure.

### Build Time Target Not Met

**Found during:** Task 7
**Issue:** Plan targeted <25 min full build. LDA+GGA takes 12m32s (promising), but MGGA crates OOM during compilation due to CubeCL proc macro memory consumption on large derivative functions.
**Root cause:** The CubeCL proc macro's memory usage is superlinear in function size. The largest MGGA functions (lxc_pol: 30K-58K lines) require >4GB RAM per rustc invocation.
**Path forward:** Plan 03 should focus on actual helper function extraction to split large functions into smaller callable units, which would directly reduce proc macro workload.

## Decisions Made

1. **Maintained deferred kernel lists**: Rather than force-compiling all functionals (which would OOM), maintained the existing deferred lists while ensuring all source files are generated with incremental structure.

2. **Fixed translator bugs inline**: Import path and xc_integrate translation bugs were discovered and fixed as part of the re-translation work (deviation Rule 1/3).

## Known Stubs

None -- all generated code is complete functional translations from maple2c C sources.

## Self-Check: PASSED
