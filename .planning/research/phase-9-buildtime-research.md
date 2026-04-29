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

### Quantitative payoff estimate (rec 4) — REVISED 2026-04-29

The 2026-04-29 design pass uncovered a constraint that was elided in the
initial estimate: **launch wrappers cannot be merged across kernels with
different scalar-arg signatures** because each `#[cube(launch_unchecked)]`
wrapper has a fixed signature derived from the underlying expansion function.

Roster-grounded reality:
- 43/106 GGA functionals are zero-scalar (uniform signature → mergeable per (batch, level, spin))
- 63/106 GGA functionals are parameterized
- **50 distinct scalar-tuple signatures** across the 63 parameterized functionals — meaning each parameterized kernel has a near-unique signature; very few merge-eligible groups beyond the zero-scalar set

Realistic launch-wrapper reduction (paired translator + dispatch migration):
- **GGA**: 1,741 → ~1,300-1,400 launches (~20-25% reduction). The zero-scalar collapse saves ~340 wrappers; the 50-distinct-signature parameterized set is largely unmergeable.
- **MGGA**: similar fraction expected (assuming similar zero-scalar/parameterized split — needs roster instrumentation to confirm)
- Combined: **20-30% launch-wrapper reduction**, not the 50-70% initial hand-wave

Furthermore, the dominant macro work is in **kernel-body expansion**, not the launch wrapper itself. The launch wrapper is generated boilerplate around the expansion function; saving it cuts proc-macro work by perhaps 5-15%, not 50%.

**Revised expected speedup from rec 4 alone (paired translator + dispatch migration):** roughly **5-15% of total proc-macro time**, in the context of an already-merged `[profile.*.build-override]` (rec 1) and consolidated sub-crates (rec 5).

### Revised Phase 9 priority order under relaxed-memory constraint

| Pri | Action | Already done? | Realistic speedup | Implementation cost |
|---|---|---|---|---|
| 1 | Lift `jobs = 3` cap in `.cargo/config.toml` | No (commented but not removed) | **3-5×** (parallelism unlocked) | 1-line change, ~5 min |
| 2 | Consolidate sub-crates (`--bin-limit 500K`) | Defaults raised in tools (commit `055dcf22`); re-run pending | **~2×** (cargo coordination drop) | Re-run splitters; benchmark RAM; ~1 hr |
| 3 | Per-batch launch wrapper consolidation (rec 4) | Tools surface added; emitter not implemented (this commit cycle preserved that) | **~10-15%** (revised down from earlier estimate) | 1-2 weeks: emitter + dispatch macro update + coordinated regeneration + MGGA dispatch generator |

**Recommendation reordering**: ship Pri 1 + Pri 2 first. They deliver the dominant speedup at trivial implementation cost. Pri 3's marginal payoff likely doesn't justify its 1-2 week cost until/unless Pri 1 + Pri 2 leave a measurable bottleneck specifically in proc-macro launch-wrapper expansion.

### Detailed Pri 3 implementation plan (when Phase 9 chooses to do it)

The wrapper-emitter must group functionals by **identical signature**, not just by batch:

1. **Enumerate signature groups** per (batch, derivative_level, spin):
   - Bucket functionals whose `(out_buf_count, scalar_arg_tuple)` match exactly
   - Each bucket of size ≥ 2 becomes a merge candidate; size 1 stays per-kernel

2. **Emit one wrapper per merge candidate** as `src/eval/gga_dispatch/batch{N}/launches_{level}_{spin}_sig{H}.rs`:

   ```rust
   #[cube(launch_unchecked)]
   pub fn batch{N}_{level}_{spin}_sig{H}(
       /* common args: rho, sigma, [out_bufs], [scalar_args] */,
       functional_id: u32,
   ) {
       // CubeCL supports comptime branching but runtime match is constrained.
       // Most likely shape: a runtime if-else chain, since CubeCL match
       // semantics don't currently express dispatch over multiple #[cube]
       // function calls cleanly.
       if functional_id == FN_A_ID { kernel_a(/* args */); }
       else if functional_id == FN_B_ID { kernel_b(/* args */); }
       /* ... */
   }
   ```

   **Gating concern**: confirm CubeCL's runtime-branching semantics actually compile cleanly when each branch calls a different `#[cube]` helper. If the IR specializes per branch, this may not save expansion work — needs a spike.

3. **Update `ten_arm_dispatch_gga!`** to call the appropriate per-signature wrapper based on the functional's signature group. The macro grows a layer of indirection but each launch_unchecked call is now per-signature-group rather than per-functional.

4. **Translator coordination**: kernels in merge-candidate groups translate with `--cube-style plain` (their own launch wrapper omitted; they're called as `#[cube]` helpers from the per-signature wrapper). Singletons keep `--cube-style launch`. The translator needs a `--functional-list` allow-list for the plain mode, OR the `cube-style` decision moves into a per-functional config file. Either is doable but adds complexity.

5. **MGGA dispatch generator**: presently MGGA dispatch is hand-wired in `src/eval/mgga_dispatch/mod.rs` (no Python generator). Phase 9 must either:
   - (a) Create `tools/generate_mgga_dispatch.py` mirroring the GGA pattern, then apply per-signature consolidation
   - (b) Apply per-signature consolidation directly to the hand-wired MGGA dispatch — feasible but doesn't compound across regenerations
   - (a) is the long-term right answer; (b) is faster for a one-shot

6. **Validation**: gate the migration on `cargo build --timings` before/after measurements showing the predicted ~10-15% improvement. If actual measurement shows < 5% improvement, abandon and revert.

## Decisions locked by this research

- **Default tools behavior unchanged for translators and dispatch generator** in this analysis pass. All new annotation behavior is opt-in via CLI flag so existing pipelines, CI, and re-runs are unaffected. This avoids any half-migration where some kernels emit `launch_unchecked` and others don't.
- **Splitter defaults raised** (commit `055dcf22`) from 50K → 500K lines per sub-crate, since memory is no longer the binding constraint. Memory-tight systems opt in via `--target-max 50000` / `--bin-limit 50000`.
- **The Pri 3 migration to `--cube-style plain` + `--launch-mode per-batch` is atomic per kernel family** — translators and dispatch generator must run together, then the entire kernel-{family} workspace must be regenerated.
- **Pri 3 implementation deferred**: realistic payoff (10-15% of proc-macro time) likely doesn't justify the 1-2 week implementation cost in isolation. Re-evaluate after Pri 1 + Pri 2 ship and measure remaining bottleneck.

## Companion artifacts

This research persists alongside the tools changes from commits:
- `b51abf7d` build: optimize proc-macro builds via `[profile.*.build-override]`
- `9a2d456f` build(tools): add `--cube-style {launch,plain}` flag to all 4 translators
- `4fc43c9a` build(tools): add `--launch-mode` flag to GGA dispatch generator
- `553f4e35` build(tools): make sub-crate splitters' bin sizes configurable
- `055dcf22` build(tools): raise splitter defaults to 500K (memory-permissive)
