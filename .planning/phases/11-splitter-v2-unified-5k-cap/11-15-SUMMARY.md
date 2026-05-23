---
phase: 11-splitter-v2-unified-5k-cap
plan: 15
subsystem: testing
tags: [g3-prereq, cubecl-010, launch-abi, math-test-modules, test-gated-drift, per-p-entry-gate, turbofish]

requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 14)
    provides: "the PROVEN cubecl-0.10 launch+readback ABI in src/kernel/launch.rs:81-83 / 146-156 (from_raw_parts 2-arg/by-value, .clone() on read-back handle, launch returns (), read_one().expect()) — the source-of-truth mirrored verbatim here"
provides:
  - "All 6 crates/kernels/math/src test modules compile clean under cubecl 0.10: cargo build -p libxc-kernel-math --tests --jobs 1 EXIT 0 (peak RSS ~800 MB, 10.6s, no OOM)"
  - "The 11-10 (G-3) compile sweep's cargo-test path no longer breaks on the 0.9 launch-ABI drift — 11-10 is UNBLOCKED"
  - "The 11-12 oracle's math-crate test path compiles under 0.10"
  - "Discovered + recorded: 3 PRE-EXISTING runtime numerical test failures in the math crate (out of 11-15 scope; flagged for follow-up)"
affects: [11-10, 11-12, 11-13, cubecl-migration, mgga-parity]

tech-stack:
  added: []
  patterns:
    - "cubecl-0.10 test-harness launch ABI: ArrayArg::from_raw_parts(handle, len) 2-arg/no-turbofish; input handle moved by value, output (read back) handle .clone()'d; launch_unchecked returns () (no .unwrap()); read_one(handle).expect(...) on the now-Result return"
    - "Generic #[cube] helper calls (safe_cbrt<F>, pow_*<F>, piecewise{3,5}<F>, compute_total/zeta<F>, spin_scaling<F>, clamp_zeta<F>) MUST carry explicit ::<f64> turbofish when called from a concrete test kernel under 0.10 (PATTERN.md Rule 9/10; memory project_cubecl_turbofish_required)"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-15-SUMMARY.md
  modified:
    - crates/kernels/math/src/piecewise.rs
    - crates/kernels/math/src/powers.rs
    - crates/kernels/math/src/polynomials.rs
    - crates/kernels/math/src/erf.rs
    - crates/kernels/math/src/dft_quantities.rs
    - crates/kernels/math/src/spin.rs

key-decisions:
  - "Mirrored launch.rs:81-83/146-156 verbatim (do NOT guess 0.10 signatures): from_raw_parts 2-arg, output handle .clone(), launch returns (), read_one().expect() — confirmed by cubecl-core-0.10.0/.../array/launch.rs:47 (from_raw_parts takes 0 generic args + 2 value args; ArrayArg<R> element type comes from the launch_unchecked::<CpuRuntime> turbofish, not the call)"
  - "DEVIATION (+1 file): spin.rs is a 6th file in the same crate with identical 0.9 drift (10 from_raw_parts, 4 read_one, 3 launch-unwrap) that the plan's 5-file inventory missed. The crate-wide `--tests` gate compiles it, so it MUST be migrated for the gate to pass. Migrated with targeted per-harness edits (its run_total_zeta harness has TWO read-back outputs h_total+h_zeta — both .clone()'d — and h_zeta is an OUTPUT there but an INPUT in run_clamp_zeta, so a global replace was unsafe)"
  - "DEVIATION (kernel-body turbofish, E0282): the plan said 3 transforms suffice, but the gate also failed on 15 generic-helper calls lacking ::<f64> under 0.10's #[cube] macro — powers(9: safe_cbrt/pow_2_3/pow_4_3/pow_5_3/pow_3_2/pow_1_4/pow_7_3/pow_2/pow_3), piecewise(2: piecewise3/5), spin(4: compute_total/compute_zeta/spin_scaling/clamp_zeta). erf/polynomials/dft already carried the turbofish (no error). Added ::<f64>; this is the established repo fix (Rule 9/10), not a math change"
  - "Plan's gate is COMPILE-only (cargo build --tests). Ran the actual tests too (light, single crate) for honesty: 62 pass / 3 fail. The 3 failures are NOT launch-ABI: a buffer-passing bug yields garbage/panics, not a 4e-8 drift; erf/powers (same migration pattern) pass; and bessel::ref_i0_small_arg is in bessel.rs — a file 11-15 never touched. They are pre-existing math-precision issues, now surfaced because the crate compiles+runs under 0.10 for the first time. Out of 11-15 scope (launch-ABI only)"

patterns-established:
  - "Per-`-p` --tests compile gate as the ENTRY gate (memory project_phase11_structural_without_compile): it caught what grep alone could not — a whole missed file (spin.rs) AND 15 missing-turbofish E0282s the plan's 3-transform model did not anticipate"
  - "Two-output read-back harness: each output handle that is read_one'd after launch gets its own .clone(); inputs (even same-named in a sibling harness) move by value"

requirements-completed: []

duration: ~40min
completed: 2026-05-24
---

# Phase 11 / Plan 15: Math test-module cubecl-0.10 launch-ABI migration — Summary

**The cubecl-0.9 launch+readback drift in the math crate's `#[cfg(test)] mod tests` host drivers (6 files, not 5) is migrated to the 0.10 ABI; `cargo build -p libxc-kernel-math --tests` is green (exit 0, ~800 MB, no OOM), unblocking the 11-10 G-3 compile sweep's cargo-test path.**

## Performance

- **Duration:** ~40 min
- **Completed:** 2026-05-24
- **Tasks:** 2/2 (Task 1: piecewise+polynomials+erf; Task 2: powers+dft_quantities+gate) + deviation (spin.rs, turbofish)
- **Files modified:** 6
- **Commit:** `e04eb5f863` (path-scoped `git commit --only`, exactly 6 files, 120 insertions / 120 deletions)

## Accomplishments

### Launch-ABI migration (per-file site counts)

| file | from_raw_parts | read_one | launch .unwrap() | +::<f64> turbofish | notes |
|------|---------------:|---------:|-----------------:|-------------------:|-------|
| piecewise.rs    | 10 | 2 | 2 | 2  | piecewise3/5 (varying arity) |
| powers.rs       | 18 | 9 | 9 | 9  | 9 single-arg pow_* kernels |
| polynomials.rs  |  7 | 2 | 2 | 0  | trailing scalar args nc/np/nq PRESERVED; helper turbofish pre-existing |
| erf.rs          |  4 | 2 | 2 | 0  | erf_approx/erfc_approx::<f64> pre-existing |
| dft_quantities.rs | 11 | 4 | 4 | 0 | wigner_seitz_rs/tf_kinetic/alpha/reduced_gradient::<f64> pre-existing |
| **spin.rs** (DEVIATION) | 10 | 4 | 3 | 4 | 2-output total_zeta harness; compute_total/zeta/spin_scaling/clamp_zeta |
| **total** | **60** | **23** | **22** | **15** | |

Each transform mirrors `src/kernel/launch.rs` verbatim:
- `ArrayArg::from_raw_parts::<f64>(&h, n, 1)` → `from_raw_parts(h, n)` (drop turbofish, vec-factor, `&`); **output** handle read back after launch → `from_raw_parts(h.clone(), n)`; **input** handles move by value.
- `…launch_unchecked::<CpuRuntime>(…).unwrap();` → `…(…);` (0.10 returns `()`).
- `client.read_one(h)` → `client.read_one(h).expect("read_one failed during output buffer read-back")` (0.10 returns `Result`).

### Verdicts

- **Grep (T-11-15-01):** ZERO `from_raw_parts::<f64>` and ZERO launch `).unwrap();` across all 6 files; every `read_one(` is `.expect(...)`-terminated.
- **Diff (T-11-15-02):** confined to test-module launch/read/turbofish lines only — no `#[cube]` helper-body or production-path edits (120/120 symmetric).
- **Compile gate:** `cargo build -p libxc-kernel-math --tests --jobs 1` → **exit 0**, 10.57 s, peak RSS **~800 MB** (light single crate, no OOM — confirms the plan's non-OOM claim).

## Deviations

1. **+spin.rs (6th file).** Plan inventory listed 5 files; spin.rs in the same crate carries identical 0.9 drift and is compiled by the crate-wide `--tests` gate. Required to make the gate green. Migrated with targeted per-harness edits because (a) its `run_total_zeta` harness reads back **two** outputs (`h_total`, `h_zeta` — both `.clone()`'d) and (b) `h_zeta` is an output there but an input in `run_clamp_zeta`, so a global replace would have wrongly cloned the input.
2. **+15 `::<f64>` turbofish (E0282).** The plan's 3-transform model assumed kernel bodies were correct, but powers(9)/piecewise(2)/spin(4) test kernels call generic `<F: Float>` helpers without turbofish — illegal under 0.10's `#[cube]` macro. Added `::<f64>` per the established repo fix (PATTERN.md Rule 9/10).

Both deviations are within 11-15's goal (a green `-p libxc-kernel-math --tests` gate) and were *discovered by running the gate* — the exact value of the per-`-p` ENTRY gate.

## Out-of-scope finding (flagged, NOT fixed in 11-15)

Running the suite (beyond the compile-only gate) yielded **62 pass / 3 fail**. The failures are **pre-existing math-precision issues, NOT launch-ABI**:

| test | file | observed | tolerance |
|------|------|----------|-----------|
| `bessel::tests::ref_i0_small_arg` | bessel.rs (**untouched by 11-15**) | — | — |
| `dft_quantities::tests::test_dimensionless_alpha_uniform` | dft_quantities.rs | 0.999999960263572 vs 1.0 (~4e-8) | max_relative 1e-14 |
| `dft_quantities::tests::test_tf_kinetic_rho_one` | dft_quantities.rs | — | — |

Evidence they are not migration-caused: (1) a buffer-passing bug produces garbage/panics, not a 4e-8 drift; (2) erf/powers use the identical migration pattern and pass; (3) the bessel failure is in a file 11-15 never edited. These surfaced because the crate compiles+runs under 0.10 for the first time. The dft `alpha`/`tf_kinetic` quantities are MGGA-adjacent — plausibly related to the already-recorded **MGGA f64 parity** follow-up (11-12 / the planned Phase 12 entry). **Recommend routing to that follow-up; do NOT block 11-10** (the sweep is a per-`-p` COMPILE sweep and does not run these tests).

## Next

11-10 (G-3) is unblocked. It is a USER-RUN, multi-day, paced/resumable f64-only compile sweep (jobs=1) — the assistant runs only `cargo tree` / `--dry-run` prep. See `.continue-here.md` and the 11-10 plan.
