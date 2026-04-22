---
phase: 4
slug: bulk-kernel-translation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-10
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` + cargo test |
| **Config file** | Cargo.toml (workspace) |
| **Quick run command** | `cargo test -p libxc_rs --lib -- kernel::lda` |
| **Full suite command** | `cargo test -p libxc_rs-verify` |
| **Estimated runtime** | ~60 seconds (LDA), ~300 seconds (full with GGA+MGGA) |

---

## Sampling Rate

- **After every task commit:** Run oracle test for the specific functional just translated
- **After every plan wave:** Run `cargo test -p libxc_rs-verify` (full family batch)
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds (per-functional oracle test)

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 01 | 1 | KERN-03 | — | N/A | integration | `cargo test -p libxc_rs-verify -- lda_oracle` | No -- Wave 0 | ⬜ pending |
| 04-02-01 | 02 | 2 | KERN-04 | — | N/A | integration | `cargo test -p libxc_rs-verify -- gga_oracle` | No -- Wave 0 | ⬜ pending |
| 04-03-01 | 03 | 3 | KERN-05 | — | N/A | integration | `cargo test -p libxc_rs-verify -- mgga_oracle` | No -- Wave 0 | ⬜ pending |
| 04-01-02 | 01 | 1 | KERN-06 | — | N/A | integration | Oracle comparison (implicit in KERN-03/04/05) | N/A | ⬜ pending |
| 04-01-03 | 01 | 1 | KERN-07 | — | N/A | unit | `cargo test -p libxc_rs --lib -- kernel::lda` | Partial (LDA_X) | ⬜ pending |
| 04-01-04 | 01 | 1 | KERN-08 | — | N/A | unit | Existing dispatch tests | Yes | ⬜ pending |
| 04-01-05 | 01 | 1 | KERN-09 | — | N/A | unit | Compilation check (each function exists) | N/A | ⬜ pending |
| 04-01-06 | 01 | 1 | VERIFY-02 | — | N/A | integration | All oracle tests pass | No -- Wave 0 | ⬜ pending |
| 04-01-07 | 01 | 1 | VERIFY-03 | — | N/A | integration | Oracle exc tests (tolerance <= 10^-12) | No -- Wave 0 | ⬜ pending |
| 04-01-08 | 01 | 1 | VERIFY-04 | — | N/A | integration | Oracle vxc tests (tolerance <= 10^-10) | No -- Wave 0 | ⬜ pending |
| 04-01-09 | 01 | 1 | VERIFY-05 | — | N/A | integration | Oracle fxc tests (tolerance <= 10^-8) | No -- Wave 0 | ⬜ pending |
| 04-01-10 | 01 | 1 | VERIFY-06 | — | N/A | integration | Oracle kxc tests (tolerance <= 10^-6) | No -- Wave 0 | ⬜ pending |
| 04-01-11 | 01 | 1 | VERIFY-07 | — | N/A | integration | Oracle lxc tests (tolerance <= 10^-4) | No -- Wave 0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `verify/tests/lda_oracle.rs` — batch oracle tests for all 42 LDA functionals
- [ ] `verify/tests/gga_oracle.rs` — batch oracle tests for all 130 GGA functionals
- [ ] `verify/tests/mgga_oracle.rs` — batch oracle tests for all 90 MGGA functionals
- [ ] `verify/src/lib.rs` — `oracle_gga_all()` and `oracle_mgga_all()` functions extending existing oracle infra
- [ ] `src/math/powers.rs` — `pow_3_2`, `pow_1_4`, `pow_7_3`, `pow_2`, `pow_3` math functions

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CubeCL compilation of 100K-line MGGA kernel | KERN-05 | Runtime resource limits not testable automatically | Translate mgga_c_rmggac first, verify it compiles and runs without timeout |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
