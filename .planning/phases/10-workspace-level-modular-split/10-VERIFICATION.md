---
status: passed
phase: 10-workspace-level-modular-split
verified: 2026-05-26
method: goal-backward (inline; evidence committed to git + log/)
plans_complete: 4/4
success_criteria: 8/8
---

# Phase 10 Verification — Workspace-Level Modular Split

**Verdict: PASSED.** All 4 plans executed; all 8 ROADMAP success criteria verified against the live codebase. The monolithic root `libxc_rs` crate is now a layered Cargo workspace (`libxc-core` ← `libxc-eval` ← `libxc-compat` + thin facade) with compiler-enforced boundaries, the public surface preserved line-for-line, and oracle parity unchanged at 1e-12.

## Goal
Refactor root `libxc_rs` into a layered workspace: `libxc-core` (data, no compute/CubeCL), `libxc-eval` (orchestration, one-way dep on core), `libxc-compat` (extern-C shim, depends on both, depended on by neither), root = thin facade over `api/` re-exporting the curated surface.

## Success Criteria (goal-backward)

| SC | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| SC-1 | 4 target crates exist (core/eval/compat + facade) | ✓ | `cargo metadata --no-deps` shows libxc-core/eval/compat; root libxc_rs is the facade |
| SC-2 | `cargo tree -p libxc-core` zero cubecl/kernel deps | ✓ | log/10-01-tree-core.log — only bitflags/bytemuck/thiserror |
| SC-3 | `cargo tree -p libxc-eval` has core, NOT compat | ✓ | log/10-02-tree-eval.log |
| SC-4 | `cargo tree -p libxc-compat` has core+eval; nothing depends on it | ✓ | log/10-03-tree-compat.log + `--invert` (only compat itself) |
| SC-5 | Root public surface unchanged | ✓ | log/10-03-surface-diff.log — ZERO removed `use libxc_rs::` paths; only the intentional `libxc_rs::deferred` delta (D-11) |
| SC-6 | Per-family verify pass/fail set matches pre-refactor snapshot | ✓ | log/10-final-parity-{lda,gga,mgga}.log == log/10-baseline-parity-*.log (LDA 2/2, GGA 2/2, MGGA 2/2; zero failures) |
| SC-7 | Oracle parity preserved at 1e-12 (LDA/GGA/MGGA witnesses) | ✓ | same logs — all pass at the locked tolerance tiers |
| SC-8 | Per-`-p` build zero new warnings | ✓ | `#![deny(warnings)]` on every crate; all `cargo check` gates EXIT 0 |

## Gate strategy (user-confirmed, deviation from plan default)
Phase 10 is a kernel-invariant module-move refactor on a RAM-constrained box. Per the user's decision, the in-phase compile gates were **kernel-free** (`cargo check -p libxc_rs --no-default-features --lib`) + **per-family** (`cargo check -p libxc-eval --features oracle-<fam>`), with `cargo tree` for boundary topology — never `--workspace`/all-kernel builds (OOM). The full-tree green-ness is anchored by Phase 11/12; the USER-RUN per-family oracle is the authoritative numerical gate.

## Key deviations (all forced/correctness; full detail in plan SUMMARYs)
- **FunctionalId field `pub(crate)` → `pub`** (10-01): ~240 cross-crate construction sites; the plan's research claimed "only PROPAGATION_RULES crosses" (wrong). Additive, no SC-5 path change.
- **PROPAGATION_RULES option A not B** (10-01): `pub use` of a `pub(crate)` const is E0364; made the const `pub` (file + xtask emitter, regen-proof).
- **math shim relocated to eval, not deleted** (10-02): verify uses `libxc_rs::math::*` (the plan's D-02 delete was wrong).
- **compat pins eval `default-features=false` + forwards features** (10-03): the plan's compat Cargo.toml would force all 306 kernels into every umbrella check via feature-unification → OOM.
- **mgga_dispatch `$crate` macro metavar fix** (10-02 latent, caught at the final MGGA oracle): the repoint sed mangled `$crate::error::` → `$libxc_core::`; latent behind `#[cfg(feature=oracle-mgga)]`. Build bug, not a parity regression. See memory `feedback_sed_repoint_corrupts_macro_dollar_crate`.

## Commits
- 10-00 `1bdc8e0f50`, `60c993427c` · 10-01 `33d349ce28`, `a64e6e80a2` · 10-02 `5fbd512c99`, `4825fbae05` · 10-03 `91f69f298f`, `247932a619` (fix), `6ea6e8bcd3`

## Outstanding / notes
- libxc-compat cdylib (`libxc_rs.so`/`.a`) is excluded from `[workspace] default-members` — its all-kernel link is OOM-heavy; build deliberately with `cargo build -p libxc-compat`. Its extern-C symbol surface was relocated as a unit (no symbols dropped/renamed) but an `nm`-diff of the built cdylib was NOT run (OOM-heavy build) — deferred to a deliberate cdylib build.
- D-03 xtask regen idempotency was verified statically (5/6 generated files byte-identical to the Wave-0 snapshot; generated_propagation.rs differs only by the intentional pub bump); a live `cargo xtask generate-metadata` regen was deferred.
