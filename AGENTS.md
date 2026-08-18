# Project Agents Guide

## Project

`libxc_rs` is a Rust re-architecture of the public `libxc 7.0.0` API surface. The library keeps upstream capability reachability, but replaces the original C-style surface with a three-layer Rust design: compatibility shims, a typed safe core, and ergonomic high-level APIs.

**Core value:** deliver full libxc public capability coverage through a safer Rust API, with no C/Fortran in the production path.

**CPU only.** The CubeCL substrate that previously served CPU and GPU from one kernel source was retired in `docs/adr/0001-rayon-over-cubecl.md`, and its archived kernel tree was deleted on 2026-08-18. Kernels are plain Rust over `&[f64]` slices, parallelised with rayon. There is no GPU path.

CubeCL survives in exactly one place: `crates/kernels-rayon/math/src/vector.rs`,
behind an **optional, default-off `cubecl` feature** (`cargo test -p
libxc-rkernel-math --features cubecl`). It holds `#[cube]` forms of the
primitives plus two launchable kernels — a scalar one and a `Vector<F, N>`
lane-vectorised one — driven through a `ComputeClient`, with tests that check
the output against plain Rust and that lane widths 1/2/4 agree.

It is default-off because this crate is a dependency of all 266 kernel crates
and `cubecl` pulls ~235 transitive crates; `cargo tree -e normal` on the default
feature set shows zero cubecl.

Four things learned there that constrain any future CubeCL kernel path:

- **A `#[cube]` fn cannot call a plain Rust fn** (`E0433: not a crate or
  module`). The scalar helpers in the sibling modules are unreachable from a
  kernel, which is why the primitives exist twice.
- **`Vector<F, N>` does not implement `Float`.** It implements the individual
  op traits (`Powf`, `Sqrt`, `Exp`, `Log`, `Erf`, `Tanh`, `ArcTan`, ...) but not
  the supertrait bundling them, so `fn f<F: Float>(..)` is scalar-only. Shared
  primitives must be bounded on the specific traits, or written twice.
- **`select` takes a scalar `bool`.** The lane-mask form is `select_many` with
  a `Vector<bool, N>` condition, which is what a comparison on a `Vector`
  (`r.less_equal(thr)`) yields. `piecewise3` is the single most common call in
  the maple2c sources (102,833 sites), so this matters.
- **There is no `cbrt`.** `POW_1_3` is the third most common call (13,478
  sites) and would have to be hand-written; `powf(x, 1/3)` is not bit-identical
  to the scalar `cbrt_f64`. Everything else the kernels need — `atan`, `atan2`,
  `tanh`, `erf` — is present, contrary to the CubeCL manual's algebra table.

Manual: `/home/user/Documents/workspace/cubecl_manual/manual/Cubecl`.


## Layout

| path | role |
|------|------|
| `crates/libxc-core` | data layer: model, metadata, registry, input/output types, dimensions |
| `crates/kernels-rayon` | generated plain-Rust kernels (266 functional crates + `math`) |
| `crates/libxc-reval` | rayon eval layer: stride-aware parallel sweep, per-family dispatch, routing |
| `crates/libxc-compat` | C-ABI shim |
| `tools/translate_rayon` | the kernel emitter: maple2c C -> rayon Rust (see below) |
| `bench-vs-libxc` | head-to-head speed/memory benchmark against C libxc (see below) |
| `crates/libxc-eval` | orchestration types the facade and C-ABI take (`Functional`, `EvaluationWorkspace`); no longer holds any kernel path |

`docs/adr/0001-rayon-over-cubecl.md` records the decision, the measurements behind it, and — importantly — what the decision is *not* based on.


## Key Constraints

- Numerical execution is plain Rust + rayon (ADR 0001). CubeCL is present only as
  an **optional, default-off** feature of `crates/kernels-rayon/math`, providing
  `Line<F>` forms of the primitives for a future explicitly-vectorised kernel
  path; nothing in the build depends on it.
- Kernels are generated. **Never hand-edit anything under `crates/kernels-rayon/`** — regenerate with `tools/translate_rayon/`.
- f64 only. Energy relative error must stay within 1e-12 of the libxc oracle.
- Maple2c formula translations must preserve floating-point operation order.
- The redesign cannot silently drop public functions, IDs, metadata paths, or removed-ID diagnostics.
- Public APIs must use typed Rust boundaries and `thiserror` v2 errors.
- libxc is an oracle for verification only; it is not part of the production runtime.
- Repeated workloads must reuse workspaces and caches rather than reallocating on hot paths.
- Kernel crates must stay cheap to compile. The retired CubeCL tree needed >12 min and 1.5 GB RSS for a single MGGA functional; `mgga_c_tpssloc` could not be built at all on 30 GB. The current tree builds all 266 crates in ~24 min wall.


## Regenerating kernels

```bash
python3 tools/translate_rayon/from_maple.py --all                # kernels
python3 tools/translate_rayon/extract_params.py --json tools/translate_rayon/params.json
python3 tools/translate_rayon/gen_eval.py                        # eval layer + routing
```

`from_maple.py` reads `libxc-master/src/maple2c/<fam>_{exc,vxc}/<func>.c` --
libxc's own Maple-generated C -- and emits one Rust function per
(functional, order, spin). 2,648 functions across 266 functionals, ~100 s.

**This replaced a three-stage transform of an archived CubeCL tree**
(`xform.py` -> `flatten.py` -> `vnmerge.py`), which was deleted along with the
tree it read. That tree was never the real source: it too was generated from
these same maple2c files, and every pass in the old pipeline existed to undo
damage the CubeCL emitter had done to fit under `cubecl-macros`' memory ceiling
-- splitting each function into `partN` pieces that re-derived shared
intermediates 2-16x over, fanning those across 231,749 `chunkN.rs` files, and
spilling five MGGA functionals into 39 `_pN` companion crates.

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

## Build and editor hygiene

The generated kernel tree (266 crates) is workspace **`exclude`d**, not merely left out of `default-members`. `default-members` has no effect on `cargo check --workspace`, which is what rust-analyzer runs; as members they added hundreds of units to every editor session. Excluded, they are ordinary path dependencies.

- `cargo check -p libxc-rkernel-<f>` works for anything reachable from a member.
- Otherwise: `cargo check --manifest-path crates/kernels-rayon/<fam>/<f>/Cargo.toml`.
- Excluded harnesses need `--manifest-path`: `verify/`, `verify-canary/`, `crates/kernels-rayon/{verify,oracle}/`.


**Rayon-tree builds are parallelism-bound, not memory-bound.** A rayon kernel rustc peaks at ~0.2-2.3 GB, and `jobs` also caps rustc's codegen-unit parallelism through the jobserver. The workspace profiles use `codegen-units = 16`; with `jobs = 12` the monster crates (e.g. `mgga_c_kcis`, 16 MB source since the fan-out flattening; 58 MB before) drop from ~5 min to ~1.5 min each, and codegen-unit count is runtime-neutral for these kernels (measured: identical ns/pt and checksums at CGU 2 vs 16). Note that a standalone `--manifest-path` build of an excluded kernel crate does not see the workspace `[profile.*]` at all — it uses cargo's defaults, which are already CGU 16 for release.


## Performance against libxc

`bench-vs-libxc` (`cargo run --release -p bench-vs-libxc --bin xcvs`) is the
only harness that times both sides. It runs four legs -- serial libxc,
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
elimination, loop-invariant hoisting, register-pressure scheduling and
`powf` -> cbrt rewrites are all closed off with numbers, and the remaining
headroom (6.2x, measured) is in the libm transcendentals, not the translator.

**Explicit SIMD is opt-in per functional.** `from_maple.py` emits a kernel as
`wide::f64x8` only for the `(functional, order, spin)` triples in its
`SIMD_FUNCS` allowlist. The kernels already loop-vectorise 8-wide, so forcing
explicit SIMD where LLVM did *not* decline is a regression (`gga_x_pbe` 0.55x).
**The SIMD transcendentals are bit-exact (2026-08-18):** `exp`, `ln` and the
cube-root family come from `libxc_rkernel_math::simd`, bit-identical per lane
to the scalar calls the scalar kernels make (exp/ln replicate glibc's
`__ieee754_{exp,log}_fma` schedule from disassembly, with the MIT
optimized-routines tables; cbrt replicates `powers::cbrt_f64`; asserted over
~7M inputs in `math/tests/simd_exact.rs`). So a kernel whose transcendentals
are exp/ln/sqrt/cbrt-family only produces output **bit-identical to its scalar
form** — the allowlist gate for such kernels is measured speed with an
unchanged `bench-vs-libxc` fingerprint (`lda_c_vwn` 5.1x; `mgga_c_scan` 1.5x;
`mgga_c_tpssloc` 1.26x). Only `atan`/`tanh`-class calls still use `wide`'s
~1 ulp forms, so kernels using those (e.g. `lda_c_vwn`) remain
tolerance-checked. `#[inline(always)]` on the `simd::` functions is
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
- **The grid loop now vectorises 8-wide (AVX-512), not 2-wide SLP.** The note in
  the CLAUDE.md risk table about "always SSE, `xmm` only" described the
  pre-`target-cpu` build. Anything that puts a function boundary or a libm call
  inside the loop still destroys it.

## Verification

| harness | what it proves | invocation |
|---------|----------------|------------|
| `crates/libxc-reval` (`revalcheck`) | chunked parallel evaluation is bit-identical to a whole-grid call | `cargo run --release -p libxc-reval --bin revalcheck` |
| `crates/kernels-rayon/oracle` | rayon backend matches **C libxc 7.0.0** within 1e-12 | `cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml` |

`revalcheck` only shows the chunked sweep agrees with a whole-grid call. The oracle harness is the one that shows the numbers are right — prefer it when judging correctness.

**Emitter changes are gated on the libxc oracle, not on an old-vs-new diff.**
Regenerate, then run `crates/kernels-rayon/oracle` (parity with C libxc 7.0.0
within 1e-12) and `revalcheck` (chunked evaluation bit-identical to a whole-grid
call). `bench-vs-libxc` additionally prints an order-sensitive fingerprint over
`to_bits()` of every output, which makes a change that is *supposed* to be
value-preserving -- a scheduling or codegen change -- checkable exactly rather
than by tolerance.

The previous process compared each regenerated crate against the previously
emitted one bit-for-bit, and `rkverify` compared the rayon tree against the
CubeCL tree. Both were removed: comparing against libxc and the oracle is the
check that shows the numbers are *right*, whereas old-vs-new only ever showed a
transform was faithful to something that was itself unverified.

That harness counts **NaN-vs-NaN differences separately** from real ones, and the distinction matters. It feeds each input array independently at random, which for MGGA produces points outside the functional's domain (`tau` below the von Weizsäcker bound `sigma/8rho`), and those evaluate to NaN. Deduplicating a computation can flip the *sign bit* of such a NaN — `mgga_x_scan` shows 1,679 of them — because a value the split form derived twice down two expression paths is now derived once. No finite value changes: the gate is 0 real mismatches, and NaN payload is IEEE-unspecified anyway. `revalcheck` and the oracle harness use physical inputs and do not hit this.


## Density screening is a correctness requirement, not a tuning knob

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
average under `MIN_RUN` (64) points runs the kernel over everything and re-zeros
the screened points instead of splitting. Splitting into ~1.7-point runs costs
about 14 ns per call and made `gga_x_b88` *slower* than doing nothing (1.98 ->
6.84 ns/pt). Real quadratures order points by radial shell, so their empty
points are contiguous and always take the fast route.

## Known gaps

- `libxc-reval` routes 156 of 266 functionals. The other 110 are listed in `crates/libxc-reval/src/routing.rs::UNSUPPORTED` **with the reason** (custom `ext_params` setters that transform values, defaults written as C expressions, or no libxc registration) and return `None`. Do not wire these by guessing constants — a wrong default is silently wrong physics.
- The `LdaFunctional`/`GgaFunctional`/`MggaFunctional` enums cover only 168 of 305 functionals, so typed dispatch reaches 100 of the 156 wired ones; the rest are name-only.
- The facade and C-ABI still take `Functional` and `EvaluationWorkspace` from `crates/libxc-eval`. Both are cubecl-free, so no CubeCL enters the default build, but the layering is untidy.
- Kernel correctness rests on `crates/kernels-rayon/oracle` (C libxc parity) and `revalcheck` (chunked vs whole-grid). The oracle covers **unpolarized LDA/GGA only**, so polarized and MGGA kernels have no direct parity test against libxc -- the largest remaining coverage gap.
- The maple2c rewrite was validated against the tree it replaced before that tree was regenerated: of 2,648 emitted functions, **2,420 were token-for-token identical** (numbers compared by value, not spelling), 218 differed only because the old ones had been reconstructed by `vnmerge` and carried its `vN` names, and 3 differed by a redundant paren. All 8 `bench-vs-libxc` output fingerprints and the full oracle result (7/344 over tolerance, same three functionals) were unchanged across the rewrite.
- `revalcheck` reports **4 differing values in `gga_c_op_pw91 Lxc Polarized`** (chunked vs whole-grid). Pre-existing and reproduced on an untouched tree; the other 482,775,350 values are bit-identical.
- **7 of 344 oracle field comparisons exceed the 1e-12 contract** (first measured 2026-08-17, when the harness was made to run at all). Worst by far is `gga_x_fd_lb94`: v2rho2 1.1e-1, zk 4.6e-2, vrho 4.4e-2 — percent-level, so a real defect, not FP noise. Then `gga_x_beefvdw` (v2rho2 1.5e-11, vsigma 1.0e-12) and `gga_c_hcth_a` (vsigma 1.4e-12), which are marginal. **These are pre-existing, not from the flatten/value-merge passes**: all three crates are bit-identical to their pre-merge form (816,585 values each, 0 mismatches). Suspect the `ext_params` defaults or the CubeCL-era translation of those functionals, and start by diffing the emitted constants against libxc's own.
- The oracle harness had never run before that date: it did not compile (`drop(out)` could not end a borrow that `dispatch_*_by_name` ties to the input's lifetime) and looked functionals up by bare name when the registry is keyed by the `XC_`-prefixed macro name. Both are fixed. It is still untracked (`??` in git) — commit it.
- `bench-vs-libxc`'s elementwise cross-check flags `mgga_c_r2scan` (`vtau`, 9e-8) and `mgga_x_scan` polarized (`vsigma`, 3.5e-9) against C libxc. Unrelated to the threshold screening -- present with and without it, and on a grid with no below-threshold points at all. Not yet diagnosed; the grid feeds `tau` close to the von Weizsaecker bound, where libxc's `work_mgga_inc.c` clamps and this tree does not, so check that before assuming a formula bug.
- The rayon oracle harness (`crates/kernels-rayon/oracle`) compares against C libxc for **unpolarized** LDA/GGA only. The polarized split-kernel paths (fixed 2026-08-16: loop bound was `first_buf.len()` even when that buffer has D>1 elements per point, sweeping D× too far — 2,495 files regenerated with `len() / D`) are exercised bitwise by `revalcheck` but have no oracle-parity test yet.


## Reference

`docs/Adaptive Precision Architecture for High-Accuracy Quantum Chemistry on Commodity GPUs.md` — background on the GPU precision question. Historical: the GPU path no longer exists.

## optimise Cubecl kernel manual
/home/user/Documents/workspace/cubecl_manual/manual/Cubecl

## Optimise in Rust
/home/user/Documents/workspace/cubecl_manual/manual/optimiser
