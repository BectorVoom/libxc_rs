---
research_for: Phase 9 (Reduce kernel build time)
written: 2026-04-29
context: Phase 5 cumulative `cargo check -p libxc_rs` took 216m17s cold-cache; `cargo test --tests` estimated 15+ hr. Triggered build-time analysis grounded in `docs/manual/Cubecl/cubecl_macro_fanout_manual.md`.
status: ready_for_phase_9_planning
---

# Build-time reduction research

## Problem

Cold-cache `cargo check -p libxc_rs` took **216m17s** (verified Phase 5 closure run).
Cold-cache `cargo test -p libxc_rs-verify --tests` was estimated at **15+ hours** (10% in 2hr; killed). The 700+ kernel sub-crate cascade is the bottleneck.

## Evidence (collected 2026-04-29)

### Workspace shape

| Family | Sub-crates | Aggregate crate | Notes |
|---|---|---|---|
| MGGA | 108 | `crates/kernel-mgga/` | Path-deps listed explicitly in aggregate's Cargo.toml |
| GGA | 58 | `crates/kernel-gga/` | Same pattern |
| LDA | 1 (monolithic) | `crates/kernel-lda/` | 482 .rs files, 311,599 LOC |
| **Total** | **170 crates** | | **7,604,160 lines kernel code** |

Representative sub-crate `crates/kernel-mgga-1a/`: 16 .rs files, 49,973 LOC, 14 `#[cube(launch)]` functions, 0 plain `#[cube]` helpers.

### Macro fan-out signals (the smoking gun)

| Annotation | Count workspace-wide |
|---|---:|
| `#[cube(launch_unchecked)]` | **3,945** |
| Plain `#[cube]` helpers | 64 |
| `#[derive(CubeType)]` | 0 |
| `#[derive(CubeLaunch)]` | 0 |
| `#[cube]` on traits/impls | 1 |
| Per-numeric-type duplicates (`_f32`, `_f64`) | 0 |

Launch:plain ratio is **62:1**. The CubeCL fan-out manual's #1 anti-pattern — *"Every helper is launchable"* — describes libxc_rs precisely. Other fan-out signals are clean.

### Cargo profile state (before this commit cycle)

```toml
[profile.dev]
debug = 0
codegen-units = 16
incremental = false        # intentional: sccache can't cache incremental

[profile.test]
debug = 0
codegen-units = 16
incremental = false
```

No `[profile.*.build-override]`, no `[profile.dev.package."*"]`. `.cargo/config.toml` has `jobs=3` (RAM cap) and a commented-out `rustc-wrapper = "sccache"`.

### Where launches are invoked

Each kernel's `launch_unchecked::<CpuRuntime>` is called directly from the dispatch macros:
- `src/eval/gga_dispatch/mod.rs` — `ten_arm_dispatch_gga!` macro, 10 call sites (one per derivative × spin)
- `src/eval/mgga_dispatch/mod.rs` — analogous

So every translated kernel needs to keep its own launch wrapper unless the dispatch layer is restructured.

## Recommendations and status

| # | Action | Status (this commit cycle) | Future work |
|---|---|---|---|
| 1 | `[profile.dev.build-override] opt-level=3` for proc-macros | **DONE** in `Cargo.toml` (commit `b51abf7d`) | None — pure additive, immediate ~30-50% faster cold builds |
| 2 | Enable `incremental=true` in dev/test profiles | **NOT DONE** — incremental=false is intentional for sccache compatibility (documented in `.cargo/config.toml`) | Decide between sccache and incremental for the project's primary cache strategy |
| 3 | sccache or `CARGO_INCREMENTAL=1` env in CI | **EXISTS** as a documented opt-in in `.cargo/config.toml`; rustc-wrapper line is commented | CI pipeline can uncomment for cross-machine cache hits |
| 4 | `#[cube(launch)]` → `#[cube]` downgrade | **HALF-DONE**: tools surface added (`tools/translate_*.py --cube-style {launch,plain}` and `tools/generate_gga_dispatch.py --launch-mode {per-kernel,per-batch}`). The `plain` / `per-batch` paths exist but the per-batch wrapper emitter is intentionally unimplemented and errors out with detailed migration guidance. | **PHASE 9 P0**: implement per-batch wrapper emission — see "Phase 9 implementation plan for rec 4" below |
| 5 | Sub-crate consolidation (108 → ~10 MGGA) | **HALF-DONE**: tools surface added (`rebatch_mgga.py --target-max`, `resplit_gga.py --bin-limit`). Defaults unchanged (50K) to preserve Phase 8 P08 OOM mitigation. | **PHASE 9 P1**: benchmark single-crate compile RAM at larger bin sizes; choose new default with evidence; re-run splitters |
| 6 | `comptime!()` for fixed numeric constants | **N/A**: codebase already imports named constants (`M_PI` etc.) from `libxc_kernel_math::constants`, which are already compile-time. No inline-literal pattern that comptime!() addresses. | None — already efficient |
| 7 | `cargo --workspace --exclude` for partial iteration | Not implemented — would need a doc/scripts page, not tools change | Phase 9 nice-to-have |

## Phase 9 implementation plan for rec 4

To enable `--cube-style plain` + `--launch-mode per-batch`, the dispatch layer needs:

1. **Per-batch wrapper modules** — generated as `src/eval/gga_dispatch/batch{N}/launches/{level}_{spin}.rs`. One `#[cube(launch_unchecked)]` per (batch, derivative_level, spin_mode):
   - 5 levels (exc/vxc/fxc/kxc/lxc) × 2 spins × 58 batches = **580 wrappers** for GGA (vs ~1741 today; ~3× reduction)
   - Same shape for MGGA: 5 × 2 × 108 = **1080 wrappers** (vs ~1743 today; ~1.6× reduction)
   - Inside each wrapper: `match` on FunctionalId discriminator → call the underlying `#[cube]` kernel
   - The `match`-and-call pattern works because `#[cube]` helpers can be invoked from inside another `#[cube]` body without a launch surface

2. **Updated `ten_arm_dispatch_gga!` / MGGA equivalent** — swap each `kernel::launch_unchecked::<CpuRuntime>(...)` for the corresponding `batch{N}::launches::{level}_{spin}::launch_unchecked::<CpuRuntime>(...)`.

3. **Updated batch{N}.rs files** (existing per-batch dispatch helpers from `emit_launch_helper`) — they currently call individual kernels' launch_unchecked; they would change to call the new per-batch wrappers and pass the FunctionalId discriminator.

4. **Coordinated regeneration** — the kernels themselves must be re-translated with `--cube-style plain` in the SAME pass, otherwise per-kernel launches no longer exist and the build breaks. This is a single atomic GSD plan, not split across multiple plans.

5. **MGGA dispatch generator** — there is no `tools/generate_mgga_dispatch.py` today; MGGA dispatch lives partly hand-written in `src/eval/mgga_dispatch/mod.rs`. Phase 9 needs to either generate MGGA dispatch from a roster (matching the GGA pattern) or apply the per-batch wrapper change manually to MGGA. The GGA path is the model.

### Quantitative payoff estimate (rec 4 only)

- 3,945 launch wrappers → ~580 + 1080 = **1,660** per-batch wrappers (~58% reduction)
- Each launch wrapper expansion is ~10× the macro work of a plain `#[cube]` helper (per the manual)
- Net macro-work reduction estimated at **~50-70% of total proc-macro time**
- Combined with rec 1 (`build-override` already merged), expected cold `cargo check -p libxc_rs` from 216m → **40-80m** range. Test compile from 15+ hr → ~3-5 hr range.

These are estimates. Phase 9 P0 must run `cargo build --timings` before and after to measure.

## Decisions locked by this research

- **Default tools behavior unchanged** in this analysis pass. All new behavior is opt-in via CLI flag so existing pipelines, CI, and re-runs are unaffected. This avoids any half-migration where some kernels emit `launch_unchecked` and others don't.
- **The migration to `--cube-style plain` + `--launch-mode per-batch` is atomic per kernel family** — translators and dispatch generator must run together, then the entire kernel-{family} workspace must be regenerated.
- **Sub-crate consolidation (rec 5) is gated on benchmarking** — Phase 8 P08 chose 50K lines/crate specifically to dodge >20 GB RSS. Lifting to 100K+ requires evidence that current rustc/CubeCL behavior tolerates it without OOM.

## Companion artifacts

This research persists alongside the tools changes from commits:
- `b51abf7d` build: optimize proc-macro builds via `[profile.*.build-override]`
- `9a2d456f` build(tools): add `--cube-style {launch,plain}` flag to all 4 translators
- `4fc43c9a` build(tools): add `--launch-mode` flag to GGA dispatch generator
- `553f4e35` build(tools): make sub-crate splitters' bin sizes configurable
