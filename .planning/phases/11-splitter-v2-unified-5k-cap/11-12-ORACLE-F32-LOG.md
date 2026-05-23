# 11-12 (G-2) — Family-chunked oracle: log

**Status (2026-05-23, RE-SCOPED):** G-2 = the memory-safe **family-chunked f64 oracle**, and its
infrastructure is BUILT + cfg-validated (D1 source-cfg ✓, D2 harness repair ✓, S1 launch.rs generic ✓).
The **f32 sweep (G4 / Task 2/3) is RE-DEFERRED as milestone-scale** — see the 2026-05-23 update below:
the kernels are f64-concrete by design (2491 files, 0 generic), so real f32 needs a translator
re-architecture + full kernel regen, not a dispatch change, and is in tension with the f64-only/1e-12
core value. REMAINING for G-2: run the per-family f64 oracle (heavy, user-run) to validate the harness
bodies, then write 11-12-SUMMARY.md.
**Sessions:** 2026-05-22 (Path A Cargo mechanism, cargo-tree-proven) + 2026-05-23 (D1/D2/S1 source,
user-run compile gates).
**Machine constraints honored:** `.cargo/config.toml` UNTOUCHED; the assistant ran only `cargo tree`
(no compile); all compiling checks were user-run.

---

## UPDATE 2026-05-23 — f32/G4 RE-DEFERRED (milestone-scale); G-2 re-scoped to the f64 oracle

While implementing the f32 path (user chose "implement f32 dispatch/launch now"), a foundational
blocker surfaced that invalidates the whole f32-as-a-dispatch-task premise:

**The kernels are f64-concrete by design — they are NOT generic over the float type.**
- `crates/kernels/.../src/*.rs` kernel fns are `pub fn k(rho: &Array<f64>, …, scalar: f64, …)` with
  hardcoded `pow_1_3::<f64>` / `piecewise::<f64>` helper calls.
- Repo-wide: **2491** kernel files use `&Array<f64>`; **0** use generic `&Array<F>`.
- This is intentional: CLAUDE.md mandates "Precision: **f64 only**"; the core value is "energy rel
  err ≤ 1e-12" (unreachable in f32's ~1e-7); the translator preserves "exact maple2c FP operation
  order" in f64. `LIBXC_RS_F32` / D-19a "f32 secondary" was an aspiration never realized at the kernel
  layer — its only reader (`parity_phase11.rs`) flips a tolerance but computes in f64 (placeholder).

The launch infers `F` from argument types, so an f64-concrete kernel CANNOT be launched as f32 (no f32
monomorphization exists). Real f32 would require: re-architect the translator (`tools/translate_v2`)
to emit float-generic kernels → regenerate all ~2491 kernel files → reconcile FP-operation-order →
then the dispatch f32 path. That is **milestone-scale, OOM-heavy, and in tension with the f64-only/
1e-12 design** — not part of 11-12.

**Disposition (user, 2026-05-23):** RE-DEFER f32/G4 as out-of-scope/milestone-scale. **G-2 is
re-scoped to the memory-safe family-chunked f64 oracle** (the genuinely valuable, on-design
deliverable). Keep S1 (`launch.rs` generic over `F: Pod`, commit 86cf732e09) as a harmless
backward-compatible foundation should generic kernels ever be pursued. The f32 wiring tasks below
(D3/D4/D5) are SUPERSEDED by this re-defer.

**Remaining for G-2 (f64):** run the per-family f64 oracle (heavy, user-run) — this also validates the
D2 harness-body repair (the Phase-05 oracle harnesses had not compiled since the D-10a restructure):
```
cargo test -p libxc_rs-verify --no-default-features -F oracle-lda  --test lda_oracle  -j1 -- --test-threads=1 --nocapture
cargo test -p libxc_rs-verify --no-default-features -F oracle-gga  --test gga_oracle  -j1 -- --test-threads=1 --nocapture
cargo test -p libxc_rs-verify --no-default-features -F oracle-mgga --test mgga_oracle -j1 -- --test-threads=1 --nocapture
```
Each pulls only its family's kernels (+ math + the fixed 6-witness dev-dep floor) — memory-safe,
paced per family.

### f64 oracle RESULTS (2026-05-23, user-run) — G-2 (f64) CLOSED

| Family | Result |
|--------|--------|
| LDA  | ✓ pass (compiled + ran, no failures) |
| GGA  | ✓ pass (compiled + ran, no failures) |
| MGGA | ✗ test_all_mgga_oracle_unpol: 6 of 12 routed exc functionals fail (pol test passed) |

D2 harness repair validated for all three families (they compile + run; one LDA owned-Vec drift fixed
in bf7c4b6eb3). MGGA exc failures (rel_err vs C oracle, **f64**): `mgga_x_th` 2.0e-1 · `mgga_x_2d_js17`
1.1e-2 · `mgga_c_cs` 9.2e-3 · `mgga_x_pkzb` 3.7e-3 · `mgga_x_pbe_gx` 1.5e-3 · `mgga_x_tm` 9.2e-4.

**ATTRIBUTION:** genuine pre-existing **MGGA f64 correctness gaps**, NOT a harness/f32/τ-clamp issue —
the G-1 τ-clamp IS applied (`mgga_dispatch/mod.rs:280-282`: result→`tau_clamped`→`tau_handle`→launch).
`mgga_x_th` (20%) is almost certainly a per-functional translation bug; the smaller ones may be
residual `work_mgga` regularization beyond the τ-clamp. The now-runnable family-chunked oracle is the
first thing to exercise these. **Routed to a dedicated MGGA-parity roadmap effort** (per-functional
translation debug + work-driver regularization) — out of G-2's "build the oracle path" scope. Updates
memory `project_translator_missing_workmgga_tau_clamp` (τ-clamp alone is insufficient).

**G-2 (f64) success criterion met:** the memory-safe family-chunked oracle runs to completion across
all families; residual MGGA failures are attributed, not silently passed. See 11-12-SUMMARY.md. For
11-13/G-5: correct ROADMAP SC-#5/G4 wording (f32 = milestone follow-up) + add the MGGA-parity gap.

The ROADMAP G4/SC-#5 wording ("full-649 f32 oracle") should be corrected to reflect that f32 is a
milestone-scale follow-up, not a Phase-11 gate (route via 11-13 / G-5 closure).

---

## Landed path: **Path A (feature-gate the umbrella by family)**

The plan offered Path A (feature-gate kernel deps by family) PREFERRED vs Path B (per-family verify
sub-crate bypassing the umbrella). **Path A landed**, and it is feasible because the umbrella's
per-functional kernel references are cleanly family-grouped:

- `src/kernel/lda.rs` (43 re-exports), `src/kernel/gga.rs` (131), `src/kernel/mgga.rs` (106) —
  generated by `tools/generate_kernel_reexports.py`.
- dispatch is family-split: `dispatch_lda` (`src/eval/dispatch.rs`), `dispatch_gga`
  (`src/eval/gga_dispatch/`), `dispatch_mgga` (`src/eval/mgga_dispatch/`).

### What changed this session (Cargo plumbing only — cheaply provable, no compile)

1. **`Cargo.toml`** — all 280 per-functional kernel path-deps under `crates/kernels/{lda,gga,mgga}/`
   marked `optional = true` (via `tools/make_kernel_deps_optional.py`). `libxc-kernel-math` stays
   non-optional. Added `[features]`:
   - `default = ["oracle-lda", "oracle-gga", "oracle-mgga"]` — bare `cargo build -p libxc_rs`
     activates every family, so the **default umbrella build is byte-identical to before** (preserves
     11-14's hard-won `cargo check -p libxc_rs --lib` EXIT 0 — by construction, since the cfg surface
     added later is all `#[cfg(feature = "oracle-<fam>")]` with every family in `default`).
   - `oracle-lda` (43 `dep:` entries) / `oracle-gga` (131) / `oracle-mgga` (106).
2. **`verify/Cargo.toml`** — `libxc_rs = { path = "..", default-features = false }` + verify
   `oracle-{lda,gga,mgga} = ["libxc_rs/oracle-<fam>"]` + `default = [all three]`. A single-family
   verify build forwards only that family's umbrella feature.

### Memory-safety PROOF (`cargo tree -e no-dev`, unique kernel crates — NO compilation)

| Build configuration                                  | lda-fam | gga-fam | mgga-fam | math |
|------------------------------------------------------|--------:|--------:|---------:|-----:|
| `-p libxc_rs` (default = all)                        |      43 |     131 |      106 |    1 |
| `-p libxc_rs --no-default-features -F oracle-lda`    |  **43** |       0 |        0 |    1 |
| `-p libxc_rs --no-default-features -F oracle-gga`    |       0 | **131** |        0 |    1 |
| `-p libxc_rs --no-default-features -F oracle-mgga`   |       0 |       0 |  **106** |    1 |
| `-p libxc_rs --no-default-features` (no family)      |       0 |       0 |        0 |    1 |
| `-p libxc_rs-verify --no-default-features -F oracle-mgga` (verify, no-dev) | 0 | 0 | **106** | 1 |

Each family feature resolves to **only that family's kernels + shared math** — the OOMing all-281
umbrella graph is gone for chunked builds, while `default` is unchanged. (mgga-fam = 106 = on-disk
packages incl. the 14 `mgga_c_tpssloc`/`mgga_c_revtpss` `_pK` shard crates + 2 facades, matching
`build_roster()` package count, NOT the logical-92.)

### Known floor: verify test-build dev-deps (documented, NON-OOM)

`verify/Cargo.toml` `[dev-dependencies]` lists ~16 individual kernel crates for
`verify/tests/parity_phase11.rs` (the `PHASE11_SMOKE` + `PHASE11_WORST_CASE` witnesses). Cargo
compiles ALL dev-deps for any `cargo test` build, and Cargo does not support feature-gating
dev-dependencies. So a `cargo test -p libxc_rs-verify --no-default-features -F oracle-mgga` build
pulls mgga(106) + a **fixed 6-crate cross-family witness floor** (3 LDA + 3 GGA from `PHASE11_SMOKE`):

```
verify oracle-mgga, INCLUDING dev-deps: lda-family=3  gga-family=3  mgga-family=106  math=1
```

This is a small constant set (all tiny kernels), NOT the 281-umbrella — it does NOT cause OOM
(threat T-11-12-02 is specifically the full-umbrella pull). Optional future tightening: convert the
witnesses to regular optional deps behind a `parity-witness` feature and gate `parity_phase11.rs`
behind it. Not required for G-2 memory-safety.

---

## DEFERRED follow-up (turnkey) — the heavy / uncompilable-this-session work

> Rationale for deferral: the umbrella **cannot be `cargo check`ed cheaply** (even a default check
> compiles all 280 kernels — the OOM/multi-hour build). So all umbrella SOURCE edits + the harness
> repairs are uncompilable this session and must be iterated against a real (heavy) build under
> `#![deny(warnings)]`. Doing them blind risks regressing 11-14's green default build. They are
> therefore bundled WITH the deferred per-family compile-gate, where they are validatable.

### D1. Umbrella source cfg-gating (so a single-family build actually COMPILES)

Make the source match the feature graph. Gate everything family-specific with
`#[cfg(feature = "oracle-<fam>")]`. Because every family is in `default`, the **default build stays
identical**; only `--no-default-features -F oracle-<fam>` exercises the gates.

Files / mechanism:
- **Generated** (modify the GENERATOR, then regen — durable across D-13 regen, do NOT hand-edit output):
  - `tools/generate_kernel_reexports.py` → emit `#[cfg(feature = "oracle-<fam>")]` per re-export
    (or gate the whole `src/kernel/<fam>.rs` body). Regen `src/kernel/{lda,gga,mgga}.rs`.
  - `tools/generate_gga_dispatch.py` / `tools/generate_mgga_dispatch.py` → gate the per-functional
    dispatch files + their `mod` lines in `*_dispatch/mod.rs` behind the family feature. Regen.
    (These generators are already at the cubecl-0.10 launch ABI per 11-14 — regen is launch-safe.)
- **Hand-written** (gate carefully; `#![deny(warnings)]` turns unused-import / unreachable-pattern
  into hard errors):
  - `src/kernel/mod.rs` — `pub mod {lda,gga,mgga};` → `#[cfg(feature = "oracle-<fam>")] pub mod ...;`
  - `src/eval/mod.rs` — `pub mod gga_dispatch; pub mod mgga_dispatch;` + the three
    `pub use ...::dispatch_<fam>;` → gate each behind its family feature.
  - `src/lib.rs:34` — `pub use eval::{dispatch_lda, dispatch_gga, dispatch_mgga};` → split into three
    `#[cfg(feature = "oracle-<fam>")] pub use ...;`
  - `src/functional/evaluate.rs` — the central router calls `dispatch_lda` (≈L43), `dispatch_gga`
    (≈L67), `dispatch_mgga` (≈L91). Gate each `use` (L15–17) + each match arm. **deny(warnings)
    trap:** a catch-all `_ => Err(UnsupportedFunctional)` is needed when a family is absent, but
    becomes an *unreachable pattern* (denied) when all families are present — guard it with
    `#[cfg(not(all(feature="oracle-lda", feature="oracle-gga", feature="oracle-mgga")))]`.
  - `src/model/{lda,mgga}_functional.rs` — `from_id` consults `is_deferred` (now in math, not gated)
    — verify these still compile family-isolated.
- **Cheap validation surface for D1:** `cargo check -p libxc_rs --lib --no-default-features`
  (NO family ⇒ zero kernels ⇒ math + umbrella core only ⇒ fast). This exercises every
  `#[cfg(not(...))]` catch-all path and catches gross cfg errors without compiling kernels.
  Then the real per-family entry gate: `cargo check -p libxc_rs --lib --no-default-features
  -F oracle-lda -j1` (LDA = smallest, 43 kernels — still multi-hour cold; pace it).
- **Empirical D1 surface (cheap-check run 2026-05-22, `/tmp/11-12-check.log`):** the no-family
  `cargo check -p libxc_rs --lib --no-default-features` fails with EXACTLY 304 errors, all of which
  are deactivated-kernel-crate references (266 × E0432 + 38 × E0433) — NO other error code, so the
  Cargo plumbing is sound and math + umbrella-core compile cleanly with zero kernels. The 304
  references D1 must `#[cfg]`-gate are confined to **4 files**: `src/kernel/gga.rs` (131),
  `src/kernel/mgga.rs` (92), `src/kernel/lda.rs` (43) — all generated by
  `generate_kernel_reexports.py` — plus `src/eval/dispatch.rs` (38, LDA dispatch kernel-module refs,
  hand-written). The GGA/MGGA `*_dispatch/funcs/` files produced ZERO errors (they reach kernels only
  via the `crate::kernel::{gga,mgga}` façade, so gating the 3 re-export files covers them). ⇒ D1 for
  the no-family config = gate those 4 files; the `evaluate.rs` router + `lib.rs:34` pub-use +
  `eval/mod.rs`/`kernel/mod.rs` mod-decls then need per-family gating for single-family
  (`-F oracle-<fam>`) configs to compile.

### D2. Repair the stale Phase-05 oracle harnesses (they do NOT compile today)

`verify/tests/{lda,gga,mgga}_oracle.rs` were last touched in `200131c1d2` (Phase 05), before D-10a
deleted the per-family façade crates.
- `lda_oracle.rs:33` `use libxc_kernel_lda::deferred::is_deferred;`
  → `use libxc_kernel_math::deferred::lda::is_deferred;`
- `mgga_oracle.rs:41` `use libxc_kernel_mgga::deferred::is_deferred as is_deferred_mgga;`
  → `use libxc_kernel_math::deferred::mgga::is_deferred as is_deferred_mgga;`
- `is_deferred` now lives in `crates/kernels/math/src/deferred.rs` (`pub mod lda` L77 / `pub mod mgga`
  L186). verify reaches math only TRANSITIVELY today, so add a direct dev-dep:
  `libxc-kernel-math = { path = "../crates/kernels/math" }` (no extra build cost — already pulled).
- Add a per-family gate at the top of each oracle file so single-family builds compile only their
  oracle: `#![cfg(feature = "oracle-<fam>")]`.

### D3. Wire the f32 env-gate + per-functional tolerance lookup into the oracle harnesses

`verify/tests/parity_phase11.rs` already has the pattern: `f32_tolerance_for(name)` (loads
`crates/kernels/math/tests/f32_tolerance_overrides.toml`; default 1e-6; Brent-class 1e-4; asserts the
**1e-3 HARD CEILING**) + `LIBXC_RS_F32=1` env gate. Lift these into a shared `libxc_rs_verify` lib
helper and call from `{lda,gga,mgga}_oracle.rs`: f64 stays default (1e-12/1e-10 tiers);
`LIBXC_RS_F32=1` opts into f32 with the per-functional tolerance bounded by 1e-3.

### D4. Run the f32 sweeps (Task 2) — paced, per family, jobs=1

For each family, FIRST re-confirm memory-safety, then run (tee to `.cache/11-12-oracle-f32-<fam>.log`):
```
cargo tree -e features -p libxc_rs-verify --no-default-features -F oracle-<fam> | grep libxc-kernel-<other-fam>   # MUST be empty
LIBXC_RS_F32=1 cargo test -p libxc_rs-verify --no-default-features -F oracle-<fam> --test <fam>_oracle -j1 -- --test-threads=1 --nocapture
```
Record per family: cargo-tree re-confirmation, functionals run, pass/fail, per-functional
`max_rel_err`, peak-RSS. Sum of functionals run across families MUST equal the routed total (no
silent family skips). With the G-1 von Weizsäcker τ-clamp present
(`src/eval/mgga_dispatch/prepare.rs::tau_von_weizsacker` — confirmed present this session), MGGA f32
failures reflect real codegen, not missing regularization.

### D5. Task 3 checkpoint — accept per-functional f32 tolerance overrides

Present Task-2 `max_rel_err` per failing functional; propose overrides = smallest power of 10
≥ measured error, **strictly < 1e-3**; any error ≥ 1e-3 = real f32 codegen defect, attribute (not
silently passed). On approval, write to `crates/kernels/math/tests/f32_tolerance_overrides.toml`,
re-run the affected family, record final disposition here. Only THEN create `11-12-SUMMARY.md` and
close G-2.

---

## Why no `11-12-SUMMARY.md` yet

A `*-SUMMARY.md` marks a plan complete (phase-plan-index `has_summary`). 11-12 is PARTIAL, so no
SUMMARY is written — the plan stays correctly incomplete for resumption. This log is the progress
artifact (cf. `11-06-SUMMARY-HALT.md`, `11-07-SWEEP-HALT-lda.md`).
