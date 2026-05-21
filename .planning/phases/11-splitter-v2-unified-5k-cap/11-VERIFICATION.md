---
status: gaps_found
phase: 11-splitter-v2-unified-5k-cap
generated: 2026-05-22
source: synthesized gap index (NOT a fresh gsd-verifier run)
provenance:
  - .planning/phases/11-splitter-v2-unified-5k-cap/11-FINAL-METRICS.md (phase-11 PARTIAL end-state)
  - .planning/phases/11.1-translator-rule-3-emit-fix-sweep-to-green/11.1-SUMMARY.md (hand-back §"Phase 11 Hand-Back Per D-01")
  - .planning/phases/11.1-translator-rule-3-emit-fix-sweep-to-green/11.1-03-G4-DEFERRAL.md
  - .planning/phases/11.1-translator-rule-3-emit-fix-sweep-to-green/11.1-02-IDEMPOTENCY-DEFERRAL.md
  - .planning/ROADMAP.md (Phase 11 + Phase 11.1 closure-status sections)
hardware: 30 GB RAM, 16 cores; .cargo/config.toml pinned jobs=1 (DO NOT EDIT — user caps by hand)
---

# Phase 11 — Verification / Gap Index (re-open)

This is **not** a fresh verifier run. Phase 11 closed PARTIAL (`ac9729a51d`) and never reached
`verify_phase_goal`; Phase 11.1 (the translator Rule-3 emit fix) then closed narrow and **handed
5 closure items back to a re-opened Phase 11** (11.1-SUMMARY §"Phase 11 Hand-Back Per D-01").
This file consolidates those into the standard gap format so `/gsd:plan-phase 11 --gaps` has a
single entry point. The authoritative detail lives in the `provenance` docs above.

## Already Verified — DO NOT re-plan (structural goals MET in 11 / 11.1)

- **D-10a clean-slate restructure** — 266 per-functional subcrates; no `lda-N`/`gga-N`/`mgga-N` numbered parents (ROADMAP SC #1).
- **5K line cap** — splitter v2 + per-functional subcrate split (`split_per_functional_subcrate.py`) (ROADMAP SC #2/#3).
- **D-13 launch budget invariant** — 1654 routed / 0 unrouted / 22 math/src (ROADMAP SC #7).
- **Dispatch tree regen** — `src/eval/{gga,mgga}_dispatch/` resolves against post-collapse façade (ROADMAP SC #8).
- **Translator Rule-3 chunk-body emit fix** — P1/P2/P3 (`tools/translate_v2/`, 11.1 Plans 01/02).
- **11-PATTERN.md Rules 1-10**, dual-precision (f64/f32) infra, math/ generic `<F: Float>` refactor.
- **G3 mgga_c_b94 f64 parity at 1e-12** — PASS via standalone `verify-canary/` crate (builds 1 kernel).

## Gaps (re-opened Phase 11 scope) — gap_closure plans needed

Ordered by dependency. G-1 is the critical path (gates G-2).

### G-1 — work_mgga input regularization (NEW, systemic) — CRITICAL PATH
- **What:** Add libxc `work_*.c` input preprocessing to the translator/dispatch — the von
  Weizsäcker τ-clamp `τ ≥ σ/(8ρ)` at minimum (investigate other `work_*.c` thresholds too).
- **Why:** Rust production dispatch passes raw τ; libxc clamps it. Any MGGA point with τ<τ_W
  diverges at the %-level — NOT an f32-precision or tolerance-override issue. Found while
  root-causing the G3 pt0 2.3% divergence (fixed in the canary host driver only; production
  dispatch still lacks it). Gates a meaningful G-2.
- **Source:** 11.1-03-G4-DEFERRAL.md §"Additional finding"; memory `project_translator_missing_workmgga_tau_clamp`.
- **Depends on:** none. **Cost:** design + implementation; no full-tree build needed to develop.
- **Done when:** the clamp (and any other identified work-driver preprocessing) is applied in the
  production MGGA evaluation path (translator/dispatch, not just the canary driver), with a
  direct-call unit/parity check (single-kernel, no umbrella build) proving a τ<τ_W point now matches libxc.

### G-2 — memory-safe full-649 oracle path + G4 f32 sweep — DEPENDS ON G-1
- **What:** (a) Build a memory-safe way to evaluate the full-649 oracle WITHOUT the OOMing
  monolithic all-281-kernel build (deferral proposes: feature-gate `libxc_rs` kernel deps by
  family → run each family's oracle against only its kernels, in chunks). (b) Run G4
  (`LIBXC_RS_F32=1` oracle sweep) with per-functional tolerance overrides bounded by the **1e-3
  hard ceiling** (D-12/D-24).
- **Why:** G4 is ROADMAP SC #5 / 11-08 Task 3 / D-24. Inherently exercises every functional through
  `libxc_rs::eval::dispatch_*` which statically links the whole kernel tree → OOMs at jobs=3,
  multi-hour+OOM-risk at jobs=1 on this 30 GB box. The family-chunked path is the unblocking design.
- **Source:** 11.1-03-G4-DEFERRAL.md.
- **Depends on:** G-1 (without τ-clamp, MGGA oracle fails widely for non-precision reasons).
- **Cost:** HIGH / OOM-prone — must be chunked + jobs=1; plan must NOT assume a single monolithic build.
- **Done when:** full-649 f32 oracle runs to completion via the chunked path; all functionals within
  per-functional tolerance ≤ 1e-3 ceiling, with any residual failures attributed (not silently passed).

### G-3 — full-266 compile sweep (G1/G2 only ran 50-sample)
- **What:** Drive `python3 tools/batched_compile_sweep.py` to `VERDICT: ALL_OK` across all 266
  on-disk subcrates at **f64 and f32** (LDA 43 + GGA 131 + MGGA 92). 11.1 verified only a
  50-subcrate representative sample.
- **Why:** ROADMAP SC #4 / SPEC-11-R4 (clean per-`-p` build of all subcrates). This IS the per-`-p`
  compile ENTRY-GATE codification (memory `project_phase11_structural_without_compile`). Note:
  P1-grep still shows ~1114 hits concentrated in `gga_c_gaploc` (now concrete-f64 → likely benign,
  but UNVERIFIED at full-tree scope) — this sweep is what confirms it.
- **Source:** 11.1-SUMMARY hand-back item 4; ROADMAP Phase 11.1 closure SC #3/#5.
- **Depends on:** none (independent of G-1). **Cost:** ~22–44h wall-clock at jobs=1 — must be
  RESUMABLE/chunked (manifest-driven), runnable in segments; do NOT require one unbroken session.
- **Done when:** `batched_compile_sweep.py` manifest shows all 266 subcrates PASS at both precisions
  (or any exception is explicitly excepted with rationale, per the splitter-floor reality).

### G-4 — D-LOCK-D idempotency proof
- **What:** Prove translator idempotency: `translate --family all` → `git diff --stat
  crates/kernels/` zero-diff for the **264 non-sharded** functionals. Handle `mgga_c_tpssloc` +
  `mgga_c_revtpss` (hier-CSE + `split_per_functional_subcrate.py` shards) separately — either
  exclude-and-document the split as a deterministic post-process, or make the split pipeline
  reproducible end-to-end and prove zero-diff including shards.
- **Why:** ROADMAP SC #6 (pipeline idempotent). A vanilla `translate --family all` re-emits the two
  sharded functionals FLAT → committing it would clobber the tpssloc/revtpss OOM fix
  (`project_tpssloc_oom_resolution`). The proof must respect the split pipeline.
- **Source:** 11.1-02-IDEMPOTENCY-DEFERRAL.md.
- **Depends on:** none (but easiest after G-3 confirms the tree compiles). **Cost:** moderate;
  regen is translator-only (no cargo build) → low OOM risk.
- **Done when:** zero-diff demonstrated for the 264, and tpssloc/revtpss handled per chosen option
  with the OOM-fix layout preserved.

### G-5 — closure items + phase.complete (GATED on G-1..G-4)
- **What:** (a) 11-06 Legs 2/3/4 + Task 8 — rewrite 11-06-SUMMARY PARTIAL→COMPLETE honestly once
  gates are green. (b) 11-08 Task 2 — CLAUDE.md (D-03a) + ROADMAP success-criteria wording
  (D-12/D-13), delete the 5 obsolete tools, remove the `LIBXC_RS_BYPASS_DEFERRED` env-gate. (c)
  Invoke `phase.complete 11` (note: gsd-sdk `phase.complete` handler is unimplemented here — closure
  is ROADMAP/STATE-effective, done manually; see commit `c4480dd881`).
- **Why:** Final phase close; honest SUMMARY can only be written after G-1..G-4 land.
- **Source:** 11.1-SUMMARY hand-back item 5.
- **Depends on:** G-1, G-2, G-3, G-4. **Cost:** low (docs/cleanup) — but BLOCKED until the rest pass.
- **Done when:** SUMMARYs reflect reality, obsolete tooling/env-gate removed, ROADMAP/STATE mark
  Phase 11 complete.

## Hard constraints for ALL gap plans (machine reality)

1. **jobs=1, inline sequential.** `.cargo/config.toml` is hand-pinned to `jobs=1` — DO NOT edit it.
   No plan may assume parallel cargo or a multi-job build.
2. **No monolithic all-kernel builds.** Anything touching the `libxc_rs` umbrella pulls all 281
   kernels → OOM on 30 GB. Single-functional checks MUST use the direct-kernel-call escape hatch
   (the `verify-canary/` pattern), not `libxc_rs::eval::dispatch_*`.
3. **Per-`-p` cargo compile is an ENTRY gate, not an exit gate** (memory
   `project_phase11_structural_without_compile`) — declare nothing green without it.
4. **Long-running gaps (G-2, G-3) must be resumable/chunked** so the user can run them in segments
   and pace via `--wave N`.
