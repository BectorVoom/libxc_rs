# Phase 9: Reduce Kernel Build Time — Specification

**Created:** 2026-04-29
**Ambiguity score:** 0.16 (gate: ≤ 0.20)
**Requirements:** 6 locked

## Goal

Default `cargo build` (no features) compiles only LDA-family kernels in ≤ 3 minutes wall-clock; GGA and MGGA families are opt-in via `--features gga`, `--features mgga`, or `--features all-kernels`; all 25 previously-deferred GGA functionals compile through every derivative order (exc/vxc/fxc/kxc/lxc) under `--features gga`; and no generated kernel `.rs` file exceeds 5,000 lines.

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

1. **Default-build family scope (BUILD-OPT-02)**: `cargo build` with no features compiles only LDA-family kernels.
   - Current: `cargo build` resolves `libxc-kernel-gga` and `libxc-kernel-mgga` as unconditional dependencies; LDA+GGA alone takes 12m32s (Plan 09-02 measurement); full workspace OOMs on large MGGA crates.
   - Target: `cargo build` with no features completes in ≤ 180 seconds wall-clock (warm sccache, typical 6-job dev hardware) and does not compile any `libxc-kernel-gga*` or `libxc-kernel-mgga*` crate.
   - Acceptance: `time cargo build` reports real time ≤ 180s, AND `cargo build -v 2>&1 | grep -E 'Compiling libxc-kernel-(gga|mgga)'` returns 0 lines.

2. **Family feature gates (BUILD-OPT-03)**: Root `Cargo.toml` defines opt-in `gga`, `mgga`, and `all-kernels` features.
   - Current: No `[features]` section in root `Cargo.toml`; both GGA and MGGA deps are non-optional.
   - Target: Root `Cargo.toml` contains a `[features]` section with `default = []`, `gga = ["dep:libxc-kernel-gga"]`, `mgga = ["dep:libxc-kernel-mgga"]`, `all-kernels = ["gga", "mgga"]`. `libxc-kernel-gga` and `libxc-kernel-mgga` declared with `optional = true`. `libxc-kernel-lda` remains non-optional.
   - Acceptance: `cargo check`, `cargo check --features gga`, `cargo check --features mgga`, and `cargo check --features all-kernels` all exit 0; `cargo metadata --no-deps --format-version 1 | jq '.packages[] | select(.name=="libxc_rs") | .features | keys'` includes `gga`, `mgga`, `all-kernels`.

3. **Cfg-gated kernel re-exports (BUILD-OPT-03)**: Source code does not reference `libxc-kernel-gga` or `libxc-kernel-mgga` without a feature guard.
   - Current: `src/kernel/mod.rs:2-3` has unguarded `pub use libxc_kernel_gga as gga;` and `pub use libxc_kernel_mgga as mgga;`.
   - Target: Every reference to `libxc_kernel_gga`, `libxc_kernel_mgga`, `gga::*`, or `mgga::*` in `src/` lives behind a corresponding `#[cfg(feature = "gga")]` or `#[cfg(feature = "mgga")]` attribute (or inside an already-gated module).
   - Acceptance: `cargo check` (no features) succeeds; `grep -rn 'libxc_kernel_gga\|libxc_kernel_mgga' src/ | grep -v 'cfg(feature'` returns 0 unguarded use/pub-use lines.

4. **All 25 deferred GGA functionals compile at full derivative-order coverage**: Every `gga_*` functional listed as deferred in RESEARCH.md compiles under `cargo build --features gga` through exc, vxc, fxc, kxc, and lxc, polarized and unpolarized.
   - Current: 25 functionals have at least one file (typically `lxc_pol.rs`, occasionally `kxc_pol.rs`) above the ~5K-line CubeCL proc-macro OOM threshold; they are deferred via commented `// pub mod` entries or absent from `lib.rs` entirely.
   - Target: All 25 functionals expose every applicable derivative-order module (`exc_unpol`, `vxc_unpol`, `fxc_unpol`, `kxc_unpol`, `lxc_unpol`, plus `_pol` variants where the source supports them) as unconditional `pub mod` entries; no `#[cfg(feature = "order-kxc")]` or `#[cfg(feature = "order-lxc")]` attributes remain.
   - Acceptance: `cargo build --features gga` exits 0 without OOM, AND `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"order-kxc"\|cfg(feature *= *"order-lxc"' {} +` returns nothing, AND every functional named in RESEARCH.md §"Why the 25 GGA Functionals Are Deferred" has all expected order modules present (verified by inspection of each `mod.rs`).

5. **Per-file line-count cap (≤ 5,000 lines)**: No generated kernel `.rs` file exceeds the empirical CubeCL proc-macro OOM threshold.
   - Current: Largest file is `gga_c_ft97/lxc_pol.rs` at 37,787 lines; multiple files (lxc_pol of `gga_x_wpbeh`, `gga_c_pbe_erf_gws`, `gga_c_optc`, `gga_c_q2d`, etc.) exceed 5,000 lines; the empirical OOM threshold per `#[cube(launch_unchecked)]` is ~5,000–5,500 lines.
   - Target: Every `.rs` file in `crates/kernel-lda*/src/`, `crates/kernel-gga*/src/`, and `crates/kernel-mgga*/src/` is ≤ 5,000 lines. Files exceeding this must be split (e.g., by helper-function extraction with computation-identical output preserving 1e-12 oracle parity).
   - Acceptance: `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} + | awk 'NF==2 && $1 > 5000 {n++} END {exit n}'` returns exit code 0 (no offending files).

6. **No regression on profile single-source-of-truth (BUILD-OPT-03)**: All `[profile.*]` settings remain centralized in the workspace root.
   - Current: Already true per Plan 02 — root `Cargo.toml` carries `[profile.dev]`, `[profile.dev.build-override]`, `[profile.release]`, `[profile.release.build-override]`, `[profile.test]`, `[profile.test.build-override]`, and no sub-crate has any `[profile.*]` section.
   - Target: No new `[profile.*]` section is introduced into any sub-crate `Cargo.toml` while implementing requirements 1–5.
   - Acceptance: `grep -l '\[profile\.' crates/*/Cargo.toml` (executed at phase verification) returns no files.

## Boundaries

**In scope:**
- Add `[features]` block to root `Cargo.toml` (default/gga/mgga/all-kernels)
- Make `libxc-kernel-gga` and `libxc-kernel-mgga` optional dependencies in root `Cargo.toml`
- Add `#[cfg(feature = "gga")]` and `#[cfg(feature = "mgga")]` guards to every reference of those crates inside `src/`
- Compile all 25 deferred GGA functionals at full derivative-order coverage (exc, vxc, fxc, kxc, lxc) — no `order-*` feature gates
- Split any generated kernel `.rs` file currently exceeding 5,000 lines (by helper extraction or per-spin/per-block decomposition that preserves floating-point operation order)
- Verify default `cargo build` ≤ 3 min wall-clock target on the dev machine
- Maintain sccache + `incremental = false` configuration intact

**Out of scope:**
- Full helper-function extraction across all 239 kernels — already too large; only files exceeding the 5,000-line cap are touched (the rest keep section-comment annotations from Plan 09-01)
- GPU/CUDA/HIP/WGPU backend build-time optimization — CPU build is the bottleneck today; cubecl GPU backends are feature-gated and out of this phase's scope
- Profile-guided optimization (PGO) or LTO tuning — separate concern from feature gating and OOM mitigation
- Reducing or consolidating the existing ~170 sub-crate count — current split is treated as a fait accompli; only further splits required to meet the 5,000-line cap are added
- Removing or simplifying maple2c-translated computation — would break the 1e-12 oracle parity contract from PROJECT.md
- Cross-platform CI build-time tuning — target is the local dev machine spec; CI tuning is a future infrastructure phase
- Re-translation of LDA or MGGA functionals — only GGA's 25 deferred set is the unblock target

## Constraints

- **Per-file line cap**: ≤ 5,000 lines for every `.rs` file under `crates/kernel-{lda,gga,mgga}*/src/` (post-split). This is the empirical CubeCL `#[cube(launch_unchecked)]` proc-macro OOM threshold from RESEARCH.md.
- **Default build wall-clock**: ≤ 180 seconds for `cargo build` with no features, warm sccache, on the dev machine (6 build jobs).
- **Floating-point operation order**: All splits must preserve the exact `let` binding sequence and output-write order from the maple2c-translated source. Required for 1e-12 oracle parity per CLAUDE.md.
- **No order-feature gates**: `order-kxc`, `order-lxc`, `all-orders` features must NOT exist on any GGA sub-crate post-phase. The 25 deferred functionals are unconditionally compiled at all orders.
- **sccache configuration immutable**: `.cargo/config.toml` and the workspace `[profile.*]` `incremental = false` settings remain unchanged.

## Acceptance Criteria

- [ ] Root `Cargo.toml` contains `[features]` with `default = []`, `gga = ["dep:libxc-kernel-gga"]`, `mgga = ["dep:libxc-kernel-mgga"]`, `all-kernels = ["gga", "mgga"]`
- [ ] `libxc-kernel-gga` declared with `optional = true` in root `[dependencies]`
- [ ] `libxc-kernel-mgga` declared with `optional = true` in root `[dependencies]`
- [ ] `libxc-kernel-lda` declared without `optional = true` (always compiled)
- [ ] `src/kernel/mod.rs` has `#[cfg(feature = "gga")]` immediately before the `gga` re-export
- [ ] `src/kernel/mod.rs` has `#[cfg(feature = "mgga")]` immediately before the `mgga` re-export
- [ ] `grep -rn 'libxc_kernel_gga\|libxc_kernel_mgga' src/ | grep -v 'cfg(feature'` returns 0 unguarded references
- [ ] `cargo check` (no features) exits 0 and does not compile any `libxc-kernel-(gga|mgga)*` crate
- [ ] `cargo check --features gga` exits 0
- [ ] `cargo check --features mgga` exits 0
- [ ] `cargo check --features all-kernels` exits 0
- [ ] `time cargo build` (warm sccache, no features) reports real time ≤ 180s
- [ ] `cargo build --features gga` exits 0 with no OOM (peak RSS < 16 GB per rustc invocation)
- [ ] All 25 functionals named in RESEARCH.md §"Why the 25 GGA Functionals Are Deferred" have all expected derivative-order modules listed unconditionally in their `mod.rs`
- [ ] `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"order-' {} +` returns nothing
- [ ] `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} +` reports no file with line count > 5,000
- [ ] `grep -l '\[profile\.' crates/*/Cargo.toml` returns no files
- [ ] `.cargo/config.toml` still has `rustc-wrapper = "sccache"` (unchanged)

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

---

*Phase: 09-reduce-kernel-build-time*
*Spec created: 2026-04-29*
*Next step: /gsd-discuss-phase 9 — implementation decisions (which file-splitting strategy for the 25 deferred functionals; how to detect unguarded `gga::`/`mgga::` references; whether Plan 09-04 needs revision to cover requirements 4 and 5)*
