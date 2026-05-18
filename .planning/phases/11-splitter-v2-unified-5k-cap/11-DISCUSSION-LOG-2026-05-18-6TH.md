# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Discussion Log (6th Session)

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-18 (sixth discuss-phase session)
**Phase:** 11-splitter-v2-unified-5k-cap
**Triggered by:** 11-06 Sessions 1+2 execution outcomes (PARTIAL SUMMARY at `8cb80ce49` + 9 atomic per-file commits `9e7544efb`..`1bf0e3bf1`) + Deviations E+F (Rule 10 Phase-2 extension + full-tree regen of 266 functionals) + Task 7 (D-28 annotation) + user direction change on jobs cap (jobs=1 → per-invocation jobs=3 via dedicated sweep tool)
**Areas discussed:** 11-07 scope re-eval; batched-compile sweep architecture; failure mode; batch ordering; plan slot; jobs=3 placement; AP-2 disposition; memory `feedback_ram_constraints` update

---

## Pre-discussion: Context governance

| Option | Description | Selected |
|--------|-------------|----------|
| Update it (6th-iter) | Revise CONTEXT.md to capture Sessions 1+2 + Deviations E+F outcomes and new direction on jobs cap | ✓ |
| View it | Display 11-CONTEXT.md first, then decide | |
| Skip | Use existing CONTEXT.md as-is | |

**User's choice:** Update it

---

## Gray Area Selection (multiSelect)

Four candidate areas presented. User selected one.

| Option | Description | Selected |
|--------|-------------|----------|
| 11-07 scope re-eval | Deviation F absorbed full-tree regen; D-15 Leg 1 (compile) proven on mgga_c_b94. What's left for 11-07 — collapse, narrow to audit, skip, or keep+narrow? | ✓ |
| Gate 3 EXIT pass criteria | Original D-22 Gate 3 = mgga_c_b94 only. Expand to smoke set (3-6 functionals/family) post Deviation F or keep narrow? | |
| AP-8 retrospective | Were Deviation E + F AP-8 triggers? Codify the distinction or leave open | |
| Just bookkeeping | Skip gray-area discussion; just append a 6th revision note | |

**User's choice:** 11-07 scope re-eval

---

## Mid-discussion redirect

After Area 1 was announced, user submitted a new direction: "Please modify the upstream process in the Python pipeline tools for the C-to-Rust transition. If the compilation results in errors, adjust the workflow. Note that the Rust kernel should compile in batches of 20, using 3 parallel jobs (jobs=3)."

This collided with multiple BLOCKING constraints (AP-2, D-07, D-09, memory `feedback_ram_constraints`). Discussion paused for conflict resolution before any code or context edits.

---

## Conflict resolution: jobs=3 override

| Option | Description | Selected |
|--------|-------------|----------|
| Override is intentional | Yes — explicitly raising the cap. Requires retiring AP-2, amending D-07/D-09, updating memory. Confirm OOM risk has changed (per-functional subcrates make smaller compilation units) | ✓ |
| Per-batch jobs=3, .cargo/config stays jobs=1 | Pass `--jobs 3` per invocation only; config stays jobs=1 | |
| I misspoke — keep jobs=1 | All prior decisions stand | |
| Pause, I'll re-explain | Step back and re-explain | |

**User's choice:** Override is intentional

| Option | Description | Selected |
|--------|-------------|----------|
| Per-family translators (translate_{lda_v2,gga,mgga}.py) | The Rule-10-turbofish-emitting layer (touched yesterday in Deviation D+E) | ✓ |
| CSE/splitter layer (tools/translate_v2/*) | cse.py / per_functional.py / emit.py — locked clean by D-25 Direction A carve-out | |
| Maple driver (tools/maple_to_kernels.py) | Orchestrator — natural extension point | |
| New tool | A new orchestrator like tools/batched_compile_sweep.py | |

**User's choice:** Per-family translators (translate_{lda_v2,gga,mgga}.py)

---

## Area 1: 11-07 scope re-eval (continued) — design questions

### Q1.1 — Compile feedback architecture

| Option | Description | Selected |
|--------|-------------|----------|
| Translator invokes cargo per batch (tight loop) | Translator shells out to cargo, parses stderr. Self-contained but mixes emit + compile awareness | |
| Separate orchestrator (tools/batched_compile_sweep.py) (recommended) | Translators stay pure emit; new tool wraps + sweeps + feeds errors back | ✓ |
| Maple driver extension | Extend tools/maple_to_kernels.py with --compile-sweep mode | |
| Two-pass: translate then sweep | Translators unchanged; add standalone batch_cargo_sweep.py | |

**User's choice:** Separate orchestrator (tools/batched_compile_sweep.py)
**Notes:** Recorded in D-31. Architecture: pure orchestrator; no translator modification; deterministic; manifest output.

### Q1.2 — Failure mode

| Option | Description | Selected |
|--------|-------------|----------|
| Halt + surface (recommended, AP-8-safe) | First failure stops sweep, write .continue-here.md, exit. No auto-retry | |
| Auto-retry with batch=1, then halt | If batch=20 fails, retry sequentially at batch=1 to isolate. Halt on confirmed failure | ✓ |
| Skip failing, continue, report at end | Maximizes throughput; risks silent baseline corruption | |
| Translator-side auto-fix attempt | AP-8 high-risk — extension-as-rescue pattern. NOT recommended | |

**User's choice:** Auto-retry with batch=1, then halt
**Notes:** Recorded in D-33. Bounded 2-pass design (parallel → sequential → halt). AP-8 boundary clarified separately: bounded retry is diagnostic narrowing, not AP-8 trigger.

### Q1.3 — Batch ordering

| Option | Description | Selected |
|--------|-------------|----------|
| Alphabetical within family (recommended) | LDA all → GGA all → MGGA all (alpha within each). Deterministic, simple, matches existing roster file order | ✓ |
| Easiest-first by LOC (mirrors D-26) | Small subcrates first; bessel/kcis last in their batches | |
| Family-interleaved | Each batch mixes families. Distributes risk | |
| Routed-first, deferred-last | Routed first, then 7 deferred. Reflects D-11 routing distinction | |

**User's choice:** Alphabetical within family
**Notes:** Recorded in D-31/D-34. LDA 43 → GGA 131 → MGGA 92 alphabetical.

### Q1.4 — Plan slot

| Option | Description | Selected |
|--------|-------------|----------|
| Fold into 11-08 | Extend 11-08 with jobs=3 + batched + feedback. No new plan needed | |
| New 11-07 (replaces collapsed 11-07) | Repurpose the 11-07 slot. 11-08 stays as final close | ✓ |
| New 11-09 plan slot | Add as 11-09. Cleaner separation but adds a slot | |
| Outside Phase 11 — /gsd-quick | Tooling enhancement, not core Phase 11 work | |

**User's choice:** New 11-07 (replaces collapsed 11-07)
**Notes:** Recorded in D-34. Original 11-07 (full-tree regen + D-15 entry gate) already done in 11-06 via Deviations E+F; new 11-07 = sweep tool authoring + execution.

---

## Governance questions

### Q2.1 — Where does jobs=3 actually live?

| Option | Description | Selected |
|--------|-------------|----------|
| Flip .cargo/config.toml jobs=1 → jobs=3 (permanent) | Edit the config file. AP-2 fully retired. Affects cargo build --workspace too | |
| Keep .cargo/config.toml jobs=1; sweep passes --jobs 3 on invocation (recommended) | Config stays jobs=1; only sweep tool overrides per-invocation. AP-2 amended (not retired) | ✓ |
| New [env] toggle (LIBXC_RS_SWEEP_JOBS=3) | Env-var pathway; explicit opt-in | |

**User's choice:** Keep .cargo/config.toml jobs=1; sweep passes --jobs 3 on invocation
**Notes:** Recorded in D-32. `.cargo/config.toml` stays jobs=1 (user manages by hand). Override is per-invocation only, gated to `tools/batched_compile_sweep.py`.

### Q2.2 — AP-2 disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Amend (recommended, matches jobs=3-via-flag option) | AP-2 keeps BLOCKING for config.toml edits; explicitly permits sweep tool's per-invocation override | |
| Retire entirely | AP-2 removed; replaced by positive convention | |
| Narrow to RUST_MIN_STACK only | AP-2 stays blocking for RUST_MIN_STACK; jobs removed from its scope. Two separate concerns | ✓ |

**User's choice:** Narrow to RUST_MIN_STACK only
**Notes:** Recorded in AP-2 narrowing edit. Jobs override governance moves to D-32 (invocation policy); RUST_MIN_STACK stays an anti-pattern.

### Q2.3 — Memory feedback_ram_constraints update

| Option | Description | Selected |
|--------|-------------|----------|
| Amend with conditional (recommended) | Pre-Phase-11 = jobs>1 OOMs on umbrella crates; post-Phase-11 per-functional subcrates support jobs=3 via batched_compile_sweep.py | ✓ |
| Fully retire and rewrite | Delete old memory; write fresh one | |
| Leave memory as-is, add new memory pointing to it | Keep historical record; add superseder | |

**User's choice:** Amend with conditional
**Notes:** Memory file updated 2026-05-18. Preserved the strong "user manages config by hand" pattern; added 2026-05-18 sixth-session anomaly evidence (jobs=10 working tree, "you proceed" resolution).

---

## Working-tree anomaly resolution

### Q3.1 — `.cargo/config.toml` shows jobs=10 in working tree (vs jobs=3 in prose / jobs=1 in design)

| Option | Description | Selected |
|--------|-------------|----------|
| Revert working tree — keep jobs=1 in config, sweep uses --jobs 3 | `git checkout -- .cargo/config.toml` to restore jobs=1 | |
| Keep jobs=10 in config; revise design upward | jobs=10 is intentional; full AP-2 retirement | |
| Change config to jobs=3 (match your prose) | Edit config to jobs=3; sweep inherits | |
| I'll handle the config; you proceed | User restores file by hand; agent proceeds with locked design | ✓ |

**User's choice:** I'll handle the config; you proceed
**Notes:** Matches the pre-existing `feedback_ram_constraints` pattern ("user manages config by hand"). Agent did NOT touch `.cargo/config.toml`. CONTEXT.md updated with the locked design (jobs=1 in config, --jobs 3 in sweep tool only). The working-tree anomaly is the user's to resolve.

---

## Wrap-up

| Option | Description | Selected |
|--------|-------------|----------|
| Ready for context | Write CONTEXT.md with D-31..D-34, route to /gsd:plan-phase 11 | ✓ |
| Explore more | More gray areas (deferred-kernel inclusion, math/ handling, PATTERN.md Rule 11, etc.) | |
| Discuss Gate 3 EXIT or AP-8 retro now | Loop back to the other gray areas | |

**User's choice:** Ready for context

---

## Claude's Discretion

- Exact CLI shape of `tools/batched_compile_sweep.py` flags (long vs short, default values, argparse vs click). Planner picks.
- Per-batch peak-RSS measurement mechanism (e.g., `/usr/bin/time -v`, `psutil`, cgroup stats). Planner picks.
- `.continue-here.md` schema for sweep failures (JSON vs Markdown, required fields). Planner picks.
- Sweep manifest format (JSON vs TOML, schema for pass/fail per subcrate). Planner picks.
- Default sweep ordering of the 7 deferred kernels relative to the 266 routed (within MGGA alphabetical, or as a trailing block). Planner picks.
- Whether sweep tool also targets `crates/kernels/math/` and `crates/kernels/shared/` (they're separately-compiled support crates, not per-functional). Recommended: include them as a 0th batch since math/ failure blocks everything downstream. Planner confirms.

## Deferred Ideas (from this session)

- **PATTERN.md Rule 11 for sweep tool conventions.** Discussed but not selected (user chose "Ready for context"). Possible future addition if 11-07 execution surfaces a documentable pattern.
- **Spec_to_criterion_map automation.** The map needs manual maintenance every time SPEC-11-Rx assignments shift; could be auto-derived from plan frontmatter. Not Phase 11 scope.
- **Empirical peak-RSS calibration before locking jobs=3.** The sweep tool measures peak-RSS per batch as a built-in feature (D-32 mandates it), so this is partially absorbed; if peak exceeds ~24 GB on any batch, drop to --jobs 2 and document.
