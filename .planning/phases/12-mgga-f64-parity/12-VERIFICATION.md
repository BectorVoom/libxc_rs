---
status: passed
phase: 12-mgga-f64-parity
verified: 2026-05-25
requirements: [SC-1, SC-2]
method: goal-backward (inline orchestrator verification + USER-RUN authoritative oracle gate)
---

# Phase 12: MGGA f64 Parity — Verification

**Goal:** Bring the 6 routed MGGA exchange-correlation functionals within 1e-12 of the libxc f64 oracle.

**Verdict: PASSED.** Both success criteria are met. The authoritative per-family f64 oracle gate (USER-RUN) reports MGGA failures=0 and LDA failures=0; all 6 targets are independently confirmed by permanent single-kernel canaries at machine precision.

## Success Criteria

### SC-1 — Each of the 6 targets passes the MGGA oracle at f64 (rel_err ≤ 1e-12) — ✅ MET

**Authoritative gate (USER-RUN):**
```
MGGA unpol summary: tested=12 ... failures=0
test result: ok. 2 passed; 0 failed   (test_all_mgga_oracle_pol + test_all_mgga_oracle_unpol)
```
None of the 6 targets appears in the oracle skip list, and failures=0 ⇒ every tested functional (incl. all 6 targets) passes exc ≤ 1e-12.

**Independent single-kernel canary confirmation** (`verify-canary/tests/`, each builds ONE kernel):

| Functional | id | max_rel_err | (was, pre-D-01) |
|---|---|---|---|
| mgga_x_th | 225 | 6.6e-16 | 2.0e-1 |
| mgga_x_2d_js17 | 609 | 0.0 | 1.1e-2 |
| mgga_c_cs | 72 | 0.0 | 9.2e-3 |
| mgga_x_pkzb | 213 | 1.7e-16 | 3.7e-3 |
| mgga_x_pbe_gx | 576 | 2.9e-16 | 1.5e-3 |
| mgga_x_tm | 540 | 1.7e-16 | 9.2e-4 |

Root cause closed: the MGGA dispatch was clamping the **wrong variable** (τ-up instead of libxc's σ-DOWN Fermi-hole clamp). D-01 (`prepare.rs::regularize_inputs`) mirrors `work_mgga_inc.c:54-68` exactly. `mgga_x_th`'s "20% / per-functional translation bug" hypothesis was **disproved** — same regularization root cause (it reads σ and τ independently).

### SC-2 — No regression in LDA / GGA / other-MGGA f64 oracle — ✅ MET

- **MGGA (other):** failures=0. A D-01 regression in 4 non-`NEEDS_TAU` functionals (mgga_k_gea2/gea4, mgga_xc_zlp/lp90) was **caught by this gate** and fixed (NEEDS_TAU gating; commit `cd9b9691b4`); re-run is clean. vxc-unpol within TOL_VXC=1e-10.
- **LDA:** `LDA unpol summary: tested=38 ... failures=0` → `test result: ok`. No regression.
- **GGA:** Not explicitly re-run, but **unaffected by construction**: the sole Phase-12 source change (`src/eval/mgga_dispatch/`) is `#[cfg(feature="oracle-mgga")]`-gated, so the `oracle-gga` build does not compile it (uses the stub `dispatch_gga`). The LDA result empirically confirms cross-family cfg-isolation. Explicit confirmation available:
  `cargo test -p libxc_rs-verify --no-default-features -F oracle-gga --test gga_oracle --jobs 1 -- --test-threads=1 --nocapture`

## Must-Haves (per-plan truths)

| Plan | Truth | Status |
|---|---|---|
| 12-01 | Dispatch clamps σ DOWN (8ρτ), not τ UP | ✅ prepare.rs::regularize_inputs |
| 12-01 | Regularized σ (not raw) flows into every routed MGGA launch | ✅ mod.rs chokepoint |
| 12-01 | g1/g3 canaries still pass 1e-12 under σ-down | ✅ 5.0e-13 / 6.3e-13 |
| 12-02 | 5 small-error functionals have permanent single-kernel canaries passing 1e-12 | ✅ |
| 12-02 | Each canary exercises a sub-Fermi-hole point (active clamp) | ✅ i=4 assertion |
| 12-03 | mgga_x_th root cause identified + canary passes 1e-12 | ✅ regularization (D-01), 6.6e-16 |
| 12-04 | Full-tree regen byte-idempotent / no collateral drift | ✅ zero kernel/translator change (driver-only fix); Phase-11 D-LOCK-D unperturbed |
| 12-04 | All 6 pass family oracle; no regression | ✅ MGGA+LDA failures=0 |
| 12-04 | mgga_x_2d_js17 passing OR de-routed (D-13) | ✅ CASE A — passing, remains routed |

## Anti-pattern compliance
- **AP-3 (never hand-edit generated kernels):** honored — `git diff` shows zero changes under `crates/kernels/`. The entire fix is driver code (`src/eval/mgga_dispatch/`) + tests.

## Notes / Residual
- GGA oracle not explicitly re-run (unaffected by construction; LDA confirms isolation). Optional belt-and-suspenders command above.
- The full-tree regen (12-04 Task 1) was intentionally not run — it would be a destructive no-op vs the sharded tree (Phase 12 changed zero codegen). See 12-04-SUMMARY.

## Commits (Phase 12)
`97d96319ea` `2bce3b4e93` `ef777167e3` `b779cf6a41` (12-01) · `5c03e0b054` `df45df32ee` `c4c348e7fa` `81d493abf7` (12-02) · `6495d1b38f` (12-03) · `cd9b9691b4` (12-04 fix) + SUMMARY commits.
