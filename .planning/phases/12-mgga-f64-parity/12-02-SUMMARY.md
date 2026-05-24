---
phase: 12-mgga-f64-parity
plan: 02
subsystem: testing
tags: [mgga, parity, canary, verify-canary, cubecl, oracle, regularization]

# Dependency graph
requires:
  - phase: 12-mgga-f64-parity
    provides: "12-01 D-01 sigma-down regularization (prepare.rs::regularize_inputs) + reconciled g1/g3 canaries"
provides:
  - "6 permanent single-kernel parity canaries in verify-canary/tests/ (mgga_x_tm, mgga_c_cs, mgga_x_pkzb, mgga_x_pbe_gx, mgga_x_2d_js17, mgga_x_th) — all pass 1e-12"
  - "Empirical proof that D-01 sigma-down closed ALL 6 target functionals at the single-kernel exc-unpol level (machine precision)"
  - "Corrected memory-safe family-oracle command + verified cargo-tree chunking (oracle-mgga resolves only MGGA + math via -e no-dev)"
affects: [12-03, 12-04]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Routed kernels (#[cube(launch_unchecked)]) are launched DIRECTLY in canaries (<func>_exc_unpol::launch_unchecked::<CpuRuntime>), matching production dispatch — NO #[cube] wrapper (only unrouted plain-#[cube] kernels like b94 need a wrapper)"
    - "Single-kernel verify-canary loop = fast (sub-second), memory-safe per-functional 1e-12 gate; family oracle = the authoritative regression gate (USER-RUN, memory-safe via -p libxc_rs-verify --no-default-features -F oracle-mgga)"

key-files:
  created:
    - "verify-canary/tests/mgga_x_tm_parity.rs (id 540)"
    - "verify-canary/tests/mgga_c_cs_parity.rs (id 72)"
    - "verify-canary/tests/mgga_x_pkzb_parity.rs (id 213)"
    - "verify-canary/tests/mgga_x_pbe_gx_parity.rs (id 576)"
    - "verify-canary/tests/mgga_x_2d_js17_parity.rs (id 609)"
    - "verify-canary/tests/mgga_x_th_parity.rs (id 225)"
  modified:
    - "verify-canary/Cargo.toml (6 single-kernel path deps)"

key-decisions:
  - "Launch routed kernels DIRECTLY (deviation from the plan's wrapper template — the 6 targets are #[cube(launch_unchecked)], unlike unrouted b94). Confirmed against production launch ABI mod.rs:144-149."
  - "Family-oracle BUILD deferred to USER-RUN (user checkpoint decision) — it duplicates the authoritative 12-04 gate; single-kernel canaries provide the per-functional proof; cargo-tree chunking confirmed memory-safe without building."
  - "Corrected the plan's family-oracle command: it must be -p libxc_rs-verify (the test lives in verify/tests/mgga_oracle.rs) AND --no-default-features (default = all 3 families = all 306 kernels = OOM)."

patterns-established:
  - "cargo-tree leak-check MUST use `-e no-dev`: the verify dev-dependency cycle (kernel -> libxc_rs -> verify[dev-deps] -> libxc_rs) unifies all 3 oracle-* features and falsely shows all 306 kernels without it"

requirements-completed: [SC-1, SC-2]

# Metrics
duration: ~35 min
completed: 2026-05-25
---

# Phase 12 Plan 02: Small-error MGGA Cluster Canaries Summary

**6 permanent single-kernel parity canaries proving the D-01 σ-down fix closed ALL 6 target MGGA functionals (the 5 small-error cluster + mgga_x_th) at 1e-12 / machine precision — the family-oracle regression gate is deferred to the USER-RUN 12-04 authoritative gate.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-05-25
- **Tasks:** 3 (Task 3 Part B family-oracle build deferred to USER-RUN per checkpoint)
- **Files modified:** 7 (1 Cargo.toml + 6 test files)

## Accomplishments
- 6 single-kernel canaries added to `verify-canary/tests/`, each applying the D-01 σ-down `regularize_inputs` (mirror of `prepare.rs`) on a 5-point grid with an explicit sub-Fermi-hole point (i=4) where the clamp is asserted active (non-vacuous).
- **All 6 pass at 1e-12**: mgga_x_tm 1.7e-16, mgga_c_cs 0.0, mgga_x_pkzb 1.7e-16, mgga_x_pbe_gx 2.9e-16, mgga_x_2d_js17 0.0, **mgga_x_th 6.6e-16**.
- **Key finding:** `mgga_x_th`'s ~20% error (reported by the 11-12 family oracle) was the SAME wrong-variable regularization — D-01 closed it. NOT a per-functional translation bug (pre-resolves 12-03's investigation; D-09).
- **Key finding:** `mgga_x_2d_js17` passes exactly (0.0) at exc-unpol — the D-13 2D-escape-hatch concern is moot at this tier (12-04 confirms at family-oracle level).
- Verified `oracle-mgga` is still memory-safe / family-chunked via `cargo tree -e no-dev` (131 MGGA + math, zero LDA/GGA leak) and corrected the plan's oracle command.

## Task Commits

1. **Task 1: 6 single-kernel canary deps** — `5c03e0b054` (build)
2. **Task 2: 5 small-error cluster canaries (all pass 1e-12)** — `df45df32ee` (test)
3. **Task 3 Part A: mgga_x_th canary (passes 1e-12)** — `c4c348e7fa` (test)
   - Part B (family-oracle build): deferred to USER-RUN (see Deviations).

## Files Created/Modified
- `verify-canary/Cargo.toml` — 6 single-kernel path deps (one per target); cubecl 0.10.0 + b94 dep unchanged.
- `verify-canary/tests/mgga_x_{tm,pkzb,pbe_gx,2d_js17,th}_parity.rs`, `mgga_c_cs_parity.rs` — direct-launch single-kernel canaries, D-01 regularized, active-clamp asserted.

## Decisions Made
- **Direct launch, not wrapper** (deviation): all 6 targets are routed `#[cube(launch_unchecked)]`, so they launch directly via `<func>_exc_unpol::launch_unchecked::<CpuRuntime>(...)` — matching production `mgga_dispatch/mod.rs:144-149`. The plan's wrapper template was written for unrouted plain-`#[cube]` kernels (b94). Direct launch is simpler and identical to production.
- **Family-oracle build deferred to USER-RUN** (user checkpoint): the build is heavy and duplicates 12-04's authoritative gate; the single-kernel canaries already prove every target at 1e-12.

## Deviations from Plan

### Auto-fixed / handled

**1. [Rule 1 - Bug in plan template] Direct kernel launch instead of #[cube] wrapper**
- **Found during:** Task 2
- **Issue:** The plan's canary template wraps the kernel in a `#[cube(launch_unchecked)]` shim calling `<func>_exc_unpol(...)`. But all 6 targets are themselves `#[cube(launch_unchecked)]` (routed) — the wrapper pattern is for unrouted plain-`#[cube]` kernels (b94). 
- **Fix:** Launch the routed kernel directly: `<func>_exc_unpol::launch_unchecked::<CpuRuntime>(...)`, matching production ABI (mod.rs:144-149).
- **Verification:** All 6 canaries compile and pass 1e-12.
- **Committed in:** `df45df32ee`, `c4c348e7fa`

**2. [Rule 1 - Bug in plan command] Corrected the family-oracle invocation**
- **Found during:** Task 3 Part B
- **Issue:** The plan's `cargo test -p libxc_rs --features oracle-mgga --test mgga_oracle` is wrong twice: (a) the `mgga_oracle` test lives in the `libxc_rs-verify` package, not `libxc_rs`; (b) without `--no-default-features`, `default = [oracle-lda, oracle-gga, oracle-mgga]` activates all 306 kernels → OOM.
- **Fix:** Correct command is `cargo test -p libxc_rs-verify --no-default-features -F oracle-mgga --test mgga_oracle -j1 -- --test-threads=1 --nocapture` (matches 11-12's proven memory-safe run). The cargo-tree leak-check ALSO requires `-e no-dev` (the verify dev-dep cycle otherwise unifies all 3 oracle-* features and shows all 306).
- **Verification:** `cargo tree -e no-dev -p libxc_rs-verify --no-default-features -F oracle-mgga` → 131 MGGA + math, 0 LDA/GGA. Full build footprint (incl narrowed dev-deps) = 131 MGGA + 3 LDA + 3 GGA witnesses + math ≈ 138 kernels — NOT 306.

**Total deviations:** 2 handled (2 plan-template/command corrections). **Impact:** No scope creep; corrections make the canaries match production and prevent an OOM trap in the oracle command. The family-oracle BUILD itself is intentionally deferred to the USER-RUN 12-04 authoritative gate.

## Issues Encountered
- Initial cargo-tree leak-check (without `-e no-dev`, against `-p libxc_rs`) showed all 306 kernels resolving — a workspace feature-unification artifact of the `verify` dev-dependency cycle, NOT a real chunking regression. Resolved by using `-e no-dev` and the correct package. Documented as a pattern for 12-04.

## User Setup Required
None.

## Next Phase Readiness
- **12-03 (mgga_x_th):** pre-resolved — D-01 closed mgga_x_th at 1e-12 (canary 6.6e-16). 12-03 becomes a fast confirmation + close (no translator fix / regen needed).
- **12-04 (authoritative gate):** USER-RUN command ready:
  `cargo test -p libxc_rs-verify --no-default-features -F oracle-mgga --test mgga_oracle -j1 -- --test-threads=1 --nocapture`
  Expectation: all 6 targets pass exc 1e-12; no regression in previously-passing MGGA functionals (D-06). Note the oversized functionals (rmggac/tpss/kcisk) are already sharded for memory safety.
- **Open for 12-04:** the family-oracle BUILD has not been run with the 12-01 `mod.rs` change yet — this is the first full umbrella compile + regression check (USER-RUN).

---
*Phase: 12-mgga-f64-parity*
*Completed: 2026-05-25*
