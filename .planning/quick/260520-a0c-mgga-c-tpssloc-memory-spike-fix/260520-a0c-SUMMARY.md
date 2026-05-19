---
quick_id: 260520-a0c
slug: mgga-c-tpssloc-memory-spike-fix
date: 2026-05-20
status: partial          # Path A landed + tpssloc deferred; Path B follow-up planned
outcome: partial-mitigation+defer
duration: ~3h
commits:
  - 799bd5d94a   # feat(translate_v2): env-gate raised wrapper cap (LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER)
  - 491a87193d   # chore(workspace): exclude mgga_c_tpssloc from default-members
follow_up:
  - Path B (hierarchical sub-wrappers) — next session or new quick task
  - Plan 11.1-03 G4 workaround — skip mgga_c_tpssloc OR wait for Path B
---

# Quick task 260520-a0c — mgga_c_tpssloc memory spike fix

## Outcome: PARTIAL — landed Path A + deferred tpssloc; Path B planned as follow-up

The original target (clean single-crate `cargo build -p libxc-kernel-mgga_c_tpssloc` at jobs=1) was NOT achieved. The investigation produced:
1. Confirmed root cause: cubecl-macros 0.10 proc-macro memory accumulation during expansion of 7.9K-10K-line flat `#[cube] fn` bodies.
2. A partial translator-side mitigation (Path A) that reduces peak RSS by ~10 GB but is still insufficient.
3. A clean deferral of mgga_c_tpssloc from default-members so Phase 11.1 can proceed.
4. A documented Path B (hierarchical sub-wrappers) as the next viable fix shape.

## Symptom

User-reported: rustc OOM-killed during `#[cube]` proc-macro expansion of one of the 9 mgga_c_tpssloc lxc_pol parts (parts 19-23, 27-32; each 7.9K-10K lines, single `#[cube] fn` flat-emit). Confirmed under reproduction.

| Measurement | Baseline (HEAD) | Path A (post-fix) |
|---|---|---|
| Peak RSS observed | ~25.4 GB | ~16 GB |
| Time to OOM kill | ~4 min | ~1.5 min |
| Failure mode | SIGKILL by kernel oom_killer | SIGKILL by kernel oom_killer |
| 30 GB box safe? | No | No (closer but still over) |

## Investigation (Task 1: baseline reproduce)

`/usr/bin/time -v cargo build -p libxc-kernel-mgga_c_tpssloc`. Dependencies cached (cubecl-core, libxc-kernel-math), so only one rustc process runs — for tpssloc itself. ps sampling captured the timeline:

| Elapsed | rustc RSS | System mem used |
|---|---|---|
| 2:05 | 23.2 GB | 27 GB |
| 2:19 | 25.4 GB | 27 GB (1 GB free) |
| 3:39 | 12.3 GB | 14 GB (one expansion done, RSS dropped) |
| later | grew past safe ceiling | OOM-killed |

**Confirmed: NOT a stack overflow (RUST_MIN_STACK = 64 MB suffices).** The 25 GB RSS implies cubecl-macros 0.10 holds the entire fn body's syn AST + Scope/Context state during expansion; for a 10K-line straight-line `let`-binding body with deeply nested arithmetic RHS, this accumulates multi-GB.

## Diagnosis (Task 2: instrument CSE chunker)

Added env-gated diagnostic print at `tools/translate_v2/per_functional.py:_cse_chunk_part` return-None branch. Ran translator on mgga_c_tpssloc only (via `.planning/quick/260520-a0c-.../run_diag.py`). Result for the 9 problem parts:

```
CSE-REJECT mgga/mgga_c_tpssloc lxc_pol part21 (v4rho4_2):
  wrapper=9699L (cap=4500), n_chunks=3221, max_chunk=28L, top5=[28,26,26,26,26]
```

**Key finding:** The CSE chunker already produces ~3000 tiny chunks (26-29 lines each) — `MAX_TUPLE_ARITY=12` (cse.py:34) forces a cut roughly every 27 lines for dense 4th-derivative bodies. The resulting **wrapper** grows to 4500-9700 lines of `mod chunk{i};` + `use chunk{i}::...;` + `let t = chunk_N::<f64>(args);` plumbing. The split-threshold check rejects this wrapper as oversized → falls back to flat single-`#[cube] fn` → OOM.

| Part | n_chunks | wrapper L | max_chunk L |
|------|----------|-----------|-------------|
| 19   |     1497 |      4527 |          27 |
| 20   |     2712 |      8172 |          27 |
| 21   |     3221 |      9699 |          28 |
| 22   |     2721 |      8199 |          28 |
| 23   |     1527 |      4617 |          29 |
| 27   |     2372 |      7152 |          28 |
| 28   |     2041 |      6159 |          28 |
| 29   |     2357 |      7107 |          28 |
| 30   |     2341 |      7059 |          28 |
| 31   |     2041 |      6159 |          28 |
| 32   |     2369 |      7143 |          28 |

## Path A (landed): env-gated wrapper cap (commit 799bd5d94a)

Decoupled the wrapper line cap from the per-chunk cap in `_cse_chunk_part`. With `LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1`, the wrapper is allowed up to 15,000 lines (still bounded). Per-chunk cap stays at `split_threshold`. Default behavior unchanged so a casual full-tree regen doesn't mass-flip the D-LOCK-B set.

**Rationale:** the wrapper has thousands of cheap `let X = chunkN::<f64>(args)` statements; each statement is plain call+bind with no nested RHS arithmetic, so its macro expansion cost per statement is much lower than the flat-emit's 10K let-bindings with deeply nested arithmetic RHS.

**Result:** ~10 GB lower peak (25.4 GB → ~16 GB observed), but still OOM-killed. The accumulated state for 3000+ Local Stmt nodes still exceeds the box's safe memory budget.

**Why Path A wasn't enough:** the per-statement cost in cubecl-macros isn't dominated by RHS complexity — it's dominated by the per-Stmt overhead (Scope variable tracking, TokenStream allocation, IR-builder boilerplate). 3221 statements × per-statement overhead ≈ 15+ GB working set, still over the OOM threshold for the 30 GB box.

The translator change remains valuable: it's a needed building block for any future Path B/C/D approach. It is opt-in (env-gated), so it does not change the default behavior of regular regens.

## Path E (landed): defer mgga_c_tpssloc from default-members (commit 491a87193d)

`Cargo.toml` workspace `default-members` no longer includes `crates/kernels/mgga/mgga_c_tpssloc`. The crate stays a workspace member via the `[dependencies]` path-dep, so `cargo build -p libxc-kernel-mgga_c_tpssloc` remains available on demand.

`cargo metadata` confirms `default_members` dropped 260 → 259; tpssloc not in the set.

## Follow-up: Path B (hierarchical sub-wrappers)

**Plan:** group the leaf chunks into meta-chunk `#[cube] fn` wrappers (~12 leaves per meta), then have the top-level part wrapper call only the meta-chunks (~270 calls for a 3221-chunk part — a 12× reduction in top-level statement count). Each `#[cube] fn` at any level stays ≤500 lines, so per-expansion macro cost is bounded.

**Risk to validate up front:** the cross-meta live-set in dense 4th-derivative bodies. If a meta-chunk produces more than 12 outputs needed downstream, the 12-tuple ABI rejects it. Mitigation: write the meta-grouping to find cut points where the cross-meta live-set is ≤12 (similar to what the existing CSE chunker does at leaf level, but applied to meta boundaries).

**Estimate:** 2-4 hours of translator work + a compile cycle.

**Where to track:** propose either (a) a new quick task (260520-a0d or later), or (b) fold into Phase 11.1 Plan 03 as Task 0 before G4 oracle, since G4 needs tpssloc to compile.

## Artifacts produced

| File | Purpose |
|---|---|
| `260520-a0c-PLAN.md` | Original 3-task plan with Path A as Task 2 |
| `260520-a0c-baseline-stderr.log` | Task 1 reproduce log + ps timeline |
| `260520-a0c-cse-diag.log` | Task 2 diag output (CSE-REJECT lines for 9 problem parts) |
| `260520-a0c-cse-diag-after.log` | After raised cap — no CSE-REJECTs (confirms Path A regens cleanly) |
| `260520-a0c-postfix-stderr.log` | Path A compile attempt (still OOM-killed at ~16 GB) |
| `run_diag.py` | One-shot diag driver (emits to /tmp, doesn't touch crates/kernels) |
| `run_regen.py` | Regen driver (used during failed Path A test; not committed to git) |

## Phase 11.1 impact

- **Plan 11.1-03 G3 canary (mgga_c_b94):** unaffected — tpssloc not on the G3 path.
- **Plan 11.1-03 G4 (full-649 f32 oracle):** will need either Path B to land OR a documented "tpssloc skipped" entry in `f32_tolerance_overrides.toml` + the oracle harness's skip list.
- **Plan 11.1-04 SUMMARY rollup:** should note this deferral and the planned Path B follow-up.

## What this changes about the project

- Translator gained an opt-in path (`LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1`) for accepting CSE-chunked wrappers over the 4500-line cap. Future single-output-dense functionals (mgga_c_kcis/kcisk/revtpss/rmggac lxc_pol, lda_c_pk09 kxc_pol) get the same opt-in but are NOT regenerated by this task.
- Default `cargo build` skips mgga_c_tpssloc. Anyone needing it explicitly must `cargo build -p libxc-kernel-mgga_c_tpssloc`.
- `tools/kernel_size_exceptions.txt` is unchanged (the 9 tpssloc partN.rs entries still match HEAD since we reverted the failed regen). It will need updating when Path B lands and tpssloc is regenerated.
