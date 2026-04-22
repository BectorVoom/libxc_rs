# Deferred items surfaced by Phase 04 bulk kernel translation plans

This file tracks out-of-scope issues discovered during Phase 04 execution.
Each entry is a Rule-3 scope boundary: the discovering plan did not fix
it, but the item is captured here for a future plan.

## From 04-03 (GGA dispatch + oracle)

### D-04-03-A — Polarized GGA kernel translations have a pre-existing bug

**Discovered:** 2026-04-23 during `test_all_gga_oracle_pol`.

**Scope:** Not this plan. The 04-03 plan wires dispatch; the polarized
`*_pol.rs` kernel files were translated earlier (see `707f5fbc split
large kernel`).

**Evidence:** 39 of 42 zero-scalar GGA functionals produce mismatched
`vrho`/`vsigma`/`v2*`/`v3*`/`v4*` values in Spin::Polarized mode.
Example: `gga_x_hcth_a vxc Polarized.vrho[0]: rust=-8.335e-1
c=-6.283e-1 rel_err=3.265e-1`. The ratio is consistently ~1.33, suggesting
a `4/3` factor applied at the wrong scope in the translated polarized
kernel or an indexing issue with polarized zk.

**Plan:** Phase 04 follow-up (plan 04-06 candidate — "fix polarized GGA
kernel translation factor"). Retain the eprintln diff-list in
`test_all_gga_oracle_pol` so regressions surface until fixed.

### D-04-03-B — Per-functional scalar ext_params defaults not wired

**Discovered:** 2026-04-23 during 04-03 Task 2 implementation.

**Scope:** Plan 04-03's B3 invariant called for hardcoding libxc's
canonical ext_params default values into each batch's launch helper.
Scope proved infeasible: 293 total scalar arguments across 63 functionals,
many involving C macros (`MU_PBE`, `MU_GE`) and expressions
(`0.066725*M_PI*M_PI/3`). Full extraction + verification per-functional
is estimated at 3-5 hours of research work (plus a long cargo rebuild
after every edit).

**Evidence:** 63 GgaFunctional variants return `UnsupportedFunctional
{ reason: "per-functional scalar defaults not yet wired" }` at dispatch
time. They're enumerated correctly via `from_id`, but no kernel launch
is attempted. See `skipped_pending_params=62` in both oracle test runs.

**Plan:** Phase 04 follow-up plans (likely one per param-complexity
tier):
  * 04-06: wire zero-to-three-scalar functionals (~40 of the 63)
  * 04-07: wire the three-to-sixteen-scalar functionals
  * 04-08: wire the template kernels (`gga_x_vmt`, `gga_k_tflw`, ...)
    where a single source backs 2–15 libxc IDs through varying defaults

### D-04-03-C — gga_x_herman (libxc id 104) not routable

**Discovered:** 2026-04-23 during roster enumeration.

**Scope:** libxc id 104 is on the removed list (`xc_funcs_removed.h`).
The kernel module exists at `crates/kernel-gga-22/src/gga_x_herman/`
with a full 10-arm translation, but `FunctionalId::from_raw(104)` returns
`RemovedFunctionalId`, so `GgaFunctional::from_id` cannot reach it. The
enum therefore has 105 variants (not 106 as the 04-03 plan anticipated).

**Plan:** Accept as-is. The kernel source is preserved for future use
should libxc un-remove id 104, but no action is required now. Noted in
`src/model/gga_functional.rs` module docstring.

### D-04-03-D — 21 GGA kernel modules with PARTIAL derivative coverage

**Discovered:** 2026-04-23 during roster enumeration.

**Scope:** The roster script's `classify` function recognizes two
kernel shapes: `FULL` (all 10 arms) and `VXC_ONLY` (8 arms, no exc).
Kernel modules with any other shape (e.g. `gga_c_ft97` has only
`exc_pol` + several `lxc_pol_part*` split files) are dropped from
the roster. These modules have real translated code but are not yet
safely dispatchable.

**Evidence:** 21 modules classified PARTIAL, including:
`gga_c_ft97`, `gga_c_optc`, `gga_c_sg4`, `gga_c_hjs`, `gga_c_pbeloc`,
`gga_c_zvpbeint`, `gga_c_zvpbeloc`, `gga_x_hjs`, `gga_c_pbe_erf_gws`,
`gga_c_q2d`, `gga_c_sogga11`, `gga_x_lcgau`, `gga_c_regtpss`,
`gga_c_lcgau`, `gga_x_wpbeh`, `gga_c_gapc`, `gga_c_acggap`,
`gga_c_gaploc`, `gga_x_hjs` (dup), `gga_c_scan_e0` (dup).

**Plan:** Phase 04 or later: complete the split-file / incremental-file
translations, or rebuild the flat-arm layout. Tracked in the roster
generator's `classify` docstring.
