# 11-14 — cubecl-0.9 → 0.10 launch-ABI migration log (G-6)

Closes gap **G-6** (critical path; gates G-2/11-12): migrate the `libxc_rs` umbrella's
GGA+MGGA+LDA dispatch/launch glue from the cubecl-0.9 launch ABI to the 0.10 launch ABI so
`cargo check -p libxc_rs --lib` compiles clean.

## Confirmed cubecl-0.10 ABI (Task 1 — locked via checkpoint, user "Adopt confirmed 0.10 ABI")

Verified against (a) the **passing** 11-09 canary `verify-canary/tests/g1_tau_clamp_dispatch_parity.rs`
(compiles + passes under cubecl 0.10) and (b) the installed crate source
`cubecl-core-0.10.0`. cubecl-core version confirmed `0.10.0` (Cargo.lock).

| # | Transform | 0.9 STALE | 0.10 TARGET | Evidence |
|---|-----------|-----------|-------------|----------|
| 1 | **Arrays** | `ArrayArg::from_raw_parts::<f64>(handle, len, 1)` | `ArrayArg::from_raw_parts(handle, len)` — 2-arg, no `::<f64>` turbofish, no vectorization arg; **handle is moved by value** | `cubecl-core-0.10.0/src/frontend/container/array/launch.rs:47`: `pub unsafe fn from_raw_parts(handle: Handle, length: usize) -> Self` |
| 2 | **Scalars** | `ScalarArg { elem: x }` | bare value `x` | canary passes `PARAM_CAB`, `DENS_THRESHOLD`, … bare — **zero** `ScalarArg` in canary; `cubecl-core-0.10.0/src/frontend/element/numeric.rs:129` `impl<T: ScalarArgSettings> LaunchArg for T` |
| 3 | **Launch return** | `*::launch_unchecked::<CpuRuntime>(…).map_err(map_launch_err)?;` (also `.unwrap()`/`.expect()`) | `*::launch_unchecked::<CpuRuntime>(…);` — returns `()`; drop the Result-chain tail | canary calls `launch_unchecked` in a bare `unsafe {}` block (no `?`/`.unwrap`) |
| 4 | **Import** | `use cubecl::frontend::ScalarArg;` | (deleted — no 0.10 replacement path; scalars pass bare) | E0432 in log; path removed in 0.10 |

### Handle-ownership note (NOT in the plan's transform list — discovered during migration)

The dispatch `Ctx` structs (`GgaLaunchCtx`, MGGA ctx, `LaunchCtx`) hold handles as **`&'a Handle`**
(references) and reuse the owning `*_handle` bindings later for `read_output_buffer`. cubecl-0.9's
`from_raw_parts` took `&Handle`; cubecl-0.10's takes an **owned `Handle` by value**. So the array
transform is `from_raw_parts($ctx.rho.clone(), $ctx.rho_len)` — the reference is `.clone()`d to
produce an owned `Handle` (cheap, `Arc`-backed; the canary does `zk_h.clone()`). The original
borrowed binding survives for output read-back. This applies to all 3 Ctx-based macros; in
`launch.rs` test code `input_handle` is moved (last use) and `output_handle` is cloned (reused).

## Per-class error inventory BEFORE migration (Task 2)

From `log/libxc_rs_check.log` (`cargo check -p libxc_rs --lib`), confirmed via
`grep -oE 'error\[E[0-9]+\]' | sort | uniq -c`:

| Class | Count | Cause |
|-------|------:|-------|
| E0061 | 1049 | arg-count mismatch (dropped 3rd `from_raw_parts` arg + bare scalars) |
| E0107 | 1046 | `from_raw_parts::<f64>` turbofish (0 generics in 0.10) |
| E0599 |  804 | `.map_err(…)?` / `.unwrap()` on `launch_unchecked` (returns `()` in 0.10) |
| E0432 |  131 | `use cubecl::frontend::ScalarArg;` (removed in 0.10) |
| E0308 |    1 | type mismatch (launch return `()`) |
| **Total** | **3031** | all pre-existing 0.9→0.10 drift in the dispatch glue (NOT kernels, NOT the 11-09 τ-clamp) |

## Edit split (generated-vs-handwritten)

**GENERATORS (durable — regen preserves the 0.10 ABI):**
- `tools/generate_gga_dispatch.py` — `ten_arm_dispatch_gga!` macro: 17 array sites → `from_raw_parts(_.clone(), len)`; 10 launch arms scalar-splat `$( $scalar, )*` + dropped `.map_err(…)?`; `dt`/`zt` bare; funcs-template `ScalarArg` import deleted.
- `tools/generate_mgga_dispatch.py` — `mgga_zero_scalar_unpol_dispatch!` macro: 9 array sites → `.clone()` 2-arg; 2 launch arms de-Result-chained; `dt`/`zt` bare; funcs import deleted. **11-09 τ-clamp wiring (`mod prepare;`, `prepare::tau_von_weizsacker(...)`) preserved untouched.**

**HAND-WRITTEN (edited directly — no generator):**
- `src/eval/dispatch.rs` — `ten_arm_dispatch_lda!` + `eight_arm_vxc_only_dispatch!` macros: line-28 `ScalarArg` import deleted; 11 array sites `.clone()` 2-arg; 18 launch arms de-Result-chained; `$scalar_u`/`$scalar_p`/`$scalar` splats + `dt`/`zt` bare.
- `src/kernel/launch.rs` — test-module identity-kernel launch (lines 118-119): 2 array sites 2-arg (input moved, output cloned); `.unwrap()` dropped.

## Out-of-scope drift observed (NOT edited — for 11-10/11-12)

`crates/kernels/math/src/{piecewise,powers,polynomials,erf}.rs` carry the SAME 0.9 launch-ABI drift
in their `#[cfg(test)] mod tests` host-launch drivers (`from_raw_parts::<f64>`, `.unwrap()` on
`launch_unchecked`, `read_output_buffer` reading a `Result`). These are **test-gated**, so
`cargo check -p libxc_rs --lib` (which compiles dependency *lib* code only) does NOT reach them — out
of G-6 scope per the plan's `crates/kernels/*` ban. They will surface under `cargo test` / the 11-10
compile sweep / the 11-12 oracle and need a follow-up migration there.

## Intermediate / final umbrella check (Task 2 STEP 5 + Task 3)

_(filled after regen + `cargo check -p libxc_rs --lib`)_
