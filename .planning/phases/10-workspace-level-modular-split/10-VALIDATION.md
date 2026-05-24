---
phase: 10
slug: workspace-level-modular-split
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-05-25
supersedes: 2026-05-07 draft (workspace-wide cargo commands that OOM on this box)
---

# Phase 10 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Source of truth: `10-RESEARCH.md` `## Validation Architecture` section (2026-05-25).
> **RAM-CONSTRAINED BOX:** `cargo {check,test,build} --workspace` OOM here. ALL validation
> is per-`-p` / per-family / `cargo tree` (which does not compile). Heavy oracle sweeps are
> USER-RUN, `-j1`. NEVER edit `.cargo/config.toml`.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `#[test]` + `approx` (relative_eq) for oracle parity; `verify/` harness over `libxc-sys` oracle |
| **Config file** | Cargo-native; `verify/Cargo.toml` features gate the family-chunked builds (`oracle-{lda,gga,mgga}`) |
| **Quick run command** | `cargo tree -p <crate> -e no-dev` (boundary SCs 2/3/4 — ZERO compile, milliseconds) |
| **Per-crate check** | `cargo check -p <new-crate> --no-default-features -j1` (light, single-family) |
| **Full suite command** | per-family, USER-RUN, heavy: `cargo test -p libxc_rs-verify --no-default-features --features oracle-<fam> --test <fam>_oracle -j1` |
| **Estimated runtime** | `cargo tree`: <1 s · per-`-p` check: seconds–minutes · per-family oracle sweep: minutes (USER-RUN) |

---

## Sampling Rate

- **After every task commit:** the relevant `cargo tree -p <crate> -e no-dev` boundary assertion (SC 2/3/4) — milliseconds, no OOM.
- **After every crate-extraction merge:** `cargo check -p <new-crate> --no-default-features -j1` (light, single-family).
- **Phase gate (SC 6/7):** USER-RUN per-family oracle sweeps (LDA, GGA, MGGA separately) — diff against the pre-refactor pass/fail snapshot. The **6 known MGGA f64-parity failures (→ Phase 12)** + **3 math-precision failures** must be in the "expected fail" baseline so SC 6 holds.
- **Max feedback latency (cheap tier):** <5 s per task commit. Heavy oracle tier is gated/USER-RUN, not per-commit.
- **HARD RULE:** no `--workspace` cargo command anywhere; no `cargo watch`.

---

## Per-Task / Per-Success-Criterion Verification Map

> The planner threads each task in PLAN.md to one of these anchors (or to Wave 0). The 8 Success
> Criteria are the verification anchors — Phase 10 has no REQ-IDs.

| Anchor ID | Success Criterion | Mode | Automated Command | Pass Condition | Cost |
|-----------|-------------------|------|-------------------|----------------|------|
| V-10-01 | SC-1: four target crates exist | filesystem | `test -d crates/libxc-core -a -d crates/libxc-eval -a -d crates/libxc-compat && cargo metadata --no-deps --format-version 1 \| grep -o '"name":"libxc-[a-z]*"'` | exit 0; core/eval/compat all present | trivial |
| V-10-02 | SC-2: libxc-core zero CubeCL/kernel deps | cargo tree | `cargo tree -p libxc-core -e no-dev 2>&1 \| tee log/10-tree-core.log` | `! grep -qE 'cubecl\|libxc-kernel' log/10-tree-core.log` | cheap, no compile |
| V-10-03 | SC-3: libxc-eval deps core, NOT compat | cargo tree | `cargo tree -p libxc-eval -e no-dev 2>&1 \| tee log/10-tree-eval.log` | `grep -q libxc-core` AND `! grep -q libxc-compat` | cheap |
| V-10-04 | SC-4: libxc-compat deps both; nothing deps it | cargo tree | `cargo tree -p libxc-compat -e no-dev 2>&1 \| tee log/10-tree-compat.log` + `cargo tree -p libxc-compat --invert -e no-dev 2>&1 \| tee log/10-tree-compat-inv.log` | fwd shows libxc-core+libxc-eval; invert shows only libxc-compat itself (cdylib output) | cheap |
| V-10-05 | SC-5: root facade preserves public surface | per-`-p` check + surface grep | `grep -rhoE "use libxc_rs::[A-Za-z0-9_:]+" verify/ verify-canary/ tests/ examples/ 2>/dev/null \| sort -u > log/10-surface-after.log` ; `cargo check -p libxc_rs --lib -j1 2>&1 \| tee log/10-check-libxc_rs.log` | surface-after diffs cleanly vs Wave-0 `log/10-surface-before.log`; `-p libxc_rs --lib` EXIT 0 (~536 MB) | medium (per-`-p`) |
| V-10-06 | SC-6: test parity (NOT `--workspace`) | per-family test (USER-RUN) | per-family: `cargo test -p libxc_rs-verify --no-default-features --features oracle-<fam> --test <fam>_oracle -j1` (lda, gga, mgga separately) + `cargo test -p libxc-core` + `-p libxc-eval --no-default-features -j1` | per-family pass/fail set == Wave-0 snapshot (incl. 6 expected MGGA + 3 math-precision fails) | heavy, USER-RUN |
| V-10-07 | SC-7: oracle parity ≤1e-12 LDA/GGA/MGGA | verify/ oracle (USER-RUN) | `cargo test -p libxc_rs-verify --no-default-features --features oracle-lda --test lda_oracle -j1` (repeat gga/mgga) over the curated witness subset below | every routed `exc` rel-err ≤1e-12 (modulo the 6 known MGGA Phase-12 exceptions) | heavy, USER-RUN |
| V-10-08 | SC-8: zero new warnings | per-`-p` build | `cargo build -p libxc-core -j1` + `cargo build -p libxc-eval --no-default-features -j1` + `cargo build -p libxc_rs --lib -j1` (whole-workspace build OOMs) | no `^warning:`; `#![deny(warnings)]` already makes any warning a hard compile error | medium per-`-p` |

*The bisectability invariant (CONTEXT specifics) is reframed for this box: each task commit stays green under the relevant **per-`-p`** check, NOT `cargo check --workspace`.*

---

## Representative kernels for SC-7 (verify/'s curated subset — do NOT expand to 306)

From `verify/Cargo.toml` curated dev-deps `[VERIFIED: verify/Cargo.toml:43-63]`:
- **LDA:** `lda_x`, `lda_c_pw`, `lda_xc_teter93`
- **GGA:** `gga_x_pbe`, `gga_c_pbe`, `gga_x_b88`
- **MGGA:** `mgga_x_lta`, `mgga_x_tpss`, `mgga_x_pkzb`, `mgga_x_th` (+ worst-case `mgga_c_revtpss`, `mgga_c_kcisk`, `mgga_c_b94`, `mgga_x_r4scan`, `mgga_x_br89_explicit`, `mgga_xc_b97mv`)

This subset compiles without pulling all 306 — it is the SC-7 witness. **Do NOT expand it (OOM, per verify-crate-OOM memory).**

---

## Wave 0 Requirements

> Wave 0 captures **pre-refactor baselines** so post-refactor sampling has anchors. Runs BEFORE any
> `git mv`. **No `--workspace` commands** — capture per-family / per-`-p`.

- [ ] `log/10-surface-before.log` — `grep -rhoE "use libxc_rs::[A-Za-z0-9_:]+" verify/ verify-canary/ tests/ examples/ 2>/dev/null | sort -u` (the path set the root facade must preserve, SC-5)
- [ ] `log/10-tree-libxc_rs-before.log` — `cargo tree -p libxc_rs -e no-dev` (today's monolithic dep closure, for diff)
- [ ] `log/10-baseline-parity-{lda,gga,mgga}.log` — per-family `cargo test -p libxc_rs-verify --no-default-features --features oracle-<fam> --test <fam>_oracle -j1` pass/fail set (the SC-6 reference; MUST record the 6 expected MGGA Phase-12 fails + 3 math-precision fails as "expected fail")
- [ ] `log/10-generated-snapshot/` — byte-snapshot of xtask-generated files (`src/meta/generated*.rs` etc.) so the D-03 xtask path-edit produces a byte-identical regen (regen-idempotency reference)
- [ ] Confirm green baseline: `cargo check -p libxc_rs --lib -j1` EXIT 0 at phase start (D-13 gate, reframed per-`-p` — Phase 11 already satisfies this)

*Test infrastructure itself is COMPLETE — verify/ harness, libxc-sys oracle, and the curated subset already exercise all three families. Wave 0 is snapshot capture, not infra build. If any baseline is dirty (unexpected failures/warnings) surface to user before continuing — Phase 10 cannot start from a broken baseline.*

---

## Manual-Only Verifications

| Behavior | Success Criterion | Why Manual | Test Instructions |
|----------|-------------------|------------|-------------------|
| Heavy per-family oracle sweeps (SC 6/7) | SC-6, SC-7 | Multi-minute, memory-heavy — USER-RUN at `-j1`, not auto/per-commit | Run each `cargo test -p libxc_rs-verify --no-default-features --features oracle-<fam> --test <fam>_oracle -j1`; diff pass/fail vs Wave-0 snapshot |
| Bisectability walk | (cross-cutting, CONTEXT specifics) | Multi-commit history walk | `for c in $(git rev-list <start>..<end>); do git checkout $c; cargo check -p libxc_rs --lib -j1; done` — every commit green per-`-p` (NOT `--workspace`) |

---

## Validation Sign-Off

- [x] All success criteria 1–8 have automated/USER-RUN verify entries; planner maps each task to an anchor or Wave 0
- [x] Sampling continuity: cheap `cargo tree` boundary check available after every task
- [x] Wave 0 captures the pre-refactor snapshots BEFORE any move
- [x] No watch-mode flags; **no `--workspace` cargo commands** (RAM)
- [x] Cheap-tier feedback latency < 5 s
- [x] `nyquist_compliant: true` — per-SC anchors defined; planner threads task IDs

**Approval:** pending (regenerated 2026-05-25 against the current 306-crate / optional-deps topology — supersedes the 2026-05-08 approval, which assumed workspace-wide cargo)
