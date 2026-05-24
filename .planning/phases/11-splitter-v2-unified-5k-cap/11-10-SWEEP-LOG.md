# 11-10 (G-3) f64 compile sweep — per-segment log

f64-ONLY per-`-p` compile sweep (`tools/batched_compile_sweep.py`), chunked per family to
distinct segment manifests, then merged. USER-RUN, `--jobs 1`. No `LIBXC_RS_F32`. No umbrella build.

## Segments (all VERDICT: ALL_OK)

| family | pkgs | verdict | pass-1 | pass-2 | fail | wall-clock | peak-of-peak RSS | manifest |
|--------|-----:|---------|-------:|-------:|-----:|-----------:|-----------------:|----------|
| LDA  | 43  | ALL_OK | 43  | 0 | 0 | 984.9 s (~16 m)  | 17,007.7 MB | .cache/sweep-lda-manifest.json |
| GGA  | 131 | ALL_OK | 131 | 0 | 0 | 5,615.0 s (~94 m) | 29,671.0 MB | .cache/sweep-gga-manifest.json |
| MGGA | 131 | ALL_OK | 131 | 0 | 0 | 15,212.9 s (~4.2 h) | 26,619.1 MB | .cache/sweep-mgga-manifest.json |
| **merged** | **305** | **ALL_OK** | — | — | **0** | ~6.1 h total | env. ≤ ~29.7 GB | .cache/batched-compile-sweep-manifest.json |

All at `--jobs 1` (pass-1 == jobs=1, so pass-2 retry never needed). `.cargo/config.toml` jobs=1 untouched.

## RSS envelope note (30 GB box)

- GGA peaked at **29.67 GB** even at jobs=1 — i.e. GGA has heavy functionals right at the ceiling, but they built (no sharding needed). The earlier jobs=3 attempts peaked ~29.6 GB AND failed; jobs=1 is the safe mode.
- MGGA peaked 26.6 GB AFTER the sharding intervention (below). Pre-sharding, `mgga_c_kcisk` alone hit 30.29 GB → SIGKILL.

## MGGA interventions (this plan)

MGGA did NOT pass clean on the first attempts — two non-codegen blockers, both resolved:

1. **Disk-full (run 1):** cargo-target grew to 82 GB → `No space left on device` on `mgga_c_r2scan`. Resolved by `cargo clean`.
2. **Part-count OOM (run 2):** `mgga_c_kcisk` rustc SIGKILL @ 30.29 GB (jobs=1). Top-level part count is the driver; ~68 parts is the edge on this box. **Sharded** `mgga_c_rmggac` (115 parts), `mgga_c_tpss` (106), `mgga_c_kcisk` (69) via `tools/split_per_functional_subcrate.py mgga <func> lxc_pol --budget 40` → 25 new `_pK` shard crates (kcisk_p0..5, tpss_p0..5, rmggac_p0..12), every shard ≤40 parts; facades dropped to 11-18 parts. Roster 280 → 305 (MGGA 106 → 131). Commit `9fb8c18ac2`. The 4 borderline unsharded functionals (`mgga_x_br89` 64, `mgga_c_kcis` 60, `mgga_c_r2scan` 58, `hyb_mgga_x_js18` 51) built fine at jobs=1 in run 3 — no sharding needed.

NO documented splitter-floor exceptions: every roster package compiles at f64. NO functional was re-emitted via the translator (no codegen defect surfaced); the only structural change was sharding for memory.

## NO f32 sweep

`LIBXC_RS_F32` was never set. The kernels are f64-concrete (2491 files `&Array<f64>`, 0 generic); an f32 sweep would be a false f64-vs-f64 pass (threat T-11-12-01, user-rejected 2026-05-23). f32 is a milestone-scale follow-up, not a Phase-11 gate.
