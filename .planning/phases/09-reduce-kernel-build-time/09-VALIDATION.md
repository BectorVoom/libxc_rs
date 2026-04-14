---
phase: 9
slug: reduce-kernel-build-time
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-14
---

# Phase 9 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test / cargo check / cargo build |
| **Config file** | Cargo.toml (workspace root) |
| **Quick run command** | `cargo check --workspace` |
| **Full suite command** | `cargo build --workspace && cargo test --workspace` |
| **Estimated runtime** | ~120 seconds (check), ~300 seconds (full build) |

---

## Sampling Rate

- **After every task commit:** Run `cargo check --workspace`
- **After every plan wave:** Run `cargo build --workspace && cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 300 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 09-03-01 | 03 | 1 | BUILD-OPT-01 | — | N/A | build | `cargo check --workspace` | ✅ | ⬜ pending |
| 09-03-02 | 03 | 1 | BUILD-OPT-02 | — | N/A | build | `cargo build --features gga` | ✅ | ⬜ pending |
| 09-03-03 | 03 | 2 | BUILD-OPT-03 | — | N/A | build | `cargo build --features all-kernels` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Build time measurement | BUILD-OPT-02 | Requires timing cargo build | `time cargo build` vs `time cargo build --features gga` |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 300s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
