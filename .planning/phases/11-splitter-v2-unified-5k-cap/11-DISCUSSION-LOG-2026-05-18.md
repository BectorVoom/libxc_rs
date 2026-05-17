# Phase 11: Splitter v2 — Discussion Log (2026-05-18 Update)

> **Audit trail only.** Decisions are captured in CONTEXT.md. This log preserves the clarifications and anti-pattern analysis from the replan checkpoint.

**Date:** 2026-05-18
**Phase:** 11-splitter-v2-unified-5k-cap
**Session Type:** Discuss-phase update (replan checkpoint); no new gray areas — confirmation of prior decision + blocking anti-pattern codification

---

## Prior Decision: Option A vs C for D-02 (Confirmed 2026-05-18)

| Topic | Context | Decision |
|-------|---------|----------|
| **ABI for `<F: Float>` chunks calling concrete-f64 helpers** | 11-04 Task 1A revealed that Option A (refactor 38 helpers to generic) requires fixing 11 files with systematic syntax errors in Phase 2 automated refactoring scripts. | **LOCKED: Option C (cast-at-call-site in translator).** Keep helpers concrete-f64; translator emits `cast_into_f(helper_f64(cast_from_f(x)))` boilerplate at call sites. Acceptable for auto-generated code. Unblocks immediately. |
| **Rationale** | Option A: multi-hour script debugging + uncertain outcome. Option C: 1–2 day translator spike + proven zero-cost abstractions (§6, `cubecl_macro_fanout_manual.md`). | **Chosen for speed and lower risk.** Script debugging is itself an anti-pattern; translator-side boilerplate is acceptable in an auto-generated tree. |
| **Spike scope** | D-14: Validate D-02 via `mgga_c_b94` canary. Three gates: compile, parity, idempotency. | Spike now validates **Option C only** (not A-vs-C race). Same gate structure, ~1–2 day time-box. |

---

## Blocking Anti-Pattern Analysis (NEW 2026-05-18)

The `.continue-here.md` handoff documented three **blocking anti-patterns** that broke prior plan iterations. This session analyzed each for structural mitigations:

### AP-1: Re-executing without replanning ⚠️ **BLOCKING**

| Aspect | Analysis | Structural Fix |
|--------|----------|-----------------|
| **What it is** | Running `/gsd-execute-phase 11` against stale 11-04..06 plans. Architectural mismatch (D-02 unsolved) causes compile loops. | Plans 11-05..08 MUST have **entry-gate criteria**: canary `cargo build -p libxc-kernel-<func>` before any structural work. Reverses gate order: compile-first, not compile-after. |
| **Manifestation** | 11-01..03 claimed structural success without per-`-p` compile gates. 11-04's first per-`-p` gate surfaced the D-02 helper-layer incompatibility. | Mechanism: Every plan task touching kernel emission MUST have a `cargo build -p <canary>` entry gate that MUST GREEN before proceeding. Explicit gate failure = plan halt, not silent loop. |
| **Prevention** | This session's decision locks D-02 (Option C) upfront, eliminating the "architectural blocker unresolved" state. 11-05's spike validates Option C before 11-06..08 regen. | Planner: Add pre-task compile-check to every plan. Task structure: "compile check mgga_c_b94 → passes → proceed to translator update → regen". |

### AP-2: Modifying `.cargo/config.toml` ⚠️ **BLOCKING**

| Aspect | Analysis | Structural Fix |
|--------|----------|-----------------|
| **What it is** | Changing `[build] jobs`, `[env] RUST_MIN_STACK`, or `target-dir` in `.cargo/config.toml`. Load-bearing for D-07/D-08/D-09. | Pre-flight check task: verify file has `jobs = 1`, `RUST_MIN_STACK = 67108864`, out-of-tree `target-dir`. Explicit ban in all plan tasks: "MUST NOT modify `.cargo/config.toml`". |
| **Manifestation** | User manually capped `jobs` during the session after a temporary override. Uncapped builds OOM (exit 137) on 30GB machine. Committed `jobs = 1` is the source of truth. | Even a temporary task override (via `CARGO_BUILD_JOBS` env var or `--jobs N` flag) can cascade into OOM if the user isn't watching. |
| **Prevention** | All three D-07/D-08/D-09 values are now **immutable in the replan**. Phase 11 trusts `.cargo/config.toml` as authoritative. No env-var overrides. | **Plan task 11-05 (first task): "Pre-flight check: verify `.cargo/config.toml` has `jobs = 1`, `RUST_MIN_STACK = 67108864`, `target-dir = .cache/cargo-target`. FAIL the task if any value is wrong."** This catches accidental edits and env-var leakage. |

### AP-3: Hand-editing generated kernel files ⚠️ **BLOCKING**

| Aspect | Analysis | Structural Fix |
|--------|----------|-----------------|
| **What it is** | Manually patching `crates/kernels/{lda,gga,mgga}/*.rs` to fix compile errors. Violates D-LOCK-D idempotency: hand edits don't survive regen. | Every kernel-tree compile error MUST be fixed in `tools/translate_v2/` first. Pattern: "identify root → modify translator → regen → verify gate". **Explicit ban in plan task descriptions.** |
| **Manifestation** | 11-04 produced "Mul<F> for {float}" errors. Temptation: hand-edit generated chunks. Wrong: edits disappear on next regen, breaking idempotency. Right: fix `_wrap_f64_literals` or cse.py → regen. | D-LOCK-D is the load-bearing invariant for Phase 11. Idempotency check: run translator twice, `git diff` must be empty. Hand edits fail this check. |
| **Prevention** | Option C itself is a structural answer: cast-wrapper emission is **translator-side only**; no generated-code edits needed. No hand-editing temptation. | **Plan task template:** "If a generated file has a compile error, stop. DO NOT edit `crates/kernels/`. Instead: (1) Identify root in `tools/translate_v2/`. (2) Fix translator. (3) Regen. (4) Verify gate passes. (5) Commit translator fix + regen output atomically." |

---

## Prior Warnings Reaffirmed

| AP | Level | Status |
|----|----|--------|
| **AP-4** | warning | Preserve commit `5c379dc25` (q01 emit fixes). Option C doesn't replace `_wrap_f64_literals` regex; both literal-wrap + cast-wrapper coexist in cse.py. |
| **AP-5** | warning | 11-01..03 SUMMARYs are not stale. Their deliverables (audit tools, baseline, dispatch audit, 266-subcrate regen) survive the Option C decision. Replan reframes 11-04..08 scope, not 11-01..03. |
| **AP-6** | refactored as AP-1 | "No structural completion without per-`-p` compile gates" is now the **entry-gate pattern** of AP-1. Compile-first, then claim structural progress. |

---

## Confirmed: Next Steps (11-05..08 Replan)

The replan structure is **unchanged** in shape, but now targeting **Option C** instead of the uncertain A-vs-C race:

1. **11-04 (retroactive SUMMARY only)** — commit `39eb75f93` (D-05 structural fix). Pause documented.
2. **11-05 (D-14 spike for Option C)** — Translate `mgga_c_b94` with cast-wrapper emission. Compile + parity + idempotency gates. 1–2 day time-box.
3. **11-06 (D-16 translator update)** — Wire cast-wrapper emit into cse.py AST pass. Fix `from_raw_parts` API drift in `crates/kernels/math/tests/`.
4. **11-07 (D-15 entry-gate on full regen)** — Regen 266 subcrates. Compile-first: mgga_c_b94 gate passes all three legs. Then full regen.
5. **11-08 (per-`-p` sweep + audits + close)** — Incremental per-subcrate verify. Rewrite `audit_cube_launch.sh` per D-13. Close phase.

No gray areas remain to discuss. The replan is ready for `/gsd-plan-phase 11`.

---

*Log date: 2026-05-18*
*Session: Phase 11 discuss-phase update (checkpoint confirmation + anti-pattern codification)*
