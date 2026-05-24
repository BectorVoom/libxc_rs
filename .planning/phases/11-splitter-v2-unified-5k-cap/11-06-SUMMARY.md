---
phase: 11-splitter-v2-unified-5k-cap
plan: 06
subsystem: direction-a-prerequisites-unblocked, task-5-pending
status: COMPLETE — closed 2026-05-25 via the re-opened gap-closure plans (11-09..15, G-1..G-6 at f64). Tasks 5-8 / Gate-3 Legs resolved per the Closure Update below; Leg 3 (f32) DISPOSITIONED as a milestone follow-up (NOT passed). Session-1 PARTIAL record + FAILED-iteration history PRESERVED verbatim (AP-5)
captured: 2026-05-18
tags: [direction-A, 5th-iter, D-25, D-26, D-27, D-28, D-29, AP-7, AP-8, partial, turbofish-rule-9, translator-rule-10, math-baseline-green, mgga_c_b94-canary-green]
requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 11-05 — Phase-1 commits 466e074d0, d8cc4da0c preserved untouched)
provides:
  - PATTERN.md authored from 5 Phase-1 files per D-27 + amended with Rule 9 (cross-fn turbofish) + Rule 10 (translator carry-forward)
  - math/src/ Phase-2 baseline restored via path-scoped reset to d8cc4da0c (state-equivalent to whole-reverting 7a65f3bc6 + dcb7d517d + 233a8890d)
  - Phase-1 source-code turbofish fixes (powers/spin/lambert_w) — first commit in history where `cargo build -p libxc-kernel-math` exit 0
  - dft_quantities turbofish (Phase-2 concrete-f64 file calling Phase-1 generic helpers)
  - Translator-side turbofish emission in translate_lda_v2.py + translate_gga.py + translate_mgga.py (PATTERN.md Rule 10)
  - mgga_c_b94 Gate 3 canary regenerated and compile-green at both precisions (first generated kernel subcrate to compile against generic math/ in project history)
  - f32_tolerance_overrides.toml + verify/tests/parity_phase11.rs env-gate + LIBXC_RS_BYPASS_DEFERRED bypass (Task 4 infra)
  - 4th-iter FAILED SUMMARY archived as 11-06-SUMMARY-HALT-4TH.md (AP-5 history preservation)
deferred_to_fresh_session:
  - Task 5: Manual conversion of 9 Phase-2 files (bspline, dft_quantities*, erf, special, mbrxc, br89, integrate, expint_e1, bessel) per amended PATTERN.md with Rules 1-9 applied from the start
  - Task 6: Gate 3 EXIT (4 legs) — Leg 1 (compile) proven for f64+f32; Legs 2 (parity f64), 3 (parity f32), 4 (idempotency) pending
  - Task 7: D-28 classifier preservation headers on refactor_helpers_generic.py + symbol_class_matrix.rs
  - Task 8: Final SUMMARY rewrite when phase actually closes
deferred_to_11-07:
  - Full-tree regen via `python3 tools/maple_to_kernels.py translate --family all` (the 91 other MGGA + 131 GGA + 43 LDA functionals will pick up Rule 10 turbofish at that pass)
---

# Phase 11 Plan 06: Direction A — CLOSED COMPLETE 2026-05-25 (see Closure Update; Session-1 PARTIAL record preserved below)

## Closure Update (2026-05-25) — COMPLETE

Phase 11 closed COMPLETE via the re-opened gap-closure plans (11-09..15). The Session-1
PARTIAL state below is **preserved verbatim as history (AP-5)**; this section records how the
then-pending Tasks 5-8 and the Gate-3 Legs were ultimately resolved **at f64**.

**Gate 3 EXIT — final disposition:**
- **Leg 1 (compile):** ✓ — proven for the canary at Session 1; generalized by **11-10 (G-3)**'s
  full-roster f64 compile sweep — VERDICT ALL_OK across **305** on-disk packages, 0 fail, 0 pass=-1.
- **Leg 2 (parity f64):** ✓ CLOSED — the von Weizsäcker τ-clamp landed in the **PRODUCTION**
  `mgga_dispatch` (**11-09, G-1**; canary parity PASS at 1e-12), and the family-chunked f64 oracle
  (**11-12, G-2**) ran end-to-end: LDA ✓, GGA ✓. Parity is now demonstrated via the production
  dispatch path with the G-1 τ-clamp — not the canary host-driver workaround.
- **Leg 3 (parity f32):** **DISPOSITIONED — NOT passed.** Re-deferred as a MILESTONE-scale
  follow-up (kernels are f64-concrete by design; an f32 oracle would be a false f64-vs-f64 pass —
  threat T-11-12-01, 11-12-SUMMARY §Deviations). Explicitly NOT claimed as a Phase-11 pass.
- **Leg 4 (idempotency):** ✓ CLOSED — **11-11 (G-4)** D-LOCK-D proof SATISFIED (264 zero-diff +
  sharded-pair disposition).

**Task 8 (final SUMMARY rewrite):** this Closure Update + the COMPLETE status. The
`LIBXC_RS_BYPASS_DEFERRED` bypass (Task-4 infra) was removed at close (11-13 Task 2 Step 5; D-11
restored). **G-6 (11-14)** migrated the umbrella to the cubecl-0.10 launch ABI.

**6 MGGA f64-parity residuals routed to Phase 12 (RECORDED, not fixed here):** the 11-12 f64 oracle
surfaced 6 routed MGGA `exc` functionals failing vs the libxc C oracle at f64 — `mgga_x_th` (2.0e-1),
`mgga_x_2d_js17` (1.1e-2), `mgga_c_cs` (9.2e-3), `mgga_x_pkzb` (3.7e-3), `mgga_x_pbe_gx` (1.5e-3),
`mgga_x_tm` (9.2e-4). Root cause = per-functional translation + residual `work_mgga` regularization
beyond the τ-clamp (which IS applied @ `mgga_dispatch/mod.rs:280-282` and is NOT the cause). Tracked
as the new ROADMAP "Phase 12 — MGGA f64 Parity" entry.

_Everything below this line is the Session-1 (2026-05-18) PARTIAL record, preserved unchanged._

## Outcome

**PARTIAL — Direction A's prerequisites are unblocked, but the 9-file manual
conversion (Task 5) and Gate 3 EXIT (Task 6) have not yet executed.**

This session ran into structural blockers that no prior iteration anticipated.
Closing them consumed the budget intended for the plan's Task 5 work. The
plan is in a much stronger position to be completed in a fresh session — the
math/ baseline now actually compiles for the first time, the canary
mgga_c_b94 also compiles, and PATTERN.md has been amended with the two new
rules (9, 10) that were the proximate cause of every prior HALT.

## Tasks Completed (Session 1)

| # | Task | Status | Notes |
|---|---|---|---|
| 1 | Pre-flight (AP-2 invariants + 9-commit reachability) | ✓ DONE | as-written |
| 2 | Revert 3 Phase-2 commits | ✓ DONE (with deviation) | path-scoped reset to d8cc4da0c — whole-commit `git revert` impossible (see Deviation A) |
| 3 | Author 11-PATTERN.md | ✓ DONE + AMENDED | Rules 1-8 as-written; Rule 9 + Rule 10 added per Task 4 discoveries |
| 4 | Bootstrap dual-precision test infra | ✓ DONE (with massive scope expansion) | TOML + env-gate + BYPASS_DEFERRED + Cargo.toml dep landed; Phase-1 turbofish prerequisite committed; translator-side turbofish landed; mgga_c_b94 regen + canary compile proof landed |

## Tasks Pending (Session 2)

| # | Task | Effort | Risk |
|---|---|---|---|
| 5 | Manual conversion of 9 Phase-2 files | ~6-8h (per plan estimate) | Medium — pattern is now well-defined post Rule 9 + 10 codification |
| 6 | Gate 3 EXIT (4 legs) | ~30-60 min | Leg 1 proven; Leg 2-4 need test runs |
| 7 | D-28 classifier preservation headers | ~5 min | Low |
| 8 | Final SUMMARY rewrite | ~10 min | Low — overwrites this PARTIAL summary |

## Commits This Session (8 total)

In chronological order:

| SHA | Message |
|---|---|
| `c028a7c56` | docs(11-06): archive stale FAILED 4th-iter summary so execute-phase sees 11-06 as incomplete |
| `6148f2010` | docs(11-06): amend archived 4th-iter SUMMARY name to match plan's expected -HALT-4TH.md |
| `714a91c7b` | revert(11-06): Direction A — restore math/src/ to pre-Phase-2 baseline (D-25) |
| `06a52d180` | docs(11-06): author 11-PATTERN.md from 5 Phase-1 files per D-27 (Direction A reference) |
| `38b5bc1ee` | fix(11-06): add explicit `::<F>` turbofish to Phase-1 generic-fn calls + dft_quantities chunk → helper calls (Direction A prerequisite, AP-8 baseline correctness) |
| `68723b8ee` | docs(11-06): amend 11-PATTERN.md with Rule 9 (cross-fn turbofish) + Rule 10 (translator carry-forward) per Task 4 discovery |
| `bca665ad6` | test(11-06): bootstrap dual-precision test infrastructure for Direction A |
| `e7d1bdce4` | fix(11-06): translator emits ::<f64> turbofish for chunk → math/ helper calls (Rule 10) |
| `00b5380a1` | gen(11-06): regen mgga_c_b94 with turbofish'd helper calls (Gate 3 Leg 1 canary) |

## Structural Discoveries (the Three Blockers)

### Discovery 1: Plan's whole-commit revert strategy was impossible

Commit `dcb7d517d` ("fix(11-05): address subset of automated refactoring
syntax errors") is a 436-file mega-commit that bundled the 6 math/src/
syntax fixes with 430 unrelated additions:
- `.cache/11-04-buildlog/libxc-kernel-gga_c_acggap.err` (478,832-line file)
- `.cache/cargo-target/*` artifacts
- 30+ `.claude/agents/*.md` updates
- `.claude/.gsd-profile`

The plan's Task 2 said "git revert --no-commit dcb7d517d" assuming dcb7d517d
was small. Whole-commit revert would have obliterated all 430 unrelated
changes. **Carved out via path-scoped reset: `git checkout d8cc4da0c --
crates/kernels/math/src/`** (commit `714a91c7b`). State-equivalent to
whole-reverting the 3 Phase-2 commits, just for math/src/ scope.

### Discovery 2: Phase-1 baseline NEVER compiled

The 5th-iter plan said Direction A would "mirror the proven Phase-1 clean
pattern." Reality: the `cargo build -p libxc-kernel-math` gate (Task 4)
failed with 11× E0282/E0283 inference errors at the d8cc4da0c (Phase-1
"clean") baseline. Root cause: CubeCL `#[cube]` macro's expand-time path
cannot infer `F` for cross-function generic calls — explicit turbofish
is required.

The Phase-1 commits (`466e074d0` powers/piecewise/lambert_w, `d8cc4da0c`
polynomials/spin) left bare cross-fn calls everywhere. No prior commit
ever added the required `::<F>` turbofish. **The 11-05 SUMMARY's
"100% LOGICAL COMPLETION" claim was always factually wrong on this front.**

This is exactly the false-completion-without-compile-gate pattern recorded
in memory `project_phase11_structural_without_compile`. Three prior HALT
iterations (`75c0f5112`, `3494c80fc`, plus the one this plan supersedes)
never compile-tested the baseline either.

**Fixed in commit `38b5bc1ee`:** 27 surgical turbofish additions across
4 files:

| File | Sites | Pattern |
|------|-------|---------|
| powers.rs | 5 | `safe_cbrt(x)` → `safe_cbrt::<F>(x)` |
| spin.rs | 3 | `compute_total(...)` and `pow_4_3(...)` → `::<F>` |
| lambert_w.rs | 16 | unrolled `halley_step(...)` → `halley_step::<F>(...)` |
| dft_quantities.rs | 3 | concrete-f64 fn calling Phase-1 generic → `::<f64>` |

After fix: `cargo build -p libxc-kernel-math` exit 0 in 8.72s incremental
at f64; exit 0 in 0.10s at f32 (LIBXC_RS_F32=1).

PATTERN.md amended (commit `68723b8ee`) with Rule 9 mandating this for all
Task 5 conversions.

### Discovery 3: Generated kernel tree (~258 crates) never compiled either

After Discovery 2 fix, `cargo build -p libxc_rs-verify` still failed because
verify pulls in `libxc-kernel-gga_x_sogga11` (and ~258 other generated
kernel crates) via libxc_rs's [dependencies]. The generated chunks call
helpers (`piecewise5`, `pow_1_3`, ...) bare — same root cause as Discovery 2.

Translator audit:
- gga_x_sogga11: 37 bare helper calls
- gga_x_pbe: 37 bare helper calls
- mgga_c_b94: 11 bare helper calls
- lda_x: 0 (no Phase-1 helper usage)

The translators (`tools/translate_{lda_v2,gga,mgga}.py`) substitute
`POW_*(` → `pow_*(` and `my_piecewise*(` → `piecewise*(` without turbofish.

**Fixed in commit `e7d1bdce4`:** 3 translators amended to emit
`pow_*::<f64>(` and `piecewise[35]::<f64>(`. Substitution is idempotent
(re-running on already-turbofish'd text does not double-wrap).

**Canary regenerated in commit `00b5380a1`:** `mgga_c_b94` re-emitted via
direct `translate_mgga.emit_per_functional` invocation (avoiding the
5-minute full-family regen). 27 chunk files now turbofish'd.
`cargo build -p libxc-kernel-mgga_c_b94` exit 0 in 4m02s — **first time
in project history that a generated kernel subcrate compiles against the
generic math/ layer.**

PATTERN.md amended (commit `68723b8ee`) with Rule 10 documenting the
translator-side requirement + the 11-07 full-tree regen carry-forward.

## Deviations from the As-Written Plan

### Deviation A: Path-scoped reset instead of `git revert`

Plan Task 2 said: "Revert exactly 3 Phase-2 commits via `git revert
--no-commit`". Real path: `git checkout d8cc4da0c -- crates/kernels/math/src/`.

Why: `dcb7d517d` is a 436-file mega-commit (Discovery 1). Whole-commit revert
was impossible without 430 carve-outs.

State-equivalent to plan's intent for math/src/ scope. All Task 2 verification
gates (Phase-1 untouched, deferred.rs zero F::new, special.rs:224 has
f64::MAX) PASS. Approved by user during execution.

### Deviation B: 11-05-SUMMARY.md edit from 233a8890d not reverted

`233a8890d` rewrote 11-05-SUMMARY.md from "IN-PROGRESS — Option A spike 50%
complete" to "100% LOGICAL COMPLETION". The revert wanted to undo that edit
(which would have been more accurate per CONTEXT 5th-iter), but AP-5 says
SUMMARYs are preserved history. The current "100% LOGICAL COMPLETION" text
stays as historical artifact of the false claim that motivated this whole
iteration.

Approved by user during execution.

### Deviation C: Phase-1 source files edited (turbofish fix)

Plan said Phase-1 files (powers/piecewise/lambert_w/polynomials/spin) are
"PRESERVED untouched." Discovery 2 forced surgical edits to powers/spin/
lambert_w (commit `38b5bc1ee`). 24 lines of syntax-only additions
(`::<F>` turbofish) — algorithmic logic and signatures unchanged.

AP-5 boundary: AP-5 protects SUMMARY history, not source code. Editing
Phase-1 source to make it actually compile is in the spirit of "Phase-1 is
the proven clean reference" — the prior baseline failed that test.

Approved by user during execution.

### Deviation D: tools/translate_{lda_v2,gga,mgga}.py edited

Plan said "This plan does NOT touch `tools/translate_v2/`." Strictly true:
this commit touches `tools/translate_{lda_v2,gga,mgga}.py` (per-family
translators), NOT `tools/translate_v2/` (the CSE/splitter layer). The intent
of D-25's carve-out was to prevent splitter/CSE rework, not to forbid a
3-line helper-substitution amendment.

Without this fix, Gate 3 EXIT (Task 6 Leg 1) is structurally unreachable.

Approved by user during execution.

### Deviation E: Single-functional regen (not --family)

Plan Task 6 Leg 4 implies `python3 tools/maple_to_kernels.py translate
--family mgga --func mgga_c_b94`, but the CLI has no `--func` flag.
Workarounds:
- `--family mgga` (5+ minutes, regens all 92 MGGA functionals)
- Direct `translate_mgga.emit_per_functional` Python invocation (focused)

Chose the direct invocation for the canary regen (commit `00b5380a1`) to
keep the commit scope narrow. The other 91 MGGA functionals stay at their
pre-translator-fix state; 11-07's full-tree regen will pick them up.

Approved by user during execution.

## State at Session End

### Working tree
Clean (8 commits past `bc023835c docs(11): regen 11-06..08 plans per 5th-iter Direction A`).

### Compile gates
- `cargo build -p libxc-kernel-math` ✓ at f64 AND f32 (8.72s / 0.10s)
- `cargo build -p libxc-kernel-mgga_c_b94` ✓ at f64 AND f32 (4m02s / 0.11s)
- `cargo build -p libxc_rs-verify` ✗ blocked by ~258 un-regenerated GGA/LDA crates with bare helper calls (carry-forward to 11-07)
- `cargo build -p libxc_rs` (whole-crate) — not attempted; expected to fail on the same blocker; OOM also possible per D-12

### Plans 11-06..08 dirty?
No — only `00b5380a1` regen of mgga_c_b94 touched generated code. 91 other MGGA + 131 GGA + 43 LDA crates remain at their last-committed state.

### What a fresh session needs to know

To resume Task 5 (manual conversion of 9 Phase-2 files):

1. **Read 11-PATTERN.md cover-to-cover** — Rules 1-8 are the plan's original
   rules; Rules 9 (cross-fn turbofish, MANDATORY) and Rule 10 (translator
   carry-forward, informational) are this session's additions.

2. **Follow plan Task 5 sub-steps 5.1..5.9 in the fixed order** (bspline,
   dft_quantities, erf, special, mbrxc, br89, integrate, expint_e1, bessel —
   easiest-first; bessel LAST per D-26).

3. **Per-file gate:** `cargo build -p libxc-kernel-math` at f64 AND f32.
   Apply Rule 9 turbofish (`::<F>` for generic callers, `::<f64>` for
   concrete) at EVERY cross-fn helper call. The math/ baseline already
   compiles green — per-file failures will be from missing turbofish or
   genuine conversion errors, not pre-existing bugs.

4. **dft_quantities special case:** Already touched in commit `38b5bc1ee`
   with `::<f64>` turbofish (concrete-f64 fn calling Phase-1 generic helpers).
   When Task 5.2 converts it to generic, swap `::<f64>` → `::<F>`.

5. **deferred.rs is excluded from the 9-file conversion set** per
   PATTERN.md Rule 8 (already in d8cc4da0c-restored concrete state; D-23
   surgical preservation coincides with the d8cc4da0c reset).

6. **Per-file atomic commit:** `git commit --only -- crates/kernels/math/src/
   <file>.rs -m "refactor(11-06): convert <file>.rs to generic <F: Float>
   (Direction A, green at both precisions)"` per memory
   `feedback_path_scoped_commits`.

To resume Task 6 (Gate 3 EXIT, 4 legs):
- Leg 1 (compile) already proven for f64+f32 at commit `00b5380a1`.
- Leg 2 (parity f64): `LIBXC_RS_BYPASS_DEFERRED=1 cargo test -p libxc_rs-verify --test parity_phase11 phase11_worst_case` — first attempt; may surface latent issues.
- Leg 3 (parity f32): `LIBXC_RS_BYPASS_DEFERRED=1 LIBXC_RS_F32=1 cargo test ... phase11_worst_case_f32` — currently a TOML-loader probe (per Task 4 commit `bca665ad6`); actual f32 dispatch wiring needs Task 5 helpers generic first.
- Leg 4 (idempotency): re-run `translate_mgga.emit_per_functional('mgga_c_b94')` → expect zero git diff against current state. Should be a clean PASS.

## Prior Iteration Outcomes (preserved for record)

- **3rd-iter** (commit `75c0f5112`): HALT during three-leg gate; Phase 2 refactor
  architecturally invalid (447 × E0308 from `Float::new(val: f32)` constraint).
  Summary archived at `11-06-SUMMARY-HALT.md`.
- **4th-iter** (commit `3494c80fc`): A1 path (cast_from + surgical revert)
  executed Tasks 1-4 (Gate 1 GREEN). D-22 Gate 2 FAILED with 84+ residual
  errors; error progression 234 → 121 → 84 → 1755 → 507 demonstrated AP-8
  (non-monotonic decrease across script-extension passes). Summary archived
  at `11-06-SUMMARY-HALT-4TH.md`.
- **5th-iter Session 1** (this SUMMARY): Direction A prerequisites unblocked
  (math/ baseline + canary mgga_c_b94 compile-green for first time in history;
  PATTERN.md amended with Rules 9 + 10). Tasks 5-8 deferred to a fresh session.

## Lessons Learned

1. **Compile gate as ENTRY, not exit** (re-confirms memory
   `project_phase11_structural_without_compile`): the plan should have
   demanded `cargo build -p libxc-kernel-math` green BEFORE locking
   Direction A. Three iterations missed this because none of them ran the
   compile gate at session start. **Carry-forward proposal for 11-07/08
   plans:** Pre-flight Step 0 = `cargo build -p libxc-kernel-math` exit 0
   verification, with HALT-and-discuss-phase if red.

2. **Whole-commit revert is fragile when commits are large/mixed.** The
   plan author assumed `dcb7d517d` was small; reality was 436 files.
   Path-scoped reset to a known-good parent is more robust when the
   commits-to-revert are heterogeneously-scoped. **Carry-forward:** future
   GSD plans should sanity-check commit sizes before specifying
   `git revert` as the revert mechanism.

3. **PATTERN.md is load-bearing — amend it before Task 5 runs.** Rules 9
   and 10 had to be added DURING execution. If they had been in PATTERN.md
   from the start (as they will be when Task 5 resumes), the per-file
   conversions would have applied turbofish from the first edit, avoiding
   any re-iteration cost.

4. **Direct translator entry points are useful for canary-scoped regens.**
   `translate_mgga.emit_per_functional(c_file, func_name)` is a clean way
   to regen a single canary without the 5-minute full-family overhead. The
   CLI's lack of `--func` is a real ergonomic gap (consider adding in 11-07).

5. **AP-5's scope is SUMMARY history, not source code.** This session
   surgically edited Phase-1 source files (powers/spin/lambert_w) and the
   ground didn't crumble. The "PRESERVE untouched" language in plans is
   often a heuristic, not a hard invariant — explicit framing helps.

## Self-Check: PARTIAL

- ✓ `.cargo/config.toml` UNCHANGED (AP-2)
- ✓ Phase-2 commits' math/src/ effect REVERTED via path-scoped reset (D-25)
- ✓ D-23 surgical (9df2880b3) PRESERVED in history; its file-state coincides with d8cc4da0c reset target
- ✓ Phase-1 commits (466e074d0, d8cc4da0c) PRESERVED in history
- ✓ Classifier commits (a3aacdbec, 7e9391eff, cf59c2c08) PRESERVED in history per D-28/D-29
- ✓ 11-PATTERN.md authored + amended with Rules 9 + 10
- ✓ `cargo build -p libxc-kernel-math` exit 0 at BOTH f64 and f32 (FIRST TIME IN HISTORY)
- ✓ `cargo build -p libxc-kernel-mgga_c_b94` exit 0 at BOTH f64 and f32 (FIRST TIME IN HISTORY for any generated kernel)
- ✗ 9 Phase-2 files manually converted — PENDING Session 2 (Task 5)
- ✗ Gate 3 Leg 2 (parity f64), Leg 3 (parity f32), Leg 4 (idempotency) — PENDING Session 2 (Task 6)
- ✗ D-28 classifier preservation headers — PENDING Session 2 (Task 7)
- ✗ Final SUMMARY rewrite — PENDING Session 2 (Task 8 will overwrite this PARTIAL summary)
- ✓ AP-3 boundary respected (math/src/ is hand-maintained; generated tree only touched via translator regen)
- ✓ AP-8 not triggered (no bulk-script step in Direction A; sed used only as verification probe, output discarded; translator fix is a 3-line idempotent substitution, not an automation-extension)
