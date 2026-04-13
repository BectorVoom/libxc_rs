---
phase: 8
slug: rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-13
---

# Phase 8 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test + verify crate oracle comparisons |
| **Config file** | Cargo.toml (workspace) |
| **Quick run command** | `cargo check -p kernel-mgga 2>&1 | head -50` |
| **Full suite command** | `cargo test -p verify --test oracle_mgga -- --nocapture` |
| **Estimated runtime** | ~60 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check -p kernel-mgga 2>&1 | head -50`
- **After every plan wave:** Run `cargo test -p verify --test oracle_mgga -- --nocapture`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | TBD | — | N/A | integration | `cargo test -p verify --test oracle_mgga` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] MGGA translation tool compiles without errors
- [ ] At least one representative MGGA kernel compiles
- [ ] Oracle test infrastructure for MGGA is functional

*Existing infrastructure covers oracle comparison; tool and kernel compilation are the primary validation gates.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Generated code readability | N/A | Subjective quality | Review generated Rust code for patterns matching GGA translator output |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
