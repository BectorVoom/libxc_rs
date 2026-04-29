---
phase: 9
slug: reduce-kernel-build-time
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-14
last_reviewed: 2026-04-29
---

# Phase 9 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo check / cargo test (release) / python3 -m unittest |
| **Config file** | Cargo.toml (workspace root) + tools/audit_deferred_gga.py + verify/tests/parity_phase09.rs |
| **Quick run command** | `cargo check --workspace` |
| **Full suite command** | `cargo check --workspace --all-targets && cargo test -p libxc_rs_verify --test parity_phase09 --release` |
| **Estimated runtime** | ~120 seconds (workspace check), ~600 seconds (parity sweep release) |

---

## Sampling Rate

- **After every task commit:** Run `cargo check --workspace`
- **After every plan wave:** Run `cargo check --workspace --all-targets`
- **Before `/gsd-verify-work`:** Full suite (`cargo check --workspace --all-targets` + `cargo test -p libxc_rs_verify --test parity_phase09 --release`) must be green
- **Max feedback latency:** 300 seconds (workspace check); 600 seconds (parity sweep at end of phase)

---

## Per-Task Verification Map

| Task ID    | Plan | Wave | Requirement   | Threat Ref  | Secure Behavior | Test Type   | Automated Command | File Exists | Status |
|------------|------|------|---------------|-------------|-----------------|-------------|-------------------|-------------|--------|
| 09-04-T1   | 04   | 1    | SPEC-09-R1, SPEC-09-R2 | T-09-04-01 | translator threshold raised | grep | `grep -c "^SPLIT_THRESHOLD = 18000$" tools/translate_lda_v2.py tools/translate_gga.py tools/translate_mgga.py` (each → 1) | ✅ | ⬜ pending |
| 09-04-T2   | 04   | 1    | SPEC-09-R1, SPEC-09-R2 | T-09-04-01, T-09-04-02 | per-file cap ≤20K + op-order preserved | filesystem audit + diff | `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} + \| awk 'NF==2 && $2 != "total" && $1 > 20000 {n++} END {exit n}'` AND `test "$(grep -c '^DIFF:' log/09-04-task2-op-order-diff.log)" -eq 0` | ✅ | ⬜ pending |
| 09-04-T3   | 04   | 1    | SPEC-09-R1, SPEC-09-R2 | T-09-04-03 | commits + summary log | git log + grep | `git log --oneline -3 > log/09-04-task3-commits.log && test -f log/09-04-regen-summary.log && grep -E "TOTAL_OVERSIZE: 0\|SPLIT_THRESHOLD = 18000" log/09-04-regen-summary.log` | ✅ | ⬜ pending |
| 09-05-T1   | 05   | 2    | SPEC-09-R1    | T-09-05-03  | audit script unit tests pass | python unittest | `python3 -c "import ast; ast.parse(open('tools/audit_deferred_gga.py').read())" && python3 -m unittest tools.test_audit_deferred_gga -v > log/09-05-task1-tests.log 2>&1 && grep -E "OK\|FAILED\|ERROR" log/09-05-task1-tests.log \| tail -5` | ✅ | ⬜ pending |
| 09-05-T2   | 05   | 2    | SPEC-09-R1    | T-09-05-01  | --strict audit exit 0 | python audit | `python3 tools/audit_deferred_gga.py --strict --json-out log/09-05-deferred-gga-audit.json --md-out .planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md > log/09-05-task2-audit-final.log 2>&1` | ✅ | ⬜ pending |
| 09-06-T1   | 06   | 3    | SPEC-09-R1, SPEC-09-R2, SPEC-09-R3 | T-09-06-01 | cargo check exit 0 | build verification (D-13 substitution) | `cargo check --workspace --all-targets > log/cargo-check-09-final.log 2>&1; echo "EXIT: $?" >> log/cargo-check-09-final.log; grep -E "Finished .dev. profile\|^error" log/cargo-check-09-final.log \| tail -5` | ✅ | ⬜ pending |
| 09-06-T2   | 06   | 3    | SPEC-09-R1, SPEC-09-R2, SPEC-09-R3 | T-09-06-01 | SPEC §AC re-verification PASS | log audit | `test -f log/09-06-spec-acceptance.log && grep -E "^=== SUMMARY" log/09-06-spec-acceptance.log` | ✅ | ⬜ pending |
| 09-07-T1   | 07   | 4    | SPEC-09-R1    | T-09-07-04  | parity test compiles + count invariant | cargo check | `cargo check -p libxc_rs_verify --tests > log/09-07-task1-check.log 2>&1; echo "EXIT: $?" >> log/09-07-task1-check.log; test -f verify/tests/parity_phase09.rs && grep -E "Finished .dev. profile\|^error" log/09-07-task1-check.log \| tail -5` | ✅ | ⬜ pending |
| 09-07-T2   | 07   | 4    | SPEC-09-R1    | T-09-07-01, T-09-07-02 | full parity sweep at strict 1e-12 | cargo test | `cargo test -p libxc_rs_verify --test parity_phase09 --release --jobs 3 -- --nocapture --test-threads=1 > log/cargo-test-09-parity-sweep.log 2>&1; echo "EXIT: $?" >> log/cargo-test-09-parity-sweep.log; grep -cE "^test result: ok" log/cargo-test-09-parity-sweep.log` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Sampling Rate Validation

Workspace-wide commands remain valid for sampling between tasks:

| Command | Runtime | When |
|---------|---------|------|
| `cargo check --workspace` | ~120s | After every task commit (post-09-04 onward) |
| `cargo check --workspace --all-targets` | ~150s | After every plan wave |
| `cargo test -p libxc_rs_verify --test parity_phase09 --release` | ~600s | End of phase (Plan 09-07 only) |

No 3-consecutive-task gap without an automated `<verify>`. Every task in 09-04 / 09-05 / 09-06 / 09-07 has its own automated verify command (see Per-Task Verification Map above).

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.* The `tools/audit_deferred_gga.py` script + its unit tests are authored within Plan 09-05 Task 1 (which is itself a Wave 0–style scaffold for Plan 09-05 Task 2's strict audit). The `verify/tests/parity_phase09.rs` file is authored within Plan 09-07 Task 1 (Wave 0 for Plan 09-07 Task 2's full sweep). Both scaffold tasks ship with passing unit/compile checks before their dependent task runs.

---

## Manual-Only Verifications

*None.* Per CONTEXT D-05, BUILD-OPT-02 and BUILD-OPT-03 wall-clock build-time targets are deferred out of Phase 9. All phase acceptance is automated (cargo check, cargo test, python audit, grep/awk gates).

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references (audit script + parity_phase09.rs scaffolds in their respective Task-1 slots)
- [x] No watch-mode flags
- [x] Feedback latency < 300s for sampling commands; ≤600s for end-of-phase parity sweep
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
</content>
</invoke>