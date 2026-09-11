# Project Agents Guide

## Project

`libxc_rs` is a Rust re-architecture of the public `libxc 7.0.0` API surface. The library keeps upstream capability reachability, but replaces the original C-style surface with a three-layer Rust design: compatibility shims, a typed safe core, and ergonomic high-level APIs.

**Core value:** deliver full libxc public capability coverage through a safer Rust API, with no C/Fortran in the production path.
 Rust with Rayon parallelism and optional explicit SIMD via `rmath`/`wide`.


## Layout

| path | role |
|------|------|
| `crates/libxc-core` | data layer: model, metadata, registry, input/output types, dimensions |
| `crates/kernels-rayon` | generated plain-Rust kernels (266 functional crates + `math`) |
| `crates/libxc-reval` | rayon eval layer: stride-aware parallel sweep, per-family dispatch, routing |
| `crates/libxc-compat` | C-ABI shim |
| `tools/translate_rayon` | the kernel emitter: maple2c C -> rayon Rust (see below) |
| `bench-vs-libxc` | head-to-head speed/memory benchmark against C libxc; needs `--features libxc-oracle` (see below) |
| `libxc-sys` | FFI bindings + cmake build of the vendored C oracle. Not a workspace member, `publish = false`, and reached only through an opt-in feature -- see "What ships" |
| `crates/libxc-eval` | orchestration types the facade and C-ABI take (`Functional`, `EvaluationWorkspace`); no longer holds any kernel path |




## Key Constraints

- Numerical execution is plain Rust + rayon (ADR 0001). 
- Kernels are generated. **Never hand-edit anything under `crates/kernels-rayon/`** — regenerate with `tools/translate_rayon/`.
- f64 only. Energy relative error must stay within 1e-12 of the libxc oracle.
- Maple2c formula translations must preserve floating-point operation order.
- The redesign cannot silently drop public functions, IDs, metadata paths, or removed-ID diagnostics.
- Public APIs must use typed Rust boundaries and `thiserror` v2 errors.
- libxc is an oracle for verification only; it is not part of the production runtime.
- Repeated workloads must reuse workspaces and caches rather than reallocating on hot paths.
- Kernel crates must stay cheap to compile. 


## Regenerating kernels

```bash
python3 tools/translate_rayon/from_maple.py --all                # kernels
python3 tools/translate_rayon/extract_params.py --json tools/translate_rayon/params.json
python3 tools/translate_rayon/gen_eval.py                        # eval layer + routing
python3 tools/translate_rayon/deorbitalize.py                    # SCAN-L chain rule (libxc-eval)
```

`from_maple.py` reads `libxc-master/src/maple2c/<fam>_{exc,vxc}/<func>.c` --
libxc's own Maple-generated C -- and emits one Rust function per
(functional, order, spin). 2,648 functions across 266 functionals, ~100 s.
`--all` also emits the **fused composite kernels** in `fuse.py::FUSED`
(`--fused NAME` for one): `crates/kernels-rayon/gga/fused_{hse,pbeh}`, one
loop per libxc mix, and `gen_eval.py` wires them through
`libxc-reval/src/fused.rs`. See "Fused composites" below.



maple2c emits **one fully CSE'd function per (order, spin)**, so reading it
directly needs none of that. Consequences, all measured:

- 305 crates -> **266** (the 39 shard crates are gone).
- Tree 569 MB / ~3,500 files -> **224 MB / 2,892 files**.
- 4% fewer operations overall, and up to 31% fewer on the biggest bodies
  (`gga_c_ft97 lxc_pol`: 54,463 -> 37,621 bindings) -- maple's own CSE beats
  `vnmerge`'s reconstruction of it.

The transform is still purely structural: expressions keep maple2c's exact
operand order and grouping, so floating-point results are unchanged. That was
checked, not assumed -- see below.

**The emitter refuses to guess.** The C vocabulary is small and closed, so after
translating an expression every identifier left in it must be a known local,
parameter, input, constant or math-crate helper; anything else raises
`Untranslatable` and the functional is reported rather than emitted wrong.
Adding a functional that uses a new construct will fail loudly.

**Dimensions are parsed from `crates/libxc-core/src/dims/mod.rs`, never
hand-written.** They were hand-written once and it was wrong: polarized
`v3sigma2lapl` is `6*2 = 12` (libxc `util.c`), not the 9 that counting index
combinations suggests. A wrong stride misaligns every subsequent grid point of
that output and is invisible in a spot check.

**A kernel tree may have fewer than ten modules, and then only as its flags
say (2026-09-11).** libxc ships fewer maple2c functions for a handful of
functionals: the potential-only ones (`maple2c/*_vxc`, compiled `XC_NO_EXC`:
`gga_x_lb`, `gga_x_lbm`, `mgga_x_tb09`, `mgga_x_bj06`, `mgga_x_rpp09`,
`lda_xc_tih`, `mgga_x_2d_prhg07_prp10`) have no exc, and `lda_c_pk09` /
`mgga_c_b94` stop at kxc. `gen_eval.py` used to skip any tree without all
ten, so all nine refused every order. It now emits such a functional when
the modules present are exactly the orders its `XC_FLAGS_HAVE_*` claim, in
both spins, through a file-local `partial_dispatch!` that takes the ten-arm
macro's invocation, refuses an unclaimed order with
`UnsupportedDerivativeOrder` before `prepare` touches a buffer (libxc's own
contract), and for a no-exc functional neither demands nor writes `zk`
(`prepare_for(.., have_exc = false)`). libxc `exit(1)`s if handed a `zk`
buffer for one of these, which is why `kernel_oracle*.rs` compare them
through `xc_*_vxc`.

**libxc's deorbitalized meta-GGAs (SCAN-L, revSCAN-L, r2SCAN-L and their
correlation halves) are neither a kernel nor a mix.** `xc_deorbitalize_init`
gives them two auxiliaries -- the base meta-GGA and `mgga_k_pc07_opt`, whose
`tau = rho * e_ked` stands in for the orbital one -- and
`xc_deorbitalize_func` chain-rules every derivative through that `tau` with
Maple-generated C (`maple2c/deorbitalize_{1..4}.c`). `deorbitalize.py`
translates that C operand for operand into
`crates/libxc-eval/src/eval/deorbitalize_gen.rs`, refusing anything outside
its closed vocabulary; `eval/deorbitalize.rs` drives it. Which functional is
deorbitalized, over what, and where each parent ext_param goes is read out of
libxc's own `xc_func_type` by `verify/tests/gen_deorbitalized.rs` into
`meta::DEORBITALIZED`, not scraped from the C.

## Build and editor hygiene

The generated kernel tree (266 crates) is workspace **`exclude`d**, not merely left out of `default-members`. `default-members` has no effect on `cargo check --workspace`, which is what rust-analyzer runs; as members they added hundreds of units to every editor session. Excluded, they are ordinary path dependencies.

- `cargo check -p libxc-rkernel-<f>` works for anything reachable from a member.
- Otherwise: `cargo check --manifest-path crates/kernels-rayon/<fam>/<f>/Cargo.toml`.
- Excluded harnesses need `--manifest-path`: `verify/`, `verify-canary/`, `crates/kernels-rayon/{verify,oracle}/`.


**Rayon-tree builds are parallelism-bound, not memory-bound.** A cold release build of the kernel tree is >40 min (debug builds take minutes). A rayon kernel rustc peaks at ~0.2-2.3 GB, and `jobs` also caps rustc's codegen-unit parallelism through the jobserver. The workspace profiles use `codegen-units = 16`; with `jobs = 12` the monster crates (e.g. `mgga_c_kcis`, 16 MB source since the fan-out flattening; 58 MB before) drop from ~5 min to ~1.5 min each, and codegen-unit count is runtime-neutral for these kernels (measured: identical ns/pt and checksums at CGU 2 vs 16). Note that a standalone `--manifest-path` build of an excluded kernel crate does not see the workspace `[profile.*]` at all — it uses cargo's defaults, which are already CGU 16 for release.


## Performance against libxc

`bench-vs-libxc` (`cargo run --release -p bench-vs-libxc --features
libxc-oracle --bin xcvs`) is the only harness that times both sides. The
`--features libxc-oracle` is not optional and not decorative: it is what pulls
in `libxc-sys`, and without it the `xcvs` target is not built at all
(`required-features`). It runs four legs -- serial libxc,
caller-parallelised libxc (the honest bar), this library single-threaded, and
this library's sweep -- cross-checks them elementwise, and prints a fingerprint
over `to_bits()` of every output so a codegen change can be shown bit-exact.

Current: **2.4-4.7x faster than caller-parallelised libxc** on GGA/MGGA, a tie
on `lda_c_vwn` (which is libm-transcendental-bound in both libraries). Zero
heap allocation per evaluation on both sides, measured with a counting
allocator and `mallinfo2`. Full numbers, method, and the changes that got there:
`docs/perf/vs-libxc.md`.

`docs/perf/kernel-codegen.md` is the follow-up: five translator-side codegen
levers implemented and measured, four worth ~0 and the fifth already at its best
setting. **Read it before optimising the emitter for speed** -- bounds-check
elimination, register-pressure scheduling and `powf` -> cbrt rewrites are
closed off with numbers. Its loop-invariant-hoisting verdict ("LICM already
does this") is **superseded**: it held only while `cbrt` was inline
arithmetic, and the emitter now hoists explicitly -- see the bullet below
and `docs/perf/vs-libxc.md`, "Three bit-exact levers".

**Explicit SIMD is opt-in per functional.** `from_maple.py` emits a kernel as
`wide::f64x8` only for the `(functional, order, spin)` triples in its
`SIMD_FUNCS` allowlist. The kernels already loop-vectorise 8-wide, so forcing
explicit SIMD where LLVM did *not* decline is a regression (`gga_x_pbe` 0.55x).
Candidates are qualified by `tools/translate_rayon/simd_qualify.py`, which
tries them in batches against `bench-vs-libxc`'s `xcqual` binary (Rust legs and
a fingerprint, no C side, any order or spin -- and since 2026-09-10 no C
*build* either: `libxc-sys` is behind the crate's opt-in `libxc-oracle`
feature, which `xcqual` does not need) and records every verdict —
accepts and rejects alike, with the numbers — in `docs/perf/simd-ledger.json`.
It applies a batch through the `LIBXC_RS_SIMD_EXTRA` environment variable
rather than editing the allowlist, so an interrupted sweep leaves the tree
untouched; writing winners into `SIMD_EXACT_FUNCS` is a separate `--apply`
step.
**This project uses rmath's bit-exact path only.** Every transcendental, in
both kernel forms, resolves to a `<BitExact, FullRange>` rmath kernel, so a
SIMD kernel's output is bit-identical to its scalar form *and* to the libm C
libxc calls. That is enforced structurally, not by convention: the upstream
crate is renamed `rmath_upstream` in `crates/kernels-rayon/math/Cargo.toml`,
and `libxc_rkernel_math::rmath` is a shadow module
(`math/src/rmath_bitexact.rs`) that re-exports the crate but overrides every
transcendental with the BitExact form. A bare `rmath::` inside the math crate
is a compile error rather than silent drift, and a kernel crate — which depends
only on the math crate — cannot reach the fast path at all. There is no
approximate emitter mode: `simd.py` has one math path and `simd_body` refuses
to emit any call it could not map to a bit-exact form.

**Why that is spelled out so forcefully (2026-08-31):** rmath's *own* free
functions (`rmath::exp`, `rmath::ln`, …) are deliberately its `Fast` path —
documented as such per function, and asserted by rmath's
`tests/fast_path.rs`, which requires `rmath::exp(x) == rmath::fast::exp(x)`.
This tree called them by accident, through `from_maple.py`'s `LIBM` map and
`simd.py`'s `FREE_EXACT`, and so ran approximate math against a 1e-12 contract:
measured against glibc, `ln` differed on 22% of inputs by up to **4 ulp**,
`atan` on 25% by 2 ulp, `exp` 11%, `cbrt` 8%. Nothing caught it, because both
kernel forms called the same approximate function and therefore agreed with
each other — fingerprints never moved, and `math/tests/simd_exact.rs` passed
by comparing **rmath against rmath**. That hole is now closed by
`simd_exact.rs::rmath_free_functions_are_bit_exact_against_platform_libm`,
which compares against `f64::` itself. **Do not "fix" rmath** — its behaviour
is intended; fix the call site.

Consequence for tuning: **every SIMD speedup recorded before 2026-08-31 was
measured on the fast path and overstates what bit-exact costs.** Bit-exact
vector-vs-scalar is roughly `ln` 1.5x, `exp` 2.8x, `cbrt` 1.8x, `atan` 1.5x,
against the fast path's 4.4x/5.9x/7.6x/14.7x. The SIMD win that remains comes
mostly from removing the libm *calls* so the grid loop vectorises 8-wide, not
from faster transcendentals. `#[inline(always)]` on the `simd::` functions is
load-bearing — outlined, they cost `lda_c_vwn` 1.47x. Details and procedure:
`docs/perf/simd-kernels.md`; the pure-Rust `libm`
(rust-lang/compiler-builtins) remains 0.14x as a runtime replacement and is
only useful as an accuracy reference.

Three things from that work bind future changes:

- **`.cargo/config.toml` must keep `-C target-cpu=native` in a
  `[target.'cfg(...)']` section, not `[build]`.** Cargo takes rustflags from the
  first category that applies (`RUSTFLAGS` env -> `target.<triple>` ->
  `target.<cfg>` -> `build`) and does not merge across them, so a `[build]`
  entry here loses silently to the `[target.'cfg(target_os = "linux")']` block
  in a developer's own `~/.cargo/config.toml`. That is how the tree spent its
  life compiling as SSE2 while the C oracle compiled with `-march=native`.
- **Output buffers are zeroed per chunk in `par_sweep`, not per array in
  `prepare`.** Worth 5-10% on the parallel path, and bit-neutral.
- **`screened_call` is not optional.** See below.
- **Scalar helpers no longer bar a kernel from the SIMD emitter.** `simd.py`
  maps `xc_erfcx` / `xc_E1_scaled` to `simd::erfcx` / `simd::e1_scaled`, and
  `simd_qualify.py` admits a kernel whose helpers are all in
  `simd.LANEWISE_HELPERS`. That is what got `gga_x_wpbeh` -- both exchange
  legs of HSE06, and 95% of its cost -- onto the allowlist (2026-09-07; vxc
  2.10x unpol / 1.68x pol, fxc 3.17x, fingerprints unchanged) and took HSE06
  from a tie with libxc to 2.3-2.4x. Those two first ran the scalar helper on
  each lane (`math/src/simd.rs::lanewise`); later the same day they became
  real vector forms -- the Faddeeva-table row gathered per lane, Clenshaw on
  eight lanes, the scalar's operation order kept, so still bit-identical
  (`math/tests/simd_exact.rs` sweeps every branch) -- which took the helpers
  from about half of `wpbeh vxc`'s time to a fifth and HSE06 to 3.3x/3.1x
  (`docs/perf/vs-libxc.md`, "HSE06: the helpers"). The lane-wise fallback
  remains for negative/NaN lanes only. **`gga_x_wpbeh lxc:pol` cannot be
  compiled as SIMD on a 30 GB box**: rustc reaches 28.8 GB on the 4 MB body
  and is OOM-killed -- and because the kill lands on the whole terminal
  scope, it takes the editor session with it. A SIMD `lxc_pol.rs` for it was
  committed by mistake in edb96b2354 (with twelve other tier-4 files whose
  ledger verdicts were `deferred-contention`, none in the allowlist) and
  regenerated back to scalar on 2026-09-07; the allowlist is the source of
  truth, and a kernel file that says "explicit SIMD" in its header while its
  triple is not in `SIMD_EXACT_FUNCS` is a leftover from an interrupted
  sweep, not a decision. `kxc:pol` has a ledger accept (3.16x) but is not
  applied; if it is ever wanted, build it alone and watch RSS. Other helpers
  (`lambert_w` already has a real vector form; bessel, dilogarithm, br89,
  integrate do not) could be admitted the same way if a hot kernel needs it.
- **Composite GGAs (`evaluate_mixed_gga`) run leaf by leaf, not sweep by
  sweep.** The grid is split as `par_sweep` splits it (`sweep_gga::par_leaves`,
  generated), and on each leaf every auxiliary runs into a leaf-sized buffer
  leased from `EvaluationWorkspace::leaf_scratch` (a pool, one buffer per
  worker that was ever busy at once) and is folded into the caller's output
  while the leaf is in cache. Scratch is `O(workers * leaf * components)`
  instead of `O(np * components)` -- 0.6 / 1.2 MB for HSE06 vxc at any grid
  size, against 4 / 8 MB whole-grid and the 56 / 614 MB that
  `EvaluationWorkspace::new` used to allocate up front -- and the whole-grid
  scratch is now lazy (`scratch_len` is the promise, `scratch_allocated` the
  fact), so a GGA composite never materialises it. Bit-identical to the
  one-chunk evaluation: same zero, same `+= w * aux` per element in the same
  order; `bench-vs-libxc` asserts `rust-1t == rust-Nt` bitwise on every
  composite case and `eval::mix` tests do the same for PBE0, B3LYP and HSE06.
  PBE0 got 10% from losing the serial passes; HSE06 nothing measurable, its
  cost is the kernel. The LDA and MGGA mixed paths still use the whole-grid
  scratch. `set_min_chunk` is process-wide, so tests that touch it take
  `MIN_CHUNK_GUARD`.
- **HSE06 and PBE0 (and HSE03/12/12s, PBE50, PBE0-1/3) run as one fused
  kernel, not as a mix (2026-09-07).** `tools/translate_rayon/fuse.py`
  concatenates the auxiliaries' maple2c bodies, value-numbers them so a
  subexpression two legs share is computed once, specialises HSE's first
  `wpbeh` leg on its pinned `omega = 0` by an abstract fold over the C
  (a signed zero absorbs products, drops out of sums, decides comparisons;
  anything else it reaches keeps its libxc definition), and emits the mix's
  own `out += w_k * (0.0 + v_k)` inline. Bit-identical to the mix by
  construction and by gate: `bench-vs-libxc` evaluates every composite case
  both ways and requires 0 differing values, `libxc-eval`'s
  `fused_composite_is_bit_identical_to_mix` does the same at exc/vxc/fxc on
  a screened grid, and the oracle tests go through `Functional::evaluate_gga`
  and so through the fused path. HSE06 went 29.5 -> 18.9 ns/pt unpol vxc,
  72.5 -> 49.9 pol, 673 -> 129 pol fxc (5.0x/4.8x/5.8x vs libxc), PBE0
  7.5 -> 5.8 / 13.9 -> 11.3, with **no scratch at all** (leaf pool 0, zero
  allocations per call). The dispatch (`libxc-reval/src/fused.rs`,
  generated) falls back to the mix whenever the auxiliaries, thresholds,
  order/spin arm or a specialised parameter do not match, so changing
  `_beta`/`_omega_PBE` at runtime still fuses and changing the first leg's
  omega does not. kxc/lxc stay on the mix. The two assumptions the
  specialisation rests on are stated in `fuse.py`'s module docs; do not add
  a `bind` to a leg without checking them for that functional.
  **Each leg screens and clamps at its own `dens_threshold` (2026-09-10).**
  libxc's `xc_mix_init` calls `xc_func_init` per auxiliary, so HSE06's legs
  are `gga_x_wpbeh` twice at 1e-14 and `gga_c_pbe` at 1e-12, and PBE0's are
  `gga_x_pbe` at 1e-15 and `gga_c_pbe` at 1e-12. One shared threshold gave two
  of three legs a screen libxc does not use. The fused kernels now take one
  `dens_threshold_k` per leg, feed it to that leg's own maple2c guards, and
  wrap each leg's accumulation in `piecewise3(dens < dens_threshold_k, 0, ..)`
  -- so a point above one leg's threshold and below another's contributes to
  the first only, exactly as the mix does.
  What a fused kernel *cannot* reproduce is a per-leg **clamp**, because a
  clamped input is a different input and sharing one set of inputs is the whole
  point of fusing. `crate::screen::fused_legs_agree` is the guard: one linear
  pass that refuses the fused path unless every leg which survives its own
  screen would have had its inputs left alone. That holds on every benchmark
  and oracle grid and on the bulk of a molecular quadrature; where it does not,
  the call runs as the mix, which clamps per auxiliary and is correct by
  construction. `eval::mix`'s `fused_composite_is_bit_identical_to_mix`
  asserts the predicate before comparing, so the gate cannot quietly decay
  into mix-against-mix.

  `XCVS_NO_FUSED=1` times the mix path; `libxc_eval::eval::set_fused_enabled`
  is the process-wide switch.
- **Loop-invariant statements are hoisted by the emitter, `/ 2^k` is
  folded to `* 2^-k`, and `simd::cbrt` got a vectorised bit-exact form
  (2026-09-07).** `kernel-codegen.md`'s "LICM already hoists" verdict was
  true only while `cbrt` was inline arithmetic; rmath's bit-exact `cbrt` is
  a per-lane loop LLVM cannot move, so every SIMD kernel recomputed its
  `cbrt(pi^2)`, `cbrt(zeta_threshold)`, ... on every 8-point step (three of
  `gga_x_pbe`'s four cube roots, 14.6 of its 20.3 ns/pt).
  `simd.py::split_invariant` (both emit paths) moves every `let` that reads
  only constants/params/thresholds above the loop; `from_maple.py::
  fold_pow2_div` rewrites power-of-two literal divisors (exact by IEEE, and
  what GCC does to libxc's C); the exponent-surgery fix for `cbrt` was
  built here first (`i64x8` + `bytemuck`, 4.9 -> 2.2 ns/elem) and then
  **ported upstream into `rmath` itself** the same day
  (`~/workspace/rmath`, `src/kernels/double/cbrt.rs`, "Cycle 9"), so
  `math/src/simd.rs::cbrt` is now a one-line `rmath::cbrt(x)` delegation
  rather than a second copy of the algorithm -- `bytemuck` was dropped
  from the math crate's `Cargo.toml` accordingly. Confirmed bit-identical
  by a full regen + rebuild + oracle/`revalcheck` cycle after the switch:
  every fingerprint below is unchanged. All three
  are bit-neutral by construction and gated by unchanged fingerprints;
  `gga_x_pbe vxc unpol` 20.3 -> 5.4 ns/pt, PBE0 5.0-5.5x vs libxc, HSE06
  6.1x. Divisions by non-power-of-two constants (144 per point left in
  `fused_hse vxc unpol`) are the largest remaining cost and must stay
  divisions. Any math-crate change rebuilds all 266 kernel crates for the
  bench (~25 min); prototype on the math crate alone first.
- **The grid loop now vectorises 8-wide (AVX-512), not 2-wide SLP.** The note in
  the CLAUDE.md risk table about "always SSE, `xmm` only" described the
  pre-`target-cpu` build. Anything that puts a function boundary or a libm call
  inside the loop still destroys it.

## What ships, and what is only here for verification

The package directory of the published `libxc_rs` crate **is the repository
root**, so by default `cargo package` would sweep up everything beside it: 372
MB of vendored C in `libxc-master/`, the `bench-vs-libxc` harness, the
`verify/` suite, and 22,654 Windows `:Zone.Identifier` files whose names cargo
refuses outright (`cannot package a filename with a special character ':'`).
Before 2026-09-10 `cargo package -p libxc_rs` simply failed on that last point.

Three mechanisms keep the two apart, and they do different jobs:

1. **`[package] include` in the root `Cargo.toml` -- an allowlist.** This is
   what decides the tarball; it is 4 entries and ships 11 files. A denylist
   would have to be kept in step with every new top-level directory. Note
   `src/model` is deliberately *not* in it: that is a symlink into
   `crates/libxc-core/src/model` which the regeneration tools want on disk and
   which nothing in this crate compiles (`lib.rs` re-exports
   `libxc_core::model`), and packaging a symlink out of the package root is at
   best unportable.

2. **Opt-in `libxc-oracle` features on `bench-vs-libxc` and `xtask`.** These
   are the only workspace members that touch `libxc-sys`, and they now do so
   through `optional = true` + `dep:`. This is what stops a build from
   *compiling* the oracle, which `include` cannot do: `cargo build`,
   `cargo test`, `cargo check -p bench-vs-libxc`, and `simd_qualify.py`'s
   `xcqual` build all used to run cmake over `libxc-master/` first -- roughly
   an hour, dominated by `mgga_x_br89.c` -- and none of them do now (measured:
   zero `libxc-sys` units in `cargo build -v` and in the `xcqual` build).
   `xcvs` carries `required-features = ["libxc-oracle"]` so the target is
   skipped rather than failing to link, and `xtask generate-metadata` reports
   the flag it needs instead of not existing.

   **`cargo check --workspace` is the exception, and it is a cargo limitation
   rather than an oversight.** Cargo makes a path dependency of a member into a
   member whether or not it appears in `exclude`, so `libxc-sys` is still
   enumerated (`cargo metadata` reports it) and still checked. Prising it out
   would mean excluding `bench-vs-libxc` and `xtask` as well, and they would
   then stop sharing the workspace lockfile -- which is precisely how
   `verify/Cargo.lock` drifted to `wide` 1.7.0 against the library's 1.6.1. A
   benchmark that resolves a different SIMD crate than the library it measures
   is not a benchmark, so that trade was refused.

3. **`publish = false`** on `libxc-sys`, `verify`, `verify-canary`, `xtask`,
   `bench-vs-libxc` and the rayon `oracle` crate, so none of them can reach a
   registry even by accident.

**The root crate has no dev-dependency on `verify` any more.** It used to, and
that single line meant a plain `cargo test` on the library built the whole C
oracle -- cargo strips a path-only dev-dep at publish time, so this was never a
*packaging* problem, but it was an hour of everybody's time. The two root test
files that needed it (`invariants_mgga.rs`, `oracle_c_libxc_parity.rs`) moved
to `verify/tests/`, where the other 17 oracle files already live. A consequence
worth knowing: nothing in the workspace depends on `verify/` now, so its
`exclude` entry finally takes effect and it needed its own `[workspace]` table
(cargo: "current package believes it's in a workspace when it's not").

`cargo package --list -p libxc_rs` is the check. It should print 11 lines.

**A standalone harness needs the root's profiles *and* the root's lockfile, or
it is not testing the library you ship.** Making `verify/` genuinely standalone
had two consequences that are easy to miss and were both fixed on 2026-09-10:

- Its own `[profile.*]` was cargo's defaults rather than the root's, and
  profile flags go into every unit's fingerprint -- so all 266 kernel crates
  missed the shared target directory and rebuilt from scratch. `verify` now
  mirrors the root profiles exactly. Measured: one transitional rebuild of the
  tree, after which a `cargo test --manifest-path verify/Cargo.toml` that
  follows a root build recompiles ~6 units (`libxc-reval`, `libxc-eval`,
  `libxc-compat`, `libxc_rs`, `toml`) instead of 266.
- Its own `verify/Cargo.lock` came into force for the first time (it had been
  inert while the root's dev-dependency made `verify` a de-facto member), and
  it had drifted: **26 dependencies resolved to different versions, including
  `wide` 1.7.0 against the library's 1.6.1**. `wide` is the explicit-SIMD crate
  -- `f64x8` -- so the oracle would have been validating kernels built against
  a different vector implementation than the one that ships, which is exactly
  the class of difference the harness exists to catch. The lock is now a copy
  of the root's; keep it that way.

  `crates/kernels-rayon/oracle/Cargo.lock` still shows 24 such mismatches, but
  `wide` and `safe_arch` agree there and the rest (`bytemuck`, `bitflags`) are
  inert for numerics. It also pins `opt-level = 2`, so it cannot share
  artifacts regardless. Pre-existing, and left alone.

## Verification

**Run the `verify/` suite in two parts.** Every test there needs libxc linked
in as the oracle, except `compat_smoke`, which needs `--features c-abi` -- and
that feature is mutually exclusive with the rest, because the compat shim
exports the same C symbol names as libxc itself, so a binary links one or the
other:

```bash
# The 22 oracle files. They must be named: `cargo test` builds *every* target
# before it runs any, and `compat_smoke` does not link without `c-abi` (mold:
# undefined symbol: xc_func_init, xc_lda_exc, ...), so leaving it in the
# invocation silently reports nothing at all rather than 22 results.
cargo test --release --manifest-path verify/Cargo.toml \
    --test kernel_oracle --test kernel_oracle_fxc --test composite_oracle \
    --test composite_diagnose --test gen_aux_overrides --test hse06_oracle \
    --test screening_helpers --test wpbeh_domain --test root_finders \
    --test libm_parity --test input_sanitisation \
    --test hybrid_oracle --test hybrid_type_oracle \
    --test metadata_oracle --test mixed_oracle --test parity_phase09 \
    --test parity_phase11 --test invariants_mgga \
    --test oracle_c_libxc_parity \
    --test refusal_sweep --test deorbital_oracle --test gen_deorbitalized

cargo test --release --manifest-path verify/Cargo.toml --features c-abi \
    --test compat_smoke                                                         # on its own
```

**`LIBXC_RS_FP_CONTRACT=off`** rebuilds the vendored oracle with
`-ffp-contract=off`, so its C evaluates `a*b + c` as written instead of
contracting it into an FMA the way GCC does by default and rustc never does.
It is an attribution tool, not a mode: it compares against a libxc nobody
builds.

Budget an hour each way. It reuses the same cmake build directory, so toggling
it recompiles all 319 objects, and the wall clock is dominated by two files:
`mgga_x_br89.c` takes ~40-55 minutes on its own at 5.3 GB of RSS (with
contraction *on* as well as off -- this is `-march=native -O2` on a maple2c
body, not something the flag causes), with `gga_c_ft97.c` a distant second.
Everything else finishes in the first few minutes.

| file | what it proves |
|---|---|
| `kernel_oracle.rs` | every routed kernel (454) vs libxc, **both spins, all three families**, zk + first derivatives |
| `kernel_oracle_fxc.rs` | second derivatives of every routed LDA/GGA kernel (308), both spins |
| `composite_oracle.rs` | every composite functional (124 GGA + 34 MGGA + 2 LDA) vs libxc |
| `composite_diagnose.rs` | when a composite fails: diffs our mix against libxc's own `xc_func_type` through the FFI |
| `gen_aux_overrides.rs` | regenerates `meta::generated_aux_overrides` from libxc |
| `hse06_oracle.rs` | the HSE family, and `gga_x_wpbeh` at a non-default screening parameter |
| `screening_helpers.rs` | `xc_erfcx` and `xc_E1_scaled` against libxc's C directly |
| `root_finders.rs` | `xc_mgga_x_br89_get_x`, `xc_mgga_x_mbrxc_get_x` and `LambertW` against libxc's C directly -- the three helpers whose answer depends on a stopping rule |
| `input_sanitisation.rs` | the `rho`/`sigma`/`sigma_ab`/`tau` clamps and the Fermi-hole curvature bound, each on a grid built to violate exactly that bound (and asserting the violation, so none can pass vacuously) |
| `libm_parity.rs` | `rmath::erf`/`erfc` against the platform libm libxc calls; `simd_exact.rs` cannot cover these two because Rust's `f64` has neither |
| `wpbeh_domain.rs` | where `gga_x_wpbeh` diverges as a function of reduced gradient |
| `invariants_mgga.rs` | MGGA `zk`/`vxc` invariants against libxc; **moved here from the root crate's `tests/` on 2026-09-10** so `cargo test` on the library stops building the C oracle |
| `oracle_c_libxc_parity.rs` | broad C-parity sweep; moved here for the same reason |
| `refusal_sweep.rs` | every public id, both spins, every order its flags claim, through `BatchEvaluator`: a refusal not on its allowlist fails, and so does an allowlisted refusal that starts working -- the list can only shrink. Return codes, not values |
| `deorbital_oracle.rs` | the deorbitalized SCAN-L family (700-704, 718, 719) against libxc at every order, both spins, and at non-default ext_params |
| `gen_deorbitalized.rs` | `meta::DEORBITALIZED` is still what libxc's `xc_func_type` says (`LIBXC_RS_WRITE_DEORBITALIZED=1` regenerates it) |

`gga_oracle.rs`, `lda_oracle.rs`, `mgga_oracle.rs`, `lda_x_oracle.rs` and
`lda_x_stress.rs` were **deleted** on 2026-09-03. The first three were gated on
`oracle-{gga,lda,mgga}` features removed with the CubeCL backend and compiled
to zero tests; the last two were written against the removed
`libxc_rs::kernel::launch` API and did not compile. `kernel_oracle*.rs` covers
their ground and more.


| harness | what it proves | invocation |
|---------|----------------|------------|
| `crates/libxc-reval` (`revalcheck`) | chunked parallel evaluation is bit-identical to a whole-grid call | `cargo run --release -p libxc-reval --bin revalcheck` |
| `crates/kernels-rayon/oracle` | rayon backend matches **C libxc 7.0.0** within 1e-12 | `cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml` |

`revalcheck` only shows the chunked sweep agrees with a whole-grid call. The oracle harness is the one that shows the numbers are right — prefer it when judging correctness.



That harness counts **NaN-vs-NaN differences separately** from real ones, and the distinction matters. It feeds each input array independently at random, which for MGGA produces points outside the functional's domain (`tau` below the von Weizsäcker bound `sigma/8rho`), and those evaluate to NaN. Deduplicating a computation can flip the *sign bit* of such a NaN — `mgga_x_scan` shows 1,679 of them — because a value the split form derived twice down two expression paths is now derived once. No finite value changes: the gate is 0 real mismatches, and NaN payload is IEEE-unspecified anyway. `revalcheck` and the oracle harness use physical inputs and do not hit this.


## Screening and sanitisation are correctness requirements, not tuning knobs

libxc screens below-threshold points *outside* the maple2c body
(`work_*_inc.c`: `if(dens < p->dens_threshold) continue;`), so the screen covers
every functional. Only some kernels in this tree carry a `dens_threshold` guard
of their own -- the exchange functionals mostly do, the correlation functionals
mostly do not (`lda_c_vwn`, `gga_c_lyp`, `mgga_c_r2scan` have none). Calling the
kernel body directly therefore gave the raw formula value on the empty tail of a
molecular grid where libxc gives zero: **measured at 100% relative error on
`zk`** for `lda_c_vwn` and `gga_c_lyp` on a grid with 40% of points below
threshold, while guarded functionals agreed to 1e-15.

`screened_call` in the generated `sweep_*.rs` applies libxc's own test before
the kernel sees a point. Removing or bypassing it reintroduces the defect for
every unguarded functional. It is bit-neutral where a guard already existed
(those outputs are `piecewise3(guard, 0.0, ..)` terms collapsing to `+0.0`),
which is checked by fingerprint.

It has a deliberate second route: a chunk whose above-threshold runs would
average under `MIN_RUN` (128) points runs the kernel over everything and re-zeros
the screened points instead of splitting. Splitting into ~1.7-point runs costs
about 14 ns per call and made `gga_x_b88` *slower* than doing nothing (1.98 ->
6.84 ns/pt). Real quadratures order points by radial shell, so their empty
points are contiguous and always take the fast route.

**The screen is only half of what libxc does before the maple2c body runs, and
the other half was missing until 2026-09-10.** `work_{lda,gga,mgga}_inc.c`
also *sanitises* the inputs it is about to use:

```c
my_rho[0]   = m_max(p->dens_threshold, rho[0]);
my_sigma[0] = m_max(p->sigma_threshold * p->sigma_threshold, sigma[0]);
if(p->info->flags & XC_FLAGS_NEEDS_TAU){
  my_tau[0] = m_max(p->tau_threshold, tau[0]);
  my_sigma[0] = m_min(my_sigma[0], 8.0*my_rho[0]*my_tau[0]);   /* Fermi hole */
}
/* polarized: the same for spin 1, then */
s_ave = 0.5*(my_sigma[0] + my_sigma[2]);
my_sigma[1] = clamp(my_sigma[1], -s_ave, +s_ave);
```

None of that is optional. The Fermi-hole curvature bound is **on by default in
libxc's own CMake** (`if(NOT DISABLE_FHC)` -> `-DXC_ENFORCE_FERMI_HOLE_CURVATURE`),
so the oracle this library is measured against has it. Any grid that feeds the
input arrays independently -- which is what the oracle and `revalcheck`
harnesses do -- lands `tau` below the von Weizsaecker bound `sigma/(8 rho)` at
a large fraction of its points; libxc clamps `sigma` down to `8 rho tau` there
and this tree did not. The `+-s_ave` clamp on the cross term is the same story
for polarized GGA and MGGA, and `m_max(dens_threshold, rho)` for a polarized
point with one empty spin channel. `verify/tests/input_sanitisation.rs` pins
each clamp on points chosen to trigger it, which is the only way to see them:
a grid built to stay inside the domain (`bench-vs-libxc`'s `grid::mgga` does
this deliberately) never exercises one.

`crates/libxc-reval/src/screen.rs` (generated) carries it, and the generated
`sweep_*.rs` apply it in `screened_call`. Three things about the
implementation are load-bearing:

- **`m_max` is not `f64::max`.** libxc's is `(((x)<(y)) ? (y) : (x))`, so
  `m_max(t, NAN)` is `t`. The ternaries are reproduced, not approximated, and
  the `+-s_ave` clamp is two ternaries rather than `f64::clamp` because a NaN
  has to come out as `-s_ave`.
- **The fast path is the predicate, and the predicate is the transform.**
  `needs_sanitise` runs `sanitise_point` and compares bit for bit, so it cannot
  drift from what `sanitise_into` would write. On a physical quadrature nothing
  clamps -- `sigma_ab` really is bounded by `(sigma_aa+sigma_bb)/2`, `tau`
  really does satisfy the von Weizsaecker bound -- so the kernel keeps reading
  the caller's own slices with no copy. The copy (a per-worker `thread_local`
  scratch, *taken* out of its cell rather than borrowed, so a composite
  dispatching an auxiliary on the same worker cannot double-borrow) only
  happens on the randomised grids the harnesses build.
- **The screen still reads the caller's `rho`, never the sanitised one.** Every
  sanitised `rho` is at least `dens_threshold` by construction, so screening on
  it would admit the whole tail. `run_screened_with` takes the raw array as a
  separate argument for that reason.

**`XC_FLAGS_NEEDS_TAU` decides the tau clamp and the Fermi-hole bound**, and
when it is clear libxc passes `my_tau = {0.0, 0.0}` -- its `my_tau` is declared
outside the point loop and written only under the flag. For 23 of the 24 such
functionals the emitted kernel loads `tau` and never reads the binding, so the
caller's value goes straight through and no copy is forced;
`mgga_x_2d_prhg07_prp10` is the one that *does* read it (`XC_FLAGS_2D |
XC_FLAGS_NEEDS_LAPLACIAN`, no `NEEDS_TAU`, and a maple2c body that uses `tau`
anyway) and gets the zeros. `gen_eval.py::kernel_reads_tau` decides this per
kernel by reading the emitted source rather than assuming either way -- an
earlier version of this work assumed "no flag means no use", and that
functional is what caught it.

## Thresholds are per functional

`xc_func_init` seeds every threshold from the functional's own info block:

```c
func->dens_threshold  = func->info->dens_threshold;
func->sigma_threshold = pow(func->info->dens_threshold, 4.0/3.0);
func->zeta_threshold  = DBL_EPSILON;
func->tau_threshold   = 1e-20;
```

`dens_threshold` is 1e-15 for 432 of the 649 functionals, but 1e-14 for 113,
1e-12 for 40, 1e-13 for 10, 1e-32 for 8, and single functionals sit as high as
5e-7. This tree carried one global `Thresholds::default()` until 2026-09-10, so
**217 functionals ran with a different screen and different input clamps than
libxc**. `FunctionalMeta::default_density_threshold` had been generated from
libxc all along and was never read.

`Thresholds::for_functional(id)` is the constructor to use anywhere a
functional is in hand; `Functional::new` uses it. Two details:

- `sigma` is `density.powf(4.0/3.0)`, evaluated, not tabulated: `pow(1e-15,
  4.0/3.0)` is `1.0000000000000027e-20`, not the double nearest `1e-20`. The
  old hardcoded `1e-24` had no counterpart in libxc at all.
- The setters ignore a non-positive argument, as libxc's
  `xc_func_set_*_threshold` do, and recurse into the auxiliaries either way.

**Each auxiliary of a mix keeps its own.** `xc_mix_init` calls `xc_func_init`
per aux, and `xc_mix_func` then evaluates each through the full `xc_gga` /
`xc_mgga` entry point, so each screens and clamps at its own threshold.
HSE06's legs are `gga_x_wpbeh` twice at 1e-14 and `gga_c_pbe` at 1e-12; PBE0's
are `gga_x_pbe` at 1e-15 and `gga_c_pbe` at 1e-12. See "Fused composites" for
what that costs the fused path.

## Root-finders: the stopping rule is part of the answer

Three helpers are iterations rather than expressions -- `br89`'s exchange-hole
inversion, `mbrxc`'s cuspless-hole inversion, and `lambert_w` -- and all three
were CubeCL-era transcriptions that ran a **fixed** number of unrolled steps
with branchless `select`, because `#[cube]` kernels had no dynamic loops.
Brent's method does not stand still once it is inside the tolerance: it keeps
interpolating and bisecting within the bracket. Running 60 steps therefore
returns a different point of the same bracket than libxc's "return `(a+b)/2` as
soon as `|b-a| < TOL`", and BR89's `TOL = 5e-12` is **absolute**, so at a root
of 3e-8 it pins only about four significant digits. That is the residual
`mgga_x_br89`, `mgga_x_br89_1`, `mgga_x_b00`, `mgga_x_mggac` and the composite
`hyb_mgga_xc_br3p86` all carried, and no amount of re-checking the formulas
explained it.

`crates/kernels-rayon/math/src/brent.rs` is now a single transcription of
libxc's `xc_math_brent`, loop and early return included, shared by both
inversions. `lambert_w` had three further gaps: `eps` was `1e-15` rather than
`DBL_EPSILON` (so `CBRT(eps)`, the small-`z` series cutoff, was 1.0e-5 instead
of 6.06e-6), the `z` just below `-1/e` case was missing, and `w != -1.0` had
become `|w + 1| < 1e-300`. libxc also returns **0.0** when the iteration limit
is reached, which is now reproduced; the `f64x8` form freezes a converged lane
instead of returning, which is the same thing, and `math/tests/simd_exact.rs`
holds it bit-identical to the scalar. `mbrxc_x_Q` additionally had an `exp`
underflow guard copied from `br89_x_Q` -- libxc's `br89_x_Q` has one, its
`mbrxc_x_Q` does not, and it zeroed `exp(-arg)` from `arg > 115` upward where
the true value is still ~1e-50.

**Test these against libxc's C directly, not through a functional.**
`verify/tests/root_finders.rs` calls the exported `xc_mgga_x_br89_get_x`,
`xc_mgga_x_mbrxc_get_x` and `LambertW` on a quarter-million arguments each.
A functional comparison can only ever say "`vsigma` is 3.8e-9 out"; this says
which iterate diverged.

## Known gaps

- **The public API refuses nothing it claims (2026-09-11).**
  `verify/tests/refusal_sweep.rs` evaluates every public id, both spins, at
  every order its `XC_FLAGS_HAVE_*` claim, through `BatchEvaluator`, and its
  allowlist is empty. It started at 57 functionals over five mechanisms
  (`docs/PLAN-defect-remediation-v6.md`): 36 composite MGGAs at kxc/lxc (the
  mix never grew third/fourth-order buffers), `hyb_mgga_xc_b0kcis` above vxc
  (its own-kernel half was stubbed), nine partial kernel trees plus
  `hyb_mgga_xc_b94_hyb` (the generator demanded ten modules), the seven
  deorbitalized SCAN-L functionals (no deorbitalization at all) and the four
  gds08 composites (no worker kernel). The only functional without a public id,
  `lda_k_gds08_worker`, is out of scope by design and never enters the sweep.
- **libxc's own deorbitalization reads uninitialized memory at fourth order.**
  `xc_mgga_vars_allocate_all` (`deorbitalize_func.c:188-189`) mallocs the base
  meta-GGA's `v4sigmalapltau2` and memsets `v4sigmalapl2tau` twice instead;
  `xc_mgga` zeroes a lapl-tau cross field only for a functional with both
  `NEEDS_LAPLACIAN` and `NEEDS_TAU` (`mgga.c:262`), and no SCAN-L base needs the
  Laplacian. `deorbitalize_4.c` reads that buffer into six lxc outputs
  (`v4rho2sigmalapl`, `v4rhosigma2lapl`, `v4rhosigmalapl2`, `v4sigma3lapl`,
  `v4sigma2lapl2`, `v4sigmalapl3`), so libxc's values there depend on what the
  heap held: identical to this tree on a fresh heap, noise after a long sweep
  (1.04 relative, measured). This tree zeroes the buffer. The oracles skip
  exactly those six fields (`LIBXC_UNZEROED`) and gate everything else bit for
  bit. Do not "fix" the Rust side to match.
- **A rejection in `docs/perf/simd-ledger.json` is only valid for the tree it was
  measured on.** Re-swept 2026-09-03: of the 120 hottest undecided tier-1
  candidates over two sweeps, **262 of 264 verdicts accept** (median 1.76x,
  range 0.87-2.69x); two were deferred for contention and **two rejected on
  merit**, which is what shows the driver discriminates. The allowlist went
  68 -> **308 triples across 149 functionals**, and `gga_x_b88` went from 0.87x
  *behind* libxc to 1.64x ahead. **About 1,400 candidates remain across tiers
  1-4** -- resume with `simd_qualify.py --tier N`; the ledger records every
  verdict, and `gga_x_b88 exc+vxc+fxc` at 0.96x is the visible cost of the
  tier-3 backlog rather than a regression. `gga_x_pbe` carried a standing "do not SIMD, LLVM already
  vectorises it" rejection at 0.55x. That was true when `pow_1_3` resolved to
  `powers.rs::cbrt_f64`, inline branch-free arithmetic LLVM packed 8-wide.
  Commit 31fd1ff47f repointed it at `rmath::cbrt` and 4395787e90 pinned that to
  `BitExact` -- correct numerically (bit-identical to glibc on 100% of 2M
  physical inputs, which the inline version was not) but an opaque ~9.6 ns/elem
  **call**, and a call in the grid loop stops the loop vectorising. Every kernel
  the inline cbrt had been carrying lost its vectorisation silently:
  `gga_x_b88`'s sweep went from the documented 2.18 to 9.45 ns/pt against an
  unchanged libxc. Fingerprints do not move when a loop stops vectorising, so
  nothing caught it. PBE was re-qualified and now runs 1.8-2.6x faster with
  identical fingerprints, and the sweep above confirmed the same for another
  118 triples. Any pre-2026-08-31 verdict should be assumed stale.


- **`gga_x_wpbeh`'s `vsigma` diverges from libxc as the reduced gradient goes to
  zero, and always did.** Characterised 2026-09-03 by
  `verify/tests/wpbeh_domain.rs`, which sweeps (rho, s) directly rather than
  relying on a random grid. At `omega = 0`, relative difference in `vsigma`:

  | s | 1e-8 | 1e-5 | 1e-3 | 1e-2 | 5e-2 | >= 0.1 |
  |---|---|---|---|---|---|---|
  | rel err | 6.3e0 | 1.6e-6 | 1e-9 | 1e-11 | 4e-13 | <= 2e-13 |

  `zk` and `vrho` stay at 2e-15 across the whole domain, and everything is
  machine-precision for `s >= 0.05`. This is what makes `bench-vs-libxc` report
  `gga_x_wpbeh` at 4.5e-7 and HSE06 at 1.2e-6 while the rayon oracle passes
  1221 of 1221 fields: the bench grid draws `s` uniformly from [0, 3] and so
  lands on points the oracle grid does not. **Pre-existing** -- the wpbeh
  output fingerprint (`d67311fbdf2bab7d`) is byte-identical before and after
  the 2026-09-03 erfcx/E1 fixes.

  **Diagnosed 2026-09-10: it is GCC's FMA contraction, amplified by a
  cancellation that is in the formula itself.** The `wpbeh_EG` piecewise was
  the wrong place to look. At `omega = 0`, `term1` reduces to
  `A/2 * E1_scaled(aux5) + A/2 * log(aux4/aux6)` with `aux5` proportional to
  `aux4`, and both logarithms diverge as `s -> 0` while their sum stays finite.
  `zk` loses about two digits to that and holds at 2e-15; `vsigma` is the
  derivative, where the leading terms cancel *exactly* and what is left is the
  subleading correction -- around `s = 1e-8`, `aux4 ~ s^4 ~ 1e-32`, so terms of
  order `1/aux4` cancel down to order 1 and there are no digits left. Both
  libraries compute the same ill-conditioned expression; they differ because
  GCC contracts `a*b + c` into an FMA and rustc does not, and at that
  conditioning one ulp is the whole answer. It is out of reach of any physical
  quadrature (`s` of order 0.1 to 5), which is what
  `wpbeh_vsigma_agrees_over_the_physical_range` gates.
- **Screened hybrids were wrong until 2026-09-03, and two math helpers with
  them.** HSE06 is `1.0*wpbeh(w=0) - beta*wpbeh(w=omega_PBE) + PBEc`. Three
  independent gaps meant `omega` never reached the kernel -- the generated
  dispatch took no parameters, `libxc-eval` discarded the aux's `params`, and
  there was no propagation rule for the HSE family -- so both legs evaluated the
  same unscreened function. Fixing the plumbing then exposed that
  `math/src/special.rs::xc_erfcx` was **Abramowitz & Stegun 7.1.26** (a 1.5e-7
  *absolute* erf fit) rather than libxc's Faddeeva table, and that six
  `E11_data` coefficients in `expint_e1.rs` had been transcribed 1000x too
  small. Both helpers are reachable *only* on the screened path, which is why
  `gga_oracle.rs` (wpbeh at its default `omega = 0`) never touched them.
  HSE06 `zk` went from 8.2e-3 relative error to 3.4e-14. Guarded now by
  `verify/tests/hse06_oracle.rs` and `verify/tests/screening_helpers.rs`, the
  latter comparing both helpers against libxc's own C.
- **Runtime `ext_params` reach the kernels via a name-built permutation, never
  positionally.** libxc's `copy_params` writes `ext_params[i]` into slot `i` of
  the C params struct, so struct order *is* ext_params order; the kernel's
  argument order comes from maple2c and differs for **160 of 276** functionals
  (`gga_c_pbe` is `[gamma, BB, beta]` against libxc's `[_beta, _gamma, _B]`).
  `extract_params.py` emits `ext_to_kernel` per functional and refuses to emit
  one at all unless every metadata default lands bit-for-bit on the kernel
  default it feeds -- which is what makes "pass the defaults" a provable no-op.
  18 functionals are refused on that gate (transforming setters such as
  `gga_x_lspbe`'s `mu += alpha*(1+kappa)`); they keep their compiled-in
  constants and reject runtime ext_params rather than running with a wrong one.
- **`hyb_mgga_xc_b0kcis` is its own kernel PLUS its mix.** It is the only
  functional in libxc 7.0.0 whose info block carries both a work pointer and an
  `xc_mix_init` init. `xc_mgga_new` evaluates the kernel and *then* calls
  `xc_mix_func`, with no guard between them, so the functional is
  `mgga_c_kcis + (0.75*gga_x_b88 + 1.0*mgga_c_kcis)` -- twice the KCIS
  correlation, confirmed against libxc to 1.7e-16. Treating it as either-or
  gets it 96% wrong (kernel alone) or 20% wrong (mix alone).
  `routing::*_has_own_kernel` is the predicate; `evaluate_mgga` runs the mix
  first and adds the kernel through the workspace scratch, **not** straight
  into the output -- `prepare` *takes* the caller's buffers, so dispatching
  into the output leaves every field `None` and silently discards whatever was
  accumulated afterwards.
- **Composite functionals ran their auxiliaries on the wrong constants until
  2026-09-03: 52 of 125 composite GGAs disagreed with libxc.** HSE06 was not a
  special case. When `xc_mix_init` builds a composite, each auxiliary starts on
  *its own* defaults and the parent's init or setter then overrides them --
  `hyb_gga_xc_lc_blyp` hands its `_omega` of 0.33 to `gga_x_ityh` (own default
  0.2), `gga_x_sogga` replaces PBE's `_kappa` 0.804 with 0.552. This tree built
  the auxiliaries and never applied the overrides, so the entire long-range
  corrected family (`lc_*`, `lcy_*`, `lrc_*`, `cam_*`, `hjs_*`, `hiss`,
  `whpbe0`) evaluated a different functional than its name.
  `meta::generated_aux_overrides` now carries those 141 assignments, read out
  of libxc's own `xc_func_type` by `verify/tests/gen_aux_overrides.rs` rather
  than scraped from its C. Gate: `verify/tests/composite_oracle.rs`, 0
  unexpected failures; diagnosis: `verify/tests/composite_diagnose.rs`.
  **The table is a snapshot at the parent's default ext_params** -- change a
  parent parameter that feeds an auxiliary and it goes stale unless
  `composite_setters` or `PROPAGATION_RULES` also describes the relationship.
  Only the HSE family and the nine generated copy rules have that today.
- ~~**Composite MGGAs**: `composite_oracle.rs::composite_mgga_survey` is
  reporting-only~~ **Closed 2026-09-10.** `composite_mgga_matches_libxc` is an
  assertion now and passes; `hyb_mgga_xc_b0kcis` is under the gate, and
  `hyb_mgga_xc_br3p86`'s `vsigma` residual was the BR89 inversion's stopping
  rule (see "Root-finders"). Original note follows.
- **Composite MGGAs could not evaluate at all until 2026-09-03** (36 of 39
  failed with "output buffer 'vlapl' size mismatch"). `evaluate_mixed_mgga`
  gated the *auxiliary's* buffers on the parent's `NEEDS_LAPLACIAN`/`NEEDS_TAU`
  flags; that gate belongs on the accumulation, not on the buffers the kernel
  demands. Survey now compares 34, with two over the gate --
  `hyb_mgga_xc_b0kcis` (zk 2.6e-1, a real disagreement) and
  `hyb_mgga_xc_br3p86` (vsigma 2.1e-7, zk within contract). Neither is fixed.
  `composite_oracle.rs::composite_mgga_survey` is reporting-only until they
  are.
- ~~Five composite GGAs remain over the gate~~ **Closed 2026-09-11.** Four
  (`gga_k_gds08`, `ghds10`, `ghds10r`, `tkvln`) mix `lda_k_gds08_worker`, which
  libxc numbers **100001** and keeps out of its public `xc_funcs.h`. There was
  no kernel for it (an earlier version of this note said there was; there was
  not), so they refused at construction. It now has one, emitted by
  `from_maple.py` as the only `INTERNAL_WORKERS` entry and routed under the id
  the parents' generated auxiliary lists already carried: 100001 truncated to
  the `u16`, 34465 (`meta::LDA_K_GDS08_WORKER`). Its metadata is
  `meta::internal_auxiliary`, written by hand and checked against libxc's info
  block by `composite_oracle.rs::gds08_worker_metadata_matches_libxc`; the
  registry does not know it, so it can only be reached as one of those four
  auxiliaries. The C ABI reports it as 100001 (`meta::libxc_number`). The fifth,
  `gga_xc_beefvdw`, passes against a wheel-built oracle.
- `libxc-reval` routes 156 of 266 functionals. The other 110 are listed in `crates/libxc-reval/src/routing.rs::UNSUPPORTED` **with the reason** (custom `ext_params` setters that transform values, defaults written as C expressions, or no libxc registration) and return `None`. Do not wire these by guessing constants — a wrong default is silently wrong physics.
- The `LdaFunctional`/`GgaFunctional`/`MggaFunctional` enums cover only 168 of 305 functionals, so typed dispatch reaches 100 of the 156 wired ones; the rest are name-only.
- Kernel correctness rests on `verify/tests/kernel_oracle*.rs` (C libxc parity for all 454 routed kernels, **both spins, all three families**, first and second derivatives), `crates/kernels-rayon/oracle` (unpolarized LDA/GGA, kept as a second opinion) and `revalcheck` (chunked vs whole-grid). The polarized/MGGA gap this list used to call "the largest remaining" was closed on 2026-09-03, and closing it is what found the `b0kcis` defect. **Third and fourth derivatives of the routed kernels are still uncovered**, as are MGGA second derivatives of single kernels. Composite MGGAs (vxc through lxc, both spins, every field) and the deorbitalized SCAN-L family are covered since 2026-09-11 and are bit-identical to libxc (`composite_oracle.rs::composite_mgga_higher_orders_match_libxc`, `deorbital_oracle.rs`).
- The maple2c rewrite was validated against the tree it replaced before that tree was regenerated: of 2,648 emitted functions, **2,420 were token-for-token identical** (numbers compared by value, not spelling), 218 differed only because the old ones had been reconstructed by `vnmerge` and carried its `vN` names, and 3 differed by a redundant paren. All 8 `bench-vs-libxc` output fingerprints and the full oracle result (7/344 over tolerance, same three functionals) were unchanged across the rewrite.
- ~~`revalcheck` reports **4 differing values in `gga_c_op_pw91 Lxc Polarized`**~~ **Gone as of 2026-09-10**: `revalcheck` is clean over 1,736,268,725 values across 322 LDA+GGA functionals, both spins, all five orders. The likely reason is the input sanitisation -- those four values came from points whose raw inputs are outside the functional's domain, and libxc's clamps now put them back inside before the kernel sees them.
- **9 of 1221 oracle field comparisons exceed 1e-12** (2026-08-31, down from 48). **All nine are `v2rho2` (5) or `vsigma` (4); `zk` has none**, so the project's stated contract -- *energy* relative error <= 1e-12 -- is met. The harness applies 1e-12 uniformly to `zk`/`vrho`/`vsigma`/`v2rho2`, which is stricter than that.
  They are not translation errors. Constants, call counts, parameters, thresholds and every math function were checked against the maple2c source and glibc. What remains is accumulated floating-point divergence from a differently-compiled implementation: **GCC contracts `a*b+c` into FMA by default and rustc does not** (`gga_c_optc.o` carries 40,564 FMA instructions). Rebuilding the oracle's libxc with `-ffp-contract=off` removes `gga_x_beefvdw` and `hyb_gga_xc_wb97x_d` outright and takes `wb97x_d3` from 4.1e-11 to 5.5e-12; it is **not the default**, because it compares against a libxc nobody builds -- but it is now reachable as `LIBXC_RS_FP_CONTRACT=off` (see Verification) precisely so the attribution can be demonstrated rather than argued. Worst remaining: `hyb_gga_xc_wb97x_d3` v2rho2 4.7e-11, `gga_x_beefvdw` v2rho2 1.5e-11, then six between 1.0e-12 and 8.4e-12.

  **The attribution is no longer circumstantial (2026-09-10).**
  `verify/tests/root_finders.rs` compares the three iterative helpers against
  libxc's own exported C on a quarter-million arguments each. Against a stock
  GCC oracle, 41% of `xc_mgga_x_br89_get_x` calls differ (worst 6.1e-5
  relative, which is 1.8e-12 absolute -- inside the solver's own 5e-12 bracket
  tolerance); rebuilt with `-ffp-contract=off`, **0 of 250,011 differ**, and
  the same for `xc_mgga_x_mbrxc_get_x` (0 of 250,008) and `LambertW` (0 of
  250,014). The mechanism is directly observable without libxc at all: GCC
  compiles `2.0*M_E*z + 2.0` at `z = -1/e` to 3.88e-17 by contracting it into
  an FMA, and to exactly 0.0 with `-ffp-contract=off`, which is what Rust
  computes. In `LambertW` that one difference flips a branch -- `sqrt(0) - 1`
  is exactly -1, so the Halley step's `w != -1.0` guard fires and it returns
  immediately, while libxc starts from -0.99999999377, exhausts its fifteen
  iterations and returns its "should never happen" 0.0.

  So: matching a stock libxc bit for bit would mean reproducing GCC's
  contraction *decisions*, which are made on gimple after its own CSE and
  reassociation and are not recoverable from the maple2c source. Emitting
  `mul_add` by guesswork would be silently wrong wherever the guess missed.
  What can be said, and now is: **where the two compilers evaluate the same
  expressions, this library is bit-identical to libxc.**
- Four real defects were fixed to get there, all found by `crates/kernels-rayon/oracle/tests/diagnose.rs` (dumps ours-vs-libxc pointwise; `XCDIAG=<name> ... --test diagnose -- --nocapture`):
  1. **Composed functionals were wired to an unrelated kernel** (15 failures). `extract_params.py` paired every `xc_func_info_` block in a libxc `.c` file with that file's one `maple2c` include. Files also define `xc_mix_init` composites that have no formula -- so `hyb_gga_xc_apbe0` was evaluating `gga_c_zvpbeloc`, 238x off. libxc marks the difference with a work pointer (`NULL, &work_gga, NULL`) vs an init fn and none; that is now required, and the 9 affected functionals are reported as UNSUPPORTED rather than guessed.
  2. **`gga_x_fd_lb94` integrated what libxc doesn't** (8). Its `FT_inter` returns `-3/4 * ...`, and `-3/4` is **integer division** in C, so the integrand is identically zero and both `xc_integrate` calls vanish. We had 886 lines of correct Gauss-Legendre computing the intended value.
  3. **`zeta_threshold` was 1e-10; libxc uses `DBL_EPSILON`** (10). This is not a screening knob -- the maple bodies evaluate `zeta_threshold^(4/3)` and add it into terms of order 1. Fixing it cleared all four `gga_c_optc` failures and six others. `Thresholds::default()` now mirrors `functionals.c`. Still divergent and worth attention: libxc's `dens_threshold` and `sigma_threshold` are **per-functional** (`info->dens_threshold`, and `sigma = dens^(4/3)`), while this tree carries one global default -- harmless on the oracle grid, wrong for low-density points.
  4. **The harness scored cancellation dust as signal** (2). `gga_k_tfvw`/`gga_k_absp4` have an identically-zero `vrho`; libxc's own answer is exact `0.0` at one grid point and 1e-20..1e-14 elsewhere. `worst_rel` now skips an element only when *both* sides are below `scale * 1e-12` (scale = that functional's max `|zk|`); anything carrying magnitude still faces the full relative tolerance.
- **`xc_integrate` is QUADPACK now** (`math/src/quadpack.rs`, a transcription of `dqagse` from `libxc-master/src/integrate.c`). The hand-written Gauss-Legendre it replaced was accurate to ~1e-12 of the *true* integral and still missed libxc by 7.8e-8, because libxc runs `dqagse` to only 1e-10 -- matching it needs the *same* approximation, not a better one. That cleared the four `lda_x_1d_{soft,exponential}` failures. The old code existed because QUADPACK "uses malloc and function pointers, which are not available in `#[cube]` kernels"; CubeCL is gone, so that no longer binds. Note `lda_x_1d_exponential` integrates from **1e-20**, not 0.
- **HSE06 `fxc` disagrees with libxc on `v2sigma2`** on the bench grid
  (2.1e0 unpolarized, 2.9e2 polarized relative, 2026-09-07), identically on
  the mix and the fused path, so it is not the fusion. `zk`/`vrho` agree.
  The bench grid draws the reduced gradient uniformly from [0, 3] and so
  lands where `gga_x_wpbeh`'s `vsigma` already diverges as `s -> 0` (the
  `wpbeh_domain.rs` gap above); the second sigma derivative amplifies it.
  Not diagnosed beyond that; the composite `fxc` cases are new to the bench.
- `bench-vs-libxc`'s elementwise cross-check flags `mgga_c_r2scan` (`vtau`,
  9.035e-8) and `mgga_x_scan` polarized (`vsigma`, 3.476e-9) against C libxc.
  **Still open, and the Fermi-hole clamp is not the explanation** -- that was
  the obvious suspect and it was checked on 2026-09-10. `grid::mgga` constructs
  `tau` *above* `tau_W = sigma/(8 rho)` on purpose ("staying above it keeps the
  point inside the domain, so the kernel runs its real branch"), so
  `sigma <= 8 rho tau` holds at every point and the clamp is the identity on
  this grid; both numbers are unchanged to three figures by implementing it.
  What the grid does do is put `tau` *close* to the bound, where r2SCAN's
  `alpha` is a difference of nearly equal quantities -- the same conditioning
  story as `gga_x_wpbeh` at small `s`, and consistent with the FMA attribution
  above, but not measured. `kernel_oracle.rs` passes both functionals on its
  own grid.

  The clamp itself is not in question: libxc applies it, this tree did not, and
  `verify/tests/input_sanitisation.rs` shows the difference it makes on points
  that actually violate the bound.
- The rayon oracle harness (`crates/kernels-rayon/oracle`) compares against C libxc for **unpolarized** LDA/GGA only. The polarized split-kernel paths (fixed 2026-08-16: loop bound was `first_buf.len()` even when that buffer has D>1 elements per point, sweeping D× too far — 2,495 files regenerated with `len() / D`) are exercised bitwise by `revalcheck` but have no oracle-parity test yet.


## Optimise in Rust
/home/user/Documents/workspace/cubecl_manual/manual/optimiser
