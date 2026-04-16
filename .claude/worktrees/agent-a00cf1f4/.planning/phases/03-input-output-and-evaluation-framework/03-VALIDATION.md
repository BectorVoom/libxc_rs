---
phase: 3
slug: input-output-and-evaluation-framework
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-09
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[cfg(test)]` + `cargo test` |
| **Config file** | Cargo.toml (already configured) |
| **Quick run command** | `cargo test --lib` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --lib`
- **After every plan wave:** Run `cargo test`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | IO-01 | — | N/A | unit | `cargo test --lib input` | ❌ W0 | ⬜ pending |
| 03-01-02 | 01 | 1 | IO-02 | — | N/A | unit | `cargo test --lib output` | ❌ W0 | ⬜ pending |
| 03-01-03 | 01 | 1 | IO-03 | — | N/A | unit | `cargo test --lib output::mask` | ❌ W0 | ⬜ pending |
| 03-01-04 | 01 | 1 | IO-04 | — | N/A | unit | `cargo test --lib input` | ❌ W0 | ⬜ pending |
| 03-01-05 | 01 | 1 | IO-05 | — | N/A | unit | `cargo test --lib output` | ❌ W0 | ⬜ pending |
| 03-02-01 | 02 | 2 | EVAL-01 | — | N/A | integration | `cargo test --lib eval::dispatch` | ❌ W0 | ⬜ pending |
| 03-02-02 | 02 | 2 | EVAL-04 | — | N/A | integration | `cargo test --lib eval::dispatch` | ❌ W0 | ⬜ pending |
| 03-03-01 | 03 | 2 | EVAL-02 | — | N/A | integration | `cargo test --lib eval::mix` | ❌ W0 | ⬜ pending |
| 03-03-02 | 03 | 2 | EVAL-03 | — | N/A | unit | `cargo test --lib eval::workspace` | ❌ W0 | ⬜ pending |
| 03-03-03 | 03 | 2 | EVAL-05 | — | N/A | integration | `cargo test --lib eval::mix` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src/input/mod.rs` — input bundle types and validation tests
- [ ] `src/output/mod.rs` — output bundle types and tests
- [ ] `src/output/mask.rs` — OutputMask bitflags and tests
- [ ] `src/eval/dispatch.rs` — dispatch tests with LDA_X
- [ ] `src/eval/mix.rs` — mixed accumulation tests
- [ ] `src/eval/workspace.rs` — workspace tests
