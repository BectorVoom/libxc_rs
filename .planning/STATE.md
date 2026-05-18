---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: Phase 11 HALTED at plan 11-06 per AP-1/D-15 — three-leg gate leg 1 (`cargo build -p libxc-kernel-math`) fails with 515 errors. Architectural mismatch: 11-05 auto-script wrapped f64 named constants in F::new() but CubeCL Float::new(val: f32) cannot construct f64-precision values. Needs /gsd-discuss-phase 11 (4th iteration) for architectural decision on F::cast_from() vs Option C revival.
stopped_at: 11-06 HALT (2026-05-18). Commit `75c0f5112` — FAILED SUMMARY committed. No edits to crates/kernels/math/src/, tools/, src/model/, or verify/. .cargo/config.toml unchanged (AP-2 confirmed). Plans 11-01..05 executed; 11-06 surfaced architectural blocker via entry gate (per AP-6 design: this is the gate working as intended). Plans 11-07/11-08 are blocked behind 11-06 architectural decision.
last_updated: "2026-05-18T23:59:00Z"
last_activity: 2026-05-18 — Phase 11 plan 11-06 HALT. Discovered the 11-05 Phase 2 auto-script (`tools/refactor_helpers_generic.py`) produced 515 compile errors in `libxc-kernel-math`, dominated by 447 E0308 "expected f32, found f64" errors from `F::new(<f64 const>)` wraps. The CubeCL 0.10 Float trait declares `fn new(val: f32) -> Self` — f64 named constants (SQRT_DBL_EPSILON, RS_CONST, KF_CONST, ERX, ...) cannot pass through. The plan's "revert F::new(IDENT) to bare IDENT" rule also fails (bare f64 in F-generic body is also a type error). Proposed forward: `F::cast_from(<f64 const>)` via cubecl-core Cast trait. Requires user decision in /gsd-discuss-phase 11.
progress:
  total_phases: 11
  completed_phases: 6
  total_plans: 50
  completed_plans: 39
  percent: 78
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-09)

**Core value:** Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 XC functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.
**Current focus:** Phase 11 — splitter-v2-unified-5k-cap

## Current Position

Phase: 11 (splitter-v2-unified-5k-cap) — HALTED at plan 11-06, awaits architectural decision in /gsd-discuss-phase 11 (4th iteration)
Previous execution: 11-01..05 ✓; 11-06 HALTED with FAILED SUMMARY (`75c0f5112`)
Halt reason: `cargo build -p libxc-kernel-math` exits with 515 errors. ~508 are out-of-scope for plan 11-06's 3 named "syntax cleanup" categories. Root cause: CubeCL Float::new(val: f32) cannot construct from f64 named constants (SQRT_DBL_EPSILON, RS_CONST, KF_CONST, ERX, ...). Proposed forward path (needs user decision): F::cast_from(<f64 const>) via cubecl-core Cast trait, OR Option C revival.
Next step: `/gsd-discuss-phase 11` — 4th-iteration replan. Choose between (A1) extend `tools/refactor_helpers_generic.py` to emit `F::cast_from(<f64 const>)` for defined f64 constants, (A2) demote helper constants to f32 (probably violates 1e-12 oracle gate), (C) revive Option C cast-at-call-site in translator, or (Hybrid) Phase-1-files generic + Phase-2-files reverted to f64 + Option C at translator.
Plans: 8/8 written; 5/8 executed; 1/8 HALTED. 11-07/08 blocked behind 11-06 architectural decision.

Plan 11-03 outcome (2026-05-15):
- Task 1: verify-only re-confirmation of `95727cb36`+`97d6347be` (clean-slate
  266-subcrate restructure) — approved by user; no commit.
- Task 2 (`eea58fed7`): rewrote `tools/audit_cube_launch.sh` to the D-13
  per-design launch budget (routed one-per-output, unrouted-zero,
  math/src/ ≤22). PASS: 1654 routed pairs, 0 unrouted launchables, math=22.
- Task 3 (`f820fae90` --allow-empty): re-ran the three dispatch/re-export
  generators — zero git diff (deterministic against committed WIP
  `c3fba8089`). `audit_dispatch_tree.sh` exit 0; 0 batchN refs survive.
  Path-resolution gate ran at RUNG 2: built `libxc-kernel-lda_c_lp96`,
  `libxc-kernel-gga_x_lb`, `libxc-kernel-mgga_xc_lp90`, then `rustc --extern`
  type-checked the deep `crate::kernel::{family}::<func>::<output>::<fn>`
  re-export paths — exit 0. Blocker B1 closed.

Wave 2 is finished under D-13. Next plan: 11-04.

Plans: Phase 06 still has 3 of 4 executed (09-04, 09-05, 09-06 ✓; 09-07 oracle parity sweep pending; old 09-01/02/03 archived under `archive-pre-round4/`) — paused while Phase 11 is in flight.
Last activity: 2026-05-15 — Phase 11-03 complete

## Phase 11 — PAUSED at Plan 11-05, Option A → Option C Pivot (2026-05-18)

Attempted Option A (refactor 38 helpers to generic `<F: Float>`) blocked by Phase 2 automated-script systematic errors in 11 files. User elected to **pivot to Option C (cast-at-call-site in translator)** at 14:00 UTC. 

State: commit dcb7d517d marks partial fixes and checkpoint. Next session: replan with Option C.

Carry-forward from 11-05 attempt:
- Commit d8cc4da0c: Manual Phase 1 refactoring (5 files) validated as working ✓
- Commit dcb7d517d: Partial fixes + analysis of remaining errors
- Decision: abandon this path, move to translator-level approach

## Phase 05 — Gap Closure Resolved (2026-05-02)

The "Pending Resumption" block previously here was stale. The gap-closure work
flagged as paused on 2026-04-29 was actually completed on 2026-05-02. Concrete
evidence:

| Plan | WIP commit (paused) | Real fix commits (after resume) | SUMMARY |
|------|---------------------|---------------------------------|---------|
| 05-04 | df5324f1 | c20a0225 → 50508037 → 08996314 | 20.6 KB ✓ |
| 05-05 | 861f21dd | 0afc877a → 41bffc29 → 46ce5b9c → 69a372f8 | 17.7 KB ✓ |
| 05-06 | (no wip) | cb634de1 → 01f6039a → 6c5ac9f1 | 19.8 KB ✓ |
| 05-07 | 71fdddd8 | 45896f06 → b4cd019c | 9.8 KB ✓ |

The original locked worktree branches (`worktree-agent-a5c0fcda…`,
`…a841c937…`, `…a47664d4…`) and WIP SHAs (`eb08f4ab`, `3cbc49f9`, `ae6b847c`)
referenced in the older note are no longer reachable in git — superseded by
the clean fix commits above.

Outstanding: re-run `/gsd-verify-work 5` to upgrade `05-VERIFICATION.md`
(currently `status: human_needed`, dated pre-resume 2026-04-28) to `pass`.

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
| Phase 11 P03 | 25min | 3 tasks | 1 files |

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
- [Phase 11-03]: D-13 launch budget — audit_cube_launch.sh asserts routed (functional,output) one-per-output, unrouted-zero, math/src/ <=22; flat <=23 count form retired
- [Phase 11-03]: path-resolution gate ran at RUNG 2 (build 3 spot-check routed subcrates + rustc --extern the deep re-export paths); RUNG 1 (cargo check -p libxc_rs) would compile all 268 kernel deps (D-12 OOM risk)

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 2 is the key technical risk gate: CubeCL must produce bit-accurate f64 results for LDA_X canary kernel before bulk translation begins
- CubeCL lacks erf/erfc and cbrt intrinsics -- must be implemented as pure #[cube] functions
- Large MGGA kernels (50K-100K lines) may exceed GPU compiler limits -- test early in Phase 4

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260508-q01 | Update cubecl to 0.10.0 in all workspace | 2026-05-08 | 784c8fc8 | [260508-q01-update-cubecl-010-workspace](.planning/quick/260508-q01-update-cubecl-010-workspace/) |
| 260509-q01 | Raise TARGET_MAX 50000→500000 in splitter tools and re-run (re-runs were no-ops on current tree) | 2026-05-09 | 8be648ce | [260509-q01-split-thresholds-10x](.planning/quick/260509-q01-split-thresholds-10x/) |
| 260509-q03 | Unified maple-to-kernel driver (translate+split) with explicit splitting-criteria knobs | 2026-05-09 | 37820e2d | [260509-q03-maple-to-kernel-driver](.planning/quick/260509-q03-maple-to-kernel-driver/) |
| 260509-q04 | Add --repack to split_lda_subcrates.py and consolidate LDA 4→2 sub-crates (-2 workspace members) | 2026-05-09 | a8fe9020 | [260509-q04-consolidate-lda-4-to-2](.planning/quick/260509-q04-consolidate-lda-4-to-2/) |
| 260509-q05 | Fix resplit_gga + rebatch_mgga merge logic; consolidate GGA 59→8 and MGGA 109→14 (-144 sub-crates total) | 2026-05-09 | bab60f19 | [260509-q05-consolidate-gga-mgga](.planning/quick/260509-q05-consolidate-gga-mgga/) |
| 260509-q06 | Fix resplit_gga orphan-leak + further reduce: LDA 2→1, GGA 8→5, MGGA 14→8 (-10 sub-crates, -1810 orphan files) | 2026-05-09 | ff5637ac | [260509-q06-reduce-kernels-more](.planning/quick/260509-q06-reduce-kernels-more/) |
| 260509-q07 | Move all 17 kernel sub-crates under crates/kernels/ parent dir; update 7 splitter tools to match | 2026-05-09 | d4fd678a | [260509-q07-kernels-parent-dir](.planning/quick/260509-q07-kernels-parent-dir/) |
| 260509-q08 | Reduce GGA/MGGA/LDA per-crate size: GGA 5→8 (300K), MGGA 8→14 (300K, 4 solo-oversized), LDA 1→2 (100K); fix latent rebatch_mgga.py update_workspace path bug | 2026-05-09 | 3224d347 | [260509-q08-reduce-gga-mgga-files](.planning/quick/260509-q08-reduce-gga-mgga-files/) |
| 260510-q01 | Investigate cargo build OOM root cause: RUST_MIN_STACK 1.87 GiB → 64 MiB typo (was 30× too large); split mgga-{8,9,11} solo-oversized crates via --target-max=350000 (mgga-8→8a/8b, mgga-9→9a/9b, mgga-11→11a/11b); add --target-max parsing + post-q07 path fix to split_oversized_mgga.py | 2026-05-10 | 58753e18 | [260510-q01-investigate-kernel-oom](.planning/quick/260510-q01-investigate-kernel-oom/) |
| 260510-q02 | Restore mgga_x_2d_prp10 module deferral (libxc id 211, missing Bessel I0/I1) lost in q06/q08 lib.rs regeneration | 2026-05-10 | 28a6ea65 | [260510-q02-restore-prp10-deferral](.planning/quick/260510-q02-restore-prp10-deferral/) |
| 260512-q01 | Routing-aware translator emit: emit `#[cube]` for unrouted functionals (closes regen-reintroduces-launch_unchecked loop); fix `demote_unrouted_kernels.py` glob (was no-op since `crates/kernel-* → crates/kernels/*` move); 32 lda-2 entry kernels demoted | 2026-05-12 | 61c9f620 | [260512-q01-routing-aware-translator-emit](.planning/quick/260512-q01-routing-aware-translator-emit/) |
| 260512-q02 | Fix translator merge-suffix filename overflow (was hitting Linux 255-byte path limit on lxc-level kernels with 40+ output fields); regen mgga-14's mgga_x_br89_explicit + mgga_x_r4scan at SPLIT_THRESHOLD=6000 (max line 21,679 → 5,352, unblocks mgga-14 OOM) | 2026-05-12 | 22640588 | [260512-q02-fix-merge-filename-overflow](.planning/quick/260512-q02-fix-merge-filename-overflow/) |
| 260514-q01 | Split mgga-2 and nearby large MGGA kernels: re-emitted all mgga-2 functionals plus mgga_c_ccalda; targeted files now ≤5K lines | 2026-05-14 | 0506d0e5 | [260514-q01-split-mgga-2-large-kernels](.planning/quick/260514-q01-split-mgga-2-large-kernels/) |

## Session Continuity

Last session: 2026-05-15T02:55:30.719Z
Stopped at: Phase 11 context REVISED — per-functional subcrates unification target; D-04/D-05/D-10/D-LOCK-A revised, D-11/D-12 added; plans 11-02..06 stale, replan required
Resume file: None
