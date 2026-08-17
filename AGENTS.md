# Project Agents Guide

## Project

`libxc_rs` is a Rust re-architecture of the public `libxc 7.0.0` API surface. The library keeps upstream capability reachability, but replaces the original C-style surface with a three-layer Rust design: compatibility shims, a typed safe core, and ergonomic high-level APIs.

**Core value:** deliver full libxc public capability coverage through a safer Rust API, with no C/Fortran in the production path.

**CPU only.** The CubeCL substrate that previously served CPU and GPU from one kernel source was retired in `docs/adr/0001-rayon-over-cubecl.md`. Kernels are now plain Rust over `&[f64]` slices, parallelised with rayon. There is no GPU path.


## Layout

| path | role |
|------|------|
| `crates/libxc-core` | data layer: model, metadata, registry, input/output types, dimensions |
| `crates/kernels-rayon` | generated plain-Rust kernels (305 functional crates + `math`) |
| `crates/libxc-reval` | rayon eval layer: stride-aware parallel sweep, per-family dispatch, routing |
| `crates/libxc-compat` | C-ABI shim |
| `tools/translate_rayon` | the kernel emitter (see below) |
| `archive/kernels-cubecl` | **archived** CubeCL kernel tree — still buildable, see below |
| `crates/libxc-eval` | **archived-adjacent** CubeCL eval path; still holds the orchestration types the facade uses |
| `tools/archive/cubecl` | **archived** CubeCL emitter |

`docs/adr/0001-rayon-over-cubecl.md` records the decision, the measurements behind it, and — importantly — what the decision is *not* based on.


## Key Constraints

- Numerical execution is plain Rust + rayon. Do not reintroduce CubeCL.
- Kernels are generated. **Never hand-edit anything under `crates/kernels-rayon/`** — regenerate with `tools/translate_rayon/`.
- f64 only. Energy relative error must stay within 1e-12 of the libxc oracle.
- Maple2c formula translations must preserve floating-point operation order.
- The redesign cannot silently drop public functions, IDs, metadata paths, or removed-ID diagnostics.
- Public APIs must use typed Rust boundaries and `thiserror` v2 errors.
- libxc is an oracle for verification only; it is not part of the production runtime.
- Repeated workloads must reuse workspaces and caches rather than reallocating on hot paths.
- Kernel crates must stay cheap to compile. The retired CubeCL tree needed >12 min and 1.5 GB RSS for a single MGGA functional; `mgga_c_tpssloc` could not be built at all on 30 GB.


## Regenerating kernels

```bash
python3 tools/translate_rayon/translate.py --all          # kernel bodies
python3 tools/translate_rayon/translate_math.py --write   # math modules
python3 tools/translate_rayon/extract_params.py --json /tmp/params.json
python3 tools/translate_rayon/gen_eval.py                 # eval layer + routing
```

The translator works by **mechanically transforming the archived CubeCL tree**, not by re-deriving from the Maple sources. That is deliberate: the transform preserves floating-point operation order exactly, so it cannot introduce a translation bug the CubeCL tree does not already have, and the result is checkable by *bit* comparison rather than by tolerance. This is why `archive/kernels-cubecl/` must stay buildable.

`translate.py --all` runs two passes over each functional. Both are structure-only or value-preserving, so the emitted kernels stay bit-checkable against the CubeCL tree:

1. **Flatten** (`flatten.py`) — the CubeCL chunk/meta helper fan-out (231,749 `chunkN.rs` files) existed only to bound cubecl-macros memory. Each helper call becomes its body as a scope-preserving block expression. Verified bit-for-bit on `gga_c_ft97` (chunked, 1.9M values) and `mgga_c_revtpss_p2` (meta-nested, 177k values).
2. **Value-merge** (`vnmerge.py`) — maple2c re-derived every shared intermediate in each part, so an output's parts recompute the same values 2-16x over. The parts are merged into one loop and each distinct value computed once, by *value numbering* (names cannot be trusted across parts: maple2c restarts its CSE numbering, so `t10` in part1 and part2 may be different expressions). Verified on `gga_c_gapc` (1.9M values), `mgga_c_kcis` (3.9M values) and seven more.

The chunk-first struct-interface outputs (`gga_c_pbe`, `mgga_x_pbe_gx` lxc_pol) are already CSE-optimal and are copied verbatim by both passes.

**Know what each pass buys before touching it.** Flatten improves everything. Value-merge removes 2-16x of the *arithmetic* — a large runtime and RSS win — but it does **not** speed up compilation, and past a point it slows it down: one merged function is one codegen unit, and rustc's frontend is single-threaded per crate, so collapsing 40 part functions into 1 serialises that crate's codegen. Measured per crate, merged vs flattened: `gga_c_gapc` 52s wall/71s CPU vs 81s/384s (merge wins outright, because its parts were already in one crate), but `mgga_x_br89` 68s/77s vs 15s/68s (wall 4.6x worse). **Compilation wall-clock on this tree is governed by how many independent compilation units exist, not by how much arithmetic they contain.**

That is also why `--inline-shards` is **off by default**. `mgga_c_{tpss,tpssloc,revtpss,rmggac,kcisk}` are split across 39 `_pN` companion crates (a cubecl-macros memory workaround). Pulling those parts into the parent lets the merge dedup across them — `mgga_c_tpssloc` collapses 314,187 definitions to 54,767 — but it trades 39 crates that compiled in parallel for 5 serial ones, and a `--timings` run put the three worst at 959s / 873s / 806s of a 13,176 unit-second build, about 20% of the total in three crates. Full-build wall-clock went 11m27 to 19m41-23m02. Keep the shards separate unless you are optimising evaluation throughput and do not care about build time.

`vnmerge.py` also runs standalone (`vnmerge.py <crate>/src`) for debugging, and takes `--cap=N` to split a merged output into several functions so they can land in different codegen units. The default (uncapped) is deliberate: capping buys back less than it costs, because the duplication is a *global* shared prefix rather than part-local — `mgga_c_kcis` at cap 16k gave ~13% build time back for more than half the deduplication, and `mgga_c_rmggac` at cap 8k reached 135s wall against 244s uncapped and 26s sharded.


## Build and editor hygiene

The two generated kernel trees (305 rayon + 306 archived) are workspace **`exclude`d**, not merely left out of `default-members`. `default-members` has no effect on `cargo check --workspace`, which is what rust-analyzer runs; with them as members an editor session checked ~900 crates. Excluded, they are ordinary path dependencies.

- `cargo check -p libxc-rkernel-<f>` works for anything reachable from a member.
- Otherwise: `cargo check --manifest-path crates/kernels-rayon/<fam>/<f>/Cargo.toml`.
- Excluded harnesses need `--manifest-path`: `verify/`, `verify-canary/`, `crates/kernels-rayon/{verify,oracle}/`.

**Building anything CubeCL-side needs care.** Pass `--jobs 1` explicitly — `.cargo/config.toml` now defaults to `jobs = 12`, sized for the rayon tree, and does NOT protect CubeCL builds — and narrow the family with `--no-default-features --features oracle-<fam>`. `oracle-mgga` does not fit in 30 GB at any job count. Rough single-job times: `oracle-lda` ~21 min, `oracle-gga` ~60-90 min.

**Rayon-tree builds are parallelism-bound, not memory-bound.** A rayon kernel rustc peaks at ~0.2-2.3 GB, and `jobs` also caps rustc's codegen-unit parallelism through the jobserver. The workspace profiles use `codegen-units = 16`; with `jobs = 12` the monster crates (e.g. `mgga_c_kcis`, 16 MB source since the fan-out flattening; 58 MB before) drop from ~5 min to ~1.5 min each, and codegen-unit count is runtime-neutral for these kernels (measured: identical ns/pt and checksums at CGU 2 vs 16). Note that a standalone `--manifest-path` build of an excluded kernel crate does not see the workspace `[profile.*]` at all — it uses cargo's defaults, which are already CGU 16 for release.


## Verification

| harness | what it proves | invocation |
|---------|----------------|------------|
| `crates/kernels-rayon/verify` (`rkverify`) | rayon kernels are **bit-identical** to the CubeCL ones | `cargo run --release --manifest-path crates/kernels-rayon/verify/Cargo.toml` |
| `crates/libxc-reval` (`revalcheck`) | chunked parallel evaluation is bit-identical to a whole-grid call | `cargo run --release -p libxc-reval --bin revalcheck` |
| `crates/kernels-rayon/oracle` | rayon backend matches **C libxc 7.0.0** within 1e-12 | `cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml` |
| `verify/` | the archived CubeCL path vs libxc | `cargo test --manifest-path verify/Cargo.toml --features oracle-<fam> --jobs 1` |

The first two only show the *migration* was faithful. The oracle harness is the one that shows the numbers are right — prefer it when judging correctness.

**Emitter changes need an old-vs-new bitwise gate.** `rkverify` only covers `gga_x_pbe`, so a change to `flatten.py`/`vnmerge.py` is checked by building the *previous* emitted crate alongside the new one and comparing every output buffer bit-for-bit over a random grid. Snapshot the tree before regenerating (`cp -r crates/kernels-rayon/{lda,gga,mgga,math} <snap>/`), then per crate: rename the snapshot package to `-old`, point its `math` dep at the repo, generate a harness that calls each `pub fn` entry point in both crates, and diff by `to_bits()`. The pilot scripts are in the session scratchpad (`pilot/gen_shard_cmp.py`, `pilot/verify.sh`); they parse each parameter's per-point stride off the kernel's own index expressions, so they work for any functional without hand-written signatures.

That harness counts **NaN-vs-NaN differences separately** from real ones, and the distinction matters. It feeds each input array independently at random, which for MGGA produces points outside the functional's domain (`tau` below the von Weizsäcker bound `sigma/8rho`), and those evaluate to NaN. Deduplicating a computation can flip the *sign bit* of such a NaN — `mgga_x_scan` shows 1,679 of them — because a value the split form derived twice down two expression paths is now derived once. No finite value changes: the gate is 0 real mismatches, and NaN payload is IEEE-unspecified anyway. `revalcheck` and the oracle harness use physical inputs and do not hit this.


## Known gaps

- `libxc-reval` routes 156 of 266 functionals. The other 110 are listed in `crates/libxc-reval/src/routing.rs::UNSUPPORTED` **with the reason** (custom `ext_params` setters that transform values, defaults written as C expressions, or no libxc registration) and return `None`. Do not wire these by guessing constants — a wrong default is silently wrong physics.
- The `LdaFunctional`/`GgaFunctional`/`MggaFunctional` enums cover only 168 of 305 functionals, so typed dispatch reaches 100 of the 156 wired ones; the rest are name-only.
- The facade and C-ABI still take `Functional` and `EvaluationWorkspace` from `crates/libxc-eval`. Both are cubecl-free, so no CubeCL enters the default build, but the layering is untidy.
- Only `gga_x_pbe` vxc unpolarized has been through the vs-CubeCL bit-exactness gate; the other 304 compile but are unverified against the CubeCL tree. They *are* covered old-vs-new across the flatten and value-merge passes (see Verification), which chains back to the CubeCL tree only as far as the pre-flatten emitter.
- **7 of 344 oracle field comparisons exceed the 1e-12 contract** (first measured 2026-08-17, when the harness was made to run at all). Worst by far is `gga_x_fd_lb94`: v2rho2 1.1e-1, zk 4.6e-2, vrho 4.4e-2 — percent-level, so a real defect, not FP noise. Then `gga_x_beefvdw` (v2rho2 1.5e-11, vsigma 1.0e-12) and `gga_c_hcth_a` (vsigma 1.4e-12), which are marginal. **These are pre-existing, not from the flatten/value-merge passes**: all three crates are bit-identical to their pre-merge form (816,585 values each, 0 mismatches). Suspect the `ext_params` defaults or the CubeCL-era translation of those functionals, and start by diffing the emitted constants against libxc's own.
- The oracle harness had never run before that date: it did not compile (`drop(out)` could not end a borrow that `dispatch_*_by_name` ties to the input's lifetime) and looked functionals up by bare name when the registry is keyed by the `XC_`-prefixed macro name. Both are fixed. It is still untracked (`??` in git) — commit it.
- The rayon oracle harness (`crates/kernels-rayon/oracle`) compares against C libxc for **unpolarized** LDA/GGA only. The polarized split-kernel paths (fixed 2026-08-16: loop bound was `first_buf.len()` even when that buffer has D>1 elements per point, sweeping D× too far — 2,495 files regenerated with `len() / D`) are exercised bitwise by `revalcheck` but have no oracle-parity test yet.


## Reference

`docs/Adaptive Precision Architecture for High-Accuracy Quantum Chemistry on Commodity GPUs.md` — background on the GPU precision question. Historical: the GPU path no longer exists.
