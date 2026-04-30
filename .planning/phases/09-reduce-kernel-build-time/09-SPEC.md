# Phase 9: Reduce Kernel Build Time — Specification

**Created:** 2026-04-29
**Revised:** 2026-04-29 (during /gsd-discuss-phase 9 — scope narrowed; see Interview Log Round 4)
**Ambiguity score:** 0.16 (gate: ≤ 0.20)
**Requirements:** 3 locked (down from 6 after Round 4 — see Interview Log)

## Goal

All 25 previously-deferred GGA functionals compile through every derivative order (exc/vxc/fxc/kxc/lxc) under the default `cargo build`, which compiles the full LDA + GGA + MGGA kernel set unconditionally; no generated kernel `.rs` file exceeds 20,000 lines.

**Phase title note:** Phase is named "reduce-kernel-build-time" for historical reasons. Per Round 4, the build-time-reduction objective (default-build family scoping, family feature gates, ≤180s wall-clock target) was dropped: `gga` and `mgga` remain non-optional dependencies and the default `cargo build` continues to compile every kernel crate. The phase's residual deliverable is unblocking the 25 deferred GGA functionals at full derivative-order coverage, plus retaining forward-guard caps on file size and profile single-source-of-truth.

## Background

The libxc_rs workspace currently has ~170 kernel sub-crates (1× kernel-math, 1× kernel-lda, 1 facade + ~60 batch sub-crates for GGA, 1 facade + ~80 batch sub-crates for MGGA). Plans 09-01, 09-02, and 09-03 are complete:

- 09-01: Translators (`tools/translate_{lda_v2,gga,mgga}.py`) gained shared-preamble + incremental-delta annotations.
- 09-02: All 239 kernel functionals re-translated with the new annotations; LDA monolithic files split into per-(level, spin) subdirectories.
- 09-03 (committed `b5d4c742`, no SUMMARY.md written): 131 GGA functionals re-split into ~22 sub-crates via first-fit-decreasing bin packing; subsequent splits pushed the count to ~60 GGA sub-crates and ~80 MGGA sub-crates to combat CubeCL proc-macro OOM.

Verified done from RESEARCH.md and live config:

- sccache: `.cargo/config.toml` has `rustc-wrapper = "sccache"`, `incremental = false` is set in workspace `[profile.dev]`/`[profile.test]`/`[profile.release]`, no sub-crate has `[profile.*]`.

What remains (Plan 09-04 NOT executed):

- Root `Cargo.toml` has no `[features]` section; `libxc-kernel-gga` and `libxc-kernel-mgga` are unconditional dependencies. Every `cargo build` compiles all kernel families.
- `src/kernel/mod.rs` unconditionally re-exports both GGA and MGGA crates; making them `optional = true` will not compile until the re-exports are cfg-gated.
- 25 GGA functionals (largest: `gga_c_ft97` `lxc_pol.rs` = 37,787 lines) exceed the empirical CubeCL `#[cube(launch_unchecked)]` proc-macro OOM threshold of ~5,000 lines per function. They have not yet been compiled at full derivative-order coverage.

## Requirements

1. **All 25 deferred GGA functionals compile at full derivative-order coverage**: Every `gga_*` functional listed as deferred in RESEARCH.md compiles under the default `cargo build` through exc, vxc, fxc, kxc, and lxc, polarized and unpolarized.
   - Current: 25 functionals have at least one file (typically `lxc_pol.rs`, occasionally `kxc_pol.rs`) above the historical ~5K-line CubeCL proc-macro OOM threshold; they are deferred via commented `// pub mod` entries or absent from `lib.rs` entirely. Per Round 4 the cap is now ≤20,000 lines (Requirement 2), which the dev machine compiles without OOM, so the 25 functionals can be unblocked without further file-splitting.
   - Target: All 25 functionals expose every applicable derivative-order module (`exc_unpol`, `vxc_unpol`, `fxc_unpol`, `kxc_unpol`, `lxc_unpol`, plus `_pol` variants where the source supports them) as unconditional `pub mod` entries; no `#[cfg(feature = "order-kxc")]` or `#[cfg(feature = "order-lxc")]` attributes remain (none exist today — the cap was satisfied via translator `_partN` splits, not feature gates).
   - Acceptance: `cargo build` exits 0 without OOM, AND `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"order-kxc"\|cfg(feature *= *"order-lxc"' {} +` returns nothing, AND every functional named in RESEARCH.md §"Why the 25 GGA Functionals Are Deferred" has all expected order modules present (verified by inspection of each `mod.rs`).

2. **Per-file line-count cap (≤ 20,000 lines)**: No generated kernel `.rs` file exceeds the relaxed compile-budget threshold (raised from 5,000 → 10,000 → 20,000 on 2026-04-29 during /gsd-discuss-phase 9 after user confirmed sufficient RAM headroom on the dev machine; see Interview Log Round 3).
   - Current: 0 files exceed 20,000 lines. Largest file in the codebase is `crates/kernel-mgga-5/src/mgga_c_b94/kxc_pol.rs` at 16,703 lines (well below 20K). 16,138 / 14,127 / 13,913 follow.
   - Target: Every `.rs` file in `crates/kernel-lda*/src/`, `crates/kernel-gga*/src/`, and `crates/kernel-mgga*/src/` remains ≤ 20,000 lines. New translations (e.g., enabling the 25 deferred GGAs at full orders for Requirement 1) must respect the same cap; further splitting via translator `_partN`/`_partN_subM` is the mechanism if any new oversize file is produced.
   - Acceptance: `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} + | awk 'NF==2 && $2 != "total" && $1 > 20000 {n++} END {exit n}'` returns exit code 0 (no offending files; `total` row from multi-file `wc -l` excluded).

3. **No regression on profile single-source-of-truth**: All `[profile.*]` settings remain centralized in the workspace root.
   - Current: Already true per Plan 02 — root `Cargo.toml` carries `[profile.dev]`, `[profile.dev.build-override]`, `[profile.release]`, `[profile.release.build-override]`, `[profile.test]`, `[profile.test.build-override]`, and no sub-crate has any `[profile.*]` section.
   - Target: No new `[profile.*]` section is introduced into any sub-crate `Cargo.toml` while implementing Requirement 1 (the unblock work).
   - Acceptance: `grep -l '\[profile\.' crates/*/Cargo.toml` (executed at phase verification) returns no files.

### Removed Requirements (Round 4 — 2026-04-29)

The following were removed during /gsd-discuss-phase 9 after the user instructed "Do not make `mgga` optional" and "Drop the ≤180s default-build target and default-build is lda,gga,mgga":

- **Original Requirement 1 (Default-build family scope, ≤180s, BUILD-OPT-02)** — REMOVED. Default `cargo build` continues to compile LDA + GGA + MGGA unconditionally. No wall-clock target is enforced for the default build in this phase.
- **Original Requirement 2 (Family feature gates: gga, mgga, all-kernels, BUILD-OPT-03)** — REMOVED. No `[features]` section is added to the root `Cargo.toml`; `libxc-kernel-gga` and `libxc-kernel-mgga` remain non-optional dependencies.
- **Original Requirement 3 (Cfg-gated kernel re-exports, BUILD-OPT-03)** — REMOVED. `src/kernel/mod.rs` keeps unguarded `pub use libxc_kernel_gga as gga;` and `pub use libxc_kernel_mgga as mgga;`. The unguarded `use libxc_kernel_mgga::deferred::is_deferred` in `src/model/mgga_functional.rs:43` likewise stays unguarded.

Roadmap requirements `BUILD-OPT-01`/`BUILD-OPT-02`/`BUILD-OPT-03` are therefore **partially satisfied** by Phase 9 (only `BUILD-OPT-01` — sccache + incremental=false — was already done before this phase started). The remaining build-time-reduction work (family feature gates, ≤180s target) is now out of scope for Phase 9 and would need to be re-introduced in a future phase if/when desired.

## Boundaries

**In scope:**
- Compile all 25 deferred GGA functionals at full derivative-order coverage (exc, vxc, fxc, kxc, lxc, polarized + unpolarized) — under the default `cargo build` (no features)
- For each deferred functional: ensure the source files exist in the appropriate `crates/kernel-gga-*` sub-crate, all expected derivative-order modules are listed unconditionally in the functional's `mod.rs`, and the functional is exported from the relevant sub-crate `lib.rs` (no `// pub mod` comments, no `#[cfg(feature = "order-*")]` attributes)
- Re-translate any deferred functional whose source is missing, partial, or stale via `tools/translate_gga.py`, respecting the ≤20,000-line per-file cap (split via `_partN`/`_partN_subM` if a new file would exceed it)
- Verify oracle parity (1e-12 relative) for every newly-unblocked derivative-order module against libxc 7.0.0 via the `verify/` harness
- Verify the workspace builds end-to-end with `cargo build` (no features) and `cargo check` post-unblock
- Maintain sccache + `incremental = false` configuration intact (already correct per Plan 09-03)
- Maintain profile single-source-of-truth (already correct — no sub-crate carries `[profile.*]`)

**Out of scope:**
- Adding `[features]` to root `Cargo.toml` and making `libxc-kernel-gga` / `libxc-kernel-mgga` optional dependencies (removed in Round 4 — both stay non-optional)
- Cfg-gating any reference to `libxc_kernel_gga` or `libxc_kernel_mgga` in `src/` (removed in Round 4 — unguarded use stays)
- Reducing default `cargo build` wall-clock to ≤180s or any other wall-clock target (removed in Round 4 — no wall-clock target remains)
- Splitting files that are already ≤20,000 lines — the cap is a forward guard, not a remediation backlog (0 files violate today)
- Helper-function extraction or per-spin/per-block multi-launch decomposition — translator `_partN` splitting is the only file-size mechanism in scope
- GPU/CUDA/HIP/WGPU backend build-time optimization — CPU build is the bottleneck today; cubecl GPU backends are feature-gated and out of this phase's scope
- Profile-guided optimization (PGO) or LTO tuning — separate concern from kernel unblock
- Reducing or consolidating the existing ~170 sub-crate count — current split is treated as a fait accompli
- Removing or simplifying maple2c-translated computation — would break the 1e-12 oracle parity contract from PROJECT.md
- Re-translation of LDA or MGGA functionals — only GGA's 25 deferred set is the unblock target
- Resurrecting the deferred MGGA functionals (the 6 listed in `crates/kernel-mgga/src/deferred.rs`) — those are blocked on Brent-method root-finders, not on file-size, and remain explicitly out of scope

## Constraints

- **Per-file line cap**: ≤ 20,000 lines for every `.rs` file under `crates/kernel-{lda,gga,mgga}*/src/` (post-split). Raised from 5,000 → 10,000 → 20,000 on 2026-04-29 after user confirmed sufficient RAM headroom on the dev machine. Cap is retained as a forward guard for new translations rather than a backlog of splits to perform.
- **Floating-point operation order**: All translator splits must preserve the exact `let` binding sequence and output-write order from the maple2c-translated source. Required for 1e-12 oracle parity per CLAUDE.md.
- **No order-feature gates**: `order-kxc`, `order-lxc`, `all-orders` features must NOT exist on any GGA sub-crate post-phase. The 25 deferred functionals are unconditionally compiled at all orders.
- **sccache configuration immutable**: `.cargo/config.toml` and the workspace `[profile.*]` `incremental = false` settings remain unchanged.
- **No family feature gates**: Per Round 4, no `[features]` section adds `gga`/`mgga`/`all-kernels` to the root `Cargo.toml` in this phase. `libxc-kernel-gga` and `libxc-kernel-mgga` remain non-optional dependencies. The default `cargo build` continues to compile every kernel crate.

## Acceptance Criteria

- [ ] All 25 functionals named in RESEARCH.md §"Why the 25 GGA Functionals Are Deferred" have all expected derivative-order modules (`exc_unpol`, `vxc_unpol`, `fxc_unpol`, `kxc_unpol`, `lxc_unpol`, plus `_pol` variants where the source supports them) listed unconditionally in their `mod.rs`
- [ ] All 25 functionals are exported from their owning sub-crate's `lib.rs` (no commented-out `// pub mod` entries for them remain)
- [ ] `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"order-' {} +` returns nothing (no `order-kxc`/`order-lxc`/`all-orders` cfg attributes)
- [ ] `cargo build` (no features) exits 0 with no OOM (peak RSS < dev-machine total) — output redirected to `log/cargo-build-09-final.log` per project convention
- [ ] `cargo check` (no features) exits 0 — output redirected to `log/cargo-check-09-final.log`
- [ ] Oracle parity: every newly-unblocked `gga_*` (functional × derivative order × spin) passes the `verify/` harness at 1e-12 relative error against libxc 7.0.0
- [ ] `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} +` reports no file with line count > 20,000
- [ ] `grep -l '\[profile\.' crates/*/Cargo.toml` returns no files (profile single-source-of-truth preserved)
- [ ] `.cargo/config.toml` still has `rustc-wrapper = "sccache"` (unchanged)
- [ ] Root `Cargo.toml` does NOT introduce a `[features]` section with `gga`/`mgga`/`all-kernels` (Round 4 removal is honored)
- [ ] `libxc-kernel-gga` and `libxc-kernel-mgga` remain declared without `optional = true` in root `[dependencies]` (Round 4 removal is honored)

## Ambiguity Report

| Dimension          | Score | Min  | Status | Notes                                                        |
|--------------------|-------|------|--------|--------------------------------------------------------------|
| Goal Clarity       | 0.95  | 0.75 | ✓      | Concrete ≤ 3 min target locked in round 1                    |
| Boundary Clarity   | 0.78  | 0.70 | ✓      | 25 deferred GGAs explicitly in scope at full orders          |
| Constraint Clarity | 0.78  | 0.65 | ✓      | Per-file 5,000-line cap; ≤ 180 s default-build target locked |
| Acceptance Criteria| 0.78  | 0.70 | ✓      | 18 pass/fail criteria, all command-verifiable                |
| **Ambiguity**      | 0.16  | ≤0.20| ✓      |                                                              |

## Interview Log

| Round | Perspective | Question summary                                                | Decision locked                                                                          |
|-------|-------------|-----------------------------------------------------------------|------------------------------------------------------------------------------------------|
| 1     | Researcher  | Default-build wall-clock target?                                | ≤ 3 minutes (≤ 180s)                                                                     |
| 1     | Researcher  | Are the 25 deferred GGA functionals in scope?                   | Yes — all derivative orders, no order-feature gates                                      |
| 1     | Researcher  | What is the locked crate-split design?                          | Per-file line-count cap (mandate splitting where files exceed limit)                     |
| 2     | Boundary K. | Unit and value of the line-count cap?                           | Per-file ≤ 5,000 lines (matches CubeCL proc-macro OOM threshold from RESEARCH.md)        |
| 2     | Boundary K. | Spec-gate decision after constraint locked?                     | Write SPEC.md (gate passed at ambiguity 0.16)                                            |
| 3 (post-spec) | User       | Per-file cap during /gsd-discuss-phase 9?                       | Raised from ≤5,000 → ≤10,000 → ≤20,000 lines (2026-04-29) — dev machine has RAM headroom. Final cap of 20,000 means **0 files violate** today (largest existing file is 16,703 lines); Requirement 5 becomes a forward guard rather than a remediation backlog. |
| 4 (post-spec) | User       | Should `mgga` become optional? Should the ≤180s default-build target be kept?    | Both rejected (2026-04-29): "Do not make `mgga` optional" + "Drop the ≤180s default-build target and default-build is lda,gga,mgga." Original Requirements 1, 2, 3 (default-build family scope, family feature gates, cfg-gated re-exports) **removed entirely**. Roadmap requirements `BUILD-OPT-02` and `BUILD-OPT-03` are deferred to a future phase. Phase 9 narrows to: unblock 25 deferred GGAs at full orders + maintain forward-guard caps. Plan 09-04 (which assumed feature-gating) is now obsolete and must be replaced. |

---

*Phase: 09-reduce-kernel-build-time*
*Spec created: 2026-04-29*
*Spec revised: 2026-04-29 (Round 3: line-count cap raised to 20K; Round 4: feature-gating + ≤180s wall-clock requirements removed)*
*Next step: /gsd-discuss-phase 9 (current session) — capture implementation decisions for the narrowed scope: how to unblock the 25 deferred GGA functionals (re-translate vs uncomment-existing; sub-crate placement for new files); how to handle the now-obsolete Plan 09-04 (delete vs archive); oracle-parity verification methodology for newly-enabled derivative orders.*
