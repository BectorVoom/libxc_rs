---
phase: 11
slug: splitter-v2-unified-5k-cap
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-05-13
---

# Phase 11 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Source: distilled from `11-RESEARCH.md` § Validation Architecture. Locked decisions D-05 (1e-12 rel err on energy + routed derivatives at f64), D-07 (inline executor, jobs=1), D-08 (`RUST_MIN_STACK=67108864`), D-09 (`.cargo/config.toml` is source of truth) are the operating envelope.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` + `approx` 0.5.1 (`relative_eq!` for 1e-12 comparisons) + `libxc-sys` (FFI to libxc 7.0.0 oracle) |
| **Config file** | `verify/Cargo.toml` (separate test crate); workspace `Cargo.toml` `[profile]` settings |
| **Quick run command** | `cargo test -p libxc_rs-verify --test parity_phase11 -- --test-threads=1 --nocapture phase11_smoke` |
| **Full suite command** | `cargo test -p libxc_rs-verify -- --test-threads=1 --nocapture` |
| **Estimated runtime** | smoke audit < 5s per file-size / launch-count check; smoke parity ~minutes (10 functionals); full sweep multi-hour |

---

## Sampling Rate

- **After every task commit:** Run smoke audits — `tools/audit_kernel_size.py`, `tools/audit_subcrate_collapse.sh`, `tools/audit_cube_launch.sh` (each < 5s)
- **After every plan wave:** Run `cargo build --workspace` (under D-07/D-08/D-09 envelope) + `parity_phase11::phase11_smoke` set (~10 representative LDA/GGA/MGGA functionals across orders/spins)
- **Before `/gsd-verify-work`:** Full oracle sweep + `tools/test_idempotency.sh` must be green
- **Max feedback latency:** 5 seconds for per-task smoke; minutes for per-wave parity; multi-hour for phase-gate full sweep

---

## Per-Task Verification Map

> Populated by gsd-planner from PLAN.md task IDs. Each task references its requirement (P11-INV-N) and the automated command that proves it.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| {N}-01-01 | 01 | 0 | P11-INV-A1 (tuple ABI spike) | — | tuple-return `<F: Float>` `#[cube]` round-trips through cubecl-macros 0.10 IR | spike | `cargo test -p libxc_rs-verify --test spike_tuple_return_cube` | ❌ Wave 0 creates | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-1 | — | No `crates/kernels/{family}-N` directories exist | smoke | `tools/audit_subcrate_collapse.sh` | ❌ Wave 0 creates | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-2 | — | All `crates/kernels/**/*.rs` are ≤5000 lines | smoke | `tools/audit_kernel_size.py` | ❌ Wave 0 creates | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-3 | — | `cargo build --workspace` succeeds under D-08/D-09 envelope | smoke | `cargo build --workspace` | ✅ via cargo | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-4 | — | Oracle parity 1e-12 on energy + routed derivatives at f64 | unit + integration | `cargo test -p libxc_rs-verify --test parity_phase11` | ❌ Wave 0 creates `parity_phase11.rs` | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-5 | — | `#[cube(launch_unchecked)]` count ≤ 22 (pre-phase baseline) | smoke | `tools/audit_cube_launch.sh` | ❌ Wave 0 creates | ⬜ pending |
| {N}-XX-XX | XX | N | P11-INV-6 | — | Pipeline idempotent (re-run produces no diff) | integration | `tools/test_idempotency.sh` | ❌ Wave 0 creates | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

> Planner: replace placeholder `{N}-XX-XX` rows with concrete task IDs once PLAN.md files are written. Every implementation task that touches splitter code, kernel emission, or subcrate layout MUST cite at least one P11-INV-* requirement and bind it to one of these automated commands.

---

## Wave 0 Requirements

These artifacts MUST exist and pass before Wave 1 (any kernel re-translation) begins:

- [ ] `verify/tests/spike_tuple_return_cube.rs` — minimal `#[cube] fn t<F: Float>(x: F, y: F) -> (F, F) { (x + y, x - y) }` + launch + assert. **Hard gate** for D-02 ABI; if it fails, D-02 must be re-discussed (likely fall back to `&mut F` out-params or per-functional named structs) before any bulk regen.
- [ ] `tools/audit_kernel_size.py` — exits non-zero if any `crates/kernels/**/*.rs` > 5000 lines (P11-INV-2)
- [ ] `tools/audit_subcrate_collapse.sh` — exits non-zero if any `crates/kernels/{lda,gga,mgga}-N` directories exist (P11-INV-1)
- [ ] `tools/audit_cube_launch.sh` — exits non-zero if `#[cube(launch_unchecked)]` count in `crates/kernels/` exceeds 22 (P11-INV-5)
- [ ] `tools/test_idempotency.sh` — runs the splitter twice, diffs the emitted tree, exits non-zero on diff (P11-INV-6)
- [ ] `verify/tests/parity_phase11.rs` — modeled on `parity_phase09.rs`; defines `phase11_smoke` (~10 functionals representative of LDA/GGA/MGGA across orders/spins) and the full worst-case sweep (mgga_c_revtpss, mgga_c_kcisk, mgga_c_b94, mgga_x_r4scan, br89_explicit, mbrxc)

**Pre-phase baselines to record (Wave 0):** snapshot `#[cube(launch_unchecked)]` count (= 22 today, all under `crates/kernels/math/`), oversized-file count (= 237 today, max 16,703 lines), workspace member count, `cargo build --workspace` peak RSS under jobs=1.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| `.cargo/config.toml` invariants preserved | D-08, D-09 | One-off config diff at phase gate; not worth wiring a per-task assertion | Confirm `[build] jobs = 1`, `[build] target-dir = ".cache/cargo-target"`, `[env] RUST_MIN_STACK = "67108864"` are unchanged from pre-phase baseline |
| `CLAUDE.md` precision policy updated per D-03a | D-03a | Documentation edit; verified by reviewer reading the section | Confirm "f64 only" → "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate" |
| RAM ceiling holds end-to-end on user's machine | D-07 | Hardware-bound observation; can't be reliably automated in CI | Run `cargo build --workspace` inline (no worktree), monitor peak RSS, confirm OOM does not trigger |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies declared
- [ ] Sampling continuity: no 3 consecutive tasks without an automated verify command
- [ ] Wave 0 covers all ❌ MISSING references in the per-task map
- [ ] No watch-mode flags in any test command (incompatible with single-shot CI sampling)
- [ ] Feedback latency < 5s for per-task smoke audits
- [ ] `nyquist_compliant: true` set in frontmatter once planner has populated all task IDs

**Approval:** pending
