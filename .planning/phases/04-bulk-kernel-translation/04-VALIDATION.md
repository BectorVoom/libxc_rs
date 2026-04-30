---
phase: 4
slug: bulk-kernel-translation
status: complete
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-10
last_updated: 2026-04-24
kernel_counts:
  lda_compiled: 37
  lda_deferred: 4
  gga_compiled: 106
  gga_deferred: 0
  mgga_compiled: 86
  mgga_deferred: 6
  total_compiled: 229
  total_translatable: 235
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
| 04-01-01 | 01 | 1 | KERN-03 | — | N/A | integration | `cargo test -p libxc_rs-verify -- lda_oracle` | No -- Wave 0 | ✅ green |
| 04-02-01 | 02 | 2 | KERN-04 | — | N/A | integration | `cargo test -p libxc_rs-verify -- gga_oracle` | No -- Wave 0 | ✅ green |
| 04-03-01 | 03 | 3 | KERN-05 | — | N/A | integration | `cargo test -p libxc_rs-verify -- mgga_oracle` | No -- Wave 0 | ✅ green |
| 04-01-02 | 01 | 1 | KERN-06 | — | N/A | integration | Oracle comparison (implicit in KERN-03/04/05) | N/A | ✅ green |
| 04-01-03 | 01 | 1 | KERN-07 | — | N/A | unit | `cargo test -p libxc_rs --lib -- kernel::lda` | Partial (LDA_X) | ✅ green |
| 04-01-04 | 01 | 1 | KERN-08 | — | N/A | unit | Existing dispatch tests | Yes | ✅ green |
| 04-01-05 | 01 | 1 | KERN-09 | — | N/A | unit | Compilation check (each function exists) | N/A | ✅ green |
| 04-01-06 | 01 | 1 | VERIFY-02 | — | N/A | integration | All oracle tests pass | No -- Wave 0 | ✅ green |
| 04-01-07 | 01 | 1 | VERIFY-03 | — | N/A | integration | Oracle exc tests (tolerance <= 10^-12) | No -- Wave 0 | ✅ green |
| 04-01-08 | 01 | 1 | VERIFY-04 | — | N/A | integration | Oracle vxc tests (tolerance <= 10^-10) | No -- Wave 0 | ✅ green |
| 04-01-09 | 01 | 1 | VERIFY-05 | — | N/A | integration | Oracle fxc tests (tolerance <= 10^-8) | No -- Wave 0 | ✅ green |
| 04-01-10 | 01 | 1 | VERIFY-06 | — | N/A | integration | Oracle kxc tests (tolerance <= 10^-6) | No -- Wave 0 | ✅ green |
| 04-01-11 | 01 | 1 | VERIFY-07 | — | N/A | integration | Oracle lxc tests (tolerance <= 10^-4) | No -- Wave 0 | ✅ green |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [x] `verify/tests/lda_oracle.rs` — batch oracle tests for 37 compiled LDA functionals (4 deferred tracked in `crates/kernel-lda/src/deferred.rs`)
- [x] `verify/tests/gga_oracle.rs` — batch oracle tests for 106 compiled GGA functionals
- [x] `verify/tests/mgga_oracle.rs` — batch oracle tests for 86 compiled MGGA functionals (6 deferred tracked in `crates/kernel-mgga/src/deferred.rs`)
- [x] `verify/src/lib.rs` — `oracle_gga_all()` and `oracle_mgga_all()` functions extending existing oracle infra
- [x] `src/math/powers.rs` — `pow_3_2`, `pow_1_4`, `pow_7_3`, `pow_2`, `pow_3` math functions

Actual post-refresh kernel counts: 37 LDA compiled + 106 GGA compiled + 86 MGGA compiled = 229 compiled of 235 translatable (phases 8 and 9 split out the oversized kernel translation work that was originally scoped into this phase). Pre-refresh planning documents used higher estimates (~42 LDA, ~130 GGA, ~90 MGGA) that are now superseded.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CubeCL compilation of 100K-line MGGA kernel | KERN-05 | Runtime resource limits not testable automatically | Translate mgga_c_rmggac first, verify it compiles and runs without timeout |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 60s for per-functional oracle tests (full-matrix `cargo xtask verify-phase-4` is a longer-running phase-gate job, not per-task sampling)
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** complete 2026-04-24 — all 11 Phase 4 requirements (KERN-03..09 + VERIFY-02..07) closed. See `04-COVERAGE.md` for per-requirement evidence and `cargo xtask verify-phase-4` for the single-command phase-gate run.
