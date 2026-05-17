---
phase: 11-splitter-v2-unified-5k-cap
plan: 08
type: execute
wave: 4
depends_on: [11-07]
files_modified:
  - tools/audit_kernel_size.py
  - tools/audit_subcrate_collapse.sh
  - tools/audit_cube_launch.sh
  - CLAUDE.md
  - .planning/ROADMAP.md
  - .planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md
autonomous: true
requirements: []
user_setup: []

must_haves:
  truths:
    - "All 259 routed per-functional subcrates compile via `cargo build -p libxc-kernel-<func>` (per-`-p` sweep)"
    - "Per-functional-subcrate invariant confirmed: no numbered subcrates, family dirs are plain directories (D-10a audit passes)"
    - "5K line-cap confirmed: no file >5K lines in kernel tree (audit_kernel_size passes)"
    - "Launch budget confirmed per D-13: routed=1-per-output, unrouted=0, math/src≤22"
    - "Idempotency holds: regen re-run produces zero diff (P11-INV-6)"
    - "Phase success criteria updated in ROADMAP.md per D-12 reinterpretation"
    - "CLAUDE.md updated per D-03a (f64 policy revised)"
    - "Phase-level validation complete and documented"
  artifacts:
    - path: "tools/audit_kernel_size.py"
      provides: "5K line-cap audit (D-LOCK-B enforcement)"
      contains: "max_lines = 5000"
    - path: "tools/audit_subcrate_collapse.sh"
      provides: "Per-functional-subcrate invariant audit (D-10a)"
      contains: "per-functional-subcrate check, family-dir-plain check"
    - path: "tools/audit_cube_launch.sh"
      provides: "D-13 per-design launch budget (routed/unrouted/math assertions)"
      contains: "three assertions, not flat count"
    - path: "CLAUDE.md"
      provides: "Updated f64 policy per D-03a"
      contains: "f64 default for oracle gating, f32 opt-in no correctness"
    - path: ".planning/ROADMAP.md"
      provides: "Phase 11 success criteria corrected per D-12"
      contains: "Criterion #1 (per-functional), #4 (per-`-p` incremental)"
    - path: ".planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md"
      provides: "Phase 11 final metrics, gate audit results, close documentation"
  key_links:
    - from: "per-`-p` cargo build"
      to: "audit_kernel_size.py + audit_subcrate_collapse.sh + audit_cube_launch.sh"
      via: "gate verification"
      pattern: "cargo build -p"
    - from: "CLAUDE.md D-03a"
      to: "Phase 11 precision policy"
      via: "project-level constraint update"
      pattern: "f64 default"

---

<objective>
**What:** Execute per-`-p` incremental verification (D-12 reinterpretation), run phase-level audits, update project constraints, and close Phase 11.

**Purpose:** Validate all 259 routed per-functional subcrates compile independently, confirm all Phase 11 invariants hold, and document final metrics. Close the replan and hand off to Phase 10 (workspace modular split).

**Output:** Phase 11 DONE — per-functional subcrates ≤5K, all audits pass, no blockers for Phase 10.
</objective>

<execution_context>
@.planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md (D-12, D-13, D-03a, P11-INV-1..6)
@.planning/phases/11-splitter-v2-unified-5k-cap/11-03-SUMMARY.md (D-13 audit form)
@.planning/ROADMAP.md (Phase 11 success criteria, Phase 10 sequencing)
</execution_context>

<context>
@CLAUDE.md
@.planning/ROADMAP.md
@.planning/phases/11-splitter-v2-unified-5k-cap/11-BASELINE.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Per-`-p` subcrate verification sweep (D-12 incremental gates)</name>
  <files></files>
  <action>
Per D-12, build verification is per-subcrate, not a whole-workspace gate (the legacy `cargo build --workspace` OOM is the structural reason for per-functional subcrates). Run incremental builds:

**Process:**
```bash
# Get all 259 routed functional subcrates (exclude the 7 deferred)
ROUTED=$(python3 tools/kernel_routing.py --list-routed)

# Build each per-`-p` (in sequence, respecting jobs=1 from .cargo/config.toml)
for func in $ROUTED; do
  echo "Building libxc-kernel-$func..."
  cargo build -p libxc-kernel-$func 2>&1 | tee -a log/11-08-build-$func.log
  if [ $? -ne 0 ]; then
    echo "ERROR: cargo build -p libxc-kernel-$func failed"
    # Investigate and fix; do NOT hand-edit kernel files (AP-3)
    exit 1
  fi
done

# Summary
echo "All 259 routed subcrates built successfully"
```

**Failure recovery:** If any subcrate fails to compile, **do NOT hand-edit the kernel file**. Instead:
1. Identify the root cause (usually a translator bug or API drift).
2. Fix the translator (`tools/translate_v2/` or the family translator).
3. Re-run the splitter for that functional.
4. Rebuild.

This maintains P11-INV-6 (idempotency); hand edits break it.

**Optimization:** Incremental builds against `.cargo/config.toml` `target-dir = .cache/cargo-target` (D-09) reuse prior work; expect 20–40 minutes total for all 259 per-`-p` builds on this machine, not 20+ hours.

**Verify:** All build logs succeed; no per-`-p` exits with non-zero.
  </action>
  <verify>
    <automated>
      # Count successful builds
      grep -l "Finished" log/11-08-build-*.log | wc -l && \
      # Expected: 259 (one per routed functional)
      grep -l "error:" log/11-08-build-*.log | wc -l && echo "Expected: 0 error logs"
    </automated>
  </verify>
  <done>All 259 routed subcrates build successfully via per-`-p` incremental gates (D-12 form).</done>
</task>

<task type="auto">
  <name>Task 2: Run all three phase-level audit scripts (D-10a, D-13, D-LOCK-B)</name>
  <files></files>
  <action>
Execute the three audit gates that define Phase 11's invariants:

**Audit 1 — File size cap (D-LOCK-B):**
```bash
python3 tools/audit_kernel_size.py --strict 2>&1 | tee log/11-08-audit-size.log
# Expected exit 0: no file >5K lines (or documented exceptions per D-LOCK-B)
```

**Audit 2 — Per-functional-subcrate structure (D-10a):**
```bash
bash tools/audit_subcrate_collapse.sh 2>&1 | tee log/11-08-audit-collapse.log
# Expected exit 0:
#  - 0 numbered subcrates (lda-N, gga-N, mgga-N)
#  - 0 family-level crates (lda/Cargo.toml, gga/Cargo.toml, mgga/Cargo.toml)
#  - ≥260 per-functional subcrates present
#  - Family directories (lda/, gga/, mgga/) exist as plain folders
```

**Audit 3 — Launch budget per-design (D-13):**
```bash
bash tools/audit_cube_launch.sh 2>&1 | tee log/11-08-audit-launch.log
# Expected exit 0:
#  - Assertion 1: routed functionals have exactly 1 #[cube(launch_unchecked)] per output module
#  - Assertion 2: unrouted functionals have 0 launchables
#  - Assertion 3: crates/kernels/math/src (excluding tests) ≤22 launches
```

**Combined result:** If all three exit 0, all Phase 11 structural invariants hold.
  </action>
  <verify>
    <automated>
      python3 tools/audit_kernel_size.py --strict > /dev/null 2>&1 && echo "PASS: size audit" || echo "FAIL: size audit" && \
      bash tools/audit_subcrate_collapse.sh > /dev/null 2>&1 && echo "PASS: collapse audit" || echo "FAIL: collapse audit" && \
      bash tools/audit_cube_launch.sh > /dev/null 2>&1 && echo "PASS: launch audit" || echo "FAIL: launch audit"
    </automated>
  </verify>
  <done>All three audits pass: file-size cap, per-functional-subcrate structure, launch budget per-design.</done>
</task>

<task type="auto">
  <name>Task 3: Update CLAUDE.md per D-03a (f64 policy revision)</name>
  <files>CLAUDE.md</files>
  <action>
Update the CLAUDE.md constraints section to reflect the f64/f32 policy shift per D-03a.

**Find and replace in CLAUDE.md § "Constraints":**

**Old (pre-Phase-11):**
```
- **Precision:** f64 only; no silent f32 fallback; energy relative error <= 10^-12 vs libxc oracle
```

**New (Phase 11 +):**
```
- **Precision:** f64 is the default and the sole oracle-gating target; all kernel chunks compile generically over `<F: Float>` to enable f32 opt-in at launch time with no correctness guarantee. A typed error is returned at device-query time if the user selects f64 but the device lacks f64 support (WGPU concern). See Phase 11 decisions D-02, D-03, D-05 for rationale and constraints.
```

**Rationale:** Kernel chunks are now generic over `<F: Float>` per D-02 (Option A locked in 11-05). f32 is available as a launch-time choice but carries no oracle gate. This is a **controlled relaxation** of "no silent f32 fallback" — f32 is explicit, not silent; correctness is f64-only.

**Also update § "Operation order":**
Add: "CSE-aware chunking (Phase 11) may introduce named temporaries that legitimately reorder floating-point operations. Oracle gate is energy + all derivatives at 1e-12 relative error, not bit-exact f64 accumulation."
  </action>
  <verify>
    <automated>grep -A 5 "f64 is the default" CLAUDE.md | head -8</automated>
  </verify>
  <done>CLAUDE.md updated per D-03a (f64 default, f32 opt-in, no correctness gate for f32).</done>
</task>

<task type="auto">
  <name>Task 4: Update ROADMAP.md Phase 11 success criteria per D-12 reinterpretation</name>
  <files>.planning/ROADMAP.md</files>
  <action>
Update the Phase 11 "Success Criteria" section in `.planning/ROADMAP.md` to reflect D-12's reinterpretation of build verification.

**Find the Phase 11 section (around line 237) and update:**

**Old criterion #1:**
```
"only family-level crates" → (incomplete, from pre-2026-05-14)
```

**New criterion #1:**
```
✓ **P11-INV-1 (per-functional-subcrate structure):** `find crates/kernels -maxdepth 1 -type d` shows **no numbered** `lda-N/gga-N/mgga-N` children and **no Cargo.toml** in `crates/kernels/{lda,gga,mgga}/` — only per-functional crates `crates/kernels/{lda,gga,mgga}/<func>/`. Family dirs are plain folders.
```

**Old criterion #4:**
```
"`cargo build --workspace` succeeds on the user's RAM-constrained machine"
```

**New criterion #4 (D-12 reinterpretation):**
```
✓ **P11-INV-D12 (per-`-p` incremental gates, not whole-workspace):** `cargo build --workspace` is currently OOMing due to the kernel-crate sprawl. Per D-12, build verification is **per-subcrate** via `cargo build -p libxc-kernel-<func>` incremental loops. All 259 routed subcrates compile successfully via per-`-p` gates. Whole-workspace build verification is deferred to Phase 10 (workspace modular split) after per-functional subcrates are decomposed into logical layers (libxc-core, libxc-eval, libxc-compat).
```

**Also add a new criterion for D-15:**
```
✓ **P11-INV-D15 (compile-first entry gate):** The canary functional (`mgga_c_b94`) passes a three-leg compile-first gate: kernel subcrate compiles, dispatch tree expands, oracle parity at ≥1e-12 on energy + routed derivatives.
```

**Update criterion #7 reference:**
```
Old: "CubeCL macro fan-out audit clean: `#[cube(launch)]` count does not increase from pre-phase baseline"
New: "CubeCL macro fan-out audit per D-13: routed functionals have exactly 1 `#[cube(launch_unchecked)]` per output module; unrouted functionals have 0; `crates/kernels/math/src/` ≤22. Total routed launch count ~1654 across all subcrates, sound under per-functional-subcrate isolation (each subcrate compiles independently with its own ~10 launches, not all ~1654 in one invocation)."
```
  </action>
  <verify>
    <automated>grep -A 3 "per-\`-p\`" .planning/ROADMAP.md | head -5</automated>
  </verify>
  <done>ROADMAP.md updated: success criteria #1, #4, and #7 reworded per Phase 11 locked decisions (D-10a, D-12, D-13, D-15).</done>
</task>

<task type="auto">
  <name>Task 5: Create final 11-08-SUMMARY.md and document phase close</name>
  <files>.planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md</files>
  <action>
Create the final phase-level SUMMARY documenting metrics, gates, and outcomes.

**Content structure (see 11-BASELINE.md for pre-phase comparison):**

```markdown
---
phase: 11-splitter-v2-unified-5k-cap
plan: 08
type: documentation
completed: 2026-05-18 (date TBD at execution)
---

# Phase 11 Completion Summary

## Metrics

| Metric | Pre-Phase | Post-Phase | Status |
|--------|-----------|-----------|--------|
| Numbered subcrates | 27 | 0 | ✓ D-10a (all deleted) |
| Family-level crates | 3 | 0 | ✓ D-10a (all deleted, dirs remain) |
| Per-functional subcrates | 0 | 266 | ✓ D-10 (one per functional) |
| Files >5K lines | 235 | 0 | ✓ D-LOCK-B (5K cap) |
| Max file line count | 16,703 | ≤5,000 | ✓ CSE chunking |
| Dispatch staleness (batchN refs) | 18 (B1) | 0 | ✓ Blocker B1 closed |
| Launch-unchecked routed count | 22→23 (old flat) | 1,654 (per-design) | ✓ D-13 reinterpretation |
| Workspace members (kernel-related) | 35 | ≥266 | ✓ Per-functional count |

## Gates Locked

| Gate | Criterion | Status |
|------|-----------|--------|
| **P11-INV-A1** (D-02 spike) | Tuple-return + `<F: Float>` round-trip | ✓ PASS (11-01) |
| **P11-INV-1** (per-functional structure) | No numbered, no family crates | ✓ PASS (11-03 re-confirmation, 11-08 audit) |
| **P11-INV-2** (CSE splitter) | compute-line partitioning + tuple-return chunking | ✓ PASS (11-02) |
| **P11-INV-5** (launch budget per-design) | routed 1-per-output, unrouted 0, math ≤22 | ✓ PASS (11-03 + 11-08) |
| **P11-INV-6** (idempotency) | regen re-run produces zero diff | ✓ PASS (11-07) |
| **P11-INV-D12** (per-`-p` incremental) | All 259 routed subcrates compile via per-`-p` | ✓ PASS (11-08) |
| **P11-INV-D13** (launch audit) | Three-assertion per-design budget | ✓ PASS (11-08) |
| **P11-INV-D15** (compile-first entry gate) | mgga_c_b94 kernel + dispatch + parity 1e-12 | ✓ PASS (11-07) |

## Decisions Locked

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **D-02** (chunk ABI) | Option A: generic `<F: Float>` helpers | Simpler, no call-site wrapping, per cubecl_macro_fanout_manual §6 |
| **D-03** (precision) | f64 default + oracle gate; f32 opt-in no gate | Controlled relaxation, explicit f32, no silent fallback |
| **D-05** (verify gate) | Narrowed dev-deps + per-iteration smoke | Structural OOM fix + practical iter gates |
| **D-10** (emission target) | Per-functional subcrates, not per-family | Per-functional isolation fixes OOM at compilation-unit boundary |
| **D-11** (deferred kernels) | 7 deferred omitted from default-members | Cargo workspace mechanism, not features |
| **D-13** (launch budget) | Per-design 3 assertions vs flat ≤23 | Matches per-functional-subcrate isolation benefit |

## Deliverables Locked

| Item | Delivered | Status |
|------|-----------|--------|
| Splitter v2 (CSE + per-functional emit) | 11-02 | ✓ tools/translate_v2/ |
| 266 per-functional subcrates | 11-07 | ✓ crates/kernels/{lda,gga,mgga}/<func>/ |
| Dispatch tree (per-functional paths) | 11-07 | ✓ src/kernel/{lda,gga,mgga}.rs + src/eval/*dispatch/ |
| D-15 compile-first entry gate | 11-07 | ✓ mgga_c_b94 kernel+dispatch+parity PASS |
| CLAUDE.md f64 policy (D-03a) | 11-08 | ✓ Updated |
| ROADMAP.md success criteria | 11-08 | ✓ Reworded per D-12/D-13/D-15 |

## Blockers / Risks Closed

| Item | Pre-Phase | Status |
|------|-----------|--------|
| Kernel compile OOM (`cargo check --workspace`) | Blocking Phase 4-9 | ✓ **FIXED** by per-functional subcrate isolation |
| Dispatch tree staleness (Blocker B1) | 18 unresolved batchN refs | ✓ **CLOSED** in 11-03, regenerated in 11-07 |
| Oversized kernel files (8–15K lines) | 235 files >5K | ✓ **FIXED** by CSE chunking (0 files >5K) |
| D-02 ABI mismatch (helper-layer blocker) | Blocking 11-04 onwards | ✓ **RESOLVED** by refactoring 38 helpers to generic (11-05) |

## Anti-Patterns Prevented (AP-1..6)

| AP | Pattern | Prevention |
|----|---------|-----------|
| **AP-1** | Re-execute without replanning | ✓ Replan locked D-14..D-17; compile-first gates in every plan |
| **AP-2** | Modify `.cargo/config.toml` | ✓ D-07/08/09 (jobs=1, RUST_MIN_STACK, sccache) preserved in all tasks |
| **AP-3** | Hand-edit kernel files | ✓ D-LOCK-D idempotency rule; all fixes via translator, not manual edits |
| **AP-4** | Revert q01 emit fixes | ✓ q01 commit `5c379dc25` preserved; D-16 builds on it |
| **AP-5** | Redo 11-01/02/03 work | ✓ SUMMARYs preserved; replan reframes 11-04..08 only |
| **AP-6** | Declare completion without per-`-p` gates | ✓ Every plan has compile-first or per-`-p` entry gate |

## Forward Dependencies

**Phase 10 (Workspace Modular Split)** depends on Phase 11 completing. The per-functional-subcrate structure is a cleaner foundation for Phase 10's libxc-core / libxc-eval / libxc-compat split.

## Retention for Future Review

- 11-BASELINE.md (pre-phase snapshot)
- 11-CONTEXT.md (locked decisions D-01..D-17)
- 11-DISCUSSION-LOG.md (discuss-phase transcript)
- All 11-01..08 SUMMARY files (execution history)
```

**Write this to the file, adding actual execution dates and metrics as the executor runs Task 1–4.**
  </action>
  <verify>
    <automated>test -f .planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md && wc -l .planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md</automated>
  </verify>
  <done>11-08-SUMMARY.md created documenting final phase metrics, gates, decisions, deliverables, and forward dependencies.</done>
</task>

<task type="auto">
  <name>Task 6: Commit all updates (CLAUDE.md, ROADMAP.md, SUMMARY)</name>
  <files></files>
  <action>
Commit the phase-closure documentation and policy updates:

```bash
git add CLAUDE.md .planning/ROADMAP.md .planning/phases/11-splitter-v2-unified-5k-cap/11-08-SUMMARY.md

git commit -m "docs(phase-11-close): final metrics, gate audits, policy updates (D-03a, D-12)

Phase 11 complete — all 266 per-functional subcrates emitted at ≤5K lines per file.

Per D-12: per-\`-p\` incremental verification (259 routed subcrates build) replaces whole-workspace gate (OOMing). Gate audits pass: size cap (D-LOCK-B), subcrate structure (D-10a), launch budget per-design (D-13).

Per D-03a: CLAUDE.md updated — f64 default + oracle gate, f32 opt-in no gate.
Per D-12: ROADMAP.md success criteria reworded for per-\`-p\` incremental model.

Blockers B1 (dispatch staleness) and compile OOM both closed.

Ready for Phase 10 (workspace modular split).

Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>"
```
  </action>
  <verify>
    <automated>git log --oneline -1</automated>
  </verify>
  <done>Phase 11 closed and committed; ready to hand off to Phase 10.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| Per-`-p` build integrity | Each subcrate must build independently without cross-subcrate state. |
| Audit determinism | All three audits must be reproducible (not flaky). |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-11-08-01 | I | Per-`-p` build failure on subset of subcrates | mitigate | Investigate root cause (translator bug or API drift); fix and re-run. Do not hand-edit kernel files. |
| T-11-08-02 | R | Audit false negatives (pass when shouldn't) | mitigate | Verify audit scripts logic carefully; spot-check audit results by hand. |
| T-11-08-03 | I | ROADMAP/CLAUDE.md update introduces contradiction | mitigate | Cross-check updated text against 11-CONTEXT.md decisions. |
</threat_model>

<verification>
- [ ] All 259 routed subcrates compile via per-`-p` (zero failures)
- [ ] Audit 1 (size): zero files >5K
- [ ] Audit 2 (structure): zero numbered, zero family crates, ≥266 per-functional
- [ ] Audit 3 (launch): routed 1-per-output, unrouted 0, math≤22
- [ ] CLAUDE.md updated per D-03a
- [ ] ROADMAP.md updated per D-12/D-13/D-15
- [ ] 11-08-SUMMARY.md created with final metrics
- [ ] All changes committed
</verification>

<success_criteria>
1. **All 259 routed subcrates build successfully** via per-`-p` incremental gates (D-12).
2. **All three audits pass:** file-size cap (D-LOCK-B), per-functional-subcrate structure (D-10a), launch budget per-design (D-13).
3. **Project constraints updated:** CLAUDE.md (D-03a precision policy), ROADMAP.md (D-12/D-15 success criteria).
4. **Phase 11 fully closed:** Final metrics documented, gates locked, decisions captured.
5. **No regression:** All prior deliverables (11-01..07) intact; Phase 10 can begin.
</success_criteria>

<output>
After completion, the phase directory contains:
- 11-01-SUMMARY.md through 11-08-SUMMARY.md (execution history)
- 11-CONTEXT.md (locked decisions)
- 11-BASELINE.md (pre-phase metrics)
- 11-DISCUSSION-LOG.md (user decisions)

User can now proceed to Phase 10 (Workspace Modular Split).
</output>
