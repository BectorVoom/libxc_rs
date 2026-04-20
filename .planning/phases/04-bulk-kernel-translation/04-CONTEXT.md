# Phase 4: Bulk Kernel Translation - Context (REFRESHED)

**Gathered:** 2026-04-10
**Refreshed:** 2026-04-20
**Status:** Ready for planning (plans 04-02..04-05 archived as stale — see `_stale-pre-replan-2026-04-20/`)

<refresh_note>
## Why This Context Was Refreshed

The original Phase 4 CONTEXT (2026-04-10) assumed kernels would live in `src/kernel/{lda,gga,mgga}/` with a one-file-per-functional hand-translation workflow (D-01 through D-12).

Between plan 04-01 and the rest of Phase 4, the team changed course and executed Phases 8 and 9 ahead of the remaining Phase 4 plans. That work invalidated most of Phase 4's original task breakdown:

- **Phase 8** extracted LDA/GGA/MGGA kernels into independent workspace sub-crates (`crates/kernel-lda`, `crates/kernel-gga` + 22 GGA sub-crates, `crates/kernel-mgga` + 37 MGGA sub-crates), with `libxc_kernel_math` shared.
- **Phase 8** built `tools/translate_mgga.py` from scratch and batch-translated all 92 MGGA functionals; 6 are **deferred** (br89/mbrxc root-finders missing).
- **Phase 9** ported incremental-delta code generation to LDA and GGA translators (`tools/translate_lda_v2.py`, `tools/translate_gga.py`), batch re-translated all 239 kernels into per-(level, spin) subdirectories, fixed `crate::math` → `libxc_kernel_math` import paths, and added `xc_integrate` translation. 4 LDA functionals are **deferred** (kxc_pol/lxc_pol > 10K lines bust CubeCL proc-macro stack).
- **Phase 9** feature-gated compilation so default `cargo build` compiles only LDA, with `--features gga` / `--features all-kernels` for larger sets; consolidated `[profile.*]` to the workspace root.
- Oracle tests `verify/tests/{lda,gga,mgga}_oracle.rs` exist but only run the **C oracle** across every functional as a smoke check. Comment in each file explicitly says: *"Per-functional Rust-vs-oracle comparison tests will be activated as kernels are translated."* No Rust kernel output is actually compared to C oracle output at required tolerance tiers yet.
- `src/eval/dispatch.rs` still hard-dispatches only `lda_x`. `dispatch_gga` and `dispatch_mgga` do **not** exist. The `use crate::kernel::lda::lda_x::{...}` import now resolves through the `pub use libxc_kernel_lda as lda;` re-export in `src/kernel/mod.rs`.

So Phase 4's remaining scope is **not** "translate kernels" — that's done. It is:

1. Generalize the dispatch layer to route all translated functionals (LDA/GGA/MGGA) to the correct CubeCL launch wrapper.
2. Activate per-functional **Rust-vs-C-oracle numerical comparison** in each family's oracle test, at the tolerance tiers required by VERIFY-03..07, across all applicable derivative orders and both spin modes.
3. Document and exclude the 4 deferred LDA + 6 deferred MGGA functionals with machine-readable tracking (the MGGA side already has `crates/kernel-mgga/src/deferred.rs`; LDA needs the equivalent).
4. Deliver a green cross-family verification sweep that closes requirements KERN-03..09 and VERIFY-02..07.

This refresh locks those decisions. Plan 04-01 (kernel-math infrastructure + oracle harness scaffolding) stays as-is and is already committed.
</refresh_note>

<domain>
## Phase Boundary

Phase 4 now delivers **dispatch coverage + numerical oracle verification** for every translated kernel in `crates/kernel-lda`, `crates/kernel-gga*`, and `crates/kernel-mgga*`. Kernel translation itself is **complete** (via phases 8/9) and is not re-done here. Deferred functionals (4 LDA + 6 MGGA) are scope-excluded with a machine-readable tracking artifact in each family.

**In scope:**
- `dispatch_lda` generalization to all 37 compiled LDA functionals (incl. `lda_xc_tih` _vxc).
- New `dispatch_gga` routing all 106 compiled GGA functionals (incl. `gga_x_lb` _vxc).
- New `dispatch_mgga` routing all 86 compiled MGGA functionals (incl. `mgga_x_2d_prp10` and `mgga_x_tb09` _vxc).
- Per-functional Rust-vs-oracle tests in `verify/tests/{lda,gga,mgga}_oracle.rs` at tolerance tiers exc ≤ 1e-12, vxc ≤ 1e-10, fxc ≤ 1e-8, kxc ≤ 1e-6, lxc ≤ 1e-4, across all applicable derivative orders and both spin modes.
- Deferred-functional tracking (`crates/kernel-lda/src/deferred.rs` mirroring MGGA pattern) and test-suite skipping for deferred IDs.
- Verification sweep closing KERN-03..09 and VERIFY-02..07.

**Out of scope:**
- Kernel source generation / retranslation. Already done. If a kernel is wrong, open a Phase 4 task to fix *that* kernel — do not re-translate the batch.
- Enabling the 10 deferred functionals (tracked but left disabled until root-finders / split strategies land in a later phase).
- Feature-gate additions or build-time tuning (Phase 9's remit).
- Any GPU backend other than `cubecl/cpu` unless KERN-09 explicitly requires it.

</domain>

<decisions>
## Implementation Decisions

### Kernel Source (Locked by phases 8/9)
- **D-01-R:** Kernel Rust source lives in workspace sub-crates under `crates/kernel-{lda,gga,mgga}*`. Every functional is already in its own subdirectory split per (derivative-level, spin-mode). The top-level `libxc_kernel_{lda,gga,mgga}` crates are facades that re-export sub-crate batches.
- **D-02-R:** The four special `_vxc` files are already translated alongside their families (`lda_xc_tih` in `crates/kernel-lda`, `gga_x_lb` in a `kernel-gga-*` batch, `mgga_x_2d_prp10` and `mgga_x_tb09` in `kernel-mgga-*` batches). Phase 4 only wires their dispatch entries — no translation work.
- **D-03-R:** Deferred lists are authoritative: **4 LDA deferred** (`lda_c_pk09`, `lda_xc_ksdt`, `lda_c_pw_erf`, `lda_c_pmgb06`); **6 MGGA deferred** (`mgga_c_b94`, `mgga_x_br89`, `mgga_x_mbr`, `mgga_x_mbrxc_bg`, `mgga_x_mbrxh_bg`, `mgga_x_mggac`) — these are the authoritative names taken directly from `crates/kernel-mgga/src/deferred.rs` (W7 fix, 2026-04-20: earlier drafts of this line listed `mgga_c_scan_vv10 subset`, `mgga_x_mbrxc`, `mgga_c_lp90` which do NOT match the actual file; always treat `crates/kernel-mgga/src/deferred.rs` as source of truth).

### Dispatch Layer (Phase 4's real work)
- **D-04-R:** Generalize `src/eval/dispatch.rs::dispatch_lda` from its current `lda_x`-only shape into an ID-based two-level match: outer match on functional ID (or name-derived enum) selects the launch wrapper set, inner match on `(DerivativeOrder, Spin)` picks the concrete `#[cube]` variant. Preserve the current zero-then-accumulate contract and the BUILD-04 "no raw launch calls outside kernel::launch" invariant.
- **D-05-R:** Add `dispatch_gga(input: &GgaInput, order, output: &mut GgaOutput, alpha, thresholds)` mirroring the LDA shape, but with `sigma` input and GGA output fields. Follow the existing `src/input/GgaInput` and `src/output/GgaOutput` contracts.
- **D-06-R:** Add `dispatch_mgga(input: &MggaInput, order, output: &mut MggaOutput, alpha, thresholds)` with `sigma`, `lapl`, `tau` inputs and conditional `vlapl`/`vtau` output handling per-functional.
- **D-07-R:** Each dispatch function must return `Err(LibxcRsError::Unsupported { … })` (or the existing error variant) for deferred functional IDs, matching the machine-readable deferred list — do not panic, do not silently produce zeros.
- **D-08-R:** Dispatch tables are code-generated **optional**: if hand-maintained match arms become unwieldy (likely for 37/106/86-way matches), the planner may introduce a `build.rs` or xtask that emits the match from a small manifest. Either approach is acceptable; commit-by-commit readability wins over cleverness.

### Verification Strategy (Phase 4's proof)
- **D-09-R:** Replace the smoke-check bodies in `verify/tests/{lda,gga,mgga}_oracle.rs` with per-functional Rust-vs-C-oracle comparison. For each functional × applicable order × both spins: run Rust kernel via `dispatch_*`, run C oracle via the `oracle_{lda,gga,mgga}_all` helper, compare corresponding output arrays.
- **D-10-R:** Tolerance tiers (VERIFY-03..07, D-10 from original context): `exc (zk)` ≤ 1e-12; `vxc` family ≤ 1e-10; `fxc` family ≤ 1e-8; `kxc` family ≤ 1e-6; `lxc` family ≤ 1e-4. Each tier tested independently (one test per order so failures pinpoint the offending derivative level).
- **D-11-R:** Test inputs: the four reference systems from `verify/` fixtures (H, Li, BrOH neutral, BrOH cation) already gated by `FLAGS_HAVE_EXC` and the derivative flags. Reuse, don't recreate.
- **D-12-R:** Deferred functionals are **skipped with a test-level `#[ignore]` + reason** (or a filter in the test harness) keyed on the deferred list — CI must not flag them as failures, but they must remain visible as "skipped 10" in test output so coverage drift is detectable.
- **D-13-R:** Cross-family verification sweep (final plan) runs the full matrix single-shot (`cargo test -p libxc_rs-verify --features all-kernels` or equivalent) and records pass/skip counts in a summary block for Phase 4 sign-off.

### Ordering and Granularity
- **D-14-R:** Family order: LDA → GGA → MGGA. Each family ships as one plan: dispatch wiring + per-functional oracle activation + family-level green test run. Commit granularity is per-functional inside each family plan (preferred for bisect), but batch commits are acceptable if the functional count in a single dispatch arm change demands it.
- **D-15-R:** Large-kernel CubeCL compilation risk (original D-05/D-06) is **closed** by phases 8/9 — 86/92 MGGA functionals compile, including previously-feared 100K-line cases via per-(level, spin) splitting. No canary step needed.
- **D-16-R:** Plan count estimate: **3 new plans** (LDA dispatch+verify, GGA dispatch+verify, MGGA dispatch+verify) plus an optional **cross-family sweep plan**. Planner may merge LDA's plan into 04-01's follow-up if ergonomic; total plan count for Phase 4 should not exceed 5 (including 04-01 already done).

### Claude's Discretion
- Whether dispatch tables are hand-written, macro-generated, or xtask-generated from a manifest.
- Whether Rust-vs-oracle comparison uses `approx::assert_relative_eq!`, a custom per-field helper, or a shared `verify/src/cmp.rs` utility.
- Whether deferred functionals use `#[ignore = "..."]`, a runtime skip filter, or cfg-gating.
- Parallelism of the oracle test suite (serial for determinism vs `cargo test` default).
- Exact Cargo feature-gate names for activating larger families in CI.
- Whether to add an xtask target (`cargo xtask verify-phase-4`) that runs the full matrix and prints a phase-sign-off summary.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Current Kernel Structure (POST phases 8/9)
- `crates/kernel-lda/src/lib.rs` — 37 compiled + 4 deferred LDA modules, source of truth for LDA functional coverage
- `crates/kernel-gga/src/lib.rs` — facade re-exporting 22 GGA sub-crate batches (`kernel-gga-1a` … `kernel-gga-22`)
- `crates/kernel-mgga/src/lib.rs` — facade re-exporting 37 MGGA sub-crate batches (`kernel-mgga-1a` … `kernel-mgga-37b`)
- `crates/kernel-mgga/src/deferred.rs` — canonical `DeferredMgga` struct + `DEFERRED_MGGA_FUNCTIONALS` const array (6 entries: `mgga_c_b94`, `mgga_x_br89`, `mgga_x_mbr`, `mgga_x_mbrxc_bg`, `mgga_x_mbrxh_bg`, `mgga_x_mggac`). Mirror this pattern in `crates/kernel-lda/src/deferred.rs`.
- `crates/kernel-math` — shared `#[cube]` math primitives used by every kernel crate

### Dispatch Layer (what Phase 4 changes)
- `src/eval/dispatch.rs` — 534 lines; currently hard-wired to `lda_x`. This is the main file being extended.
- `src/eval/mod.rs` — only re-exports `dispatch_lda` today; will gain `dispatch_gga`, `dispatch_mgga`.
- `src/kernel/mod.rs` — top-level kernel re-exports (`libxc_kernel_lda as lda`, etc.). Use these paths from dispatch code, not `crates/...` paths.
- `src/kernel/launch.rs` — `cpu_client`, buffer helpers, `calculate_launch_config` — reuse unchanged
- `src/kernel/dispatch_key.rs` — existing key scheme for (order, spin) routing — extend if needed

### Verification Harness
- `verify/tests/lda_oracle.rs` (173 lines) — currently smoke-only; gets per-functional Rust-vs-C comparison
- `verify/tests/gga_oracle.rs` (370 lines) — same, for 106 GGA functionals
- `verify/tests/mgga_oracle.rs` (274 lines) — same, for 86 MGGA functionals
- `verify/src/lib.rs` — exposes `oracle_lda_all`, `oracle_gga_all`, `oracle_mgga_all`, `oracle_func_flags`, `FLAGS_HAVE_EXC/VXC/FXC/KXC/LXC`; extend only if a required helper is missing
- `verify/build.rs` — cmake + bindgen oracle build; do not modify unless a linker error demands it
- `verify/tests/lda_x_oracle.rs` — existing per-functional reference, shows the intended per-(functional, order, spin) test pattern to generalize

### Upstream Context / Decisions
- `.planning/phases/08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe/08-09-SUMMARY.md` — MGGA VXC oracle tests + deferred tracking
- `.planning/phases/09-reduce-kernel-build-time/09-02-SUMMARY.md` — batch re-translation summary (LDA/GGA/MGGA counts, deferred lists)
- `.planning/phases/04-bulk-kernel-translation/04-01-SUMMARY.md` — kernel-math power functions + oracle GGA/MGGA harness scaffolding already done
- `.planning/phases/04-bulk-kernel-translation/04-RESEARCH.md` — technical research (read for oracle infrastructure, accumulation semantics); some specifics about `src/kernel/` layout are now obsolete but decision rationale remains relevant
- `.planning/phases/04-bulk-kernel-translation/04-VALIDATION.md` — Nyquist validation strategy

### Contracts
- `docs/design/libxc_rs_detailed_design.md` §17 (oracle verification plan), §9.9 (kernel module structure — note the module paths are the `libxc_kernel_*` re-exports now)
- `.planning/REQUIREMENTS.md` — full text of KERN-03..09 and VERIFY-02..07
- `.planning/ROADMAP.md` — Phase 4 goal block

</canonical_refs>

<code_context>
## Existing Code Insights

### Already Delivered
- `src/kernel/launch.rs`: `cpu_client()`, buffer helpers, launch config — reused unchanged
- `src/kernel/mix.rs`: Mixed-functional accumulation — unchanged
- `src/eval/dispatch.rs::dispatch_lda`: the LDA dispatch skeleton (currently `lda_x`-only) — template for the generalization
- `verify/` harness with oracle bindings, fixtures, helpers — ready
- `crates/kernel-{lda,gga,mgga}` + 59 numbered sub-crates — 37 LDA + 106 GGA + 86 MGGA functionals compiled
- `crates/kernel-mgga/src/deferred.rs` — deferred-tracking pattern (to mirror for LDA)
- `tools/translate_{lda_v2,gga,mgga}.py` — **reference only** for understanding naming conventions; do not re-run during Phase 4

### What Phase 4 Must Touch
- `src/eval/dispatch.rs` — generalize `dispatch_lda`; add `dispatch_gga`, `dispatch_mgga`
- `src/eval/mod.rs` — export the two new dispatch functions
- `src/kernel/dispatch_key.rs` — extend if the new dispatch needs it
- `verify/tests/{lda,gga,mgga}_oracle.rs` — rewrite bodies to do per-functional Rust-vs-C comparison at tolerance tiers
- `crates/kernel-lda/src/deferred.rs` (NEW) — mirror the MGGA pattern for the 4 LDA deferred functionals
- `crates/kernel-lda/src/lib.rs` — expose `deferred` module
- Possibly `xtask/src/` — new target for the full phase-4 verification sweep (optional, per D-16-R)

### Established Patterns
- `#[cube(launch_unchecked)]` with `ABSOLUTE_POS` + bounds check — already standard
- Per-functional kernel function naming: `exc_unpol`, `exc_pol`, `vxc_unpol`, `vxc_pol`, `fxc_unpol`, …, `lxc_pol` (10 per functional for LDA; more for GGA/MGGA because of cross-derivative fields)
- Launch wrappers per functional (in the sub-crates) already exist — dispatch just imports and calls them
- `DeferredMgga { name, c_lines, blocked_by, reason }` struct — mirror for LDA
- Oracle test file shape: `FunctionalTestCase { id, name }` const array + iteration — extend with actual comparison

</code_context>

<specifics>
## Specific Ideas

- Treat `crates/kernel-mgga/src/deferred.rs` as the canonical deferred-tracking pattern. Copy its shape (struct + const array + docstring) verbatim for LDA.
- Expect the 106-way GGA dispatch match to be the longest single arm in the codebase — plan to split by sub-crate batch so one match arm per `kernel-gga-{N}{a..g}` is feasible.
- Oracle tests should test derivative orders **independently** (D-09 carryover): one `#[test]` per `(functional, order, spin)` so a single failure doesn't mask others at lower orders.
- For deferred IDs, prefer a test-harness-level filter (e.g., `if DEFERRED.contains(&id) { eprintln!("skip {name}"); return; }`) so the ignored count is visible in cargo test output.
- `dispatch_gga` / `dispatch_mgga` must zero caller buffers before accumulating (same invariant as `dispatch_lda`), because kernel writes are `+=`-accumulated.

</specifics>

<deferred>
## Deferred Ideas

- Enabling the 4 LDA deferred functionals (requires splitting kxc_pol/lxc_pol per-(order,spin) below the 10K-line CubeCL stack limit, or restructuring the translator output). Tracked, not scheduled.
- Enabling the 6 MGGA deferred functionals (requires `xc_mgga_x_br89_get_x` / `xc_mgga_x_mbrxc_get_x` Brent's-method root-finders as `#[cube]` primitives in `kernel-math`). Tracked, not scheduled.
- GPU backend oracle verification beyond `cubecl/cpu`. Post-Phase-4.
- Feature-gate tuning of the oracle suite (e.g., a `verify-full` feature that enables all-kernels + cross-family). Can be added here if ergonomic, but not required.

</deferred>

---

*Phase: 04-bulk-kernel-translation*
*Context gathered: 2026-04-10; refreshed: 2026-04-20 after phases 8 and 9 completed out-of-order.*
*D-03-R MGGA deferred names corrected: 2026-04-20 (W7 — aligned with `crates/kernel-mgga/src/deferred.rs`).*
