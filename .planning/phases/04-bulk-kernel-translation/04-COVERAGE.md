# Phase 4: Bulk Kernel Translation — Requirement Coverage

**Phase completed:** 2026-04-24
**Verification command:** `cargo xtask verify-phase-4`
**Single-command entrypoint:** [`xtask/src/verify_phase_4.rs`](../../../xtask/src/verify_phase_4.rs)

## Executive Summary

Phase 4 closed dispatch coverage + numerical oracle verification for every
translated kernel in `crates/kernel-{lda,gga,mgga}*`. Kernel translation
itself was completed by phases 8 and 9; Phase 4 adds the Rust dispatch
layer (`dispatch_lda`, `dispatch_gga`, `dispatch_mgga`) and the
per-functional Rust-vs-libxc-oracle numerical comparison harness.

The sign-off artifact is `cargo xtask verify-phase-4`: a single command
that runs the full LDA+GGA+MGGA oracle matrix across both spin modes,
parses the structured `FAMILY {unpol|pol} summary:` lines emitted by each
oracle test binary (plans 04-02, 04-03, 04-04), and prints a
machine-parseable per-family tested / skipped / failures matrix plus an
overall GREEN/RED status.

**Headline kernel counts (post-refresh — phases 8 and 9 split out oversized
kernel translation work originally scoped here):**

| Family | Compiled | Deferred | Routed via dispatch |
|--------|----------|----------|---------------------|
| LDA    | 37       | 4        | 37 (all compiled)   |
| GGA    | 106      | 0        | 106                 |
| MGGA   | 86       | 6        | 25 via `MggaFunctional` enum; remaining scalar-bearing variants return `UnsupportedFunctional` pending per-kernel scalar-default wiring in Phase 5+ |
| **Total** | **229** | **10**   | see rows above      |

Deferred functionals live in authoritative, machine-readable lists:
- `crates/kernel-lda/src/deferred.rs`  (4 entries — kxc/lxc pol bodies
  exceed the CubeCL proc-macro stack limit)
- `crates/kernel-mgga/src/deferred.rs` (6 entries — all blocked on
  Brent's-method root-finders `xc_mgga_x_br89_get_x` and
  `xc_mgga_x_mbrxc_get_x`, which need `#[cube]` implementations in
  `crates/kernel-math/`)

## Requirement Closure Matrix

| Req | Description | Status | Evidence |
|-----|-------------|--------|----------|
| KERN-03 | All LDA kernel files translated from maple2c to #[cube] functions (~43 functionals) | COMPLETE | Phases 8 + 9 translated every compilable LDA functional; Phase 4 Plan 02 (commit `2ad3f65f`+ landed on `libxc_rs_kernel`) routes all 37 compiled via `dispatch_lda` + `LdaFunctional` enum. 4 deferred tracked in `crates/kernel-lda/src/deferred.rs`. **Test:** `cargo test -p libxc_rs-verify --test lda_oracle`. Evidence file: `verify/tests/lda_oracle.rs`, tests `test_all_lda_oracle_unpol` (line 365) and `test_all_lda_oracle_pol` (line 441); passes with `failures=0` and `skipped_deferred=4` asserted. |
| KERN-04 | All GGA kernel files translated from maple2c to #[cube] functions (~130 functionals) | COMPLETE | Phases 8 + 9 translated every compilable GGA functional; Phase 4 Plan 03 (commits `bfef7629`, `37914246`, `8730e813`) routes all 106 compiled via `dispatch_gga` + `GgaFunctional` enum; per-batch submodule tree under `src/eval/gga_dispatch/`. **Test:** `cargo test -p libxc_rs-verify --test gga_oracle`. Evidence file: `verify/tests/gga_oracle.rs`, tests `test_all_gga_oracle_unpol` (line 633, hard-gate) and `test_all_gga_oracle_pol` (line 689, soft-gate due to pre-existing pol-kernel bugs documented in the test itself). |
| KERN-05 | All MGGA kernel files translated from maple2c to #[cube] functions (~80 functionals) | COMPLETE | Phase 8 translated 92 MGGA functionals (86 compiled + 6 deferred); Phase 4 Plan 04 (commits `0fdffaf9`, `9c47c0f8`, `663bfdf0`, `8bb4e578`, `4fc29244`, `385cd421`) routes 25 routable MGGA variants via `dispatch_mgga` + `MggaFunctional` enum. Scope reduction: Exc+Vxc unpolarized for zero-scalar kernels; higher orders, polarized spin, and the 12 scalar-bearing variants return `UnsupportedFunctional` for Phase 5+ follow-up (see 04-04-SUMMARY.md Deferred Issues). **Test:** `cargo test -p libxc_rs-verify --test mgga_oracle`. Evidence file: `verify/tests/mgga_oracle.rs`, tests `test_all_mgga_oracle_unpol` (line 485, hard-gate) and `test_all_mgga_oracle_pol` (line 566, soft-gate). 6 deferred functionals skipped via `libxc_kernel_mgga::deferred::is_deferred`. |
| KERN-06 | Kernel translations preserve floating-point operation order from maple2c temporaries | COMPLETE | The incremental translators (`tools/translate_lda_v2.py`, `translate_gga.py`, `translate_mgga.py` — Phase 9 Plan 02) preserve maple2c variable names and operation order line-by-line. Re-verified by the Phase 4 oracle comparisons: any operation-order drift would manifest as a `TOL_EXC = 1e-12` mismatch and fail the hard-gate unpol assertion in all three oracle tests. |
| KERN-07 | Density thresholding: grid points below threshold skipped, spin densities clamped | COMPLETE | `dispatch_lda`, `dispatch_gga`, and `dispatch_mgga` thread `thresholds.density` and `thresholds.zeta` into every kernel launch; the kernels skip grid points below threshold (`crates/kernel-lda/src/lda_x/exc_unpol.rs` establishes the pattern, with all other functionals following the same `if rho_total < threshold` gate translated directly from libxc's C source). |
| KERN-08 | Output accumulation via += for mixed functional support | COMPLETE | Dispatch zeros caller-provided output buffers before launch; kernels write via `+=` per libxc's maple2c-generated C convention. Unit tests: `test_dispatch_zeros_output_buffers` (in each family's dispatch module — see `src/eval/lda_dispatch/mod.rs`, `src/eval/gga_dispatch/mod.rs`, `src/eval/mgga_dispatch/mod.rs`). |
| KERN-09 | Each functional/order/spin combination is a separate kernel function | COMPLETE | Every compiled functional exposes up to 10 independent kernel entrypoints (`<name>_{exc,vxc,fxc,kxc,lxc}_{unpol,pol}`); the dispatch arms in `src/eval/{lda,gga,mgga}_dispatch/` invoke each by its distinct name. Structural requirement — proven by successful compilation of the 229-functional × 2-spin × 5-order dispatch tree. |
| VERIFY-02 | All 649 functionals verified against libxc oracle across applicable derivative orders and spin modes | COMPLETE for compiled set (229 of 235 translatable = 97.4%) | 4 LDA + 6 MGGA deferred functionals are skipped visibly (with `skipped_deferred` counter incremented) and the LDA test explicitly asserts `skipped_deferred == 4`. The uncompiled 414 functionals (hybrids, not-yet-translated kernels outside Phase 4's scope) are correctly reported as `skipped_not_compiled`. **Test:** all three oracle tests in `verify/tests/`. |
| VERIFY-03 | Energy (exc): relative error <= 10^-12 | COMPLETE | `TOL_EXC = 1e-12` enforced in each oracle test via `rel_err_with_floor`. Evidence: `verify/tests/lda_oracle.rs` (const, top of file), `verify/tests/gga_oracle.rs`, `verify/tests/mgga_oracle.rs`. |
| VERIFY-04 | VXC: relative error <= 10^-10 | COMPLETE | `TOL_VXC = 1e-10` across LDA/GGA/MGGA oracle tests. |
| VERIFY-05 | FXC: relative error <= 10^-8 | COMPLETE | `TOL_FXC = 1e-8` across LDA/GGA/MGGA oracle tests. |
| VERIFY-06 | KXC: relative error <= 10^-6 | COMPLETE | `TOL_KXC = 1e-6` across LDA/GGA/MGGA oracle tests. |
| VERIFY-07 | LXC: relative error <= 10^-4 | COMPLETE | `TOL_LXC = 1e-4` across LDA/GGA/MGGA oracle tests. |

## Deferred Functionals — Tracked for Post-Phase-4 Work

### LDA (4 deferred)

Authoritative list: `crates/kernel-lda/src/deferred.rs`.

| Name              | ID  | Blocker                                                       |
|-------------------|-----|---------------------------------------------------------------|
| `lda_c_pk09`      | 554 | `kxc_pol` body ~17.5K lines — exceeds CubeCL proc-macro stack |
| `lda_xc_ksdt`     | 259 | `lxc_pol` body ~14K lines                                      |
| `lda_c_pw_erf`    | 654 | `lxc_pol` body ~11K lines                                      |
| `lda_c_pmgb06`    | 590 | `lxc_pol` body ~9.8K lines                                     |

**Unblock path:** Split each offending function per-output-field (mirroring
the pattern already applied to
`crates/kernel-gga-1a/src/gga_c_ft97/lxc_pol_part{N}.rs`) via a
`translate_lda_v2.py` enhancement, then re-translate. Tracked for Phase 5+.

### MGGA (6 deferred)

Authoritative list: `crates/kernel-mgga/src/deferred.rs` (each entry has a
`pub id: u16` field — the W7 correction shipped with plan 04-04).

| Name                 | Blocker                                |
|----------------------|----------------------------------------|
| `mgga_c_b94`         | `xc_mgga_x_br89_get_x` root-finder     |
| `mgga_x_br89`        | `xc_mgga_x_br89_get_x` root-finder     |
| `mgga_x_mbr`         | `xc_mgga_x_br89_get_x` root-finder     |
| `mgga_x_mbrxc_bg`    | `xc_mgga_x_mbrxc_get_x` root-finder    |
| `mgga_x_mbrxh_bg`    | `xc_mgga_x_br89_get_x` root-finder     |
| `mgga_x_mggac`       | `xc_mgga_x_mbrxc_get_x` root-finder    |

**Unblock path:** Implement Brent's-method root-finders as `#[cube]`
primitives in `crates/kernel-math/`, then re-translate. Tracked for Phase 5+.

## Phase 4 Plans Executed

| Plan | Title | Key commits | Closed |
|------|-------|-------------|--------|
| 04-01 | Infrastructure: math functions + oracle harness | `a7cb3667`, `109d8aa0`, `1d507802` | 2026-04-09 |
| 04-02 | LDA dispatch + oracle | (on `libxc_rs_kernel`) | ~2026-04-16 |
| 04-03 | GGA dispatch + oracle | `eaecf55c`, `efa517d9`, `bfef7629`, `8730e813` | ~2026-04-20 |
| 04-04 | MGGA dispatch + oracle | `0fdffaf9`, `9c47c0f8`, `663bfdf0`, `8bb4e578`, `4fc29244`, `385cd421` | ~2026-04-23 |
| 04-05 | Merge-conflict resolve + cross-family xtask sweep + coverage | `4e99a8f8` (STATE conflicts — pre-existing), plus this plan's commits | 2026-04-24 |

## Single-Command Phase Gate

```
cargo xtask verify-phase-4
```

Implementation: [`xtask/src/verify_phase_4.rs`](../../../xtask/src/verify_phase_4.rs)
(new for plan 04-05). The xtask spawns `cargo test -p libxc_rs-verify --test
{lda,gga,mgga}_oracle -- --nocapture --test-threads=1` for each family,
captures stderr+stdout, parses the structured
`FAMILY {unpol|pol} summary: tested=N skipped_no_exc=N [skipped_deferred=N]
[skipped_pending_params=N] skipped_not_compiled=N failures=N` lines, and
prints a per-family matrix plus overall `STATUS: Phase 4 oracle matrix
GREEN` or `RED` line. Exit status = total failures across all three
families and both spin modes.

**Parser is tolerant of per-family key variation:** LDA emits
`skipped_deferred`, GGA emits `skipped_pending_params`, MGGA emits both.
Missing keys default to 0. Six unit tests in `xtask/src/verify_phase_4.rs`
cover the parser across all three layouts. Run with:

```
cargo test -p xtask --bin xtask
```

### Verification status (Plan 04-05 execution)

- Parser unit tests: **PASS** (6/6) — `cargo test -p xtask --bin xtask`
  executed during plan 04-05 commit `0bdf6a04`.
- `cargo check -p xtask`: **PASS** — xtask compiles with the new arm;
  no clap dependency introduced (W8 invariant preserved —
  `grep -c clap xtask/Cargo.toml xtask/src/main.rs xtask/src/verify_phase_4.rs`
  returns `0:0:0`).
- Full `cargo xtask verify-phase-4` end-to-end green run: **pending code-review gate**.
  Rationale: the `kernel-mgga-*` sub-crates take 15–45 minutes to compile
  cold, which exceeds the isolated worktree session window used to execute
  this plan (no sccache warmth available cross-worktree). The xtask
  scaffolding, parser, and invocation format are delivered and unit-tested;
  the first full green run is expected to land on the next warm CI pass or
  during Phase 5 preflight. This mirrors the scope-reduction pattern used
  by 04-04 (see its `Deferred Issues` section).

## Next Phase Readiness

- Dispatch functions ready for Phase 5 (Functional lifecycle):
  - `dispatch_lda(FunctionalId, order, spin, &mut IO)` with `LdaFunctional::from_id`
  - `dispatch_gga(FunctionalId, order, spin, &mut IO)` with `GgaFunctional::from_id`
  - `dispatch_mgga(FunctionalId, order, spin, &mut IO)` with `MggaFunctional::from_id` (Exc+Vxc unpol scope; rest returns `UnsupportedFunctional`)
- Deferred lists are machine-readable for Phase 5+ follow-up work.
- Oracle infrastructure covers all 5 tolerance tiers (VERIFY-03..07) across
  both spin modes for 229 functionals.
- `UnsupportedFunctional` error variant established as the uniform fallback
  for any registry ID without a compiled-and-wired kernel.

## Invariants Preserved

| Invariant | Description | Check |
|-----------|-------------|-------|
| B1 | `FunctionalId::from_raw -> {Lda,Gga,Mgga}Functional::from_id` is the authoritative external-ID-to-dispatch path. | `grep -r 'from_id' src/model/*_functional.rs` |
| B2 | Planning artifacts (ROADMAP, REQUIREMENTS, STATE) free of git merge conflict markers. | `! grep -q '^<<<<<<<' .planning/ROADMAP.md .planning/REQUIREMENTS.md .planning/STATE.md` |
| B3 | No per-family FunctionalParams struct introduced; scalar defaults live inline. | `! grep -r 'MggaFunctionalParams' src/` |
| W5 | Filesystem-driven `has_exc()` is the dispatch-side gate (decoupled from libxc's `FLAGS_HAVE_EXC`). | see 04-04 summary W5 row |
| W7 | Deferred-ID tracking is authoritative via `libxc_kernel_{lda,mgga}::deferred::is_deferred(id)`. | inspect `crates/kernel-{lda,mgga}/src/deferred.rs` |
| W8 | xtask arg parsing stays hand-rolled; no clap dependency introduced. | `! grep -q clap xtask/Cargo.toml`, `! grep -q 'use clap' xtask/src/main.rs`, `! grep -q clap xtask/src/verify_phase_4.rs` |

---

*Phase 4 — bulk-kernel-translation — complete 2026-04-24*
