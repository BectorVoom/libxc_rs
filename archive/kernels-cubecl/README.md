# Archived: CubeCL kernel tree

The `#[cube]` / CubeCL kernel crates — 305 per-functional crates plus
`math`, 254,180 kernel bodies, ~1.5 GB. Formerly `crates/kernels/`.

Superseded by `crates/kernels-rayon/` (plain Rust, rayon backend). See
`docs/adr/0001-rayon-over-cubecl.md` for the decision and the measurements.

## This tree is archived but deliberately still buildable

**Do not delete it, and do not let it rot.** It is the reference the rayon
kernels are verified against:

- `crates/kernels-rayon/verify` (`rkverify`) runs the same functional through
  both trees and requires the outputs to be **bit-identical**.
- So far only **`gga_x_pbe` vxc unpolarized** has been through that gate. The
  other 304 functionals have been *compiled* but not bit-verified. Verifying
  them requires this tree.

It stays a workspace *member* so `cargo build -p libxc-kernel-<func>` still
works, but it is **not** in `default-members`, so a bare `cargo build` no longer
compiles it.

## Known limits of this tree

- `libxc-kernel-mgga_c_tpssloc` **cannot be compiled on a 30 GB machine**
  (~25 GB peak in `cubecl-macros` expansion). It was excluded from
  `default-members` for that reason. The rayon equivalent checks in ~27 s.
- Building anything that pulls a whole family needs `--jobs 1` and a narrowed
  feature (`--no-default-features --features oracle-<fam>`); the default
  all-family build does not fit in memory. `oracle-mgga` does not fit at any
  job count.
- Single-job check times: `oracle-lda` ~21 min, `oracle-gga` ~60–90 min.

## Still pointing here

- `crates/libxc-eval` — the CubeCL eval path, also superseded (by
  `crates/libxc-reval`) but left intact so the two can be compared.
- `crates/kernels-rayon/verify` — the bit-exactness gate.
- `verify/`, `verify-canary/`, `xcbench/`.

The emitter that generated this tree is archived separately under
`tools/archive/cubecl/`.
