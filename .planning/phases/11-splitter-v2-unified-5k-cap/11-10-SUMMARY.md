---
phase: 11-splitter-v2-unified-5k-cap
plan: 10
subsystem: infra
tags: [g3, compile-sweep, per-p-entry-gate, f64, SPEC-11-R4, mgga-sharding, oom]

requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 15)
    provides: "the math test-module cubecl-0.10 launch-ABI migration — the sweep's cargo-test path no longer breaks on 0.9 drift"
provides:
  - "f64 per-`-p` compile sweep VERDICT ALL_OK across the FULL roster: merged .cache/batched-compile-sweep-manifest.json = 305 records (LDA 43 + GGA 131 + MGGA 131), zero result=fail, zero pass=-1, at f64 — SPEC-11-R4 / ROADMAP SC #4 evidence for 11-13 (G-5)"
  - "Per-`-p` compile codified as the ENTRY gate (memory project_phase11_structural_without_compile) — proven across every on-disk package, not a 50-subcrate sample"
  - "3 oversized MGGA functionals sharded to fit the 30 GB box (rmggac/tpss/kcisk → 25 _pK shards); roster 280 → 305"
affects: [11-13, mgga-parity, roster-count]

tech-stack:
  added: []
  patterns:
    - "Per-`-p` f64 compile sweep, chunked per family to distinct manifests (the tool clears --manifest on start), merged + validated against len(build_roster())"
    - "Oversized-functional sharding for memory: part-count (~68) is the OOM driver on a 30 GB box; split lxc_pol via split_per_functional_subcrate.py --budget 40"

key-files:
  created:
    - .cache/batched-compile-sweep-manifest.json
    - .cache/sweep-lda-manifest.json
    - .cache/sweep-gga-manifest.json
    - .cache/sweep-mgga-manifest.json
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-10-SWEEP-LOG.md
  modified:
    - Cargo.toml
    - crates/kernels/mgga/mgga_c_rmggac (+ _p0..p12)
    - crates/kernels/mgga/mgga_c_tpss (+ _p0..p5)
    - crates/kernels/mgga/mgga_c_kcisk (+ _p0..p5)

key-decisions:
  - "f64-ONLY (no f32 sweep): kernels are f64-concrete; an f32 sweep = false f64-vs-f64 pass (T-11-12-01, user-rejected). LIBXC_RS_F32 never set."
  - "Roster count is len(build_roster()), re-derived (305), NOT a hardcoded magic number — it grew 280 → 305 from sharding and the == gate self-adjusted."
  - "DEVIATION (memory blocker, not in plan): 3 functionals (rmggac 115/tpss 106/kcisk 69 parts) OOM the 30 GB box (kcisk SIGKILL @ 30.29 GB). Sharded each one's lxc_pol via split_per_functional_subcrate.py --budget 40 → 25 _pK shards. Cargo.toml optional+oracle-mgga gating done MANUALLY (pre-11-12 splitter+optional-izer don't compose with optional deps). Commit 9fb8c18ac2."
  - "DEVIATION (disk): cargo-target hit 82 GB → ENOSPC mid-MGGA; user cargo clean. Operational, not code."

patterns-established:
  - "The per-`-p` ENTRY gate caught what nothing else did: not codegen defects (zero), but a 30 GB single-crate memory ceiling that needed structural sharding — exactly the gate's value (memory project_phase11_structural_without_compile)"

requirements-completed: []

duration: ~6.1 h sweep wall-clock (USER-RUN, paced across segments) + sharding/triage
completed: 2026-05-25
---

# Phase 11 / Plan 10 (G-3): f64 full-roster compile sweep — Summary

**Every roster package compiles at f64: merged manifest = 305 records (LDA 43 + GGA 131 + MGGA 131), VERDICT ALL_OK, zero failures — the per-`-p` compile ENTRY gate is proven across the whole tree, after sharding 3 oversized MGGA functionals to fit the 30 GB box.**

## Result

| family | pkgs | verdict | wall-clock | peak RSS (jobs=1) |
|--------|-----:|---------|-----------:|------------------:|
| LDA  | 43  | ALL_OK | ~16 m  | 17.0 GB |
| GGA  | 131 | ALL_OK | ~94 m  | 29.7 GB (at the ceiling, built clean) |
| MGGA | 131 | ALL_OK | ~4.2 h | 26.6 GB (post-sharding) |
| **merged** | **305** | **ALL_OK** | ~6.1 h | — |

- Merged manifest: `.cache/batched-compile-sweep-manifest.json`, **305 records == len(build_roster())** (re-derived via `--dry-run`), **0 result=fail, 0 pass=-1**, all at f64.
- 39 shard records present (25 new rmggac/tpss/kcisk + 14 pre-existing tpssloc/revtpss `_pK`).
- f64-ONLY: `LIBXC_RS_F32` never set (see SWEEP-LOG + the f64-concrete decision).

## How MGGA got to clean (3 runs, 2 non-codegen blockers)

1. **Disk-full** — cargo-target → 82 GB → ENOSPC on `mgga_c_r2scan`; resolved by `cargo clean`.
2. **Part-count OOM** — `mgga_c_kcisk` rustc SIGKILL @ 30.29 GB (jobs=1). Top-level part count drives rustc RSS; ~68 parts is the edge on this 30 GB box. **Sharded** rmggac (115)/tpss (106)/kcisk (69) into 25 `_pK` shards (`--budget 40`, every shard ≤40 parts); facades dropped to 11-18 parts; Cargo.toml `optional` + `oracle-mgga` gating added manually (commit `9fb8c18ac2`). Roster 280 → 305.
3. **Clean re-sweep** — all 131 MGGA packages (incl 25 shards + 4 borderline-but-OK unsharded: br89/kcis/r2scan/js18) pass at jobs=1, peak 26.6 GB.

**No codegen defects surfaced** — zero functionals re-emitted through the translator; the only intervention was sharding for memory. No splitter-floor exceptions: every package compiles.

## Significance

This is the per-`-p` compile ENTRY gate (memory `project_phase11_structural_without_compile`) proven at full-tree scope — superseding 11.1's 50-subcrate sample. It found no codegen defects but exposed a 30 GB single-crate memory ceiling that structural sharding resolved. The 305-record manifest is the SPEC-11-R4 / ROADMAP SC #4 evidence 11-13 (G-5) consumes.

## Next

Wave 3 = **11-13** (G-5 closure): rewrite 11-06-SUMMARY PARTIAL→COMPLETE, fix ROADMAP SC #4 (count == len(build_roster()) = 305) / SC #5 / G4 wording (f32 = milestone follow-up), add the Phase 12 MGGA-f64-parity entry, delete obsolete tools, remove LIBXC_RS_BYPASS_DEFERRED, manual phase-close. 11-13 has a blocking human-verify checkpoint.
