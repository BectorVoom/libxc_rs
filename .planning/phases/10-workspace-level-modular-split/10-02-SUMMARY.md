---
phase: 10-workspace-level-modular-split
plan: 02
subsystem: infra
tags: [cargo, workspace, refactor, libxc-eval, features, kernel-deps]

requires:
  - phase: 10-01
    provides: libxc-core (data layer) with FunctionalId pub + PROPAGATION_RULES pub
provides:
  - "crates/libxc-eval: orchestration layer (eval/functional/kernel) + 306 kernel deps + [features] oracle-* machinery"
  - "SC-3 proven: cargo tree -p libxc-eval shows libxc-core, NOT libxc-compat"
  - "Feature-forward chain: root re-forwards oracle-* to libxc-eval (default-features=false pin holds — 0 kernels under --no-default)"
  - "libxc_rs::math preserved via the relocated shim + facade re-export"
affects: [10-03-libxc-compat-facade]

tech-stack:
  added: []
  patterns:
    - "Family-chunked feature machinery owned by the orchestration crate; root re-forwards"
    - "Bulk cross-crate repoint via find -exec sed (crate::<core-module>:: -> libxc_core::)"

key-files:
  created:
    - crates/libxc-eval/Cargo.toml
    - crates/libxc-eval/src/lib.rs
  modified:
    - Cargo.toml (306 kernel deps + [features] moved to eval; root re-forward + eval pin; default-members)
    - src/lib.rs (re-export libxc_eval modules + math)
    - "441 moved eval files (crate:: -> libxc_core:: repoint)"

key-decisions:
  - "math shim relocated into libxc-eval (NOT deleted per plan D-02) — verify uses libxc_rs::math::*, so the facade must preserve it; routed through eval (owns the math dep) to keep root deps core/eval-only."
  - "src/workspace moved undeclared (dead placeholder, 0 refs) like layout/ in 10-01 — avoids compiling stale dead code under #![deny(warnings)]."

patterns-established:
  - "OOM-critical feature chain verified by cargo tree (no compile): --no-default-features pulls 0 per-functional kernels at every layer"

requirements-completed: []

duration: ~50min (incl. a 10m eval+LDA compile gate)
completed: 2026-05-26
---

# Phase 10 / Plan 02: Extract libxc-eval Summary

**Extracted the orchestration layer (eval/functional/kernel + math shim) into crates/libxc-eval, migrated all 306 kernel deps + the [features] oracle-* machinery from root, repointed 441 cross-crate refs to libxc_core::; SC-3 + the memory-safe feature chain proven via cargo tree, both compile gates green.**

## Performance
- **Duration:** ~50 min (incl. a 10m10s eval+LDA compile gate)
- **Completed:** 2026-05-26
- **Tasks:** 2 (one commit, bisectability)
- **Files modified:** 188 (178 renames incl. the 441 repoints, 8 adds, 2 edits)

## Accomplishments
- `crates/libxc-eval` created: eval/functional/kernel + math shim; one-way dep on libxc-core (SC-3 via cargo tree: core present, compat absent)
- 441 `crate::{model,meta,registry,input,output,dims,error}::` → `libxc_core::` (find -exec sed); intra-eval `crate::{kernel,eval,functional}::` (672 refs) untouched
- 306 kernel deps moved to eval (paths `../kernels/`, optional preserved on 305, math non-optional); [features] block moved verbatim; root re-forwards oracle-* + pins libxc-eval `default-features=false`
- **Memory-safety proven (cargo tree, no compile):** eval & root `--no-default-features` pull ZERO per-functional kernels (Pitfall 1 pin holds); root `oracle-lda` → 42 LDA + math, 0 GGA/MGGA
- Both gates green: `cargo check -p libxc-eval --features oracle-lda` EXIT 0; kernel-free umbrella EXIT 0

## Task Commits
1. **Task 1 + Task 2 (bisectability — one commit)** — `5fbd512c99` (feat)

## Decisions Made
- **math shim relocated, not deleted (plan D-02 deviation):** verify uses `libxc_rs::math::{constants,powers,erf,dft_quantities,spin}` (Wave-0 surface), so deleting the shim would break SC-5. Moved `src/math/mod.rs` into libxc-eval (which owns libxc-kernel-math) + root `pub use libxc_eval::math`. Keeps root deps core/eval-only.
- **src/workspace moved undeclared:** dead top-level placeholder (0 refs, never compiled); moved as dead weight without a `pub mod` decl (consistent with layout/ in 10-01).

## Deviations from Plan
1. **[Plan bug] D-02 'delete math shim' wrong** — verify consumes `libxc_rs::math::*`. Relocated to eval + facade re-export instead.
2. **[Minor] src/workspace undeclared** rather than `pub mod workspace;` (the plan said declare it) — it was never compiled; declaring it would newly compile stale dead code under deny(warnings).

**Total deviations:** 2. No api/compat cross-crate visibility breaks (unlike 10-01's FunctionalId) — eval's items used by api/compat are all pub.

## Issues Encountered
- First sed attempt used `grep -rlZ | xargs -0` which botched NUL separation (passed all filenames as one arg) → repoint silently didn't run. Caught immediately (count check still 441), redone with `find -exec sed` → 441→0.
- Cargo.toml migration script's own re-forward comment contained the literal `dep:libxc-kernel-`, tripping a safety assertion (false positive) — reworded; script aborted cleanly before any write, so no partial state.

## Next Phase Readiness
- 10-03 (libxc-compat + thin facade) is unblocked. Root src/ now holds only api/, compat/, lib.rs, main.rs.
- **10-03 watch-outs:** (1) facade must ALSO preserve `pub use compat::c_layout::{xc_func_info_type, xc_func_type}` (Phase-6 addition the plan's facade spec omits) → `libxc_compat::c_layout::{...}`; (2) `pub use libxc_eval::math` must remain in the facade (verify's `libxc_rs::math`); (3) watch for compat→eval pub(crate) visibility breaks.

---
*Phase: 10-workspace-level-modular-split*
*Completed: 2026-05-26*
