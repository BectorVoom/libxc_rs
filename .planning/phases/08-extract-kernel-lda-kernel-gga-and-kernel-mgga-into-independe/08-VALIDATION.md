---
phase: 8
slug: extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
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
| **Framework** | cargo test (Rust built-in) |
| **Config file** | Cargo.toml (workspace root) |
| **Quick run command** | `cargo check --workspace` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~120 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check --workspace`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 08-01-01 | 01 | 1 | N/A | — | N/A | build | `cargo check --workspace` | ✅ | ⬜ pending |
| 08-01-02 | 01 | 1 | N/A | — | N/A | build | `cargo check -p libxc-kernel-math` | ❌ W0 | ⬜ pending |
| 08-02-01 | 02 | 1 | N/A | — | N/A | build | `cargo check -p libxc-kernel-lda` | ❌ W0 | ⬜ pending |
| 08-03-01 | 03 | 1 | N/A | — | N/A | build | `cargo check -p libxc-kernel-gga` | ❌ W0 | ⬜ pending |
| 08-04-01 | 04 | 1 | N/A | — | N/A | build | `cargo check -p libxc-kernel-mgga` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/kernel-math/Cargo.toml` — new crate scaffold
- [ ] `crates/kernel-lda/Cargo.toml` — new crate scaffold
- [ ] `crates/kernel-gga/Cargo.toml` — new crate scaffold
- [ ] `crates/kernel-mgga/Cargo.toml` — new crate scaffold
- [ ] Workspace root `Cargo.toml` updated with new members

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Re-export paths match original public API | N/A | Requires checking downstream usage | Verify `libxc_rs::kernel::lda::*` still resolves |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
