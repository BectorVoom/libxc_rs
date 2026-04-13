---
phase: 8
slug: rebuild-mgga-kernel-conversion-tool-from-scratch-with-iterat
status: active
nyquist_compliant: true
wave_0_complete: true
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
| **Quick run command** | `RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga-1 2>&1 | tail -10` |
| **Full suite command** | `RUST_MIN_STACK=67108864 cargo test --test oracle_mgga -- --nocapture` |
| **Estimated runtime** | ~60 seconds |

---

## Sampling Rate

- **After every task commit:** Run `RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga-1 2>&1 | tail -10`
- **After every plan wave:** Run `RUST_MIN_STACK=67108864 cargo test --test oracle_mgga -- --nocapture`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 08-01-T1 | 01 | 1 | KERN-05, KERN-06 | T-08-01 | N/A | unit | `python3 /workspace/tools/translate_mgga.py /workspace/libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c mgga_xc_lp90 --write-to /workspace/crates/kernel-mgga-1/src 2>&1 && ls /workspace/crates/kernel-mgga-1/src/mgga_xc_lp90/ && echo "SUCCESS"` | tools/translate_mgga.py | ⬜ pending |
| 08-01-T2 | 01 | 1 | KERN-05, KERN-06 | T-08-02 | N/A | compilation | `cd /workspace && RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga-1 2>&1 \| tail -5` | crates/kernel-mgga-1/Cargo.toml | ⬜ pending |
| 08-02-T1 | 02 | 2 | KERN-05, KERN-06 | T-08-04 | N/A | compilation | `cd /workspace && RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga-1 2>&1 \| tail -5` | crates/kernel-mgga-1/src/mgga_k_gea2/ | ⬜ pending |
| 08-02-T2 | 02 | 2 | VERIFY-03 | T-08-03 | N/A | integration | `cd /workspace && RUST_MIN_STACK=67108864 cargo test --test oracle_mgga -- --nocapture 2>&1 \| tail -20` | tests/oracle_mgga.rs | ⬜ pending |
| 08-03-T1 | 03 | 3 | KERN-05, KERN-06 | T-08-05 | N/A | unit | `cd /workspace && find crates/kernel-mgga-[0-9]*/src -mindepth 1 -maxdepth 1 -type d \| wc -l` | tools/batch_translate_mgga.py | ⬜ pending |
| 08-03-T2 | 03 | 3 | KERN-05, KERN-06 | T-08-05 | N/A | compilation | `cd /workspace && RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga 2>&1 \| tail -10` | crates/kernel-mgga/src/lib.rs | ⬜ pending |
| 08-03-T3 | 03 | 3 | VERIFY-03, VERIFY-04 | T-08-06 | N/A | integration | `cd /workspace && RUST_MIN_STACK=67108864 cargo test --test oracle_mgga -- --nocapture 2>&1 \| tail -20` | tests/oracle_mgga.rs | ⬜ pending |
| 08-04-T1 | 04 | 4 | KERN-05 | T-08-07 | N/A | compilation | `cd /workspace && RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-mgga 2>&1 \| tail -5 && grep -c "DeferredMgga" crates/kernel-mgga/src/deferred.rs` | crates/kernel-mgga/src/deferred.rs | ⬜ pending |
| 08-04-T2 | 04 | 4 | VERIFY-04 | T-08-08 | N/A | integration | `cd /workspace && RUST_MIN_STACK=67108864 cargo test --test oracle_mgga test_mgga_vxc -- --nocapture 2>&1 \| tail -20` | tests/oracle_mgga.rs | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [x] Translation tool infrastructure exists (translate_gga.py provides reference pattern)
- [x] Oracle test infrastructure for MGGA exists (verify/src/lib.rs provides oracle_mgga_all)
- [x] CubeCL kernel compilation pattern established (kernel-gga-1 through kernel-gga-3 provide reference)
- [x] Sub-crate splitting pattern established (GGA sub-crate split in phase 08-04 provides precedent)

*All Wave 0 requirements are satisfied by existing infrastructure from prior phases.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Generated code readability | N/A | Subjective quality | Review generated Rust code for patterns matching GGA translator output |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify commands
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references (none -- all infrastructure exists)
- [x] No watch-mode flags
- [x] Feedback latency < 60s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved
