---
phase: 11-splitter-v2-unified-5k-cap
plan: 13
subsystem: closure
tags: [g5, closure, phase-complete, f64-only, f32-milestone, mgga-parity, d-11, phase-12]

requires:
  - phase: 11-splitter-v2-unified-5k-cap (plans 09/10/11/12/14)
    provides: "the f64 gap evidence — τ-clamp (G-1), 305-pkg f64 compile sweep (G-3), idempotency SATISFIED (G-4), family-chunked f64 oracle (G-2), umbrella 0.10 ABI (G-6)"
provides:
  - "Phase 11 closed COMPLETE (ROADMAP/STATE-effective; manual — gsd phase.complete handler unimplemented here)"
  - "Inverted f32 wording corrected: CLAUDE.md keeps 'f64 only' + f32-is-a-milestone bullet; ROADMAP SC #4 = 305 (NOT a phase gate), SC #5/G4 = f32 milestone follow-up"
  - "NEW ROADMAP 'Phase 12 — MGGA f64 Parity' entry recording the 6 routed exc failures (RECORDED, not fixed)"
  - "D-11 restored: LIBXC_RS_BYPASS_DEFERRED removed — deferred MGGA ids unconditionally rejected in production"
  - "5 obsolete numbered-subcrate-era tools deleted"
affects: [phase-12, milestone-v1.0]

tech-stack:
  added: []
  patterns:
    - "Honest phase close: COMPLETE SUMMARY cites real f64 evidence; f32 leg DISPOSITIONED-not-passed; FAILED-iteration history preserved (AP-5)"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-13-SUMMARY.md
  modified:
    - CLAUDE.md
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - src/model/mgga_functional.rs
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-06-SUMMARY.md
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-FINAL-METRICS.md

key-decisions:
  - "Task 1 human-verify checkpoint: user APPROVED closure (all 5 prior gaps passed at f64; Phase 12 placement accepted as a new dedicated ROADMAP entry)."
  - "DEVIATION from plan text: roster is 305, NOT 280 — the 11-10 sweep itself grew it by sharding rmggac/tpss/kcisk (25 _pK shards). All '280' references in the plan were written to 305 (the gate is roster-relative: count == len(build_roster()))."
  - "f32 wording INVERTED per the 2026-05-23 f64-concrete decision: kept 'f64 only', did NOT introduce 'f32 secondary 1e-6'; f32 oracle = milestone follow-up, not a Phase-11 gate."
  - "regen_phase09.py deletion: its only mentions in retained tools (maple_to_kernels.py, translate_gga.py) are docstrings noting the new pipeline REPLACED it — no live callers, safe to delete."

patterns-established:
  - "Closure honesty: the COMPLETE rewrite + the Phase 12 entry record the 6 MGGA f64 residuals rather than burying them; the f32 leg is explicitly dispositioned-not-passed."

requirements-completed: []

duration: ~closure session (post-11-10)
completed: 2026-05-25
---

# Phase 11 / Plan 13 (G-5): Closure — Summary

**Phase 11 is closed COMPLETE: all six gaps (G-1..G-6) landed at f64, the inverted f32 wording is corrected (f64-only kept; f32 = milestone follow-up), the 6 MGGA f64-parity residuals are recorded as a new Phase 12, and the obsolete tooling + the production deferred-bypass are removed.**

## Task 1 — human-verify checkpoint (PASSED)

User confirmed all prior gaps passed at f64 and approved the Phase 12 placement:
- **G-1** (11-09): von Weizsäcker τ-clamp in PRODUCTION `mgga_dispatch`; canary parity PASS 1e-12.
- **G-2** (11-12): family-chunked f64 oracle ran — LDA ✓, GGA ✓; 6 MGGA exc attributed + routed (not silently passed).
- **G-3** (11-10): full-roster f64 compile sweep VERDICT ALL_OK — merged manifest **305 records** == `len(build_roster())`, 0 fail, 0 pass=-1.
- **G-4** (11-11): idempotency proof SATISFIED.
- **G-6** (11-14): umbrella cubecl-0.10 launch-ABI migration (`cargo check -p libxc_rs --lib` EXIT 0).
No f32-oracle pass was claimed anywhere.

## Task 2 — cleanup (f32 wording INVERTED)

- **CLAUDE.md**: kept `Precision: f64 only; … <= 10^-12`; ADDED an `f32 support` bullet = MILESTONE-scale follow-up (kernels f64-concrete). Did NOT introduce "f32 secondary 1e-6". Operation-order/tech-stack bullets untouched.
- **ROADMAP**: SC #4 → record count == `len(build_roster())` = **305**, `cargo build --workspace` NOT a phase gate; SC #5 → f64 gate, full-649 f32 = MILESTONE follow-up not a gate; SC #7 → per-design fan-out budget. Checklist 11-09..15 flipped `[x]`; Phase 11 status COMPLETE + Progress row (15/15); **NEW "Phase 12 — MGGA f64 Parity"** entry (top-level checkbox + detail section) naming all 6 functionals (`mgga_x_th` 2.0e-1, `mgga_x_2d_js17` 1.1e-2, `mgga_c_cs` 9.2e-3, `mgga_x_pkzb` 3.7e-3, `mgga_x_pbe_gx` 1.5e-3, `mgga_x_tm` 9.2e-4) + root cause (per-functional translation + residual `work_mgga` regularization; τ-clamp @ mgga_dispatch/mod.rs:280-282 is NOT the cause). RECORDED only.
- **Deleted 5 obsolete tools** (`split_oversized_kernel`, `split_oversized_mgga`, `split_mgga_7_kcis`, `rebatch_mgga`, `regen_phase09`) — only inter-references + docstring mentions, no live callers. Retained: `batched_compile_sweep`, `refactor_helpers_generic`, `symbol_class_matrix.rs`, `split_per_functional_subcrate`.
- **Removed `LIBXC_RS_BYPASS_DEFERRED`** from `src/model/mgga_functional.rs` → deferred MGGA ids unconditionally rejected (D-11). `grep -r LIBXC_RS_BYPASS_DEFERRED src/` = 0. Light gate `cargo check -p libxc_rs --lib --no-default-features --jobs 1` EXIT 0.

## Task 3 — honest SUMMARY + finalize + close

- **11-06-SUMMARY** PARTIAL → COMPLETE via a Closure Update citing the f64 evidence (Legs 1/2/4 closed; Leg 3 f32 DISPOSITIONED-not-passed); Session-1 + FAILED-iteration history preserved verbatim (AP-5).
- **11-FINAL-METRICS** Closure Update: F32 rows = milestone (not passed), manifest row = 305/ALL_OK, idempotency = 11-11 SATISFIED.
- **ROADMAP/STATE**: Phase 11 marked COMPLETE (manual; handler unimplemented).

## Deviation noted

The plan was written before the 11-10 sweep sharded 3 oversized MGGA functionals; the roster grew **280 → 305**. Every "280" in the closure was written to the live `len(build_roster())` = 305. The gate is roster-relative, so this is consistent, not a goal change.

## Result

Phase 11 (Splitter v2 — Unified Kernels, 5K cap) is **COMPLETE**. Milestone v1.0 remaining: Phases 5, 6, 7. The recorded MGGA f64-parity work is available as Phase 12 when prioritized.
