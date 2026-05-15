---
phase: 11-splitter-v2-unified-5k-cap
plan: 03
subsystem: infra
tags: [cubecl, kernel-splitter, dispatch-codegen, audit-tooling, launch-budget, D-13]

# Dependency graph
requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 11-01)
    provides: Wave-0 audit tooling (audit_cube_launch.sh, audit_dispatch_tree.sh, audit_subcrate_collapse.sh, audit_kernel_size.py), D-02 chunk-ABI spike
  - phase: 11-splitter-v2-unified-5k-cap (plan 11-02)
    provides: routing-aware emit.py launch policy (one launch_unchecked per routed output module)
provides:
  - D-13 per-design launch-budget audit (tools/audit_cube_launch.sh rewritten — routed one-per-output, unrouted-zero, math/src ≤22)
  - Re-confirmed 266-subcrate clean-slate restructure (95727cb36 + 97d6347be) as Wave-2 provenance/rollback point
  - Complete per-functional dispatch tree (no batchN segment) — Blocker B1 closed under D-13
  - cargo/rustc path-resolution gate proving crate::kernel::{family}::<func> re-export paths resolve
affects: [11-04, 11-05, 11-06]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Per-design launch-budget audit: assert routed/unrouted partition from kernel_routing.py, not a flat launch_unchecked count"
    - "Audit logic as an exec'd Python block inside a thin bash wrapper (routing partition consumed in Python)"
    - "Path-resolution gate via rustc --extern against pre-built spot-check kernel rlibs (RUNG 2 of the fallback ladder)"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-03-SUMMARY.md
  modified:
    - tools/audit_cube_launch.sh

key-decisions:
  - "Task 3 produced no file changes — the c3fba8089 dispatch tree is byte-reproducible from the deterministic generators and already committed; recorded as an --allow-empty verification commit"
  - "Path-resolution gate ran at RUNG 2 (build 3 spot-check routed subcrates + rustc --extern the deep re-export paths) rather than RUNG 1 (cargo check -p libxc_rs would compile all 268 kernel deps — the D-12 OOM risk on this 30 GB machine)"

patterns-established:
  - "D-13 launch budget: every routed (functional, output-module) entry file has exactly one #[cube(launch_unchecked)]; unrouted functionals carry zero launchables anywhere; crates/kernels/math/src ≤22 (math/tests excluded)"

requirements-completed: [P11-INV-1, P11-INV-2, P11-INV-5, P11-INV-6]

# Metrics
duration: ~25min
completed: 2026-05-15
---

# Phase 11 Plan 03: Finish Wave 2 under D-13 Summary

**Rewrote `audit_cube_launch.sh` from the unsatisfiable flat `≤23` count to the D-13 per-design launch budget, re-confirmed the deterministic per-functional dispatch tree, and ran a `rustc --extern` path-resolution gate proving the `crate::kernel::{family}::<func>` re-export paths resolve — Blocker B1 closed.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-05-15 (continuation agent, post Task-1 approval)
- **Completed:** 2026-05-15
- **Tasks:** 3 (Task 1 verify-only/approved in prior session; Tasks 2–3 this session)
- **Files modified:** 1 (`tools/audit_cube_launch.sh`)

## Accomplishments

- **Task 2 — D-13 audit rewrite:** `tools/audit_cube_launch.sh` no longer asserts a flat `BASELINE=23` count. It now asserts three per-design conditions, consuming `tools/kernel_routing.py` as the routed/unrouted source of truth. Passes against the current tree: **1654 routed (functional, output) launch pairs** each with exactly one `#[cube(launch_unchecked)]`, **0 unrouted functionals** with a launchable kernel, **`crates/kernels/math/src/` = 22** (`math/tests/` excluded).
- **Task 3 — dispatch-tree completion:** the three generators (`generate_gga_dispatch.py`, `generate_mgga_dispatch.py`, `generate_kernel_reexports.py`) are confirmed deterministic — re-running all three produced zero git diff against the committed WIP `c3fba8089`. `audit_dispatch_tree.sh` exits 0; 0 `batchN` references survive in the dispatch tree.
- **Task 3 — path-resolution gate:** the gate WIP `c3fba8089` never ran. Built three spot-check routed subcrates (one per family), then `rustc --extern` type-checked the deep `crate::kernel::{family}::<func>::<output>::<fn>` re-export paths — exit 0, zero errors.

## Task 1 Re-confirmation (verify-only — completed and approved in the prior session)

Task 1 is a non-destructive re-confirmation of the committed clean-slate restructure. All eight read-only audit checks passed on substance:

1. `bash tools/audit_subcrate_collapse.sh` → exit 0 (zero numbered subcrates, zero family-level crates).
2. `python3 tools/audit_kernel_size.py --strict` → exit 0 (70 files >5000 lines, all documented D-LOCK-B exceptions; 0 unexcepted violations).
3. `find crates/kernels/{lda,gga,mgga} -maxdepth 2 -name Cargo.toml | wc -l` → ≥200 per-functional subcrates present.
4. Family dirs `lda/gga/mgga` survive as folders; `math` survives as a crate.
5. No family-level `Cargo.toml` under `crates/kernels/{lda,gga,mgga}/`.
6. Root `Cargo.toml` has 0 numbered/family-façade kernel deps.
7. `default-members` (Cargo.toml lines 297–558) excludes all 7 deferred kernels per D-11. (Note: the acceptance grep *as literally written* — `grep -A400 'default-members'` — returned 9, a confirmed false positive: the `-A400` window anchors on a line-15 comment mentioning `default-members` and sweeps into the `[dependencies]` path-deps. The actual `default-members` array has 0 deferred kernels — verified by inspecting the array directly.)
8. `git log` confirms `95727cb36` and `97d6347be` are in history.

**No destructive command ran** — no `git rm`, no splitter / `maple_to_kernels.py`, no regen invocation. **`95727cb36` (clean-slate delete) + `97d6347be` (266-subcrate regen + root manifest rewrite) are the documented provenance/rollback point for Wave 2.**

## D-13 `audit_cube_launch.sh` Rewrite (Task 2)

The original script was a flat count: `BASELINE=23`, fail if total `#[cube(launch_unchecked)]` under `crates/kernels/` exceeds 23. D-13 made that unsatisfiable (the D-10b dispatch macros call `.launch_unchecked()` per `(functional × output)` — ~1654 total). The rewrite implements **three per-design assertions**, with the audit logic as an exec'd Python block (the routing partition is most naturally consumed in Python):

- **Assertion 1 (routed one-per-output):** for each family, for each routed functional from `kernel_routing.cached_routed_funcnames(family)`, enumerate output modules from the `pub mod <output>;` lines in the subcrate's `src/lib.rs`; the entry file is `src/<output>.rs` (single-file) or `src/<output>/mod.rs` (nested-by-output, D-04); each entry file must have exactly one `#[cube(launch_unchecked)]` (excluding `//`-comment lines). `partNN` chunk files are not entry files.
- **Assertion 2 (unrouted-zero):** for each functional dir NOT in the routed set (`collect_func_dirs(family)` keys minus `cached_routed_funcnames(family)` — includes the D-11 deferred kernels and `mgga_x_br89_explicit`), count `#[cube(launch_unchecked)]` across all `.rs` files; must be 0.
- **Assertion 3 (math budget):** `#[cube(launch_unchecked)]` count in `crates/kernels/math/src/**/*.rs` must be ≤22, **explicitly excluding `crates/kernels/math/tests/`** (the Wave-0 spike `spike_tuple_return_cube.rs` is a test fixture, not a production launch wrapper).

**PASS summary against the current tree:** `A1: 1654 routed (functional, output) launch pairs · A2: 0 unrouted launchables · A3: math/src/ count = 22 (≤22)`. The script header cites D-13 and `.cargo/config.toml` as the build-env source of truth; it runs no cargo and sets no build-job/stack-size overrides.

## Dispatch-Tree Completion (Task 3)

- **Generator determinism:** re-ran `python3 tools/generate_gga_dispatch.py` (105 per-functional files + mod.rs), `python3 tools/generate_mgga_dispatch.py` (25 per-functional files + mod.rs), `python3 tools/generate_kernel_reexports.py` (43 LDA + 131 GGA + 92 MGGA re-exports). `git status --porcelain src/eval/gga_dispatch src/eval/mgga_dispatch src/kernel` was **empty** — the WIP `c3fba8089` output is byte-reproducible (P11-INV-6 / D-LOCK-D).
- **`audit_dispatch_tree.sh`:** exits 0. Post-collapse this is a *trivial* pass — the "referenced" batchN set is empty because no `batchN` references survive (`grep -rE 'batch[0-9]' src/eval/{gga,mgga}_dispatch` → 0). **This trivial pass proves only that no batchN references remain; it does NOT prove the new `crate::kernel::{family}::<func>` re-export paths resolve** — that proof is the separate path-resolution gate below.
- **D-13 launch gate:** `bash tools/audit_cube_launch.sh` (the Task-2 rewritten form) exits 0 as a load-bearing Wave-2 gate.
- **Path-resolution gate — RUNG 2 ran.** RUNG 1 (`cargo check -p libxc_rs`) would compile all 268 kernel subcrate dependencies — the known D-12 OOM risk on this 30 GB machine — so the gate went to RUNG 2: built the three smallest routed subcrates, one per family (`libxc-kernel-lda_c_lp96`, `libxc-kernel-gga_x_lb`, `libxc-kernel-mgga_xc_lp90` — all `cargo build -p` exit 0), then `rustc --edition 2024 --crate-type lib --extern ...` type-checked a throwaway file that replicates the `crate::kernel::{family}` re-export façade and references the deep dispatch path `kernel::{family}::<func>::<output>::<func>_<output>` for all three families. **`rustc` exit 0, zero errors** — the `src/kernel/{lda,gga,mgga}.rs` re-export paths resolve. The throwaway artifacts under `.cache/path-res-gate/` were removed after the gate ran. No pre-existing out-of-scope LSP errors surfaced in this rung (it only loaded the three spot-check subcrates, not the regen subcrates flagged below).

## Task Commits

1. **Task 1: Verify-only re-confirmation of Task 1's committed state** — no commit (verify-only by design; approved by user in prior session).
2. **Task 2: Rewrite `tools/audit_cube_launch.sh` to the D-13 per-design launch budget** — `eea58fed7` (feat)
3. **Task 3: Complete the dispatch-tree regeneration + run the path-resolution gate** — `f820fae90` (test, `--allow-empty`)

_Task 3 produced no file changes: the `c3fba8089` dispatch tree is byte-reproducible from the deterministic generators and already committed. The empty commit records the verification (generator determinism, audit passes, path-resolution gate) that `c3fba8089` never ran._

## Files Created/Modified

- `tools/audit_cube_launch.sh` — rewritten from the flat `BASELINE=23` count to the D-13 three-assertion per-design launch budget (167 insertions, 19 deletions).
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-03-SUMMARY.md` — this file.

## Decisions Made

- **Task 3 has no file changes.** The three dispatch/re-export generators are deterministic; re-running them reproduced the committed `c3fba8089` output exactly. Rather than skip the commit, recorded an `--allow-empty` commit so the verification milestone (generator determinism + both audits passing + path-resolution gate) is captured in git history as the marker that supersedes WIP `c3fba8089`.
- **Path-resolution gate at RUNG 2, not RUNG 1.** `cargo check -p libxc_rs` pulls 268 kernel-subcrate dependencies — the D-12 OOM risk. RUNG 2 (build 3 spot-check routed subcrates + `rustc --extern` the deep re-export paths) completes well within the RAM envelope and directly type-checks what matters: that `crate::kernel::{family}::<func>::<output>::<fn>` resolves.

## Deviations from Plan

None — plan executed as written. Tasks 2 and 3 followed the plan's `<action>` blocks; all acceptance criteria met.

## Carry-Forward Deviations from WIP `c3fba8089`

These were introduced/flagged by the prior session's WIP `c3fba8089` (an ancestor of HEAD) and are carried forward, not re-done:

1. **`deferred` registry relocation.** WIP `c3fba8089` relocated the `deferred` registry into `crates/kernels/math/src/deferred.rs` (with `lda` + `mgga` submodules) and rewired `src/model/{lda,mgga}_functional.rs` to `libxc_kernel_math::deferred::{lda,mgga}::is_deferred`. This is part of the committed WIP and is preserved.
2. **Pre-existing out-of-scope LSP compile errors.** `.continue-here.md` flags `xc_integrate` / `xc_E1_scaled` unresolved in some Task-1 regen subcrates, and a cubecl `from_raw_parts` arity mismatch in `crates/kernels/math` `#[cfg(test)]` code. These are **not caused by this session** and were not surfaced by the RUNG 2 path-resolution gate (which loaded only three clean spot-check subcrates). **Flagged for 11-04/11-05's per-subcrate gates.**

## Issues Encountered

- **Task 2 acceptance-criterion grep false positive on the comment line.** An acceptance criterion requires `grep -E 'CARGO_BUILD_JOBS|RUST_MIN_STACK|--jobs '` to return no matches. The initial header comment said "sets no `CARGO_BUILD_JOBS` / `RUST_MIN_STACK` and passes no `--jobs`" — which literally contained those tokens. Reworded the comment to "sets no build-job or stack-size environment overrides and passes no job-count flag" — no behavior change, criterion now satisfied.
- **`git commit --only -- <path>` argument order.** First commit attempt put `-m` after `--`, which `git` parsed as a pathspec. Per project memory, `--only -- <path>` is needed because sessions open with thousands of pre-staged files; the fix was `git commit -m "..." --only -- tools/audit_cube_launch.sh` (message before `--`).

## Next Phase Readiness

- **Wave 2 is finished under D-13.** Blocker B1 is closed: the dispatch tree is complete against per-functional subcrates with no `batchN` segment, the generators are deterministic, both `audit_cube_launch.sh` (D-13 form) and `audit_dispatch_tree.sh` pass, and a `rustc` path-resolution rung type-checked the re-export paths.
- **For 11-04 / 11-05:** the two carry-forward out-of-scope LSP errors (`xc_integrate` / `xc_E1_scaled` unresolved in some regen subcrates; cubecl `from_raw_parts` arity in `crates/kernels/math` `#[cfg(test)]`) must be closed by the per-subcrate build gates.
- **D-13 audit is now a reusable gate** — 11-04/11-05/11-06 should cite `tools/audit_cube_launch.sh` (per-design budget) wherever P11-INV-5 is referenced; the flat `≤23` form is gone.

## Self-Check: PASSED

- FOUND: `tools/audit_cube_launch.sh`
- FOUND: `.planning/phases/11-splitter-v2-unified-5k-cap/11-03-SUMMARY.md`
- FOUND commit `eea58fed7` (Task 2)
- FOUND commit `f820fae90` (Task 3)

---
*Phase: 11-splitter-v2-unified-5k-cap*
*Completed: 2026-05-15*
