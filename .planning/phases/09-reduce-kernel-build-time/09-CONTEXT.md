# Phase 9: Reduce Kernel Build Time — Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Mode:** discuss (interactive)

<spec_lock>
## SPEC.md is LOCKED

`09-SPEC.md` defines 3 binding requirements for this phase. Downstream agents (researcher, planner, executor) **MUST read SPEC.md before doing any work** — its requirements take precedence over anything below.

Note: SPEC.md was revised twice during this discuss session:
- **Round 3 (2026-04-29):** Per-file line-count cap raised 5,000 → 10,000 → 20,000 lines. The dev machine has confirmed RAM headroom; the historical 5K-line CubeCL proc-macro OOM threshold from RESEARCH.md was conservative. At ≤20K, **0 files violate today** (largest existing file is `mgga_c_b94/kxc_pol.rs` at 16,703 lines). Requirement 5 (now Req 2) becomes a forward guard, not a remediation backlog.
- **Round 4 (2026-04-29):** User directives "Do not make `mgga` optional" and "Drop the ≤180s default-build target and default-build is lda,gga,mgga" removed Original Requirements 1, 2, 3 (default-build family scope, family feature gates, cfg-gated re-exports). Roadmap requirements `BUILD-OPT-02` and `BUILD-OPT-03` are deferred to a future phase. Phase 9 narrows to: **unblock 25 deferred GGAs at full orders + maintain forward-guard caps**.
</spec_lock>

<domain>
## Phase Boundary

Unblock all 25 previously-deferred GGA functionals at full derivative-order coverage (exc/vxc/fxc/kxc/lxc, polarized + unpolarized) under the default `cargo build`, while preserving 1e-12 oracle parity vs libxc 7.0.0. The default build continues to compile every kernel crate (LDA + GGA + MGGA) — there is no family feature gating in this phase. The ≤20,000-line per-file cap is preserved as a forward guard for new translations.

**Out of scope** (deferred to future phases): family feature gates (`gga`, `mgga`, `all-kernels`), wall-clock targets for the default build, cfg-gating any reference to `libxc_kernel_gga`/`libxc_kernel_mgga` in `src/`.
</domain>

<decisions>
## Implementation Decisions

### Per-file line-count cap (Round 3 SPEC change)

- **D-01:** SPEC's per-file line cap is **≤ 20,000 lines** (raised from the original 5,000 across two steps: 5K → 10K → 20K). Locks Requirement 2 in the revised SPEC.
- **D-02:** Cap is a forward guard, not a remediation backlog. Today 0 files violate; no splitting work is required just to satisfy the cap.

### Family feature gating (Round 4 SPEC removal)

- **D-03:** No `[features]` section is added to root `Cargo.toml` in this phase. `libxc-kernel-gga` and `libxc-kernel-mgga` remain non-optional dependencies.
- **D-04:** `src/kernel/mod.rs` keeps unguarded `pub use libxc_kernel_gga as gga;` and `pub use libxc_kernel_mgga as mgga;`. The unguarded `use libxc_kernel_mgga::deferred::is_deferred` in `src/model/mgga_functional.rs:43` stays unguarded — no scope creep into cfg-gating.
- **D-05:** Roadmap requirements `BUILD-OPT-02` (default-build family scope, ≤900s wall-clock — relaxed from the original ≤180s on 2026-04-29 per user directive) and `BUILD-OPT-03` (family feature gates, cfg-gated re-exports) are explicitly **deferred** out of Phase 9. They need to be re-introduced in a future phase if/when desired. `BUILD-OPT-01` (sccache + incremental=false) was already done before Phase 9 started and remains untouched.

### Translator split threshold + kernel regeneration

- **D-06:** Translator internal split threshold is raised from the historical ~5,000 lines per `#[cube(launch_unchecked)]` function to **18,000 lines**. Provides a 2,000-line safety margin under the SPEC cap (D-01). Affects `tools/translate_lda_v2.py`, `tools/translate_gga.py`, `tools/translate_mgga.py`, and any related helpers (`tools/split_oversized_kernel.py`, `tools/split_oversized_mgga.py`, `tools/resplit_gga.py`).
- **D-07:** Re-generate kernels for **all three families** (LDA + GGA + MGGA) using the new 18K threshold. This is the largest churn item in the phase; it consolidates many existing `_partN_subM` files into fewer/larger ones while reducing total file count.
- **D-08:** Operation-order constraint MUST be preserved across the regen — same `let` binding sequence and output-write order as today. The threshold change only shifts split boundaries between files; it must NOT reorder operations within a function. Required for 1e-12 oracle parity per PROJECT.md.
- **D-09:** Sub-crate boundaries (the ~60 GGA + ~109 MGGA letter-suffix sub-crates) are NOT re-bin-packed in this phase. The regen produces smaller-count files inside each functional's existing directory; sub-crate `lib.rs` files are updated only to match the new `pub mod _partN…` filenames. Re-bin-packing is a separate concern deferred to a future phase if desired.

### Plan 09-04 disposition + new plan structure

- **D-10 [informational]:** Existing Plan 09-04 (which scoped feature-gating and the ≤180s target) is **obsolete after Round 4** and replaced by a new sequence:
  - **New Plan 09-04** — Raise translator split threshold to 18K (D-06) + regenerate LDA/GGA/MGGA kernels (D-07) + verify no file >20,000 lines + commit. Output every cargo run to `log/<descriptive>.log` per project convention.
  - **New Plan 09-05** — Generated audit script (`tools/audit_deferred_gga.py`) that for each of the 25 deferred functionals: enumerates expected derivative-order modules from the maple2c source, then asserts coverage against `crates/kernel-gga*/src/<functional>/mod.rs`. Produces a JSON/markdown report. Fix any gaps. Commit.
  - **New Plan 09-06** — End-to-end build verification: `cargo check 2>&1 | tee log/cargo-check-09-final.log`. Must exit 0 with peak RSS below dev-machine total. (Per user feedback memory: prefer `cargo check` over `cargo build` for compile-error verification.)
  - **New Plan 09-07** — Oracle parity full sweep through `verify/` harness: every newly-enabled (functional × derivative order × spin) tuple from the 25 deferred GGAs (≤250 tuples max) + MGGA non-regression spot-check (since MGGA was regenerated under D-07). Strict 1e-12 relative-error tolerance per PROJECT.md. Use rayon for parallel execution. Any tuple failing 1e-12 is a phase blocker.
- **D-11 [informational]:** Old 09-04-PLAN.md should be **archived** (rename to `09-04-PLAN.obsolete.md` or similar) so the plan-phase regenerator does not pick it up but the audit trail is preserved.

### Audit + verification methodology

- **D-12:** Audit (Plan 09-05) is **script-driven** — `tools/audit_deferred_gga.py` generates a reproducible report. No manual table walkthrough.
- **D-13:** Build verification (Plan 09-06) uses **`cargo check`** (not `cargo build`). Proc-macro expansion still happens (so OOM symptoms surface), no codegen/link. Output redirected to `log/cargo-check-09-final.log`.
- **D-14:** Oracle parity sweep (Plan 09-07) is **full** — all (≤25) functionals × (up to 5) orders × 2 spins, capped at ≤250 tuples. Tolerance is **strict 1e-12** relative error (no per-order relaxation). Any failure blocks the phase.

### Claude's Discretion

- Exact `_partN`/`_partN_subM` numbering/naming under the new 18K threshold — translator decides.
- Audit report file format (JSON vs markdown) for `tools/audit_deferred_gga.py` — pick whichever is easiest to scan.
- Whether the audit script also covers MGGA + LDA non-regression as a freebie, or stays narrowly on the 25 GGAs.
- Exact rayon parallelism level for the parity sweep — pick what stays safe under dev-machine memory.
- Whether the parity sweep uses one of the existing 4 oracle test systems (H, Li, BrOH, BrOH+) or all four — at minimum it needs to cover both spin-zero (H, Li) and spin-polarized (BrOH⁺) cases.

### Folded Todos

None — `todo.match-phase` returned 0 matches for Phase 9.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Locked requirements (read first)
- `.planning/phases/09-reduce-kernel-build-time/09-SPEC.md` — LOCKED requirements (3, post-revision). Goal, Boundaries, Constraints, Acceptance Criteria. Read before everything else. Note: SPEC was revised in this session — Round 3 raised the file cap, Round 4 removed feature-gating requirements.

### Phase research and prior plans
- `.planning/phases/09-reduce-kernel-build-time/09-RESEARCH.md` — **Note: this research is STALE.** It predates Plan 09-03's bin-packing splits. Specifically the "25 deferred GGA functionals" status is no longer accurate — they ARE wired into letter-suffix sub-crates today (e.g., `gga_c_ft97` lives across `kernel-gga-1b`/`-1c`/`-1d`). Use only for the canonical 25-functional list and the `kernel-math`/sub-crate template patterns; ignore its scope conclusions.
- `.planning/phases/09-reduce-kernel-build-time/09-01-SUMMARY.md`, `09-02-SUMMARY.md` — Prior plan completion summaries (translator preamble + delta annotations; LDA monolithic split).
- `.planning/phases/09-reduce-kernel-build-time/09-VALIDATION.md` — Prior validation work for this phase.
- `.planning/phases/09-reduce-kernel-build-time/09-04-PLAN.md` — **OBSOLETE.** Per D-10/D-11 this plan is replaced by the new 09-04 (translator threshold + regen). Archive but do not execute.

### Project-level constraints (project-wide)
- `.planning/PROJECT.md` — 1e-12 oracle parity contract; pure-Rust constraint; floating-point operation-order requirement; thiserror v2 boundary policy.
- `.planning/REQUIREMENTS.md` — `BUILD-OPT-01` is satisfied (sccache + incremental=false). `BUILD-OPT-02` and `BUILD-OPT-03` are now **deferred from Phase 9** (see D-05); they need their own future phase.
- `.planning/ROADMAP.md` — Phase 9 entry. Note that the original Phase 9 roadmap requirements list (BUILD-OPT-01/02/03) overstates Phase 9's deliverable post-Round 4; only BUILD-OPT-01 is fully addressed by this phase.
- `.planning/STATE.md` — Project progress tracker.
- `CLAUDE.md` — Project instructions, GSD workflow enforcement, dependency stack.

### Translators and split helpers (target of D-06/D-07 execution)
- `tools/translate_lda_v2.py` — LDA translator. Per Plan 09-01, has shared-preamble + incremental-delta annotations. Modify split threshold to 18K.
- `tools/translate_gga.py` — GGA translator. Same.
- `tools/translate_mgga.py` — MGGA translator. Same.
- `tools/split_oversized_kernel.py`, `tools/split_oversized_mgga.py`, `tools/resplit_gga.py` — Existing helpers used by Plan 09-03's bin-packing. Audit and update to honor the new 18K threshold.
- `tools/batch_translate_{lda,gga,mgga}.py` — Batch driver scripts that orchestrate the per-functional translator runs.
- `tools/generate_gga_dispatch.py`, `tools/generate_gga_roster.py`, `tools/generate_mgga_roster.py` — Roster/dispatch generators (likely also touched after regen if file structure within a functional changes).

### Verification harness (target of D-14 execution)
- `verify/` crate — bindgen against system libxc 7.0.0 oracle. Provides the parity-comparison primitives. Plan 09-07 needs to confirm whether a "sweep all (functional × order × spin) tuples" entry point already exists or needs to be added.

### Source files that consume `libxc_kernel_gga`/`libxc_kernel_mgga` (intentionally unguarded per D-04)
- `src/kernel/mod.rs:2-3` — Unguarded re-exports of `gga` and `mgga`. Stays as-is.
- `src/model/mgga_functional.rs:43` — `use libxc_kernel_mgga::deferred::is_deferred as is_deferred_mgga;`. Stays as-is.
- `src/eval/gga_dispatch/`, `src/eval/mgga_dispatch/` — Dispatch tables that bind to `batchN*` re-exports from the `kernel-gga`/`kernel-mgga` facades. Stays as-is.

### Out-of-scope reference (read for context, not for action)
- `crates/kernel-mgga/src/deferred.rs` — The 6 deferred MGGAs (Brent-method root-finders) remain explicitly out of scope per SPEC Boundaries. Do not attempt to enable.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Translator stack** (`tools/translate_*.py`, `tools/split_oversized_*.py`): Plan 09-01 added shared-preamble + incremental-delta annotations; Plan 09-02 re-translated all 239 functionals. Modifying these scripts (D-06) is the only file-emit policy change in 09-04. The infrastructure for re-running them is already in place.
- **Bin-packing infrastructure**: Plan 09-03's first-fit-decreasing bin packer split GGA into ~60 sub-crates and MGGA into ~109 sub-crates. Sub-crate boundaries are stable across the regen (D-09); only intra-functional file content changes.
- **Verify harness** (`verify/` crate): Already drives oracle comparisons via bindgen against libxc 7.0.0. Plan 09-07 leverages this; may need a new entry point for "sweep all tuples in a list."
- **`crates/kernel-mgga/src/deferred.rs`**: Pure-static metadata module providing `is_deferred(id: u16) -> bool` over a hard-coded list of 6 IDs. Used by `src/model/mgga_functional.rs` to short-circuit the 6 root-finder-blocked MGGAs early. Stays in place; no kernel deps.
- **Sub-crate template pattern**: `crates/kernel-gga-1a/Cargo.toml` and `lib.rs` follow a consistent template (cubecl + kernel-math deps, `pub mod <functional>;` per functional). Applies if any new sub-crate is added.

### Established Patterns
- **`#[cube(launch_unchecked)]` per derivative-order × spin**: Every (functional × order × spin) is a separate top-level kernel function. The translator emits these as one file per (order × spin), further sub-split via `_partN_vKrhoK_M.rs` when the function exceeds the split threshold. New 18K threshold (D-06) raises the split point; output-index bucket logic stays intact.
- **`mod.rs` aggregation**: Each functional's directory has a `mod.rs` listing all `pub mod _partN…` lines. The audit (D-12) walks these to verify coverage.
- **Letter-suffix sub-crates**: `kernel-gga-1`, when too big, became `kernel-gga-1a`, `-1b`, `-1c`, `-1d`. A single functional may span multiple suffix sub-crates (e.g., `gga_c_ft97` is in `-1b`, `-1c`, AND `-1d`, partitioned by derivative kind). This is the pattern for distributing a multi-order functional across smaller crates.
- **Oracle parity tolerance**: 1e-12 relative error against libxc 7.0.0 is the project-wide bar (PROJECT.md, CLAUDE.md). Used at every kernel-acceptance boundary, including this phase's Plan 09-07.
- **Cargo log convention** (per user feedback memory): All cargo runs redirect stdout+stderr to `log/<descriptive>.log` and analysis happens from the file, never terminal scrollback. Plans 09-04, 09-06 must follow this. Use `log/cargo-check-09-final.log`, `log/cargo-build-09-regen.log`, etc.

### Integration Points
- **`tools/translate_*.py` ↔ `crates/kernel-{lda,gga,mgga}*/src/`**: Translator output goes directly into sub-crate src/ trees. Plan 09-04 regen rewrites file contents in place; sub-crate Cargo.toml + lib.rs are touched only if `pub mod` lines change due to renamed `_partN` files.
- **`crates/kernel-{gga,mgga}*` ↔ `src/eval/{gga,mgga}_dispatch`**: Dispatch tables import via `batchN*` re-exports from the facade crates. Re-translation does not change the public per-functional API (still `dispatch_<name>(ctx, order, spin)`), so dispatch tables don't need updates.
- **`verify/` ↔ `crates/kernel-*`**: Verify harness drives kernels by calling Rust APIs and comparing against libxc oracle. Doesn't depend on internal file structure of kernel crates.
- **`src/model/mgga_functional.rs:43` ↔ `crates/kernel-mgga/src/deferred.rs`**: Stays unguarded per D-04; the only cross-crate import in src/ that interacts with optional-feature semantics, and we're not making mgga optional anyway.
- **CARGO_TARGET_DIR convention** (per user memory): Always use the shared `/home/chemtech/workspace/libxc_rs/target` directory; never override even in worktree executors.
</code_context>

<specifics>
## Specific Ideas

- "We have enough memory headroom" — user invoked this twice to relax SPEC constraints. The dev machine demonstrably handles files up to 16,703 lines today; a 20K cap and an 18K translator threshold are within proven headroom.
- "Do not make `mgga` optional" — strict directive; mgga always compiles. Any future temptation to introduce `#[cfg(feature = "mgga")]` in src/ requires a new SPEC change.
- "Drop the ≤180s default-build target and default-build is lda,gga,mgga" — strict directive; Phase 9 ships without any wall-clock guarantee.
- "Increase the splitting threshold and generate the kernels" — strict directive translated into D-06 (raise to 18K) and D-07 (regen all three families).
- Cargo verification uses `cargo check`, not `cargo build`, per the user's feedback memory. Logs always to `log/<descriptive>.log`.
</specifics>

<deferred>
## Deferred Ideas

- **Family feature gates (`gga`, `mgga`, `all-kernels`)** — Removed from Phase 9 in Round 4. Required to satisfy roadmap `BUILD-OPT-02` and `BUILD-OPT-03`. Belongs in a future phase if/when the user wants to reduce default-build cost.
- **Default-build wall-clock target (currently ≤900s; originally ≤180s, relaxed on 2026-04-29 per user directive)** — Removed from Phase 9 in Round 4. If revisited later, would need a fresh measurement on the dev machine and possibly further relaxed targets given MGGA's always-on cost.
- **Cfg-gating `libxc_kernel_gga`/`libxc_kernel_mgga` references in `src/`** — Coupled to family feature gates above. Out of Phase 9.
- **Sub-crate re-bin-packing post-regen** — With 18K-threshold regen producing fewer/larger files inside each functional, the existing ~170 sub-crate split may be over-fragmented (each sub-crate ends up smaller than necessary). Re-running the bin-packer at a larger sub-crate budget could consolidate. Not in Phase 9; a future "tighten kernel workspace organization" phase could pick this up.
- **6 deferred MGGAs (`mgga_c_b94`, `mgga_x_br89`, `mgga_x_mbr`, `mgga_x_mbrxc_bg`, `mgga_x_mbrxh_bg`, `mgga_x_mggac`)** — Blocked on Brent's-method root-finder implementation in `kernel-math`. Out of Phase 9 per SPEC Boundaries; tracked in `crates/kernel-mgga/src/deferred.rs`.
- **`09-RESEARCH.md` refresh** — Stale post-09-03; contains assumptions superseded by the SPEC revisions in this session. Either refresh during a future phase or note as deprecated. Not blocking.
- **Phase 9 directory rename** — The phase is still named `09-reduce-kernel-build-time` even though build-time reduction is no longer in scope. Rename to e.g. `09-unblock-deferred-gga-functionals` is cosmetic and would touch many references; defer indefinitely or fold into a milestone-level cleanup.
- **MGGA full-spectrum oracle re-validation** — Plan 09-07 includes MGGA non-regression spot-checks because MGGA was regenerated under D-07. A full MGGA sweep (every MGGA × every order × both spins) is a much larger job and would expand 09-07 substantially; defer the full sweep to a separate validation phase if spot-checks find any drift.

### Reviewed Todos (not folded)
None — no Phase-9-relevant todos surfaced from `todo.match-phase`.
</deferred>

---

*Phase: 09-reduce-kernel-build-time*
*Context gathered: 2026-04-29 (assistant: Opus 4.7 1M)*
*Discuss session length: 4 user directives + Round 3/Round 4 SPEC revisions; 4 originally-selected gray areas all resolved*
