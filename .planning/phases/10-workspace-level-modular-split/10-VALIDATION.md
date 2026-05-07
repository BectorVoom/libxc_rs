---
phase: 10
slug: workspace-level-modular-split
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-05-07
---

# Phase 10 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Source of truth: 10-RESEARCH.md `## Validation Architecture` section.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo` (workspace) + `verify/` regression harness |
| **Config file** | `Cargo.toml` (root, workspace) + `verify/Cargo.toml` |
| **Quick run command** | `cargo check --workspace 2>&1 \| tee log/10-NN-<step>.log` |
| **Full suite command** | `cargo test --workspace 2>&1 \| tee log/10-NN-test.log` |
| **Estimated runtime** | ~120 s for `cargo check --workspace`; verify/ oracle sweep ~30 s on representative LDA/GGA/MGGA sample |

---

## Sampling Rate

- **After every task commit:** Run `cargo check --workspace` (must stay green — bisectability invariant from CONTEXT specifics).
- **After every plan completes:** Run `cargo test --workspace` AND `cargo tree -p <touched-crate>` assertions for the success-criterion that plan addresses.
- **Before `/gsd-verify-work`:** Full suite + cargo tree assertions for all four crates + oracle parity sweep.
- **Max feedback latency:** ~120 s per task commit.

---

## Per-Task Verification Map

> Filled in by the planner. Each task in PLAN.md must reference a row here (or be added to Wave 0). The matrix below enumerates the per-success-criterion validation anchors that every plan must thread through.

| Anchor ID | Success Criterion | Validation Mode | Automated Command | Output |
|-----------|-------------------|-----------------|-------------------|--------|
| V-10-01   | SC-1: 4 target crates exist | filesystem | `test -f crates/libxc-core/Cargo.toml && test -f crates/libxc-eval/Cargo.toml && test -f crates/libxc-compat/Cargo.toml` | exit 0 |
| V-10-02   | SC-2: libxc-core has zero CubeCL/kernel-* deps | cargo tree | `cargo tree -p libxc-core --prefix none 2>&1 \| tee log/10-final-cargo-tree-core.log` | grep -E "cubecl\|kernel-(lda\|gga\|mgga\|math)" log/10-final-cargo-tree-core.log returns no matches |
| V-10-03   | SC-3: libxc-eval has libxc-core, NOT libxc-compat | cargo tree | `cargo tree -p libxc-eval --prefix none 2>&1 \| tee log/10-final-cargo-tree-eval.log` | grep "libxc-core" matches; grep "libxc-compat" returns no matches |
| V-10-04   | SC-4: libxc-compat has both; nothing depends on libxc-compat | cargo tree | `cargo tree -p libxc-compat --invert 2>&1 \| tee log/10-final-cargo-tree-compat.log` | inverted tree shows only libxc-compat itself (its cdylib output) |
| V-10-05   | SC-5: root facade preserves public surface | source-grep + check | `grep -rhE "use libxc_rs::[a-zA-Z_:]+" verify/ tests/ examples/ 2>/dev/null \| sort -u > log/10-final-public-surface.log; cargo check -p verify 2>&1 \| tee log/10-final-verify-check.log` | verify/ compiles with zero source changes; surface log diffs cleanly against pre-refactor capture |
| V-10-06   | SC-6: cargo test parity | cargo test | `cargo test --workspace 2>&1 \| tee log/10-final-cargo-test-workspace.log` | pass/fail set matches pre-refactor baseline captured in Wave 0 (`log/10-pre-baseline-test.log`) |
| V-10-07   | SC-7: oracle parity at 1e-12 | verify/ harness | `cargo test -p verify --release -- --nocapture 2>&1 \| tee log/10-final-oracle-parity.log` (representative sample: LDA_X, GGA_X_PBE, MGGA_X_TPSS at order 0+2, both spin modes) | every relative error ≤ 1e-12 |
| V-10-08   | SC-8: zero new warnings | cargo build | `cargo build --workspace 2>&1 \| tee log/10-final-cargo-build-workspace.log` | grep "^warning:" returns no matches |

---

## Wave 0 Requirements

> Wave 0 captures **pre-refactor baselines** so post-refactor sampling has anchors to diff against. The planner must include a Wave 0 plan (or task block in plan 10-01) that runs BEFORE any `git mv` happens.

- [ ] `log/10-pre-baseline-cargo-check.log` — `cargo check --workspace` output (must be green at start)
- [ ] `log/10-pre-baseline-cargo-test.log` — `cargo test --workspace` pass/fail set (the parity reference for SC-6)
- [ ] `log/10-pre-baseline-cargo-tree-libxc_rs.log` — `cargo tree -p libxc_rs` (today's monolithic dep closure, for diff inspection)
- [ ] `log/10-pre-baseline-public-surface.log` — `grep -rhE "use libxc_rs::[a-zA-Z_:]+" verify/ tests/ examples/ 2>/dev/null | sort -u` (the path set the root facade must preserve, per SC-5)
- [ ] `log/10-pre-baseline-oracle-parity.log` — verify/ regression sweep on representative LDA/GGA/MGGA (the 1e-12 reference for SC-7)
- [ ] `log/10-pre-baseline-warnings.log` — `cargo build --workspace 2>&1 | grep "^warning:"` (the warning baseline for SC-8 — zero NEW warnings means post-refactor count ≤ baseline count)

*If any baseline command fails or returns dirty (warnings, test failures), surface to user before continuing — Phase 10 cannot start from a broken baseline.*

---

## Manual-Only Verifications

| Behavior | Success Criterion | Why Manual | Test Instructions |
|----------|-------------------|------------|-------------------|
| Bisectability invariant | (cross-cutting from CONTEXT specifics) | Requires reading commit history at end-of-phase | `git rev-list <phase-start>..<phase-end>` then `for c in $(...); do git checkout $c; cargo check --workspace; done` — every commit green |

*All success criteria 1–8 have automated commands above; only the bisectability check is intrinsically a multi-commit walk.*

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify entries or are listed under Wave 0
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 captures all 6 baseline files BEFORE any move
- [ ] No watch-mode flags (`cargo watch`, etc.)
- [ ] Feedback latency < 120 s
- [ ] `nyquist_compliant: true` set in frontmatter (planner sets after wiring per-task entries)

**Approval:** pending
