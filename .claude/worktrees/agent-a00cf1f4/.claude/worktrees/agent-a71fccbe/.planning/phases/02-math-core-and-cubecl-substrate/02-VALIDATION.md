---
phase: 2
slug: math-core-and-cubecl-substrate
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-09
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust built-in) |
| **Config file** | Cargo.toml (workspace) |
| **Quick run command** | `cargo test -p libxc_rs --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p libxc_rs --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | MATH-05 | — | N/A | unit | `cargo test -p libxc_rs math::constants` | ❌ W0 | ⬜ pending |
| 02-01-02 | 01 | 1 | MATH-01 | — | N/A | unit | `cargo test -p libxc_rs math::powers` | ❌ W0 | ⬜ pending |
| 02-01-03 | 01 | 1 | MATH-03 | — | N/A | unit | `cargo test -p libxc_rs math::piecewise` | ❌ W0 | ⬜ pending |
| 02-01-04 | 01 | 1 | MATH-06 | — | N/A | unit | `cargo test -p libxc_rs math::spin` | ❌ W0 | ⬜ pending |
| 02-01-05 | 01 | 1 | MATH-04 | — | N/A | unit | `cargo test -p libxc_rs math::erf` | ❌ W0 | ⬜ pending |
| 02-01-06 | 01 | 1 | MATH-07, MATH-08 | — | N/A | unit | `cargo test -p libxc_rs math::dft_quantities` | ❌ W0 | ⬜ pending |
| 02-01-07 | 01 | 1 | MATH-08 | — | N/A | unit | `cargo test -p libxc_rs math::polynomials` | ❌ W0 | ⬜ pending |
| 02-02-01 | 02 | 2 | KERN-01 | — | N/A | unit | `cargo test -p libxc_rs kernel::launch` | ❌ W0 | ⬜ pending |
| 02-03-01 | 03 | 3 | KERN-02 | — | N/A | integration | `cargo test -p verify lda_x_oracle` | ❌ W0 | ⬜ pending |
| 02-03-02 | 03 | 3 | MATH-09 | — | N/A | integration | `cargo test -p libxc_rs math` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src/math/mod.rs` — math module with submodule declarations
- [ ] `Cargo.toml` — cubecl cpu dependency and libm dev-dependency added
- [ ] Existing tests pass: `cargo test --workspace`

*Existing infrastructure (cargo test, verify/ crate with oracle) covers framework needs.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CubeCL CPU backend produces bit-identical results to native Rust | MATH-10 | Deferred to Phase 7 per D-09 | Compare #[cube] fn output vs std::f64 operations |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
