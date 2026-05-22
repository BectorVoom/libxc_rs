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

## Regen + source-level verification (Task 2 STEP 4-6) — DONE

- `python3 tools/generate_gga_dispatch.py` → wrote `src/eval/gga_dispatch/mod.rs` + 105 funcs.
- `python3 tools/generate_mgga_dispatch.py` → wrote `src/eval/mgga_dispatch/mod.rs` + 25 funcs.
- 132 emitted files updated; committed in `f9c4ff05a8` (137 files total incl. generators + hand-written + this log).

Source-level verification (PLAN `<verification>`, all cargo-free checks PASS):

| Check | Result |
|-------|--------|
| `from_raw_parts::<f64>` in both generators | 0 |
| `ScalarArg` in both generators | 0 |
| 2-arg `from_raw_parts(` in generators | 17 (gga) + 9 (mgga), all `.clone()` |
| `from_raw_parts::<f64>` / `use …ScalarArg` in dispatch.rs | 0 / 0 |
| `from_raw_parts::<f64>` in launch.rs | 0 |
| `from_raw_parts::<f64>` in emitted gga+mgga mod.rs | 0 / 0 |
| funcs/*.rs `ScalarArg` imports (gga+mgga) | 0 |
| `tau_von_weizsacker` in generator + emitted mgga mod.rs (11-09 preserved) | 1 / 1 |
| full-tree stale-form sweep (generators+handwritten+emitted) | **0 stale forms** |

## Task 3 umbrella ENTRY gate — RUN #1 (user-driven, jobs=1): 3031 → 4

`/usr/bin/time -v cargo check -p libxc_rs --lib --jobs 1` (user-run, `/tmp/11-14-umbrella-check.log`):

- **Exit status: 101** — `error[` count **4** (down from 3031; the migration killed all 3027 genuine
  launch-ABI errors: E0107 1046 / E0061 1046 / E0599 804 / E0432 131 all → 0).
- **Peak RSS: 30,002,596 kB (~30 GB)** — hit the box ceiling but survived (no OOM-kill; Swaps: 0,
  heavy minor page faults). jobs=1 was essential.

### The 4 residuals — ALL pre-existing, NONE launch-ABI-shaped

Confirmed present in the original `log/libxc_rs_check.log` (lines 2107, 79015/79036/79053), so they
are NOT regen-introduced and the migration transforms correctly left them untouched. Both classes are
cubecl-0.10-adjacent / signature-consistency issues, fixed **in 11-14 scope** (commit `e3954802cd`):

| Error | Site | Root cause | Fix |
|-------|------|-----------|-----|
| E0308 ×1 | `src/kernel/launch.rs:76` `read_output_buffer` (real code) | cubecl-0.10 readback drift: `client.read_one()` now returns `Result<Bytes, ServerError>` (was `Bytes`) | `.expect(...)` — mirrors the passing 11-09 canary read-back |
| E0061 ×3 | `mix.rs:502/718`, `evaluate.rs:67` calling `dispatch_gga` | GGA generator never gained the `params: &dyn FunctionalParams` arg that `dispatch_mgga`/`dispatch_lda` already carry, yet all callers pass `&*params` | added the arg + `let _ = params;` **in the GENERATOR** (`generate_gga_dispatch.py`) + regen + updated 2 emitted test calls to `&NoParams`. Fixes callers WITHOUT touching out-of-scope `mix.rs`/`evaluate.rs` |

Deviation note: the plan assumed all 3031 were launch-ABI. 4 were not (1 readback + 3 dispatch_gga
signature). Both fixed within 11-14's own files (launch.rs + the GGA generator/regen). No out-of-scope
files touched.

## Task 3 — RUN #2 (after e3954802cd): 4 type errors → 0, but 4 dead_code errors

`cargo check -p libxc_rs --lib --jobs 1` (`/tmp/11-14-fix-check.log`): **0 `error[`** (the 4 residual
type errors fixed), peak RSS **~543 MB** (warm dep cache; only `libxc_rs` re-checked) — BUT **exit 101**
on **4 `dead_code` errors** (crate is `#![deny(warnings)]`, lib.rs:1):

| dead_code item | Origin | Fix |
|----------------|--------|-----|
| `map_launch_err` (dispatch.rs), `map_gga_launch_err` (gga gen), `map_mgga_launch_err` (mgga gen) | **orphaned by this migration** — dropping `.map_err(..)?` removed their only callers (0.10 launch returns `()`) | removed the 3 fns + now-unused `LaunchError` imports — in the GENERATORS + regen, and hand-written dispatch.rs (commit `1ad364b612`) |
| `as_initialized_mut` (src/compat/raw_handle.rs:49) | **pre-existing**, newly EXPOSED (crate now type-checks far enough to reach dead-code analysis) — intended C-ABI compat API, unwired | `#[allow(dead_code)]` (non-destructive; preserves API). **DEVIATION**: out-of-scope file, minimal allow; flagged for compat-layer review |

(The plan's "leave them, `allow(unused)` is present" guidance was incorrect — the crate denies dead_code.)

## Task 3 — RUN #3 (after 1ad364b612): ✅ EXIT 0 — G-6 CLOSED

`/usr/bin/time -v cargo check -p libxc_rs --lib --jobs 1` (`/tmp/11-14-fix2-check.log`):

- **Exit status: 0** — **`error[` count 0**, no `error:` lines (dead_code cleared).
- **3031 → 0** total errors. The umbrella `libxc_rs` lib compiles clean under cubecl 0.10.
- **Peak RSS: 548,588 kB (~536 MB)** at jobs=1 (run #1 warmed the 281 kernel `.rmeta`; runs #2/#3
  re-check only `libxc_rs` itself — far under the 30 GB ceiling).
- 24 warnings remain, **all in dependency kernel crates** (none in `libxc_rs`'s own `src/`); not denied
  here, non-blocking.

**G-6 CLOSED.** The per-`-p` umbrella ENTRY gate is green (memory `project_phase11_structural_without_compile`
satisfied). **G-2 / plan 11-12 is now unblocked.** Durability holds: the 0.10 ABI lives in the GGA+MGGA
generators (a future dispatch regen preserves it); the 11-09 von Weizsäcker tau-clamp survived the regen.

### Error-count trajectory (jobs=1 throughout)
| Run | Commit base | error[ | dead_code | exit | peak RSS |
|-----|-------------|-------:|----------:|-----:|----------|
| #1 | f9c4ff05a8 | 4 | 0* | 101 | ~30 GB (cold; built all 281 kernel deps) |
| #2 | e3954802cd | 0 | 4 | 101 | ~543 MB |
| #3 | 1ad364b612 | 0 | 0 | **0** | ~536 MB |

\* run #1 halted on type errors before dead-code analysis ran.
