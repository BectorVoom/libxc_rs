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
| `bench-vs-libxc` | head-to-head speed/memory benchmark against C libxc (see below) |
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
```

`from_maple.py` reads `libxc-master/src/maple2c/<fam>_{exc,vxc}/<func>.c` --
libxc's own Maple-generated C -- and emits one Rust function per
(functional, order, spin). 2,648 functions across 266 functionals, ~100 s.



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


**Rayon-tree builds are parallelism-bound, not memory-bound.** A cold release build of the kernel tree is >40 min (debug builds take minutes). A rayon kernel rustc peaks at ~0.2-2.3 GB, and `jobs` also caps rustc's codegen-unit parallelism through the jobserver. The workspace profiles use `codegen-units = 16`; with `jobs = 12` the monster crates (e.g. `mgga_c_kcis`, 16 MB source since the fan-out flattening; 58 MB before) drop from ~5 min to ~1.5 min each, and codegen-unit count is runtime-neutral for these kernels (measured: identical ns/pt and checksums at CGU 2 vs 16). Note that a standalone `--manifest-path` build of an excluded kernel crate does not see the workspace `[profile.*]` at all — it uses cargo's defaults, which are already CGU 16 for release.


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
Candidates are qualified by `tools/translate_rayon/simd_qualify.py`, which
tries them in batches against `bench-vs-libxc`'s `xcqual` binary (Rust legs and
a fingerprint, no C side, any order or spin) and records every verdict —
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
average under `MIN_RUN` (128) points runs the kernel over everything and re-zeros
the screened points instead of splitting. Splitting into ~1.7-point runs costs
about 14 ns per call and made `gga_x_b88` *slower* than doing nothing (1.98 ->
6.84 ns/pt). Real quadratures order points by radial shell, so their empty
points are contiguous and always take the fast route.

## Known gaps

- **A rejection in `docs/perf/simd-ledger.json` is only valid for the tree it was
  measured on.** Re-swept 2026-09-03: of the 120 hottest undecided tier-1
  candidates, **118 accepted** (median 1.92x, range 1.38-2.68x), two deferred
  for contention, none rejected on merit. The allowlist went 68 -> 170 triples
  across 82 functionals and `gga_x_b88` went from 0.87x *behind* libxc to
  1.57x ahead. **294 tier-1 candidates are still undecided** -- resume with
  `simd_qualify.py --tier 1`; the ledger records every verdict. `gga_x_pbe` carried a standing "do not SIMD, LLVM already
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
  the 2026-09-03 erfcx/E1 fixes. Not yet diagnosed; the first thing to check is
  the `wpbeh_EG` piecewise on `s` in `libxc-master/maple/gga_exc/gga_x_wpbeh.mpl`,
  since the divergence sits below its cutoff.
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
- **Composite MGGAs could not evaluate at all until 2026-09-03** (36 of 39
  failed with "output buffer 'vlapl' size mismatch"). `evaluate_mixed_mgga`
  gated the *auxiliary's* buffers on the parent's `NEEDS_LAPLACIAN`/`NEEDS_TAU`
  flags; that gate belongs on the accumulation, not on the buffers the kernel
  demands. Survey now compares 34, with two over the gate --
  `hyb_mgga_xc_b0kcis` (zk 2.6e-1, a real disagreement) and
  `hyb_mgga_xc_br3p86` (vsigma 2.1e-7, zk within contract). Neither is fixed.
  `composite_oracle.rs::composite_mgga_survey` is reporting-only until they
  are.
- Five composite GGAs remain over the gate, listed with reasons in
  `composite_oracle.rs::KNOWN_GAPS`. Four (`gga_k_gds08`, `ghds10`, `ghds10r`,
  `tkvln`) mix an internal libxc worker functional (id ~100001) the public
  registry does not expose, so the mix is missing a whole component; the fifth
  is `gga_xc_beefvdw` at zk 1.6e-10. LDA composites (2) have **not** been swept.
- `libxc-reval` routes 156 of 266 functionals. The other 110 are listed in `crates/libxc-reval/src/routing.rs::UNSUPPORTED` **with the reason** (custom `ext_params` setters that transform values, defaults written as C expressions, or no libxc registration) and return `None`. Do not wire these by guessing constants — a wrong default is silently wrong physics.
- The `LdaFunctional`/`GgaFunctional`/`MggaFunctional` enums cover only 168 of 305 functionals, so typed dispatch reaches 100 of the 156 wired ones; the rest are name-only.
- Kernel correctness rests on `crates/kernels-rayon/oracle` (C libxc parity) and `revalcheck` (chunked vs whole-grid). The oracle covers **unpolarized LDA/GGA only**, so polarized and MGGA kernels have no direct parity test against libxc -- the largest remaining coverage gap.
- The maple2c rewrite was validated against the tree it replaced before that tree was regenerated: of 2,648 emitted functions, **2,420 were token-for-token identical** (numbers compared by value, not spelling), 218 differed only because the old ones had been reconstructed by `vnmerge` and carried its `vN` names, and 3 differed by a redundant paren. All 8 `bench-vs-libxc` output fingerprints and the full oracle result (7/344 over tolerance, same three functionals) were unchanged across the rewrite.
- `revalcheck` reports **4 differing values in `gga_c_op_pw91 Lxc Polarized`** (chunked vs whole-grid). Pre-existing and reproduced on an untouched tree; the other 482,775,350 values are bit-identical.
- **9 of 1221 oracle field comparisons exceed 1e-12** (2026-08-31, down from 48). **All nine are `v2rho2` (5) or `vsigma` (4); `zk` has none**, so the project's stated contract -- *energy* relative error <= 1e-12 -- is met. The harness applies 1e-12 uniformly to `zk`/`vrho`/`vsigma`/`v2rho2`, which is stricter than that.
  They are not translation errors. Constants, call counts, parameters, thresholds and every math function were checked against the maple2c source and glibc. What remains is accumulated floating-point divergence from a differently-compiled implementation: **GCC contracts `a*b+c` into FMA by default and rustc does not** (`gga_c_optc.o` carries 40,564 FMA instructions). Rebuilding the oracle's libxc with `-ffp-contract=off` removes `gga_x_beefvdw` and `hyb_gga_xc_wb97x_d` outright and takes `wb97x_d3` from 4.1e-11 to 5.5e-12; it was **not adopted**, because it compares against a libxc nobody builds and only fixes a third of the tail. Worst remaining: `hyb_gga_xc_wb97x_d3` v2rho2 4.7e-11, `gga_x_beefvdw` v2rho2 1.5e-11, then six between 1.0e-12 and 8.4e-12.
- Four real defects were fixed to get there, all found by `crates/kernels-rayon/oracle/tests/diagnose.rs` (dumps ours-vs-libxc pointwise; `XCDIAG=<name> ... --test diagnose -- --nocapture`):
  1. **Composed functionals were wired to an unrelated kernel** (15 failures). `extract_params.py` paired every `xc_func_info_` block in a libxc `.c` file with that file's one `maple2c` include. Files also define `xc_mix_init` composites that have no formula -- so `hyb_gga_xc_apbe0` was evaluating `gga_c_zvpbeloc`, 238x off. libxc marks the difference with a work pointer (`NULL, &work_gga, NULL`) vs an init fn and none; that is now required, and the 9 affected functionals are reported as UNSUPPORTED rather than guessed.
  2. **`gga_x_fd_lb94` integrated what libxc doesn't** (8). Its `FT_inter` returns `-3/4 * ...`, and `-3/4` is **integer division** in C, so the integrand is identically zero and both `xc_integrate` calls vanish. We had 886 lines of correct Gauss-Legendre computing the intended value.
  3. **`zeta_threshold` was 1e-10; libxc uses `DBL_EPSILON`** (10). This is not a screening knob -- the maple bodies evaluate `zeta_threshold^(4/3)` and add it into terms of order 1. Fixing it cleared all four `gga_c_optc` failures and six others. `Thresholds::default()` now mirrors `functionals.c`. Still divergent and worth attention: libxc's `dens_threshold` and `sigma_threshold` are **per-functional** (`info->dens_threshold`, and `sigma = dens^(4/3)`), while this tree carries one global default -- harmless on the oracle grid, wrong for low-density points.
  4. **The harness scored cancellation dust as signal** (2). `gga_k_tfvw`/`gga_k_absp4` have an identically-zero `vrho`; libxc's own answer is exact `0.0` at one grid point and 1e-20..1e-14 elsewhere. `worst_rel` now skips an element only when *both* sides are below `scale * 1e-12` (scale = that functional's max `|zk|`); anything carrying magnitude still faces the full relative tolerance.
- **`xc_integrate` is QUADPACK now** (`math/src/quadpack.rs`, a transcription of `dqagse` from `libxc-master/src/integrate.c`). The hand-written Gauss-Legendre it replaced was accurate to ~1e-12 of the *true* integral and still missed libxc by 7.8e-8, because libxc runs `dqagse` to only 1e-10 -- matching it needs the *same* approximation, not a better one. That cleared the four `lda_x_1d_{soft,exponential}` failures. The old code existed because QUADPACK "uses malloc and function pointers, which are not available in `#[cube]` kernels"; CubeCL is gone, so that no longer binds. Note `lda_x_1d_exponential` integrates from **1e-20**, not 0.
- `bench-vs-libxc`'s elementwise cross-check flags `mgga_c_r2scan` (`vtau`, 9e-8) and `mgga_x_scan` polarized (`vsigma`, 3.5e-9) against C libxc. Unrelated to the threshold screening -- present with and without it, and on a grid with no below-threshold points at all. Not yet diagnosed; the grid feeds `tau` close to the von Weizsaecker bound, where libxc's `work_mgga_inc.c` clamps and this tree does not, so check that before assuming a formula b
- The rayon oracle harness (`crates/kernels-rayon/oracle`) compares against C libxc for **unpolarized** LDA/GGA only. The polarized split-kernel paths (fixed 2026-08-16: loop bound was `first_buf.len()` even when that buffer has D>1 elements per point, sweeping D× too far — 2,495 files regenerated with `len() / D`) are exercised bitwise by `revalcheck` but have no oracle-parity test yet.


## Optimise in Rust
/home/user/Documents/workspace/cubecl_manual/manual/optimiser
