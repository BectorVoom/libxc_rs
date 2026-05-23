---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: [2026-05-24] /gsd:plan-phase 11 --gaps RE-PLAN DONE (commit c4377a1e20, plan-checker PASSED all 12 dims): 3 plans in 3 waves — 11-15 (NEW, migrate 5 crates/kernels/math/src test-module 0.9→0.10 launch+readback ABI, LIGHT `-p libxc-kernel-math --tests` gate, wave 1, assistant/auto) → 11-10 (REVISED f64-ONLY full-roster sweep to len(build_roster())==280, f32 sweep DROPPED as a false f64-vs-f64 pass T-11-12-01, wave 2, USER-RUN multi-day) → 11-13 (REVISED closure: INVERT f32 wording [CLAUDE.md stays "f64 only"] + fix ROADMAP #4/#5/G4 + ADD a Phase 12 MGGA f64-parity entry [6 functionals, flagged for user redirect at the Task-1 checkpoint] + delete 5 tools + remove LIBXC_RS_BYPASS_DEFERRED + 11-06-SUMMARY PARTIAL→COMPLETE + manual phase-close, wave 3). The 4 closed gaps (11-09/11-11/11-12/11-14) NOT re-planned. NEXT: /gsd:execute-phase 11 --gaps-only (11-15 first). ───── PRIOR: /gsd:execute-phase 11 --gaps-only HALTED at the blocking-antipattern gate → user chose RE-PLAN (not execute). Both remaining gap plans are STALE vs the 11-12 f64-concrete/f32-defer decision: (11-10 G-3) Task-2 f32 sweep (LIBXC_RS_F32=1) is INVALID — no f32 kernel path exists, would be a false f64-vs-f64 pass (threat T-11-12-01, already rejected); only Task-1 f64 sweep (280 on-disk pkgs = LDA43+GGA131+MGGA106, USER-RUN multi-day, jobs=1) is real, AND the kernels/math test-module 0.9 launch-ABI drift must land first or the sweep's cargo-test path breaks. (11-13 G-5) blocking human-verify checkpoint cites the re-DEFERRED "11-12 full-649 f32 oracle within 1e-3" + Task-2/3 edits encode stale "f32 secondary 1e-6 / full-649 f32 oracle as a gate" wording — must INVERT to "f32 = milestone follow-up, not a Phase-11 gate" + ADD a new MGGA f64-parity gap (6 functionals: mgga_x_th 2e-1, mgga_x_2d_js17 1.1e-2, mgga_c_cs 9.2e-3, mgga_x_pkzb 3.7e-3, mgga_x_pbe_gx 1.5e-3, mgga_x_tm 9.2e-4). AP-Rule3-broad in the OLD .continue-here (06) is RESOLVED by the f64-concrete pivot (M_PI×f64 is no longer a type error); .continue-here refreshed to RE-PLAN-PENDING (non-blocking). No plans marked complete; no false close. NEXT: /gsd:plan-phase 11 --gaps. ───── PRIOR: Phase 11 RE-OPENED — Wave 1: 11-09 G-1 ✓, 11-11 G-4 ✓, 11-14 G-6 ✓; Wave 2: 11-12 G-2 ✓ DONE (f64 oracle), 2026-05-23. 11-12 (G-2) infra BUILT + validated: Path A feature-gate mechanism (f04e2095dc, cargo-tree-proven: oracle-{lda,gga,mgga} → only that family's kernels; default=all preserves 11-14 build) + D1 umbrella source-cfg (af1c5e1c20) + D2 stale-harness repair (5566f99467) + S1 launch.rs generic-over-F (86cf732e09). USER-RUN compile gate: `cargo check -p libxc_rs --lib --no-default-features` EXIT 0 (3m08s real compile, no-family core+math clean). ⛔ f32/G4 RE-DEFERRED as MILESTONE-SCALE (user, 2026-05-23): the kernels are f64-CONCRETE by design (2491 files `&Array<f64>`, 0 generic `&Array<F>`; CLAUDE.md "f64 only" + 1e-12 core value). Real f32 ≠ a dispatch change — it needs a translator re-arch (emit float-generic kernels) + full ~2491-file regen + FP-order reconciliation. LIBXC_RS_F32/D-19a was an aspiration never realized at the kernel layer (its only reader parity_phase11.rs computes in f64). S1 kept as harmless foundation. G-2 = the memory-safe family-chunked f64 oracle. ✅ f64 oracle RAN per family (user-run 2026-05-23): LDA ✓, GGA ✓, MGGA 6/12 routed exc FAIL (rel_err 9e-4→2e-1, mgga_x_th 20%) — genuine pre-existing MGGA f64 parity bugs ATTRIBUTED (τ-clamp IS applied @ mgga_dispatch/mod.rs:280-282, NOT the cause; per-functional translation + residual work_mgga regularization), ROUTED to a dedicated MGGA-parity roadmap effort. 11-12-SUMMARY.md written → G-2 (f64) CLOSED (oracle runs to completion; residuals attributed, not silently passed). Phase NOT complete: 11-10 G-3 (sweep, ○ deferred), 11-13 G-5 (closure — fix ROADMAP G4/SC-#5 wording [f32=milestone] + add MGGA-parity gap; depends 11-10+11-12). NEXT: 11-10 (heavy sweep) then 11-13, OR open the MGGA-parity effort. Milestone v1.0 IN PROGRESS (Phases 5, 6, 7 + Phase 11 closure remain). [Phase 11.1 COMPLETE 2026-05-22.]
stopped_at: "[2026-05-23] 11-12 (G-2) RE-SCOPED to f64 + D1/D2/S1 source landed (user-run `cargo check -p libxc_rs --lib --no-default-features` EXIT 0, 3m08s, no-family core+math clean). f32/G4 RE-DEFERRED milestone-scale: kernels are f64-CONCRETE (2491 files `&Array<f64>`, 0 generic `&Array<F>`) — real f32 needs translator re-arch + ~2491-file regen + FP-order reconciliation, against f64-only/1e-12 design; NOT a dispatch task (LIBXC_RS_F32/D-19a never realized at kernel layer). G-2 NOW = memory-safe family-chunked f64 oracle. REMAINING = run per-family f64 oracle (heavy/USER-RUN: `cargo test -p libxc_rs-verify --no-default-features -F oracle-<fam> --test <fam>_oracle -j1`; validates D2 harness bodies — Phase-05 drift may surface) → 11-12-SUMMARY + fix ROADMAP G4/SC-#5 wording via 11-13. Commits: af1c5e1c20 (D1 umbrella src-cfg: kernel/mod.rs + eval/mod.rs stubs + evaluate.rs/mix.rs eval-level imports), 5566f99467 (D2 harness: is_deferred→libxc_kernel_math::deferred::{lda,mgga} + verify dev-dep libxc-kernel-math + per-family #![cfg]), 86cf732e09 (S1 launch.rs generic-over-F:Pod buffers, f64 wrappers delegate). See 11-12-ORACLE-F32-LOG.md (2026-05-23 UPDATE section). HARD: assistant runs cargo tree only; USER runs all compiles; jobs=1; NEVER edit .cargo/config.toml. ───── HISTORICAL [2026-05-22] (D3/D4/D5 f32 turnkey below is SUPERSEDED by the re-defer): Phase 11 — 11-12 (G-2) Wave 2 PARTIAL, inline-sequential (2026-05-22, /gsd:execute-phase 11 --gaps-only; user scope = '11-12 only, then reassess' → sub-scope 'Path A infra now, defer heavy builds'). LANDED (commit f04e2095dc, build(11-12)): Path A feature-gate mechanism — tools/make_kernel_deps_optional.py made all 280 per-functional kernel path-deps optional in root Cargo.toml + added [features] oracle-{lda,gga,mgga} (default=all three, so bare `cargo build -p libxc_rs` is byte-identical to pre-change → 11-14 EXIT-0 preserved BY CONSTRUCTION); verify/Cargo.toml set libxc_rs default-features=false + forwards oracle-* features. PROVEN via `cargo tree` (NO compile): oracle-lda→lda43/gga0/mgga0, oracle-gga→gga131, oracle-mgga→mgga106, default→all, verify forwards cleanly (test-build adds only a fixed 6-witness floor, non-OOM). This IS G-2's memory-safe family-chunked mechanism (no all-281 OOM build). DEFERRED w/ turnkey spec in 11-12-ORACLE-F32-LOG.md: D1 umbrella SOURCE cfg-gating (generators generate_kernel_reexports.py + generate_{gga,mgga}_dispatch.py emit #[cfg(feature=oracle-<fam>)]; hand-gate src/kernel/mod.rs, eval/mod.rs, lib.rs:34, functional/evaluate.rs router w/ #[cfg(not(all(...)))] catch-all to dodge deny(warnings) unreachable-pattern; cheap check surface = `cargo check -p libxc_rs --lib --no-default-features` = math+core only, 0 kernels), D2 repair stale Phase-05 harnesses (lda_oracle.rs:33 / mgga_oracle.rs:41 import deleted libxc_kernel_{lda,mgga}::deferred → use libxc_kernel_math::deferred::{lda,mgga}::is_deferred + add verify dev-dep libxc-kernel-math + per-family #![cfg]), D3 f32 env-gate+tolerance lift from parity_phase11.rs::f32_tolerance_for (1e-3 ceiling), D4 paced per-family f32 sweeps, D5 Task-3 tolerance checkpoint → then write 11-12-SUMMARY.md. WHY DEFERRED: umbrella can't be cargo-checked cheaply (default check = all 280 = OOM/multi-hour); blind source-cfg under deny(warnings) risks regressing 11-14's green build, so it rides with the heavy build session. NEXT: 11-12 finish (heavy build session: D1→D5) and/or 11-10 (G-3 sweep, heavy), then 11-13 (G-5). HARD: inline sequential, jobs=1 (NEVER edit .cargo/config.toml), NO monolithic umbrella build, per-`-p` compile as ENTRY gate. ───── PRIOR (11-14, G-6 cubecl-0.10 umbrella launch-ABI migration): ✅ DONE — `cargo check -p libxc_rs --lib` EXIT 0 (3031 → 0, jobs=1, user-run, peak RSS ~536 MB; /tmp/11-14-fix2-check.log). Migration: GGA+MGGA dispatch GENERATORS (tools/generate_{gga,mgga}_dispatch.py) migrated to the 0.10 launch ABI [from_raw_parts(handle,len) 2-arg + .clone() since Ctx holds &Handle; bare scalars; launch_unchecked returns () so .map_err(..)? dropped; ScalarArg import deleted] + regen (132 files); hand-written src/eval/dispatch.rs + src/kernel/launch.rs migrated directly. 11-09 τ-clamp preserved through regen (tau_von_weizsacker present). Commits: f9c4ff05a8 (migration), e3954802cd (4 non-launch-ABI residuals: read_one→Result readback in launch.rs + dispatch_gga gained params:&dyn FunctionalParams arg to match dispatch_mgga/lda+callers), 1ad364b612 (4 dead_code under #![deny(warnings)]: removed 3 orphaned map_*_launch_err + LaunchError imports; #[allow(dead_code)] on as_initialized_mut), 6434274b5e (SUMMARY + final log). G-2/11-12 UNBLOCKED. DEVIATIONS (in 11-14-SUMMARY): handle .clone(); 4 of 3031 were NOT launch-ABI (fixed in-scope); as_initialized_mut allow is OUT-OF-SCOPE file (src/compat/raw_handle.rs) — flagged for compat-layer review. FOLLOW-UP (not G-6): crates/kernels/math/src/{piecewise,powers,polynomials,erf,dft_quantities}.rs carry the SAME 0.9 launch+readback drift in #[cfg(test)] mod tests — test-gated (the --lib gate doesn't reach them) but will break cargo test / 11-10 sweep / 11-12 oracle; migrate there. NEXT: 11-12 (G-2 full-649 f32 oracle, now unblocked) and/or 11-10 (G-3 multi-day compile sweep, ○ deferred), then 11-13 (G-5 closure). HARD: inline sequential, jobs=1 (NEVER edit .cargo/config.toml — user caps jobs by hand; working tree had it uncommented/uncapped this session), NO monolithic umbrella build, per-`-p` compile as ENTRY gate."
last_updated: "2026-05-24T00:00:00.000Z"
last_activity: 2026-05-24
progress:
  total_phases: 12
  completed_phases: 7
  total_plans: 61
  completed_plans: 52
  percent: 85
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-09)

**Core value:** Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 XC functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.
**Current focus:** Phase 11 (re-opened) — gap closure. Wave 1: 11-09 ✓ + 11-11 ✓ + 11-14 ✓. Wave 2: 11-12 (G-2) ✓ DONE (f64 oracle) — memory-safe family-chunked f64 oracle built + validated + RUN per family (mechanism f04e2095dc + D1 af1c5e1c20 + D2 5566f99467/bf7c4b6eb3 + S1 86cf732e09; SUMMARY written). f64 oracle: LDA ✓ GGA ✓; MGGA 6/12 routed exc FAIL (real pre-existing MGGA f64 parity bugs, attributed + routed to a dedicated MGGA-parity roadmap effort). f32/G4 RE-DEFERRED milestone-scale (kernels f64-concrete). Remaining in Phase 11: 11-10 G-3 (sweep, ○ deferred), 11-13 G-5 (closure — ROADMAP G4 wording fix + add MGGA-parity gap; depends 11-10+11-12). Phase NOT complete.

## Current Position

Phase: 11 (splitter-v2-unified-5k-cap) — RE-OPENED, Waves 1+2 PARTIAL (2026-05-22, inline-sequential via /gsd:execute-phase 11 --gaps-only).
Wave 1 status: 11-09 G-1 ✓ DONE (production τ-clamp + canary PASS 1e-12); 11-11 G-4 ✓ DONE (D-LOCK-D SATISFIED, 264 zero-diff); 11-14 G-6 ✓ DONE (cubecl-0.10 umbrella launch-ABI migration — `cargo check -p libxc_rs --lib` EXIT 0, 3031→0). 11-10 G-3 ○ DEFERRED (multi-day overnight compile sweep).
Wave 2 status: 11-12 G-2 ✓ DONE (f64 oracle), 2026-05-23. BUILT + validated: mechanism f04e2095dc (cargo-tree-proven: oracle-{lda,gga,mgga}→only that family; default=all preserves 11-14) + D1 umbrella source-cfg af1c5e1c20 (kernel/mod.rs #[cfg] pub mod; eval/mod.rs gated dispatch mods + #[cfg(not)] stub dispatch_{lda,gga,mgga}; evaluate.rs/mix.rs eval-level imports) + D2 harness repair 5566f99467 (is_deferred→libxc_kernel_math::deferred::{lda,mgga} + verify dev-dep libxc-kernel-math + per-family #![cfg]) + S1 launch.rs generic-over-F:Pod 86cf732e09. USER-RUN gate: `cargo check -p libxc_rs --lib --no-default-features` EXIT 0 (3m08s, no-family core+math clean). ⛔ f32/G4 RE-DEFERRED milestone-scale (2026-05-23): kernels are f64-CONCRETE (2491 `&Array<f64>`, 0 `&Array<F>`); real f32 = translator re-arch + full regen, against f64-only/1e-12; NOT a dispatch task. S1 kept as harmless foundation. ✅ f64 oracle RUN per family (user, 2026-05-23): LDA ✓, GGA ✓, MGGA test_all_mgga_oracle_unpol 6/12 routed exc FAIL (rel_err 9e-4→2e-1; mgga_x_th 20%; pol test passed). ATTRIBUTED: genuine pre-existing MGGA f64 parity bugs — τ-clamp IS applied (mgga_dispatch/mod.rs:280-282, NOT the cause); per-functional translation (mgga_x_th likely) + residual work_mgga regularization. ROUTED to a dedicated MGGA-parity roadmap effort. 11-12-SUMMARY.md written → G-2 (f64) CLOSED (oracle runs to completion; residuals attributed not silently passed). See 11-12-ORACLE-F32-LOG.md (f64 RESULTS section) + 11-12-SUMMARY.md.
Remaining: 11-13 (G-5, W3 closure — fix ROADMAP G4/SC-#5 wording [f32=milestone follow-up] + add MGGA-parity gap; depends 11-10+11-12), 11-10 (G-3 sweep, ○ deferred). Original 11-01..08 untouched (all have SUMMARYs).
✅ RESOLVED (was the 11-09 entry-gate blocker): the umbrella libxc_rs lib now COMPILES under cubecl 0.10. 11-14 migrated the launch ABI in the GGA+MGGA dispatch GENERATORS (durable across regen) [from_raw_parts 2-arg + .clone(); bare scalars; launch returns (); ScalarArg import gone] + regen + hand-written dispatch.rs/launch.rs; the 11-09 τ-clamp survived. 3 jobs=1 user-run gates: #1 3031→4 type errors (migration), #2 4 type→0 but 4 dead_code (read_one Result + dispatch_gga params arg fixed in e3954802cd; 3 orphaned map_*_launch_err + as_initialized_mut in 1ad364b612), #3 EXIT 0. See 11-14-SUMMARY.md + 11-14-MIGRATION-LOG.md. Memory: project_umbrella_cubecl010_launch_abi_drift (now resolved). FOLLOW-UP: crates/kernels/math/src test-module 0.9 launch+readback drift (test-gated; surfaces under cargo test / 11-10 / 11-12). Next: /gsd:execute-phase 11 --gaps-only (11-12 unblocked; 11-10 heavy/deferred; 11-13 closure).
Prior: Phase 11.1 (translator-rule-3-emit-fix-sweep-to-green) — COMPLETE (2026-05-22); 11.1-01..04 all executed; 11.1-SUMMARY.md committed
Previous execution: 11.1-01 ✓ (translator amend); 11.1-02 ✓ (regen + G1 f64 + G2 f32 ALL_OK on 50-sample; idempotency DEFERRED); 11.1-03 ✓ (G3 mgga_c_b94 PASS at 1e-12 — rewritten as standalone verify-canary crate building 1 kernel; G4 DEFERRED); 11.1-04 ✓ (phase close).
Key finding (→ Phase 11): the Rust dispatch omits libxc work_mgga's von Weizsäcker τ-clamp (τ≥σ/(8ρ)) — systemic MGGA parity gap; gates a meaningful G4. See memory project_translator_missing_workmgga_tau_clamp.
Hand-back to re-opened Phase 11: work_mgga τ-clamp, G4 full-649 f32 oracle, full-266 sweep, D-LOCK-D idempotency proof, 11-06 Legs 2/3/4 + Task 8, 11-08 Task 2, phase.complete 11.

## Phase 11 execute-phase run — 2026-05-18 evening session

**Outcome: PARTIAL.** Structural goals (D-10a, D-13, dispatch tree) MET. Codegen-correctness goals (SPEC-11-R4, SPEC-11-R5, D-24 f32 sweep) BLOCKED on translator Rule 3 emit gap. Phase remains open in ROADMAP; no phase.complete invocation. See `.planning/phases/11-splitter-v2-unified-5k-cap/11-FINAL-METRICS.md` for end-state metrics.

**Session arc:**

1. 11-06 Task 6 Leg 1 (mgga_c_b94 canary compile) re-confirmed GREEN at f64 + f32 — first re-verification post Deviation E+F. (4m 07s @ jobs=1.)
2. 11-06 Task 6 Leg 2 (parity f64 phase11_worst_case) HALTED at compile of verify's transitive dep graph: `gga_c_gaploc/lxc_pol/part53/chunk804.rs:13` failed with 2920 errors of the form `let t = M_PI * t_F` (Rule 3 violation: named-const M_PI inline in F-typed arithmetic — pattern P1).
3. Pivot to 11-07 (per AP-8: "real translator bug routes through /gsd:discuss-phase", interpreted by user as pivot to sweep tool which is the diagnostic instrument).
4. 11-07 Task 1 (`tools/batched_compile_sweep.py`) authored to spec at `6e2a793fb8` — 515 lines, all forbidden/required pattern checks pass.
5. 11-07 Task 2 (LDA-only sweep, 17m 22s) HALTED at `lda_c_pk09` — 789 errors of distinct shape: tuple-return chunk with bare f64 tuple member (pattern P2).
6. User disposition: stop sweep (would re-confirm same root cause across GGA/MGGA); jump to 11-08 audits.
7. 11-08 Task 1 Step 0/1/4/5/6: AP-2 pre-flight PASS; 5 audits re-run (A1/A2/A4 PASS; A3 splitter floor at 22 unexcepted >5K, max 6,674; A5 tool staleness in `split_lda_subcrates.py`). 11-FINAL-METRICS.md authored at `a470529c8c`. Step 3 (F32 smoke) deferred — translator bug would block cargo test. Tasks 2 + 3 deferred.
8. 11-08-SUMMARY.md committed at `ac9729a51d` recording phase-11 PARTIAL close.

**Two translator defect patterns (same root cause family):**

| Pattern | First observed | Exemplar | Fix |
|---|---|---|---|
| P1: Named-const ref inline in F-typed arithmetic | `gga_c_gaploc::lxc_pol::part53::chunk804.rs:13` | `let t = ... * M_PI * ...` | Wrap as `F::cast_from(M_PI)` or hoist `let pi = F::cast_from(M_PI);` at fn body top |
| P2: Tuple-return chunk with bare f64 tuple member | `lda_c_pk09::fxc_pol::part2::chunk5.rs:15` | `(t6, t7, t8)` where `t8` is bare f64 literal | Wrap tuple-return members in `F::cast_from(...)` / `F::new(...)` per Rule 2/3 boundary |

Root cause: `tools/translate_v2/` chunk-body emit path does NOT apply Rule 3 (`F::cast_from(NAMED)` / `F::new(literal)`) to f64-literal positions inside the function body. Deviation F (commits `4aaaaa7739`/`8a9f32091e`/`d26efabda6`) extended Rule 10 turbofish to cross-fn calls only.

**Phase 11.1 follow-up scope (recommended):** Amend translator chunk-body emit for P1+P2+P3-preventive; full-tree regen (supersedes Deviation F); re-run `python3 tools/batched_compile_sweep.py` until ALL_OK; resume 11-06 Task 6 Legs 2/3/4 + Task 8; resume 11-08 Task 2 (config/cleanup + LIBXC_RS_BYPASS_DEFERRED removal) + Task 3 (D-24 full-649 f32 sweep); then phase.complete 11.

**Durable phase 11 deliverables (regardless of 11.1 outcome):**

- `tools/batched_compile_sweep.py` — codifies the per-`-p` compile entry gate that memory `project_phase11_structural_without_compile` flagged as missing
- D-10a clean-slate restructure (266 per-functional subcrates)
- D-13 launch budget invariant (1654 routed / 0 unrouted / 22 math/src/)
- 11-PATTERN.md (Rules 1-10) — canonical translation conventions
- Phase-2 math/ files manually converted to generic `<F: Float>` (9 files)
- f32_tolerance_overrides.toml + LIBXC_RS_F32 env-gated parity infra

## Stale 11-06 narrative below — SUPERSEDED by the section above (kept for history)

5th-iter Session 1 outcome (2026-05-18, 8 commits): math/ baseline now compile-green for FIRST TIME IN HISTORY; mgga_c_b94 canary regenerated and compile-green at both precisions; PATTERN.md amended with Rule 9 (cross-fn turbofish, MANDATORY) and Rule 10 (translator carry-forward). Three structural blockers fixed:
  (1) dcb7d517d 436-file scope → path-scoped reset (Deviation A)
  (2) Phase-1 baseline never compiled (turbofish missing) → 27 surgical edits to powers/spin/lambert_w/dft_quantities (Deviation C, commit 38b5bc1ee)
  (3) Generated chunk tree never compiled → translator-side turbofish emission in translate_{lda_v2,gga,mgga}.py (Deviation D, commit e7d1bdce4) + mgga_c_b94 canonical regen (commit 00b5380a1)

5th-iter Session 2 outcome (2026-05-18, 9 commits): ALL 9 Phase-2 files in `crates/kernels/math/src/` manually converted to generic `<F: Float>` per 11-PATTERN.md Rules 1-9. Per-file gates GREEN at f64 AND f32 (compile + spike_cse_emit_q01). Aggregate sanity sweep: all 9 files have `<F: Float>` signatures, ZERO `F::new(<NAMED_CONST>)` Rule 3 violations. Phase-1 files (powers/piecewise/lambert_w/polynomials/spin) UNTOUCHED across all 9 commits (Step F verified after each commit). Per-file atomic commits in plan-mandated easiest-first order (bspline first, bessel LAST per D-26):

  1. `9e7544efb` bspline.rs (5 fns, 21 cast sites)
  2. `6570d948d` dft_quantities.rs (4 fns, ~13 cast sites; swapped ::<f64>→::<F> per Session 1 carry-forward)
  3. `9f8bb2000` erf.rs (3 fns, 78 F::cast_from + 21 F::new)
  4. `8a995fb99` special.rs (4 fns, 4-fn Chebyshev+Faddeeva; F::cast_from for >5-digit let-binding coefficients)
  5. `5c35eb711` mbrxc.rs (2 fns, 60 unrolled Brent iterations via Edit replace_all)
  6. `74c5321ed` br89.rs (2 fns, mixed multi-line + compact iteration layouts)
  7. `19882e5b1` integrate.rs (9 fns, 176 cross-fn turbofish sites, 32-pt GL × 4 helpers)
  8. `de49d7b59` expint_e1.rs (7 fns, 150 Clenshaw inline-coeff F::cast_from sites)
  9. `1bf0e3bf1` bessel.rs (10 fns, "highest symbol-class diversity" per D-26; 109 Clenshaw coeffs + 4 named consts)

Pattern decisions codified during conversion:

- Long-precision let-binding constants (Chebyshev/SLATEC coefficients with >5 significant digits): use F::cast_from(<f64-literal>) to preserve f64 precision in f64 mode; F::new(f32) would catastrophically truncate
- Short exact-representable literals (0.0, 0.5, 1.0, 2.0, 3.0, powers-of-2 fractions): F::new(...) per Rule 2
- Module-level `const X: f64 = ...` declarations: kept f64-typed; in-body usage wraps with F::cast_from per Rule 3
- u32-to-F casts (e.g. `idx as f64` → bspline): `F::cast_from(idx)` works via CubeCL's blanket `impl<P: CubePrimitive> Cast for P`
- cfg(test) test kernels (concrete-f64 launch path): add `::<f64>` turbofish at helper call sites (Rule 9 concrete caller form)
- cfg(test) pure-CPU code (bessel/mbrxc/br89 reference impls): UNCHANGED

Next step: fresh session — `/gsd:execute-phase 11` (continues with Task 6 of 11-06: Gate 3 EXIT 4 legs):

- Leg 1 (compile): already proven for f64+f32 at commit `00b5380a1` (mgga_c_b94 canary)
- Leg 2 (parity f64): `LIBXC_RS_BYPASS_DEFERRED=1 cargo test -p libxc_rs-verify --test parity_phase11 phase11_worst_case`
- Leg 3 (parity f32): `LIBXC_RS_BYPASS_DEFERRED=1 LIBXC_RS_F32=1 cargo test ... phase11_worst_case_f32`
- Leg 4 (idempotency): re-run `translate_mgga.emit_per_functional('mgga_c_b94')` → expect zero git diff

Then Task 7 (D-28 classifier preservation headers, ~5 min), then Task 8 (final SUMMARY rewrite overwriting current PARTIAL).

Plans: 8/8 written; 5/8 executed; 1/8 IN PROGRESS (11-06 PARTIAL — Task 5 done, Tasks 6-8 pending). 11-07/08 blocked behind 11-06 completion (and 11-07 needs to also pick up the translator-emitted turbofish for the other 91 MGGA + 131 GGA + 43 LDA functionals via full-tree regen).

Plan 11-03 outcome (2026-05-15):

- Task 1: verify-only re-confirmation of `95727cb36`+`97d6347be` (clean-slate
  266-subcrate restructure) — approved by user; no commit.

- Task 2 (`eea58fed7`): rewrote `tools/audit_cube_launch.sh` to the D-13
  per-design launch budget (routed one-per-output, unrouted-zero,
  math/src/ ≤22). PASS: 1654 routed pairs, 0 unrouted launchables, math=22.

- Task 3 (`f820fae90` --allow-empty): re-ran the three dispatch/re-export
  generators — zero git diff (deterministic against committed WIP
  `c3fba8089`). `audit_dispatch_tree.sh` exit 0; 0 batchN refs survive.
  Path-resolution gate ran at RUNG 2: built `libxc-kernel-lda_c_lp96`,
  `libxc-kernel-gga_x_lb`, `libxc-kernel-mgga_xc_lp90`, then `rustc --extern`
  type-checked the deep `crate::kernel::{family}::<func>::<output>::<fn>`
  re-export paths — exit 0. Blocker B1 closed.

Wave 2 is finished under D-13. Next plan: 11-04.

Plans: Phase 06 still has 3 of 4 executed (09-04, 09-05, 09-06 ✓; 09-07 oracle parity sweep pending; old 09-01/02/03 archived under `archive-pre-round4/`) — paused while Phase 11 is in flight.
Last activity: 2026-05-21

## Phase 11 — PAUSED at Plan 11-05, Option A → Option C Pivot (2026-05-18)

Attempted Option A (refactor 38 helpers to generic `<F: Float>`) blocked by Phase 2 automated-script systematic errors in 11 files. User elected to **pivot to Option C (cast-at-call-site in translator)** at 14:00 UTC. 

State: commit dcb7d517d marks partial fixes and checkpoint. Next session: replan with Option C.

Carry-forward from 11-05 attempt:

- Commit d8cc4da0c: Manual Phase 1 refactoring (5 files) validated as working ✓
- Commit dcb7d517d: Partial fixes + analysis of remaining errors
- Decision: abandon this path, move to translator-level approach

## Phase 05 — Gap Closure Resolved (2026-05-02)

The "Pending Resumption" block previously here was stale. The gap-closure work
flagged as paused on 2026-04-29 was actually completed on 2026-05-02. Concrete
evidence:

| Plan | WIP commit (paused) | Real fix commits (after resume) | SUMMARY |
|------|---------------------|---------------------------------|---------|
| 05-04 | df5324f1 | c20a0225 → 50508037 → 08996314 | 20.6 KB ✓ |
| 05-05 | 861f21dd | 0afc877a → 41bffc29 → 46ce5b9c → 69a372f8 | 17.7 KB ✓ |
| 05-06 | (no wip) | cb634de1 → 01f6039a → 6c5ac9f1 | 19.8 KB ✓ |
| 05-07 | 71fdddd8 | 45896f06 → b4cd019c | 9.8 KB ✓ |

The original locked worktree branches (`worktree-agent-a5c0fcda…`,
`…a841c937…`, `…a47664d4…`) and WIP SHAs (`eb08f4ab`, `3cbc49f9`, `ae6b847c`)
referenced in the older note are no longer reachable in git — superseded by
the clean fix commits above.

Outstanding: re-run `/gsd-verify-work 5` to upgrade `05-VERIFICATION.md`
(currently `status: human_needed`, dated pre-resume 2026-04-28) to `pass`.

## Performance Metrics

**Velocity:**

- Total plans completed: 21
- Average duration: --
- Total execution time: 0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01 | 3 | - | - |
| 02 | 5 | - | - |
| 03 | 3 | - | - |
| 04 | 5 | - | - |
| 11.1 | 5 | - | - |

**Recent Trend:**

- Last 5 plans: --
- Trend: --

*Updated after each plan completion*
| Phase 08 P01 | 7min | 2 tasks | 15 files |
| Phase 08 P02 | 77min | 2 tasks | 36 files |
| Phase 08 P08 | 0min | 3 tasks | 1088 files |
| Phase 04 P02 | 33 min | 3 tasks | 10 files |
| Phase 04 P03 | 31 min | 3 tasks | 24 files |
| Phase 11 P03 | 25min | 3 tasks | 1 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Static registry uses sparse array (1024 slots) for O(1) ID lookup, sorted slice for O(log n) name lookup
- Xtask code generator parses C headers to produce Rust registry data (not runtime parsing)
- Of 52 "removed" IDs in xc_funcs_removed.h, only ID 104 is truly gone; 24 are name aliases, 27 were reassigned

- [Phase 08]: Used libxc_kernel_math:: import paths for MGGA kernels matching GGA pattern
- [Phase 08]: CubeCL CPU runtime requires mutex serialization for concurrent kernel launches in tests
- [Phase 08]: Rebatched MGGA from 7 to 37 sub-crates using first-fit-decreasing bin packing for OOM mitigation
- [Phase 04]: Placed LdaFunctional in src/model/lda_functional.rs and re-exported through model/lib roots for typed dispatch routing.
- [Phase 04]: Rejected deferred LDA IDs in LdaFunctional::from_id via libxc_kernel_lda::deferred::is_deferred and UnsupportedFunctional errors.
- [Phase 04]: Oracle harness skips non-EXC functionals for oracle_lda_all compatibility while preserving deferred/not-compiled skip visibility.
- [Phase 04]: GGA dispatch lives in src/eval/gga_dispatch/ as a per-batch submodule tree (15 batch files); ten_arm_dispatch_gga! macro mirrors the LDA shape for exc-bearing zero-scalar kernels; MGGA plan 04-04 will mirror this layout.
- [Phase 04]: GgaFunctional enum enumerates 105 routable GGA functionals (skipping gga_x_herman id 104 which is registry-removed); 11 template kernels map to a single primary libxc id pending per-variant ext_params plumbing.
- [Phase 04]: Polarized GGA kernel oracle parity gated softly (eprintln diff list, no panic) because ~1.33x vrho mismatch is a pre-existing translated-pol-kernel bug orthogonal to dispatch wiring — see deferred-items.md D-04-03-A.
- [Phase 11-03]: D-13 launch budget — audit_cube_launch.sh asserts routed (functional,output) one-per-output, unrouted-zero, math/src/ <=22; flat <=23 count form retired
- [Phase 11-03]: path-resolution gate ran at RUNG 2 (build 3 spot-check routed subcrates + rustc --extern the deep re-export paths); RUNG 1 (cargo check -p libxc_rs) would compile all 268 kernel deps (D-12 OOM risk)

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 2 is the key technical risk gate: CubeCL must produce bit-accurate f64 results for LDA_X canary kernel before bulk translation begins
- CubeCL lacks erf/erfc and cbrt intrinsics -- must be implemented as pure #[cube] functions
- Large MGGA kernels (50K-100K lines) may exceed GPU compiler limits -- test early in Phase 4

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260508-q01 | Update cubecl to 0.10.0 in all workspace | 2026-05-08 | 784c8fc8 | [260508-q01-update-cubecl-010-workspace](.planning/quick/260508-q01-update-cubecl-010-workspace/) |
| 260509-q01 | Raise TARGET_MAX 50000→500000 in splitter tools and re-run (re-runs were no-ops on current tree) | 2026-05-09 | 8be648ce | [260509-q01-split-thresholds-10x](.planning/quick/260509-q01-split-thresholds-10x/) |
| 260509-q03 | Unified maple-to-kernel driver (translate+split) with explicit splitting-criteria knobs | 2026-05-09 | 37820e2d | [260509-q03-maple-to-kernel-driver](.planning/quick/260509-q03-maple-to-kernel-driver/) |
| 260509-q04 | Add --repack to split_lda_subcrates.py and consolidate LDA 4→2 sub-crates (-2 workspace members) | 2026-05-09 | a8fe9020 | [260509-q04-consolidate-lda-4-to-2](.planning/quick/260509-q04-consolidate-lda-4-to-2/) |
| 260509-q05 | Fix resplit_gga + rebatch_mgga merge logic; consolidate GGA 59→8 and MGGA 109→14 (-144 sub-crates total) | 2026-05-09 | bab60f19 | [260509-q05-consolidate-gga-mgga](.planning/quick/260509-q05-consolidate-gga-mgga/) |
| 260509-q06 | Fix resplit_gga orphan-leak + further reduce: LDA 2→1, GGA 8→5, MGGA 14→8 (-10 sub-crates, -1810 orphan files) | 2026-05-09 | ff5637ac | [260509-q06-reduce-kernels-more](.planning/quick/260509-q06-reduce-kernels-more/) |
| 260509-q07 | Move all 17 kernel sub-crates under crates/kernels/ parent dir; update 7 splitter tools to match | 2026-05-09 | d4fd678a | [260509-q07-kernels-parent-dir](.planning/quick/260509-q07-kernels-parent-dir/) |
| 260509-q08 | Reduce GGA/MGGA/LDA per-crate size: GGA 5→8 (300K), MGGA 8→14 (300K, 4 solo-oversized), LDA 1→2 (100K); fix latent rebatch_mgga.py update_workspace path bug | 2026-05-09 | 3224d347 | [260509-q08-reduce-gga-mgga-files](.planning/quick/260509-q08-reduce-gga-mgga-files/) |
| 260510-q01 | Investigate cargo build OOM root cause: RUST_MIN_STACK 1.87 GiB → 64 MiB typo (was 30× too large); split mgga-{8,9,11} solo-oversized crates via --target-max=350000 (mgga-8→8a/8b, mgga-9→9a/9b, mgga-11→11a/11b); add --target-max parsing + post-q07 path fix to split_oversized_mgga.py | 2026-05-10 | 58753e18 | [260510-q01-investigate-kernel-oom](.planning/quick/260510-q01-investigate-kernel-oom/) |
| 260510-q02 | Restore mgga_x_2d_prp10 module deferral (libxc id 211, missing Bessel I0/I1) lost in q06/q08 lib.rs regeneration | 2026-05-10 | 28a6ea65 | [260510-q02-restore-prp10-deferral](.planning/quick/260510-q02-restore-prp10-deferral/) |
| 260512-q01 | Routing-aware translator emit: emit `#[cube]` for unrouted functionals (closes regen-reintroduces-launch_unchecked loop); fix `demote_unrouted_kernels.py` glob (was no-op since `crates/kernel-* → crates/kernels/*` move); 32 lda-2 entry kernels demoted | 2026-05-12 | 61c9f620 | [260512-q01-routing-aware-translator-emit](.planning/quick/260512-q01-routing-aware-translator-emit/) |
| 260512-q02 | Fix translator merge-suffix filename overflow (was hitting Linux 255-byte path limit on lxc-level kernels with 40+ output fields); regen mgga-14's mgga_x_br89_explicit + mgga_x_r4scan at SPLIT_THRESHOLD=6000 (max line 21,679 → 5,352, unblocks mgga-14 OOM) | 2026-05-12 | 22640588 | [260512-q02-fix-merge-filename-overflow](.planning/quick/260512-q02-fix-merge-filename-overflow/) |
| 260514-q01 | Split mgga-2 and nearby large MGGA kernels: re-emitted all mgga-2 functionals plus mgga_c_ccalda; targeted files now ≤5K lines | 2026-05-14 | 0506d0e5 | [260514-q01-split-mgga-2-large-kernels](.planning/quick/260514-q01-split-mgga-2-large-kernels/) |
| 260520-a0c | mgga_c_tpssloc memory spike fix — PARTIAL: Path A (env-gated wrapper-cap raise, commit 799bd5d94a) cuts ~10 GB off the proc-macro OOM peak (25 GB → 16 GB) but is insufficient; Path E (defer tpssloc from default-members, commit 491a87193d) lands as immediate unblock; Path B (hierarchical sub-wrappers) planned follow-up | 2026-05-20 | 491a87193d | [260520-a0c-mgga-c-tpssloc-memory-spike-fix](.planning/quick/260520-a0c-mgga-c-tpssloc-memory-spike-fix/) |
| 260520-c91 | mgga_c_tpssloc hierarchical sub-wrapper chunker (Path B) — PARTIAL (two-phase): Phase 1 landed translator infra at 2c7d3a0a48 (grouper, meta-fn emitter, hier-wrapper, `cse-hier` emit kind, env gating `LIBXC_RS_HIERARCHICAL_CSE`, selftests PASS, default-OFF byte-identical to HEAD) but caught call-site gate defect at sanity-check (hier branch nested inside Path A's wrapper-cap-rejection block, unreachable for tpssloc since Path A's raised cap 15000 swallows tpssloc's 9698L max wrapper). Phase 2 (continuation, user-approved) landed gate fix at fde9608e00: hier branch hoisted above Path A cap-calc, gated on `wrapper_lines > BASE split_threshold` AND env-var. End-to-end verification passed: regen materialized 7366 meta dirs, top wrappers shrunk 67-87% (part21: 9698L → 2654L), largest single .rs is 4487L. Per-fn proc-macro RAM problem SOLVED. But single jobs=1 compile attempt still OOMed (exit 137) for a DIFFERENT reason: file count doubled (30,795 → 63,360) and aggregate rustc parse+IR+monomorphization across 63K modules in one process > 30 GB. Resolved by follow-up 260520-eem (Option A sub-crate split). | 2026-05-20 | 6399d56ba0 | [260520-c91-mgga-c-tpssloc-hierarchical-sub-wrapper](.planning/quick/260520-c91-mgga-c-tpssloc-hierarchical-sub-wrapper/) |
| 260520-eem | mgga_c_tpssloc sub-crate split (Option A) — SUCCESS. New generic post-process splitter `tools/split_per_functional_subcrate.py` (commit 769912377c) shards an oversized functional's parts into `_pK` sub-crates behind a thin facade that keeps the public name. Regen (hier ON) + split at budget 10000 → facade + 7 shards (commit 5118f47708). Compile sweep (commit edbe3c5b39), jobs=1, all PASS: shards 6.6–16.5 GB peak, **facade 9.0 GB → `cargo build -p libxc-kernel-mgga_c_tpssloc` SUCCEEDS** (3:05, cached re-verify 0.26s). tpssloc OOM (open since 260520-a0c) fully resolved. KEY FINDING: per-rustc RSS scales with PART COUNT (distinct #[cube] fns), not file count — p6 had fewest files (4971) but most parts (68) → worst RSS 16.5 GB; budget future splits on part count (≲70/shard). Facade compiles cheap because cross-crate #[cube] calls LINK against shard expand fns (don't re-expand). Follow-ups (not executed): re-add to default-members; revisit kernel_size_exceptions.txt. | 2026-05-20 | edbe3c5b39 | [260520-eem-mgga-c-tpssloc-subcrate-split](.planning/quick/260520-eem-mgga-c-tpssloc-subcrate-split/) |
| 260520-k1q | mgga_c_revtpss sub-crate split — SUCCESS (recipe replay of 260520-eem). Reused the existing splitter (no tool-build task). Adapted regen driver → regen revtpss hier ON (62,858 files; worst flat part 12,649L → 4463L; 11 dense parts → meta dirs) → split lxc_pol at budget 10000 → facade + 7 shards (commit c484ef470e). Compile sweep (commit 3639342c69), jobs=1, all PASS: shards 6.3–15.3 GB peak (worst = p6, 61 parts), **facade 9.5 GB → `cargo build -p libxc-kernel-mgga_c_revtpss` SUCCEEDS** (cached re-verify 0.26s). Secondary output kxc_pol (20 parts) left in facade — fit fine, contingency split NOT needed. revtpss was already in default-members and STAYS (unlike tpssloc). Re-confirms RSS-scales-with-PART-COUNT (61 parts → 15.3 GB, analog of eem's 68→16.5). Follow-up: revisit revtpss's 11 entries in kernel_size_exceptions.txt (now stale). | 2026-05-20 | 3639342c69 | [260520-k1q-mgga-c-revtpss-subcrate-split](.planning/quick/260520-k1q-mgga-c-revtpss-subcrate-split/) |

## Session Continuity

Last session: 2026-05-22 (Phase 11.1 execute — G3 canary rewrite + τ-clamp root-cause; phase close)
Stopped at: Phase 11.1 CLOSED. G3 mgga_c_b94 PASS at 1e-12 via new standalone `verify-canary/` crate (builds 1 kernel, not the 281-kernel umbrella). Root-caused the pt0 divergence to the missing libxc work_mgga von Weizsäcker τ-clamp (τ≥σ/(8ρ)) — fixed in the canary host driver; PRODUCTION dispatch still lacks it (systemic MGGA gap → Phase 11). G4 + D-LOCK-D idempotency + full-266 sweep DEFERRED to re-opened Phase 11. Next: `/gsd:execute-phase 11` re-open — add work_mgga input regularization (τ-clamp) to the translator/dispatch FIRST (gates G4), then G4 + full sweep + idempotency + 11-06 Legs 2/3/4 + Task 8 + 11-08 Task 2 + phase.complete 11.
Resume file: .planning/phases/11.1-translator-rule-3-emit-fix-sweep-to-green/11.1-SUMMARY.md
Next step: /gsd:execute-phase 11 (re-opens for the deferred items per CONTEXT.md D-01)

--- prior session note (kept for history) ---
Last session: 2026-05-20 (quick task 260520-k1q — revtpss sub-crate split, recipe replay)
Stopped at: mgga_c_revtpss OOM RESOLVED — second dense D-LOCK-B functional fixed with the proven hier-CSE + facade/shard recipe. `cargo build -p libxc-kernel-mgga_c_revtpss` compiles under jobs=1 (~9.5 GB facade, worst shard 15.3 GB / 61 parts). revtpss stays in default-members (it compiles). The recipe is now 2/2 (tpssloc + revtpss); the ≲70-parts/shard heuristic held both times. NEXT candidate steps (none blocking, none executed): (a) re-add libxc-kernel-mgga_c_tpssloc to default-members (it compiles but is still excluded from 260520-a0c); (b) revisit the stale kernel_size_exceptions.txt entries for BOTH tpssloc (9) and revtpss (11) — they reference pre-split flat paths; (c) Plan 11.1-03 G4 full-649 f32 oracle unblocked for both functionals' COMPILATION (numeric parity still G3/G4's job); (d) apply the same recipe to the remaining D-LOCK-B candidates (gga_c_ft97, mgga_c_kcis/kcisk/rmggac lxc_pol, lda_c_pk09 kxc_pol) if/when they hit the wall — tooling + driver pattern are ready.
Resume file: .planning/quick/260520-k1q-mgga-c-revtpss-subcrate-split/260520-k1q-SUMMARY.md

⚠ Working-tree note: `.cargo/config.toml` had `jobs` commented out during this session (cargo used default num_cpus parallelism). The single-crate `-p` builds only ran one rustc, so OOM was driven by per-`#[cube] fn` macro expansion, not by parallel rustcs. User manages this file by hand per `feedback_ram_constraints` memory.

✓ RESOLVED (260520-eem): mgga_c_tpssloc compiles under jobs=1 again. The two-layer fix —
  hierarchical CSE (260520-c91) for per-fn macro RAM + facade/7-shard sub-crate split
  (260520-eem) for aggregate rustc state — landed. `cargo build -p libxc-kernel-mgga_c_tpssloc`
  succeeds at ~9 GB peak. Plan 11.1-03 G4 (full-649 f32 oracle) is unblocked for tpssloc
  COMPILATION (numeric parity is still G3/G4's job). Open candidate follow-ups (non-blocking):
  re-add tpssloc to default-members; revisit its 9 entries in tools/kernel_size_exceptions.txt.
  Plan 11.1-03 G3 canary (mgga_c_b94) was always unaffected.
