# Phase 12: MGGA f64 Parity - Context

**Gathered:** 2026-05-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Bring the **6 routed MGGA exchange-correlation energy (`exc`) functionals** that currently
fail the libxc f64 oracle within `1e-12` relative error, with **no regression** elsewhere:

| Functional | id | rel_err (f64, vs C oracle) |
|------------|----|----------------------------|
| `mgga_x_th`      | 225 | 2.0e-1 (worst — likely a per-functional translation bug) |
| `mgga_x_2d_js17` | 609 | 1.1e-2 (2D functional — see D-13) |
| `mgga_c_cs`      | 72  | 9.2e-3 |
| `mgga_x_pkzb`    | 213 | 3.7e-3 |
| `mgga_x_pbe_gx`  | 576 | 1.5e-3 |
| `mgga_x_tm`      | 540 | 9.2e-4 |

This is a **numerical-correctness debug phase**, not a feature phase. The success criteria are
LOCKED by ROADMAP § Phase 12:
1. Each of the 6 passes the MGGA oracle at the f64 `exc` tier (≤ 1e-12 rel. err on energy).
2. No regression in LDA / GGA / other-MGGA f64 oracle results.

Discussion clarified **HOW to debug and fix**, not what to deliver. **Out of scope:**
polarized MGGA, Fxc/Kxc/Lxc derivative tiers, f32, and any functional not in the 6 above.

**Pre-planning root-cause lead (verified during discussion — NOT yet a fix):**
libxc's `work_mgga` driver clamps **σ DOWN** before each functional
(`σ ← min(σ, 8ρτ)`, `work_mgga_inc.c:67`) and applies σ/τ threshold floors
(`work_mgga_inc.c:59,64`). The Rust production dispatch instead clamps **τ UP**
(`τ ← max(τ, σ/(8ρ))`, `src/eval/mgga_dispatch/prepare.rs:43`) and applies no floors.
Both enforce the same boundary (`σ ≤ 8ρτ ⟺ τ ≥ σ/(8ρ)`) but feed **different (ρ,σ,τ)
triples** to any functional using σ and τ independently — the prime suspect for the
5 small-error functionals. `mgga_x_th`'s 20% is likely a separate translation bug.

</domain>

<decisions>
## Implementation Decisions

### Root cause & fix mechanics

- **D-01 — Mirror libxc's full `work_mgga` input regularization at the dispatch chokepoint.**
  Replace `prepare.rs`'s τ-up clamp with libxc's exact pre-functional sequence:
  σ lower floor (`σ ≥ σ_threshold²`) → τ lower floor (`τ ≥ τ_threshold`) → Fermi-hole
  curvature clamp `σ ← min(σ, 8ρτ)` (the σ-DOWN clamp, NOT τ-up). Applied once at the single
  MGGA dispatch chokepoint so every routed MGGA functional inherits byte-for-byte input parity.
  Rationale: input-level parity with the C driver is the surest route to 1e-12; the current
  τ-up clamp is on the wrong variable.

- **D-02 — This REVISITS the Phase-11 G-1 τ-clamp decision.** The G-1 clamp
  (`prepare.rs::tau_von_weizsacker`) was proven on the `mgga_c_b94` canary at ~5e-13, but it
  clamps the wrong variable vs libxc. D-01 supersedes it. The `mgga_c_b94` canary
  (`verify-canary/tests/g3_mgga_c_b94_parity.rs`) and the `g1_tau_clamp_dispatch_parity.rs`
  canary host drivers MUST be updated to the σ-down regularization and MUST still pass — this
  is an implicit pre-flight check that the regularization change didn't break the proven canary.

- **D-03 — Root-cause-routed fix locations (respecting AP-3 "no hand-edit generated kernels").**
  The executor modifies the surface where the root cause actually lives:
  - Driver/input regularization → `src/eval/mgga_dispatch/{mod,prepare}.rs` (hand-edit OK — NOT
    generated code).
  - Per-functional kernel translation bug (e.g. `mgga_x_th`) → fix `tools/translate_*` +
    regenerate (NEVER hand-edit `crates/kernels/mgga/<func>/`).
  - Shared math-helper bug → `crates/kernels/math/src/` (hand-editable, generic `<F: Float>`).

- **D-04 — Regen discipline: selective loop, full-tree close.** While iterating a per-functional
  kernel fix, single-functional regen is allowed (fast). The CLOSING regen MUST be full-tree
  (`python3 tools/maple_to_kernels.py translate --all-families` or equivalent) and byte-idempotent
  per D-LOCK-D. This catches any translator change that perturbed other families.

### Verification & gate

- **D-05 — Canary loop + family-oracle gate.** Iterate each functional via a per-functional
  `verify-canary/` crate (builds 1 kernel, memory-safe under jobs=1, seconds not hours).
  The family `mgga_oracle` (`oracle-mgga` feature) is the AUTHORITATIVE final gate — the canary
  alone is not sufficient (avoids the Phase-11.1 b94 "hollow gate" trap). The 6 new canaries
  become PERMANENT regression tests in `verify-canary/`.

- **D-06 — Regression proof (SC #2): full family-oracle re-run.** After the shared regularization
  change (which touches every MGGA functional), re-run `oracle-mgga` over ALL routed MGGA
  (including the 6 currently-PASSING ones) as the real no-regression gate, plus a cheap
  `oracle-lda` + `oracle-gga` confirm. LDA/GGA don't route through the MGGA driver so they should
  be untouched, but confirm rather than assume.

- **D-07 — Gate: all-6 target at 1e-12, bounded + escalate.** Target all 6 at the locked 1e-12.
  Bound each functional to **N=3** fix→verify cycles; after the 3rd cycle still failing with a new
  pattern, HALT to `/gsd:discuss-phase 12` for re-direction rather than grinding indefinitely
  (mirrors Phase-11.1 D-13 bounded-iteration discipline; counters the
  `project_phase11_structural_without_compile` anti-pattern).

### Scope

- **D-08 — Target surface: `exc`-unpolarized only.** Drive `exc` (energy) unpolarized to 1e-12.
  Ensure `vxc`-unpolarized does NOT regress (oracle `TOL_VXC=1e-10`) but vxc is not itself a fix
  target. Polarized MGGA and Fxc/Kxc/Lxc tiers stay deferred (already rejected in
  `mgga_dispatch/mod.rs:262-268` — pol-kernel translation bugs are a separate known issue).

- **D-13 — `mgga_x_2d_js17`: attempt, defer if 2D-structural.** PROJECT.md lists 1D/2D
  dimensionality as out-of-scope, yet this 2D functional is routed and failing (1.1e-2). Attempt
  the fix under the same D-01 regularization / D-03 translation lens. If the residual turns out to
  be an inherent 2D-dimensionality evaluation issue (genuinely out-of-scope per PROJECT.md),
  de-route it and document why — do NOT force a 2D-dimensionality fix into this phase. If it
  responds to the shared regularization fix like the others, keep it in the gate.

### Claude's Discretion

- **D-09 — Debug entry order.** Recommended: land the D-01 shared regularization fix first
  (likely closes the small-error cluster of 5 in one move), re-run the oracle, then attack the
  `mgga_x_th` residual (the 20% structural translation bug) separately. Planner/researcher may
  re-order if root-cause evidence suggests otherwise — do NOT assume `mgga_x_th` shares the
  cluster's cause.
- **D-10 — Test-point selection for canaries.** The debug canaries should include grid points that
  exercise the sub-Fermi-hole region (`σ > 8ρτ`, where D-01's clamp activates) so the
  regularization fix is actually validated, not bypassed. Exact point set is planner discretion;
  reuse the `mgga_oracle` 4-point grid (`verify/tests/mgga_oracle.rs:215-218`) as a baseline and
  add a sub-vW point if the baseline doesn't trigger the clamp.
- **D-11 — `ext_params` reproduction in canaries.** Each of the 6 may carry functional-specific
  ext_params (cf. `mgga_c_b94`'s PARAM_GAMMA/CSS/CAB in the existing canary). The canary harness
  must reproduce libxc's defaults for each. Mechanism is planner discretion (mirror the existing
  g3 canary pattern).
- **D-12 — Whether to add a dispatch-level `isfinite` fallback.** libxc's driver has an
  `isfinite` re-evaluation fallback (`work_mgga_inc.c:108+`). Planner decides if matching it is
  necessary for 1e-12 parity on the 6, or if it's only relevant to derivative tiers out of scope.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase 12 definition & the failure source
- `.planning/ROADMAP.md` §"Phase 12: MGGA f64 Parity" (lines 350-373) — goal, the 6 functionals,
  locked success criteria, the "τ-clamp is NOT the cause" note.
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-12-SUMMARY.md` — the source that surfaced and
  attributed the 6 failures (per-family f64 oracle results; §"Per-family f64 oracle results",
  §"Attribution").

### The regularization mismatch (D-01/D-02 core evidence)
- `libxc-master/src/work_mgga_inc.c` §lines 59-94 — libxc's exact pre-functional input
  regularization: σ floor (`:59`), τ floor (`:64`), **σ-down Fermi-hole clamp `:67`**, spin
  cross-term clamp (`:87-91`), and the `FUNC(...)` call (`:94`). THE reference for D-01.
- `libxc-master/src/work_mgga.c` — the preprocessor macro wrapper around `work_mgga_inc.c`.
- `src/eval/mgga_dispatch/prepare.rs` — current Rust regularization (`tau_von_weizsacker`, the
  τ-up clamp at `:43` that D-01 replaces).
- `src/eval/mgga_dispatch/mod.rs` §lines 270-283 (clamp call site) + §262-268 (pol rejection —
  D-08 scope boundary). The single MGGA dispatch chokepoint.
- `src/eval/mgga_dispatch/funcs/` — per-functional dispatch helpers (mapping name → kernel crate).

### The 6 failing kernels & their C oracle source
- `crates/kernels/mgga/{mgga_x_th,mgga_x_2d_js17,mgga_c_cs,mgga_x_pkzb,mgga_x_pbe_gx,mgga_x_tm}/src/`
  — the generated Rust kernels (exc_unpol / vxc_unpol modules). AP-3: do NOT hand-edit; fix via
  translator (D-03).
- `libxc-master/src/maple2c/mgga_exc/{mgga_x_th,mgga_x_2d_js17,mgga_c_cs,mgga_x_pkzb,mgga_x_pbe_gx,mgga_x_tm}.c`
  — the maple2c C source = the translation oracle for per-functional bugs (esp. `mgga_x_th`).
- `libxc-master/src/{mgga_x_th,mgga_x_pkzb,mgga_x_tm,mgga_x_pbe_gx,mgga_c_cs,mgga_x_2d_js17}.c`
  — the `func`/`.mpl`-level definitions + ext_params defaults.

### Verification harness (D-05/D-06)
- `verify/tests/mgga_oracle.rs` — family oracle, `#![cfg(feature = "oracle-mgga")]`, `TOL_EXC=1e-12`
  / `TOL_VXC=1e-10` (`:205-206`), 4-point unpol grid (`:215-218`). The authoritative gate.
- `verify-canary/tests/g3_mgga_c_b94_parity.rs` — the PROVEN single-kernel canary pattern (1 kernel,
  ~5e-13). Template for the 6 new canaries (wrapper `:55-72`, clamp `:96-100`, ext_params `:30-32`).
- `verify-canary/tests/g1_tau_clamp_dispatch_parity.rs` — the dispatch-level clamp canary; its host
  clamp must be updated per D-02.
- `Cargo.toml` §`oracle-mgga` feature (lines ~513-628) — per-family kernel dep gating; the 6 are all
  listed. `cargo tree -p libxc_rs --features oracle-mgga` resolves only MGGA kernels + math (proven
  memory-safe in 11-12).

### Translator (D-03/D-04)
- `tools/translate_mgga.py` — MGGA family translator (maple2c C → Rust `#[cube]`).
- `tools/maple_to_kernels.py` — top-level full-tree regen driver (D-04 close step;
  `translate --all-families`). Idempotent per D-LOCK-D.
- `tools/translate_v2/{cse.py,per_functional.py,emit.py}` — the shared emit pipeline (CSE, chunk
  body emit, literal/turbofish handling). See 11.1-CONTEXT.md for the Rule 2/3/9/10 conventions.
- `crates/kernels/math/src/constants.rs` — named f64 constants (M_PI, M_CBRT*, etc.) for verifying
  any constant-mapping bug in `mgga_x_th`.

### Conventions inherited from Phase 11 / 11.1
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-PATTERN.md` — translation conventions (Rule 3
  named-const wrap, Rule 9/10 turbofish) — relevant if `mgga_x_th`'s bug is a literal/const-wrap fault.
- `.planning/phases/11.1-translator-rule-3-emit-fix-sweep-to-green/11.1-CONTEXT.md` — D-LOCK-D
  idempotency, AP-3, bounded-iteration (D-13) discipline that D-04/D-07 inherit.
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md` — D-LOCK-A (per-functional subcrate),
  AP-1..AP-8 anti-patterns.

### Project locks
- `CLAUDE.md` §Constraints — f64-only, 1e-12, "preserve FP operation order for bit-level equivalence".
- `.cargo/config.toml` — `jobs=1`, `target-dir=.cache/cargo-target`, `RUST_MIN_STACK`. DO NOT edit
  (user manages by hand — `feedback_ram_constraints`).
- `.planning/PROJECT.md` §"Out of Scope" — 1D/2D dimensionality (the D-13 escape hatch for
  `mgga_x_2d_js17`).

### Project memory
- `project_translator_missing_workmgga_tau_clamp` — the original (partial) root-cause note; D-01
  CORRECTS it (the clamp is on the wrong variable, and there are additional floors).
- `project_g3_b94_hollow_gate` — why the family oracle is the real gate, not a single canary (D-05).
- `project_kernels_f64_concrete_f32_milestone` — f32 is out of scope; kernels are f64-concrete.
- `feedback_ram_constraints`, `feedback_kernel_build_failure`, `feedback_path_scoped_commits`.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **`src/eval/mgga_dispatch/prepare.rs` (`tau_von_weizsacker`):** the existing single regularization
  chokepoint — D-01 amends THIS function (or replaces it with a `regularize_inputs` that mirrors
  libxc's σ-floor → τ-floor → σ-clamp sequence). One edit covers all routed MGGA.
- **`verify-canary/tests/g3_mgga_c_b94_parity.rs`:** proven single-kernel parity harness (5e-13).
  Copy-adapt 6× for the targets (D-05). Demonstrates the `#[cube(launch_unchecked)]` thin-wrapper +
  direct-kernel-call + ext_params + clamp pattern.
- **`tools/maple_to_kernels.py` + `tools/translate_mgga.py`:** the regen path for D-03/D-04 kernel
  fixes; idempotent by D-LOCK-D mandate.
- **`oracle-mgga` Cargo feature:** memory-safe family-chunked build (no all-281 OOM) — the D-06 gate
  build path.

### Established Patterns
- **AP-3 (no hand-edit generated kernels):** fixes to `crates/kernels/mgga/<func>/` route through the
  translator. Driver/regularization fixes live in `src/eval/` (not generated) — hand-edit OK (D-03).
- **D-LOCK-D idempotency:** full-tree regen twice must produce zero diff (D-04 close step).
- **Bounded-iteration + HALT-to-discuss (Phase-11.1 D-13):** D-07 reuses this to avoid the
  structural-grind anti-pattern.
- **Path-scoped commits:** `git commit --only -- <path>` (sessions open with thousands of pre-staged
  files; `feedback_path_scoped_commits`).

### Integration Points
- **Dispatch chokepoint** `src/eval/mgga_dispatch/mod.rs:280-282` — where regularized inputs feed
  every MGGA kernel launch. D-01's single edit propagates here.
- **Translator → kernel tree** `tools/translate_*` → `crates/kernels/mgga/<func>/src/` — the D-03
  per-functional fix path.
- **Oracle/canary → libxc-sys** — both harnesses link the vendored libxc C as the reference oracle.

</code_context>

<specifics>
## Specific Ideas

- **The σ-down vs τ-up clamp mismatch is the headline lead.** libxc keeps τ and lowers σ to 8ρτ;
  Rust keeps σ and raises τ to σ/(8ρ). For any functional reading σ and τ independently these are
  different inputs. D-01 corrects it. This was VERIFIED by reading `work_mgga_inc.c:67` against
  `prepare.rs:43` during discussion — it is a confirmed mismatch, but whether it fully closes each
  functional's residual is for the debug/research phase to prove.
- **`mgga_x_th` (20%) is probably NOT the regularization issue.** A 20% energy error is structural —
  likely a per-functional translation bug (constant mapping, piecewise logic, or exponentiation).
  Treat it as a separate investigation (D-09) and compare its `exc_unpol` Rust against
  `maple2c/mgga_exc/mgga_x_th.c` line-by-line.
- **Don't let the b94 canary mask the change.** D-02: after switching to σ-down regularization,
  the existing g1/g3 canaries' host drivers must be updated to match and must still pass at 1e-12.
- **Sub-Fermi-hole test coverage (D-10):** if the oracle's 4-point grid never satisfies σ > 8ρτ, the
  regularization fix is untested by it — add a sub-vW point to the canaries.

</specifics>

<deferred>
## Deferred Ideas

- **Polarized MGGA dispatch** — pol-kernel translation bugs are a separate known issue
  (`mgga_dispatch/mod.rs:262-268`, deferred-items.md D-04-03-A). Not Phase 12.
- **Fxc/Kxc/Lxc derivative tiers for the 6** — out of scope; this phase is `exc`-unpol only (D-08).
- **`mgga_x_2d_js17` as a genuine 2D-dimensionality implementation** — if D-13's attempt shows the
  residual is inherently 2D, the proper 2D evaluation path is a separate (currently out-of-scope per
  PROJECT.md) effort, not Phase 12.
- **Other routed-but-unverified MGGA functionals beyond the 6** — only the 6 named failures are in
  scope; broader MGGA parity sweeps are future work.
- **f32 oracle / generic kernels** — milestone-scale (`project_kernels_f64_concrete_f32_milestone`),
  not Phase 12.

</deferred>

---

*Phase: 12-mgga-f64-parity*
*Context gathered: 2026-05-25*
