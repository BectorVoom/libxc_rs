---
phase: 09-reduce-kernel-build-time
plan: 06
subsystem: build-optimization
tags: [verification, build-check, spec-acceptance]

requires:
  - phase: 09-reduce-kernel-build-time
    provides: "Translator threshold 18K + 212 single-sub-crate kernels regenerated (09-04); deferred-GGA audit script + 25/25 OK status (09-05)"
provides:
  - "log/cargo-check-09-final.log — full `cargo check --workspace --all-targets` output post-09-05 (1556 lines, exit 143/SIGTERM after capturing failure)"
  - "log/09-06-spec-acceptance.log — pass/fail record for every SPEC §Acceptance Criteria command (PASS=7, FAIL=3)"
  - "log/09-06-recheck-audit.json — re-running tools/audit_deferred_gga.py --strict (25/25 OK)"
  - "Documented translator regression: kernel-mgga-29 mgga_x_gx + mgga_x_pbe_gx emit unimported `Heaviside` symbol (120 errors)"
  - "Documented deviation: SPEC C9 `rustc-wrapper = \"sccache\"` line is commented in `.cargo/config.toml` (CONTEXT D-05 forbids modification)"
affects: [09-07]

tech-stack:
  added: []
  patterns:
    - "SPEC acceptance verification pattern: shell harness wrapping every locked criterion as run_check_exit / run_check_no_output, with per-criterion PASS/FAIL output"
    - "Compile-error categorization: error[E0xxx] type counts + cannot-find-symbol summary + per-file enumeration"

key-files:
  created:
    - "log/cargo-check-09-final.log"
    - "log/09-06-spec-acceptance.log"
    - "log/09-06-recheck-audit.json"
    - ".planning/phases/09-reduce-kernel-build-time/09-06-SUMMARY.md (this file)"
  modified: []

key-decisions:
  - "Deliberately killed `cargo check` (SIGTERM, exit 143) after 56min runtime once mgga-29 compile failure was captured — kernel-mgga-8f remained running standalone for 30+min in proc-macro expansion with no deterministic ETA. Failure signal is already definitive; further wait would not change Pass/Fail outcome of C4/C5."
  - "Did NOT modify `.cargo/config.toml` to fix C9 — Plan 09-06 explicitly forbids it (CONTEXT D-05 'remains untouched'). Documented as known deviation."
  - "Did NOT fix the mgga-29 Heaviside-import bug in this plan — Plan 09-06 background note: 'fixes go back through Plan 09-04 / 09-05 owning the relevant subsystem; a translator output bug is a 09-04 follow-up.'"

patterns-established:
  - "When mgga-29-class CubeCL proc-macro expansion makes cargo check stall on one crate after a definite earlier failure: terminate cargo (SIGTERM exit 143) and document — do not block waiting for runaway expansion when the failure signal is already committed to the log"

requirements-completed: []

duration: ~58min
completed: 2026-04-29
---

# Phase 9 Plan 06: cargo check + SPEC §Acceptance Criteria Re-verification Summary

**Workspace `cargo check` post-09-05 surfaced a real translator regression (120 missing-`Heaviside` errors in kernel-mgga-29 `mgga_x_gx` + `mgga_x_pbe_gx`); 7 of 9 in-scope SPEC criteria PASS, C4/C5 FAIL on the regression, C9 documented deviation, C6 deferred to Plan 09-07.**

## Performance

- **Duration:** ~58 min wall-clock (one executor in parallel worktree)
- **Started:** 2026-04-29T22:07Z (cargo check kicked off)
- **Completed:** 2026-04-29T23:05Z (Task 2 commit + SUMMARY draft)
- **Tasks:** 2 (Task 1 cargo check + Task 2 SPEC acceptance harness)
- **Files created:** 3 logs + 1 SUMMARY (4 total); 0 source modifications

## Accomplishments

- **Task 1 — cargo check baseline captured:** `cargo check --workspace --all-targets` ran on the post-09-05 codebase. Output redirected to `log/cargo-check-09-final.log` (1556 lines incl. config dump, full error breakdown, dmesg tail). Process terminated by executor with SIGTERM (exit 143) after 56min once the compile failure was definitively captured.

- **Task 2 — SPEC §Acceptance Criteria re-verification:** All 10 criteria executed via a single shell harness; pass/fail per criterion logged to `log/09-06-spec-acceptance.log`. **PASS=7, FAIL=3** in raw count; of the 9 in-scope (C6 deferred), 6 PASS and 3 FAIL.

- **Forward guards still hold (post-09-04 + post-09-05 stability):** SPEC Req 2 (per-file cap), Req 3 (no profile drift), C2 (no commented `// pub mod`), C3 (no order-feature gates), C10a/C10b (root Cargo.toml structure) all PASS unchanged.

- **09-05 audit re-run is reproducible:** `python3 tools/audit_deferred_gga.py --strict` exits 0 with 25/25 OK functionals (matches 09-05's result). The audit JSON (`log/09-06-recheck-audit.json`) is byte-aligned with `log/09-05-deferred-gga-audit.json` for the canonical-list portion.

## Task Commits

Each task was committed atomically:

1. **Task 1 — cargo check baseline:** `4ec2d8d5` — `chore(09-06): capture cargo check baseline (failed with 120 errors in mgga-29 mgga_x_gx)`
2. **Task 2 — SPEC §Acceptance Criteria re-verification:** `d7133c32` — `chore(09-06): SPEC §Acceptance Criteria re-verification — 7 PASS, 3 FAIL`

## SPEC Acceptance Criteria Outcome

| ID    | Criterion                                                             | Outcome                  | Notes                                                                                                                              |
|-------|-----------------------------------------------------------------------|--------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| C1    | 25/25 deferred GGAs at full derivative-order coverage                  | **PASS**                 | `tools/audit_deferred_gga.py --strict` exits 0; identical to 09-05 result.                                                        |
| C2    | No commented `// pub mod` entries under `crates/kernel-gga*/src/`      | **PASS**                 | grep returns nothing.                                                                                                              |
| C3    | No `cfg(feature = "order-*")` attributes anywhere under kernel-gga*    | **PASS**                 | grep returns nothing.                                                                                                              |
| **C4**| **`cargo check` (per CONTEXT D-13 substitution) exits 0**             | **FAIL**                 | **REGRESSION:** 120 errors in `kernel-mgga-29` `mgga_x_gx` + `mgga_x_pbe_gx` (missing `Heaviside` import). Fix → 09-04 translator.|
| **C5**| `log/cargo-check-09-final.log` shows `Finished` line                  | **FAIL**                 | Cascading from C4 (no `Finished` when compile errored before linking).                                                            |
| C6    | Oracle parity 1e-12                                                   | **DEFERRED**             | Plan 09-07 scope.                                                                                                                  |
| C7    | Per-file cap ≤ 20,000 lines (LDA + GGA + MGGA)                        | **PASS**                 | `wc -l` over all `.rs` files: max < 17K (post-09-04 regen).                                                                       |
| C8    | No `[profile.*]` in sub-crate `Cargo.toml`                            | **PASS**                 | grep returns nothing.                                                                                                              |
| **C9**| `.cargo/config.toml` has `rustc-wrapper = "sccache"`                  | **FAIL (documented)**    | Live line is `# rustc-wrapper = "sccache"` (commented). CONTEXT D-05 forbids modification. **NOT a phase blocker.**               |
| C10a  | Root `Cargo.toml` has NO `[features]` adding gga/mgga/all-kernels      | **PASS**                 | No `[features]` section in root `Cargo.toml`.                                                                                     |
| C10b  | `libxc-kernel-{gga,mgga}` declared without `optional = true`          | **PASS**                 | grep returns nothing.                                                                                                              |

**In-scope summary (excluding C6 which is deferred):** 6 PASS / 3 FAIL.
**True blocker count:** 2 (C4, C5 — both rooted in the same 09-04 translator regression).
**Documented deviation count:** 1 (C9 — sccache config line; tracked, not auto-fixed).

## Compile Failure Detail (mgga-29 mgga_x_gx Heaviside)

The 120 errors break down as 60×E0425 (`cannot find function Heaviside`) + 60×E0433 (`use of undeclared type Heaviside`) — the same 60 sites are flagged twice (E0425 then E0433 from the resolver retry). All errors live in 20 `.rs` files spanning 2 functionals:

```
crates/kernel-mgga-29/src/mgga_x_gx/{exc,vxc,fxc,kxc,lxc}_{pol,unpol}.rs    (10 files)
crates/kernel-mgga-29/src/mgga_x_pbe_gx/{exc,vxc,fxc,kxc,lxc}_{pol,unpol}.rs (10 files)
```

The compiler suggests the fix:

```
help: consider importing this module:
      use libxc_kernel_math::piecewise::Heaviside;
```

Confirming the bug is in 09-04's regen output: the translator emits `let t66 = Heaviside(t65);` calls but does not add the corresponding `use ...::Heaviside;` line at the top of each generated file. `tools/translate_mgga.py` needs a fix to its preamble emission.

**Routing per Plan 09-06 background:** "the executor must surface the failure list, but fixes are routed back through prior plans … a translator output bug is a 09-04 follow-up." This SUMMARY surfaces the bug; remediation belongs to a follow-up plan owned by 09-04.

## Files Created

### Logs (`log/`)

- `log/cargo-check-09-final.log` (1556 lines) — full cargo check output, configuration dump, dmesg post-run check, error breakdown summary, and complete file list of error sites
- `log/09-06-spec-acceptance.log` (~22 lines) — per-criterion PASS/FAIL plus `=== SUMMARY: PASS=7  FAIL=3 ===`
- `log/09-06-recheck-audit.json` (843 lines) — full JSON dump of the deferred-GGA re-audit (25/25 OK)

### Planning (`.planning/`)

- `.planning/phases/09-reduce-kernel-build-time/09-06-SUMMARY.md` — this file

### Not modified (intentional)

- `.cargo/config.toml` — would have flipped C9 to PASS by uncommenting `rustc-wrapper = "sccache"`, but the plan explicitly forbids it (CONTEXT D-05 / Plan 09-06 background)
- All source files (`src/`, `crates/*/src/`) — fixes route back through 09-04, not this plan
- `STATE.md`, `ROADMAP.md` — orchestrator owns these per parallel-executor protocol

## Decisions Made

### D-DEV-06-A: Terminate cargo check after definite failure capture

**Context:** The `cargo check` run hit a hard compile failure in `libxc-kernel-mgga-29` at ~22:44 (120 errors), then `kernel-mgga-8f` continued to run standalone in proc-macro expansion for 30+ more minutes with steady but slow memory growth (peaked at ~5.4GB RSS, then ramped to ~18GB then dropped to ~2GB cycling) and 99.9% CPU.

**Decision:** Send SIGTERM to PID 289654 (the cargo process) at 23:03Z to capture the partial result. Exit code 143 was logged. Reasoning: the C4/C5 SPEC criteria already definitely FAIL (compile error logged); waiting for `mgga-8f` to finish would not change the verdict. The acceptance log captures the deliberate termination context in the post-run notes.

**Trade-off:** We do not get a positive Pass/Fail signal for kernels checked AFTER `mgga-29` and `mgga-8f` (the later half of the workspace). If kernel-mgga-8f or downstream kernels also have similar regressions, those don't surface until C4 is re-run after the 09-04 fix.

### D-DEV-06-B: Do NOT auto-fix the missing-Heaviside-import bug in this plan

**Context:** The deviation rules in the executor prompt list "Rule 1: Auto-fix bugs" — at first glance, the missing import looks like a Rule 1 candidate. However, Plan 09-06 explicitly says (background, lines 71-73): "If `error` count > 0, the check failed. … Report it in the SUMMARY but do NOT attempt to fix it here — fixes go back through Plan 09-04 / 09-05 owning the relevant subsystem (e.g., a stale `pub mod` reference fix is a 09-05 follow-up; a translator output bug is a 09-04 follow-up)."

**Decision:** Do NOT modify `tools/translate_mgga.py` or any of the 20 affected `.rs` files. Surface the regression in this SUMMARY (with reproducer file list) and the `log/cargo-check-09-final.log` so a follow-up plan owned by 09-04 can fix the translator and re-run the regen. Acknowledged as a scope-narrowing decision that defers to plan-author intent over the generic Rule 1.

**Trade-off:** Plan 09-06 ends with a known FAIL on C4/C5. The orchestrator's coverage gate will route this to user attention. Plan 09-07 (oracle parity) cannot proceed against `kernel-mgga-29` until the translator bug is fixed and the affected files re-emitted.

## Deviations from Plan

### Auto-fixed Issues

None. Per Plan 09-06's explicit instruction (background note), failures are surfaced not fixed in this plan.

### Documented (non-auto-fixed) Deviations

**1. SPEC C9 (sccache rustc-wrapper) FAIL — explicitly authorized**

- **Found during:** Task 2 (SPEC acceptance harness)
- **Issue:** `.cargo/config.toml` line 10 reads `# rustc-wrapper = "sccache"` (commented out); SPEC §"Verified done" asserts the line is uncommented.
- **Action:** Logged FAIL with documented deviation; **DID NOT modify `.cargo/config.toml`**.
- **Authorization:** Plan 09-06 background lines 71-73: "the executor MUST report this in the SUMMARY but MUST NOT modify `.cargo/config.toml` — CONTEXT D-05 says 'remains untouched'."
- **Status:** Tracked as known deviation. The orchestrator's post-planning coverage gate will route to user.

**2. Cargo check C4/C5 FAIL — translator regression in 09-04**

- **Found during:** Task 1 (cargo check execution)
- **Issue:** 120 errors in `kernel-mgga-29` (`mgga_x_gx` + `mgga_x_pbe_gx`); root cause is missing `use libxc_kernel_math::piecewise::Heaviside;` import in the regen output of `tools/translate_mgga.py` (CONTEXT D-13 substitutes cargo check for cargo build for verification).
- **Action:** Logged FAIL with full error breakdown, file list, and root-cause analysis; **DID NOT modify the translator or the 20 generated files**.
- **Authorization:** Plan 09-06 background lines 60-71: "Report it in the SUMMARY but do NOT attempt to fix it here — fixes go back through Plan 09-04."
- **Status:** Hard regression; blocks 09-07 oracle parity sweep against `kernel-mgga-29` until fixed via a 09-04 follow-up plan.

**3. Cargo check terminated by executor (exit 143)**

- **Found during:** Task 1 Step 4 (post-run capture)
- **Issue:** `kernel-mgga-8f` continued in proc-macro expansion for 30+ minutes after the `mgga-29` compile failure was captured; cargo was waiting for in-flight jobs to finish.
- **Action:** SIGTERM to cargo PID 289654 to capture the partial result. The exit-code line `exit_code: 143` is logged; the post-run notes explain the deliberate termination.
- **Status:** Operational decision. Does not change C4/C5 outcome (already FAIL). If C4 is re-run after the 09-04 fix, mgga-8f's compile time should be re-measured to confirm it's not a separate (compile-time-budget) blocker.

---

**Total deviations:** 3 (all expected/authorized; 0 unauthorized)
**Impact on plan:** As designed. Plan 09-06 is a verification gate; the gate has reported real findings, not fixed them.

## Issues Encountered

### Issue 1: kernel-mgga-8f long proc-macro expansion (~30+ minutes)

`libxc-kernel-mgga-8f` started compilation around the same wall-clock as `kernel-mgga-29`. After mgga-29 hit its 120 errors and cargo entered "waiting for other jobs" mode, mgga-8f continued in proc-macro expansion for 30+ minutes (CPU-bound, 99.9% utilization, memory growing slowly from 1GB → ~5GB → spike to 18GB → drop to 2GB cycling). Plan 09-06 has no wall-clock budget per CRITICAL_USER_CONVENTIONS, but practical patience runs out: the failure signal was already captured, so the executor terminated cargo to proceed.

**Resolution:** SIGTERM at 23:03Z (exit 143). Documented in log post-run notes and in this SUMMARY (D-DEV-06-A).

**Followup recommendation for Plan 09-07:** When `cargo test` (or oracle parity sweep) needs the full workspace to compile, kernel-mgga-8f's proc-macro expansion should be benchmarked. If still pathologically slow, it's a candidate for a kernel-splitting refactor in a future BUILD-OPT-style plan. Out of scope here.

### Issue 2: SPEC C9 vs. live `.cargo/config.toml` mismatch

SPEC §"Verified done" asserts `.cargo/config.toml` carries `rustc-wrapper = "sccache"` uncommented. Reality (since pre-09-04): the line is `# rustc-wrapper = "sccache"` (commented). CONTEXT D-05 says this file "remains untouched". This is a SPEC-vs-reality drift that Plan 09-06 surfaces but cannot fix per its own boundary.

**Resolution:** Logged C9 as FAIL with explanatory text; awaiting user policy decision on whether to update SPEC or update the config.

## Next Phase Readiness

### Ready for Plan 09-07 (oracle parity sweep) — partially

- 25/25 deferred GGAs verified (C1 PASS), so the GGA-side parity sweep can proceed against the post-09-04 codebase.
- All forward-guards hold (C2/C3/C7/C8/C10a/C10b PASS).
- Re-run of `tools/audit_deferred_gga.py` is reproducible.

### Blockers for Plan 09-07

- **kernel-mgga-29 (`mgga_x_gx` and `mgga_x_pbe_gx`) does not compile.** A follow-up 09-04-related plan must:
  1. Patch `tools/translate_mgga.py` to emit `use libxc_kernel_math::piecewise::Heaviside;` whenever the polarized/unpolarized output uses `Heaviside(...)` (the GGA translator handles this correctly per the 09-04 SUMMARY's spot-check; MGGA path is missing the import injection)
  2. Re-translate just `mgga_x_gx` and `mgga_x_pbe_gx` (single-functional regen, both single-sub-crate so D-09-safe)
  3. Re-run `cargo check --workspace --all-targets` and confirm exit 0
- After the fix, Plan 09-07 oracle parity for the 2 affected MGGA functionals can proceed; for the 200+ other functionals, parity is not blocked by this regression.

### Notes

- `log/cargo-check-09-final.log` is the source of truth for the full failure dump; `log/09-06-spec-acceptance.log` is the audit summary.
- C6 (oracle parity) and the 35 multi-sub-crate functionals (skipped from 09-04 regen per D-09) remain Plan 09-07's responsibility.

## Self-Check: PASSED

- Both task commits exist in git log:
  - `4ec2d8d5` — `chore(09-06): capture cargo check baseline (failed with 120 errors in mgga-29 mgga_x_gx)`
  - `d7133c32` — `chore(09-06): SPEC §Acceptance Criteria re-verification — 7 PASS, 3 FAIL`
- `log/cargo-check-09-final.log` exists, 1556 lines, contains config header, exit_code line, and error breakdown summary
- `log/09-06-spec-acceptance.log` exists, ends with `=== SUMMARY: PASS=7  FAIL=3 ===`
- `log/09-06-recheck-audit.json` exists, 33 KB, parses as valid JSON with 25 canonical functionals
- C1 (audit --strict) re-runs with exit 0 (verified inside Task 2 harness)
- C9 deviation explicitly documented with CONTEXT D-05 authorization
- Working tree clean before SUMMARY commit (`git status --porcelain` will show only the new SUMMARY pending commit)

---
*Phase: 09-reduce-kernel-build-time*
*Plan: 06*
*Completed: 2026-04-29*
