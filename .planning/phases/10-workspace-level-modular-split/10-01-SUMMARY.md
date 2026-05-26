---
phase: 10-workspace-level-modular-split
plan: 01
subsystem: infra
tags: [cargo, workspace, refactor, libxc-core, visibility]

requires:
  - phase: 10-00
    provides: pre-refactor baselines (surface, generated-file bytes, oracle pass/fail)
provides:
  - "crates/libxc-core: cubecl-free data layer (model/meta/registry/input/output/layout/dims/error + deferred)"
  - "SC-2 proven: cargo tree -p libxc-core shows zero cubecl/kernel deps"
  - "Relocated deferred id-registry (D-11) + xtask write-path repoint (D-03)"
  - "FunctionalId field widened to pub (cross-crate construction); PROPAGATION_RULES pub (regen-proof)"
affects: [10-02-libxc-eval, 10-03-libxc-compat-facade]

tech-stack:
  added: []
  patterns:
    - "Workspace leaf crate: pure-data layer with bitflags+bytemuck+thiserror only, no compute"
    - "Cross-crate visibility: pub(crate) core items used by eval/functional must widen to pub"

key-files:
  created:
    - crates/libxc-core/Cargo.toml
    - crates/libxc-core/src/lib.rs
  modified:
    - crates/kernels/math/src/lib.rs (removed deferred decl)
    - src/lib.rs (re-export libxc_core modules)
    - Cargo.toml (libxc-core dep)
    - xtask/src/main.rs + generate_metadata.rs (D-03 write paths + emitter)
    - crates/libxc-core/src/model/mod.rs (FunctionalId field pub; LibxcRsError path)
    - crates/libxc-core/src/meta/mod.rs + generated_propagation.rs (PROPAGATION_RULES pub)

key-decisions:
  - "PROPAGATION_RULES: used option A (const -> pub in file + xtask emitter) not the plan's option B (pub use of a pub(crate) const is invalid — E0364). Regen-proof; intentional 1-token delta vs Wave-0 snapshot."
  - "FunctionalId(pub(crate) u16) -> (pub u16): ~240 cross-crate construction sites in root eval/functional/kernel. One field widening vs rewriting 240 call sites. Plan research said 'only PROPAGATION_RULES crosses' — wrong."

patterns-established:
  - "Bisectability via kernel-free gate: cargo check -p libxc_rs --no-default-features --lib (per the 10-00 gate-strategy decision)"

requirements-completed: []

duration: ~45min
completed: 2026-05-26
---

# Phase 10 / Plan 01: Extract libxc-core Summary

**Extracted the cubecl-free data layer (model/meta/registry/input/output/layout/dims/error + relocated deferred registry) into crates/libxc-core; SC-2 proven, root re-exports it, both compile gates green.**

## Performance
- **Duration:** ~45 min
- **Completed:** 2026-05-26
- **Tasks:** 2 (landed in ONE commit per bisectability)
- **Files modified:** 70 (57 renames, 5 adds, 3 deletes, 5 edits)

## Accomplishments
- `crates/libxc-core` created: 8 data modules + relocated `deferred` (D-11), zero cubecl/kernel deps (SC-2 via `cargo tree`)
- xtask write paths repointed into core (D-03); 5/6 generated files byte-identical to the Wave-0 snapshot
- Root reduced to re-export `pub use libxc_core::{...}` for the moved modules; still compiles
- Both gates green: `cargo check -p libxc-core` EXIT 0; kernel-free umbrella `cargo check -p libxc_rs --no-default-features --lib` EXIT 0

## Task Commits
1. **Task 1 + Task 2 (bisectability — one commit)** — `33d349ce28` (feat)

## Decisions Made
- **PROPAGATION_RULES (option A, not plan's B):** `pub use` of a `pub(crate)` const is E0364. Made the const `pub` in both `generated_propagation.rs` AND the xtask emitter (regen-proof) + `pub mod` + shallow re-export. Intentional 1-token delta vs the Wave-0 snapshot; the other 5 generated files are byte-identical.
- **FunctionalId field `pub(crate)` → `pub`:** ~240 `FunctionalId(x)` cross-crate construction sites in root's eval/functional/kernel. One field widening resolved all 240 (vs rewriting 240 call sites to a constructor). Additive; no SC-5 path change.

## Deviations from Plan
1. **[Research gap] Cross-crate visibility far larger than "only PROPAGATION_RULES."** 240 `FunctionalId` construction sites broke (E0423). Fixed by widening the tuple field to `pub`. The plan's RESEARCH claim was wrong; flag for 10-02 (the eval modules that construct FunctionalId move there and now reference `libxc_core::` types).
2. **[Plan bug] PROPAGATION_RULES option B is invalid (E0364).** Used the plan's documented alternative (option A). Required editing the xtask emitter to stay regen-proof.
3. **[Minor] `crate::LibxcRsError` → `crate::error::LibxcRsError`** (2 sites in model/mod.rs) — relied on root's flattened re-export absent in core.

**Total deviations:** 3 (all forced by cross-crate visibility; no scope creep beyond the necessary widenings).

## Issues Encountered
- D-03 live regen idempotency (run xtask, diff bytes) DEFERRED to final verification — verified statically instead (5/6 byte-identical; generated_propagation differs by the intentional pub delta). Running xtask is a separate compile; deferring per plan allowance.

## Next Phase Readiness
- libxc-core is the foundation for 10-02 (libxc-eval) and 10-03 (libxc-compat).
- **10-02 watch-out:** the ~435 `crate::{model,error,...}::` → `libxc_core::` repoint is the big edit; FunctionalId is now `pub` so eval can construct it cross-crate. Surface any further `pub(crate)` core items eval needs.

---
*Phase: 10-workspace-level-modular-split*
*Completed: 2026-05-26*
