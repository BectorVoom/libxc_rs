---
phase: 11-splitter-v2-unified-5k-cap
plan: 07
type: execute
wave: 3
depends_on: [11-06]
files_modified:
  - crates/kernels/lda/
  - crates/kernels/gga/
  - crates/kernels/mgga/
  - Cargo.toml
  - src/kernel/lda.rs
  - src/kernel/gga.rs
  - src/kernel/mgga.rs
  - src/eval/lda_dispatch/mod.rs
  - src/eval/gga_dispatch/mod.rs
  - src/eval/mgga_dispatch/mod.rs
autonomous: false
requirements: []
user_setup: []

must_haves:
  truths:
    - "266 per-functional subcrates emitted from Maple, all files ≤5K per D-LOCK-B"
    - "mgga_c_b94 kernel subcrate compiles per `cargo build -p libxc-kernel-mgga_c_b94`"
    - "libxc_rs dispatch tree compiles per `cargo build -p libxc_rs` (dispatch macros expand correctly)"
    - "mgga_c_b94 parity vs oracle at ≥1e-12 relative error (one-shot deferred bypass per D-14)"
    - "Idempotency confirmed: splitter re-run produces zero git diff"
    - "No numbered subcrates remain; family dirs are plain directories"
  artifacts:
    - path: "crates/kernels/lda/"
      provides: "~41 per-functional subcrates (family directory, no Cargo.toml)"
    - path: "crates/kernels/gga/"
      provides: "~131 per-functional subcrates (family directory, no Cargo.toml)"
    - path: "crates/kernels/mgga/"
      provides: "~92 per-functional subcrates, including 7 deferred (family directory, no Cargo.toml)"
    - path: "Cargo.toml"
      provides: "Root manifest with ~266 per-functional subcrate deps, deferred omitted from default-members per D-11"
    - path: "src/kernel/(lda|gga|mgga).rs"
      provides: "Façade re-exporting per-functional subcrates"
      contains: "pub use libxc_kernel_<func>::*"
    - path: "src/eval/(lda|gga|mgga)_dispatch/"
      provides: "Dispatch tree routing evaluation to per-functional subcrates"
      contains: "ten_arm_dispatch_gga! / mgga_zero_scalar_unpol_dispatch!"
  key_links:
    - from: "tools/maple_to_kernels.py translate"
      to: "crates/kernels/{lda,gga,mgga}/"
      via: "per-functional subcrate emission"
      pattern: "mkdir -p"
    - from: "src/kernel/{lda,gga,mgga}.rs"
      to: "crates/kernels/{lda,gga,mgga}/<func>/"
      via: "re-export paths"
      pattern: "pub use libxc_kernel_<func>"
    - from: "src/eval/{gga,mgga}_dispatch/"
      to: "crate::kernel::{family}::<func>::*"
      via: "dispatch macro calls"
      pattern: "ten_arm_dispatch_gga!"

---

<objective>
**What:** Execute the full 266-subcrate clean-slate regen from Maple using the splitter-v2 tooling (with D-02 Option A ABI locked). Verify the compile-first entry gate (D-15) on mgga_c_b94: kernel compile + dispatch compile + oracle parity at 1e-12.

**Purpose:** This is the structural implementation of Phase 11's core value: per-functional subcrates (~264 total) with all files ≤5K. The entry gate (D-15) proves the architecture is sound before the per-`-p` sweep (11-08) scales to all subcrates.

**Output:** Regen complete; D-15 three-leg gate PASS; Blocker B1 closed; ready for 11-08 full sweep.
</objective>

<execution_context>
@.planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md (D-10, D-10a, D-10b, D-11, D-12, D-14, D-15)
@.planning/phases/11-splitter-v2-unified-5k-cap/11-03-SUMMARY.md (prior dispatch structure)
@.planning/quick/260514-q01-split-mgga-2-large-kernels/ (nested-by-output layout reference for mgga_c_b94)
</execution_context>

<context>
@CLAUDE.md
@.planning/ROADMAP.md
@.planning/phases/11-splitter-v2-unified-5k-cap/11-BASELINE.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Delete old kernel layout (27 numbered subcrates + 3 family crates)</name>
  <files>
    crates/kernels/lda-1/
    crates/kernels/lda-2/
    crates/kernels/gga-1/ through crates/kernels/gga-8/
    crates/kernels/mgga-1/ through crates/kernels/mgga-14/ (incl. 8a, 8b, 9a, 9b, 11a, 11b)
    crates/kernels/lda/Cargo.toml
    crates/kernels/lda/lib.rs
    crates/kernels/gga/Cargo.toml
    crates/kernels/gga/lib.rs
    crates/kernels/mgga/Cargo.toml
    crates/kernels/mgga/lib.rs
  </files>
  <action>
Per D-10a clean-slate, delete:
1. All 27 numbered subcrates (`lda-1`, `lda-2`, `gga-1..8`, `mgga-1..14, 8a, 8b, 9a, 9b, 11a, 11b`)
2. The three family-level `Cargo.toml` and `lib.rs` files (`crates/kernels/{lda,gga,mgga}/Cargo.toml`, `lib.rs`)
3. All path-dependencies and workspace members in root `Cargo.toml` for these 30 artifacts

**Steps:**
```bash
# Delete numbered subcrates
rm -rf crates/kernels/lda-{1,2}
find crates/kernels/gga-* -maxdepth 0 -type d -exec rm -rf {} \;
find crates/kernels/mgga-* -maxdepth 0 -type d -exec rm -rf {} \;

# Delete family crate files (not dirs — dirs stay as plain folders)
rm -f crates/kernels/{lda,gga,mgga}/{Cargo.toml,lib.rs}

# Rewrite root Cargo.toml:
# - Remove all libxc-kernel-lda*, libxc-kernel-gga*, libxc-kernel-mgga* deps
# - Remove numbered subcrates from [workspace] members + default-members
```

**Verify:** `git status` shows only deletions; no per-functional subcrates created yet.
  </action>
  <verify>
    <automated>
      find crates/kernels -maxdepth 1 -name 'lda-*' -o -name 'gga-*' -o -name 'mgga-*' | wc -l && echo "Expected: 0"
    </automated>
  </verify>
  <done>All 27 numbered subcrates and 3 family crates deleted; root Cargo.toml cleaned; family dirs exist as plain folders.</done>
</task>

<task type="auto">
  <name>Task 2: Run full regen via splitter-v2 → 266 per-functional subcrates</name>
  <files>
    crates/kernels/lda/lda_*/ (41 new)
    crates/kernels/gga/gga_*/ (131 new)
    crates/kernels/mgga/mgga_*/ (92 new)
  </files>
  <action>
**Invoke the translator in clean-slate mode per D-10a:**

```bash
cd /home/user/Documents/workspace/libxc_rs

# Full regen of all three families (LDA, GGA, MGGA)
python3 tools/maple_to_kernels.py translate --family lda --family gga --family mgga

# Expected output:
# - crates/kernels/lda/ contains ~41 subdirectories (one per LDA functional)
# - crates/kernels/gga/ contains ~131 subdirectories (one per GGA functional)
# - crates/kernels/mgga/ contains ~92 subdirectories (one per MGGA functional)
# Total: 266 per-functional Cargo crates
```

**Per D-LOCK-D idempotency:** If regen produces any files that, when re-run, differ from the first run, those are bugs in the translator — **do not hand-edit kernel files**. Fix the translator instead (per AP-3).

**Output check:** `find crates/kernels -maxdepth 2 -name Cargo.toml | wc -l` should be ≥260 (per-functional subcrates + math + shared).
  </action>
  <verify>
    <automated>
      find crates/kernels -maxdepth 2 -name Cargo.toml | wc -l && \
      find crates/kernels -name '*.rs' -exec wc -l {} \; | awk '{sum+=$1} END {print "Total lines in kernels:", sum}'
    </automated>
  </verify>
  <done>266 per-functional subcrates emitted with nested-by-output layout; total code lines within budget.</done>
</task>

<task type="auto">
  <name>Task 3: Update root Cargo.toml with per-functional deps and default-members (D-11)</name>
  <files>Cargo.toml</files>
  <action>
**Rewrite the root `Cargo.toml`** to reference the new per-functional subcrates:

1. **Add per-functional path-deps** (in `[dependencies]` section):
   ```toml
   libxc-kernel-lda_x = { path = "crates/kernels/lda/lda_x" }
   libxc-kernel-lda_c_pw = { path = "crates/kernels/lda/lda_c_pw" }
   # … continue for all 264 routed functionals (41 LDA + 131 GGA + 92 MGGA)
   # Deferred kernels (D-11) also as path-deps but omitted from default-members
   libxc-kernel-mgga_c_b94 = { path = "crates/kernels/mgga/mgga_c_b94" }
   # … other deferred
   ```

2. **Update `[workspace]` section:**
   - `members` = all ~266 per-functional subcrates + math + shared (all Cargo crates under crates/kernels/)
   - `default-members` = all except the 7 deferred per D-11:
     - `mgga_c_b94`
     - `mgga_x_br89`
     - `mgga_x_mbr`
     - `mgga_x_mbrxc_bg`
     - `mgga_x_mbrxh_bg`
     - `mgga_xc_b97m_v`
     - `mgga_x_br89_explicit` (unrouted, per D-11)

3. **Mechanically:** The simplest approach is a Python/shell script that:
   - `find crates/kernels -maxdepth 2 -name Cargo.toml | grep -v math | grep -v shared`
   - Extract functional names from paths (`crates/kernels/<family>/<func>/Cargo.toml` → `<func>`)
   - Generate the `[dependencies]` block
   - Generate the `[workspace]` members/default-members arrays
   - Splice into the root Cargo.toml

**Verify:** 
```bash
# Check workspace members count
grep -A 300 '\[workspace\]' Cargo.toml | grep 'members\s*=' -A 200 | wc -l
# Expected: ~268 (264 functional + math + shared + lda/gga/mgga family dirs don't exist as members, only subcrates)

# Check deps count
grep 'libxc-kernel-' Cargo.toml | wc -l
# Expected: ~266 deps

# Check default-members excludes deferred
grep -A 200 'default-members' Cargo.toml | grep -c 'mgga_c_b94'
# Expected: 0 (deferred omitted)
```
  </action>
  <verify>
    <automated>
      grep 'libxc-kernel-' Cargo.toml | wc -l && \
      grep -A 300 '\[workspace\]' Cargo.toml | grep -E '^\s+"crates/kernels' | wc -l
    </automated>
  </verify>
  <done>Root Cargo.toml updated with 266 per-functional path-deps and workspace members; deferred kernels excluded from default-members per D-11.</done>
</task>

<task type="checkpoint:decision" gate="blocking">
  <decision>Build and verify D-15 compile-first entry gate on mgga_c_b94 (three-leg gate per D-15)</decision>
  <context>
The D-15 entry gate (D-14's spike validation) has three requirements:
1. **Kernel compile:** `cargo build -p libxc-kernel-mgga_c_b94` succeeds
2. **Dispatch compile:** `cargo build -p libxc_rs` succeeds (full registry + macros expand)
3. **Parity:** mgga_c_b94 oracle energy + routed derivatives at ≥1e-12 relative error (one-shot deferred bypass allowed per D-14)

**If any leg fails:** The D-02 Option A choice is broken, and a third `/gsd-discuss-phase 11` is needed (per D-14 time-box). Stop the plan and document the failure mode.

**If all three pass:** Lock D-02, proceed to Task 5 (regen idempotency) and Task 6 (Task gates).
  </context>
  <options>
    <option id="gate-pass">
      <name>D-15 Gate PASS — all three legs compile and parity passes</name>
      <action>Proceed to Task 5 idempotency check</action>
    </option>
    <option id="gate-fail-compile">
      <name>D-15 Gate FAIL — kernel or dispatch compile fails</name>
      <action>Stop phase, escalate to third discuss-phase. Document error and failure mode in `.continue-here.md`</action>
    </option>
    <option id="gate-fail-parity">
      <name>D-15 Gate FAIL — parity fails (≥1e-13 error)</name>
      <action>Stop phase, escalate to third discuss-phase. Investigate whether D-02 ABI mismatch or numerical bug</action>
    </option>
  </options>
  <resume-signal>Select "gate-pass", or describe the failure for escalation</resume-signal>
</task>

<task type="auto">
  <name>Task 5: Verify idempotency (D-LOCK-D) — regen produces zero diff</name>
  <files></files>
  <action>
Per D-LOCK-D idempotency requirement, re-run the splitter and confirm the output is byte-identical:

```bash
# Checkpoint: stash current kernel tree
git stash

# Re-run full regen
python3 tools/maple_to_kernels.py translate --family lda --family gga --family mgga

# Check diff against what we just stashed
git diff crates/kernels/ | wc -l
# Expected: 0 (zero diff)

# If diff exists:
git diff crates/kernels/ | head -50
# Investigate what changed and why; splitter bug if anything changed

# Restore if needed
git stash pop
```

**Rationale:** The splitter is deterministic; running it twice must produce identical output. If idempotency fails, there is non-determinism in the splitter (e.g., dict ordering, random seeds, file glob order) that must be fixed before Phase 11 closes (P11-INV-6).
  </action>
  <verify>
    <automated>
      # After re-running regen and checking diff:
      git diff --stat crates/kernels/ | wc -l && echo "0 expected"
    </automated>
  </verify>
  <done>Idempotency confirmed: splitter re-run produces zero diff (P11-INV-6 gate).</done>
</task>

<task type="auto">
  <name>Task 6: Regenerate dispatch tree (D-10b) against per-functional subcrates</name>
  <files>
    src/kernel/lda.rs
    src/kernel/gga.rs
    src/kernel/mgga.rs
    src/eval/lda_dispatch/mod.rs
    src/eval/gga_dispatch/mod.rs
    src/eval/mgga_dispatch/mod.rs
  </files>
  <action>
The dispatch tree generators (`tools/generate_*_dispatch.py` and `tools/generate_kernel_reexports.py`) must be re-run to emit the per-functional subcrate re-exports and routing tables.

```bash
# Run the three generators
python3 tools/generate_kernel_reexports.py
python3 tools/generate_gga_dispatch.py
python3 tools/generate_mgga_dispatch.py

# Expected output:
# - src/kernel/lda.rs: per-functional re-exports
# - src/kernel/gga.rs: per-functional re-exports (no batchN segments)
# - src/kernel/mgga.rs: per-functional re-exports (no batchN segments)
# - src/eval/gga_dispatch/*.rs: dispatch routing (one file per functional or grouped)
# - src/eval/mgga_dispatch/*.rs: dispatch routing
```

**Verify:** `audit_dispatch_tree.sh` exits 0 (no stale batchN references). (Note: this is a trivial pass post-collapse — the real proof is the path-resolution gate in Task 7.)
  </action>
  <verify>
    <automated>bash tools/audit_dispatch_tree.sh && echo "PASS: dispatch tree audit"</automated>
  </verify>
  <done>Dispatch tree regenerated against per-functional subcrates; no batchN references remain.</done>
</task>

<task type="auto">
  <name>Task 7: Commit all changes (atomic)</name>
  <files></files>
  <action>
Commit the clean-slate delete + regen + manifest rewrite as a single atomic commit:

```bash
git add -A
git commit -m "feat(11-07): full 266-per-functional-subcrate regen + D-15 compile gate

Per D-10a clean-slate delete: remove 27 numbered + 3 family crates
Per D-10 emit: 266 per-functional subcrates via splitter-v2 (D-02 ABI locked)
Per D-11: deferred kernels omitted from default-members
Per D-10b: regenerate dispatch tree against per-functional paths
Per D-15: compile-first entry gate PASS (mgga_c_b94 + libxc_rs + oracle)
Per D-LOCK-D: idempotency confirmed (regen produces zero diff)

Closes Blocker B1 (dispatch staleness). Ready for 11-08 per-\`-p\` sweep.

Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>"
```
  </action>
  <verify>
    <automated>git log --oneline -1</automated>
  </verify>
  <done>All regen changes committed atomically; Blocker B1 closed; D-15 gate locked.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| Translator determinism | Splitter must be idempotent (D-LOCK-D). Non-determinism is a blocker. |
| Dispatch tree coverage | All per-functional subcrates must be re-exporting correctly (no missing paths). |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-11-07-01 | I | D-15 gate failure (compiler/linker error on mgga_c_b94) | **mitigate** | If compile fails, stop and escalate to third discuss-phase (D-14 time-box). |
| T-11-07-02 | R | Idempotency failure (regen produces diff on second run) | **mitigate** | Investigate and fix non-determinism in splitter before Phase 11 closes. |
| T-11-07-03 | I | Dispatch tree missing paths | **mitigate** | Path-resolution gate via `rustc --extern` spot-check. |
</threat_model>

<verification>
- [ ] 27 numbered subcrates deleted; 3 family crates deleted
- [ ] 266 per-functional subcrates emitted (find count ≥260)
- [ ] Root Cargo.toml updated with 266 deps and workspace members
- [ ] D-15 gate passes (kernel + dispatch compile + oracle parity ≥1e-12)
- [ ] Idempotency confirmed (zero diff on regen re-run)
- [ ] Dispatch tree regenerated, audit passes
- [ ] All changes committed atomically
</verification>

<success_criteria>
1. **Per-functional subcrates structure complete:** 266 functional directories under `crates/kernels/{lda,gga,mgga}/`, each is a Cargo crate with ≤5K per file.
2. **D-15 three-leg gate PASS:** mgga_c_b94 compiles, dispatch macros expand, oracle parity ≥1e-12.
3. **Blocker B1 closed:** Dispatch tree resolves against per-functional paths (verified by audit + spot-check).
4. **Idempotency confirmed:** Splitter is deterministic (P11-INV-6).
5. **No regression:** All prior deliverables (11-01..06) intact.
</success_criteria>

<output>
After completion, create `.planning/phases/11-splitter-v2-unified-5k-cap/11-07-SUMMARY.md`

Document:
- Regen metrics (subcrate count, max file size, total lines)
- D-15 gate results (compile times, oracle parity values)
- Idempotency confirmation
- Dispatch tree audit results
</output>
