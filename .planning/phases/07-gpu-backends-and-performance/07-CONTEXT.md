# Phase 7: GPU Backends and Performance — Context

**Gathered:** 2026-04-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 7 delivers a **multi-precision compute substrate + GPU backends + performance layer**, reshaping the original "GPU Backends and Performance" scope to accommodate f64/f32/f16 precision selection. The phase must deliver:

1. A generic-precision kernel substrate (`<F: Float>`) produced by updated `tools/translate_*.py` generators, covering all 270 maple2c kernels across LDA/GGA/MGGA.
2. A `Precision` selection mechanism: `LIBXC_RS_PRECISION={f64,f32,f16}` env var as default, `FunctionalBuilder::precision(Precision::F32)` as per-instance override. **C FFI remains f64-only**; precision is a Rust-side capability.
3. Backend integrations: CPU (all three precisions), CUDA (f64+f32), HIP (f64+f32), WGPU (f32 only). Unsupported combos return `Error::BackendPrecisionUnsupported { backend, precision }` at `Functional::new()`.
4. A two-track verification harness in `verify/`: Track 1 is f64-vs-libxc (unchanged Phase-4 tolerances); Track 2 is f32/f16-vs-Rust-f64 with per-order tolerance bands.
5. Performance targets from the existing Phase 7 success criteria (`PERF-01..05`, `VERIFY-08`), interpreted against f64 as the canonical precision.

**What this phase does NOT add:**
- f16 on CUDA/HIP/WGPU (deferred to v2).
- Per-precision C FFI entry points (C API stays 1:1 with libxc at f64).
- New functional families or oracle fixtures beyond what Phase 4 produced.

**Preserved from Phases 1–6:**
- Oracle tolerances (10^-12 exc, 10^-10 vxc, 10^-8 fxc, 10^-6 kxc, 10^-4 lxc) apply **only to f64**.
- CubeCL 0.9.0 as the compute substrate.
- Per-family dispatch pattern (`LdaFunctional`, `GgaFunctional`, `MggaFunctional` enums).
- 170 kernel sub-crates laid out by Phases 4/8 — regeneration preserves crate structure.
- f64 is default precision; no silent degradation — mismatched requests raise typed errors.

</domain>

<decisions>
## Implementation Decisions

### Codegen (Area 1)

- **D-01:** `tools/translate_lda.py`, `translate_gga.py`, `translate_mgga.py` emit **single generic `<F: Float>` kernel set**. All `f64::ln`, `f64::sqrt`, `f64::exp`, `Array<f64>` emissions are replaced with `F::ln`, `F::sqrt`, `F::exp`, `Array<F>`. One kernel source per functional compiles to all three precisions via monomorphization.
- **D-02:** **Full genericization** — every code path touching a kernel-visible value becomes `<F>`: kernel functions, input/output bundles (`LdaInput<F>`, `GgaInput<F>`, `MggaInput<F>`, matching outputs), dispatch enums (`LdaFunctional<F>` etc.), `Functional<F>`, `BatchEvaluator<F>`. Non-cube Rust-side code (oracle harness in `verify/`, `ext_params`, `xtask`) stays concrete as-needed. Public API sees a runtime-dispatching facade (see D-04a).
- **D-03:** Numeric constants (polynomial coefficients, thresholds, π, etc.) are emitted as `F::new(literal_f64)` casts at use site. The maple2c-derived f64 values remain the canonical constant table in the generator; downcasting is localized to the kernel body. **Calibration requirement:** during execution, verify that no LDA/GGA polynomial coefficient flushes to 0 under f16 (`|value| < f16::MIN_POSITIVE ≈ 6.1e-5`); if any do, flag for per-precision constant handling as a deferred concern.

### Precision Selection (Area 2)

- **D-04a:** **Env var default + builder override.**
  - `LIBXC_RS_PRECISION` environment variable (values: `f64` | `f32` | `f16`; default `f64` when unset) sets the process-wide default precision.
  - `FunctionalBuilder::precision(Precision::F32)` provides per-instance override.
  - A public `pub enum Precision { F64, F32, F16 }` is added to the crate root.
- **D-04b:** **C FFI is f64-only.** `extern "C"` entry points (`xc_func_init`, `xc_func_lda_exc`, etc.) never consult the env var and never read a `Precision` value. The C ABI preserves drop-in libxc replacement semantics exactly; f32/f16 are exposed only to Rust callers. This also keeps the existing oracle harness (which goes through C FFI) on the f64 path with zero special-casing.
- **D-05:** **All three precisions always compiled in.** No `precision-f64` / `precision-f32` / `precision-f16` Cargo features. Binary size cost is accepted in exchange for simpler dispatch (no `#[cfg]` in match arms). Feature-gating can be revisited later if build time becomes intractable.

### Verification (Area 3)

- **D-06:** **Two-track verification discipline** in `verify/`:
  - **Track 1 (f64 vs libxc C oracle):** Existing Phase-4 tolerances unchanged. Energy exc <= 1e-12, vxc <= 1e-10, fxc <= 1e-8, kxc <= 1e-6, lxc <= 1e-4. This is the correctness gate.
  - **Track 2 (f32/f16 vs Rust-f64 result):** Low-precision results compared to the Rust f64 result (not libxc). Isolates precision error from translation error; avoids multiplying libxc C calls.
- **D-07:** **Conservative per-order tolerance bands** for Track 2, to be calibrated against measured LDA_X worst-case before the phase locks them:
  - **f32:** exc 1e-6, vxc 1e-5, fxc 1e-4, kxc 1e-3, lxc 1e-2.
  - **f16:** exc 1e-3, vxc 1e-2, fxc 1e-1; **kxc and lxc are WARN-only** (report measured relative error, never fail CI — error likely unbounded).
  - Rationale: derivative order amplifies conditioning by ~one decimal order; IEEE error-propagation bounds these ranges for well-conditioned XC evaluation.
- **D-08:** **Full coverage** — all 649 functionals × 3 precisions × applicable derivative orders × both spin modes go through the verify/ harness. Test matrix grows ~3× vs Phase 4; parallelized via rayon. GPU × precision coverage follows D-10 (only real backend combos run on GPU; Track 2 precision sweeps are primarily CPU).

### Phase Shape + Backend Matrix (Area 4)

- **D-09:** **Phase 7 is enlarged** to absorb the precision refactor. Target ~6–8 plans (up from 3 in the current roadmap). Suggested plan decomposition (planner may refine):
  1. Plan 07-01 — **Doc amendments** (see D-11).
  2. Plan 07-02 — Update `tools/translate_*.py` to emit generic; regenerate kernel-math and kernel-lda/gga/mgga sub-crates.
  3. Plan 07-03 — Genericize dispatch enums, input/output bundles, `Functional<F>`, `BatchEvaluator<F>`; add `Precision` enum + env var + builder override.
  4. Plan 07-04 — Two-track verify/ harness with per-precision tolerance bands; calibrate against LDA_X.
  5. Plan 07-05 — CUDA backend (f64+f32) via cubecl-cuda; device selection, buffer management.
  6. Plan 07-06 — HIP backend (f64+f32) via cubecl-hip.
  7. Plan 07-07 — WGPU backend (f32 only) via cubecl-wgpu; runtime error for f64/f16 request.
  8. Plan 07-08 — Benchmark suite + performance targets (`PERF-01..05`, `VERIFY-08`).
- **D-10:** **Conservative backend × precision matrix:**

  | Backend | f64 | f32 | f16 |
  |---------|-----|-----|-----|
  | CPU (cubecl-cpu) | yes | yes | yes |
  | CUDA (cubecl-cuda) | yes | yes | deferred to v2 |
  | HIP (cubecl-hip) | yes | yes | deferred to v2 |
  | WGPU (cubecl-wgpu) | **no** (WebGPU spec) | yes | deferred to v2 |

  Unsupported combos return `Error::BackendPrecisionUnsupported { backend, precision }` at `Functional::new()`. WGPU + f64 request returns the same error (supersedes the old "typed f64-unsupported error" wording in the original GPU-06 requirement — see D-11).

### Prerequisite Doc Amendments

- **D-11:** **Plan 07-01 amends PROJECT.md and REQUIREMENTS.md** before any code work. Specific amendments:
  - **PROJECT.md Constraints:** `Precision: f64 only; energy relative error <= 10^-12 vs libxc oracle` → `Precision: f64 default (oracle-comparison path remains end-to-end f64, relative error <= 10^-12); f32 and f16 supported as Rust-side capabilities for real-world operation (per Track 2 tolerance bands).`
  - **PROJECT.md Constraints:** `GPU precision: No silent f32 fallback; typed error if device lacks f64 support` → `GPU precision: No silent precision degradation; typed Error::BackendPrecisionUnsupported when a backend/precision combo is unavailable (e.g. WGPU + f64).`
  - **REQUIREMENTS.md Out of Scope:** Remove `f32 evaluation mode -- precision requirements mandate f64 throughout; mixed precision would be misleading`. Add explanatory note in Context that f32/f16 are Rust-only and never compared directly to libxc.
  - **REQUIREMENTS.md GPU-06:** `f64-only precision policy: typed error if device lacks f64 support, no silent f32 fallback` → `Precision policy: f64 is the default and only precision used for oracle comparison. f32/f16 are Rust-side capabilities selectable via LIBXC_RS_PRECISION env var or FunctionalBuilder::precision(). Unsupported backend/precision combos return Error::BackendPrecisionUnsupported.`
  - **REQUIREMENTS.md Phase 7 Success Criterion #4:** `WGPU backend returns a typed error at runtime if the device lacks f64 support (no silent f32 fallback)` → `WGPU backend only supports f32 (WebGPU spec lacks f64). Requesting WGPU with f64 or f16 returns Error::BackendPrecisionUnsupported at Functional::new().`
  - **Add new requirements** to REQUIREMENTS.md under a new `### Precision` section:
    - **PREC-01:** `Precision` enum (F64, F32, F16) in public crate root.
    - **PREC-02:** `LIBXC_RS_PRECISION` env var recognized at `Functional::new()`; invalid value returns typed error.
    - **PREC-03:** `FunctionalBuilder::precision(Precision)` per-instance override.
    - **PREC-04:** Two-track verification: Track 1 (f64 vs libxc) at Phase-4 tolerances; Track 2 (f32/f16 vs Rust-f64) at per-order bands per D-07.
    - **PREC-05:** Backend × precision matrix per D-10; unsupported combos return typed error.
    - **PREC-06:** C FFI is f64-only; precision is Rust-side capability.
  - Update **Traceability** table to map PREC-01..06 to Phase 7.
  - Plan 07-01 also updates CLAUDE.md's technology stack table footer and the "Constraints" block to match.

### Claude's Discretion

- **Tolerance calibration:** D-07 bands are first-order estimates. During execution, planner may tighten or loosen after measuring LDA_X worst-case on each precision; document measured values and lock bands in a per-precision oracle fixtures file.
- **`F: Float` bound specifics:** Whether to use `cubecl::prelude::Float` directly or a local trait alias (e.g. `pub trait Precision: cubecl::prelude::Float + bytemuck::Pod { ... }`) to bundle required derived bounds. Either is acceptable; researcher should establish the idiom in Plan 07-02.
- **`Precision` enum placement:** Whether `Precision` enum lives in `src/model/` or `src/lib.rs` directly is a naming choice; enumerate when planning Plan 07-03.
- **Feature-gate escape hatch:** If compile time after monomorphization is intractable, planner may re-open D-05 and introduce `precision-f32` / `precision-f16` Cargo features as a follow-up plan within Phase 7. This is a contingency, not a default path.

### Folded Todos

None — no pending todos matched Phase 7 scope at discussion time.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project-level (authoritative)
- `.planning/PROJECT.md` — Core value, constraints, key decisions. **Will be amended in Plan 07-01** (see D-11); Plan 07-01 must run first.
- `.planning/REQUIREMENTS.md` — GPU-01..07, VERIFY-08, PERF-01..05; new PREC-01..06 added in Plan 07-01.
- `.planning/ROADMAP.md` §Phase 7 — original goal text (to be updated alongside PROJECT.md).
- `.planning/STATE.md` — current progress snapshot.
- `CLAUDE.md` — Recommended Stack table, Key Technical Risks (see especially the "WGPU backend lacks f64 on many GPUs" and "CubeCL 0.9.0 kernel compilation limits" rows, which this phase directly addresses).

### Prior-phase decisions (locked, carry forward)
- `.planning/phases/04-bulk-kernel-translation/` — Phase 4 established the 170 sub-crate layout, dispatch enum pattern, and oracle tolerance regime. Track 1 verification inherits directly.
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-CONTEXT.md` — `Functional` struct shape, `FunctionalParams` trait, auxiliary functional recursion. All must become `<F>`-parameterized.

### CubeCL generic-precision references
- `docs/manual/Cubecl/erf.md` — canonical example of `fn apply_erf<F: Float>(x: Line<F>) -> Line<F>` generic `#[cube]` functions; use as the idiom template for D-02.
- `docs/manual/Cubecl/cubecl_error_solution_guide/mismatched types.md` — CubeCL's IR lowering and how `F::exp` vs `f64::exp` dispatch inside `#[cube]`. Required reading for Plan 07-02's generator update.
- `docs/manual/Cubecl/cubecl_reduce_sum.md` — `ElemType::Float(FloatKind::{F64,F32,F16})` dispatch pattern for buffer storage type.
- `docs/manual/Cubecl/cubecl_3d_dft.md` — end-to-end CubeCL DFT example; confirms f64 usage in `#[cube]` kernels (still relevant as the f64 baseline).

### Generator source (mandatory update targets for Plan 07-02)
- `tools/translate_lda.py` (~825 lines) — LDA kernel emission. Regex stage at lines ~107–136 is where `f64::ln`/`f64::sqrt` literals are currently produced; must be updated to emit `F::ln`/`F::sqrt` with literal wrapping `F::new(...)`.
- `tools/translate_gga.py` (~1291 lines) — GGA kernel emission, same structural change.
- `tools/translate_mgga.py` (~1196 lines) — MGGA kernel emission, same structural change.
- `tools/batch_translate_lda.py`, `tools/batch_translate_gga.py`, `tools/batch_translate_mgga.py` — batch orchestration; likely minimal change but confirm.
- `tools/split_oversized_kernel.py`, `tools/split_oversized_mgga.py` — post-processing that splits large kernels; must preserve generic signatures.

### Verification harness (update target for Plan 07-04)
- `verify/` crate — existing bindgen+libxc oracle plumbing stays on Track 1 (f64 only). Track 2 is a new module: `verify/tests/low_precision_consistency.rs` or similar.
- Existing oracle fixtures for H, Li, BrOH, BrOH+ systems (per PROJECT.md Context) are reused for all three precisions; no new test-system data needed.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`tools/translate_*.py` generators (~3,312 Python lines total):** Single choke point for kernel emission. Updating these three files + regenerating gives us all 270 kernels at once — the key insight that made full genericization tractable. The regex stage at `translate_lda.py:107-136` is the focal point for f64→F substitution.
- **`crates/kernel-math/src/` (15 files):** Small, self-contained mathematical core. Genericizing here is the lowest-risk first step — can be done by hand before updating the bulk translators.
- **`crates/kernel-math/src/erf.rs`:** Cephes/libm-style piecewise rational approximation already structured with named coefficient constants. Template for D-03's `F::new(literal)` pattern.
- **`crates/kernel-math/src/lambert_w.rs`:** Currently uses `f64::exp`, `f64::EPSILON`, `f64::powf` inside non-cube code. **Stays f64** — it's a host-side numerical routine, not a `#[cube]` kernel. Serves as a boundary example.
- **`verify/` crate bindgen-based oracle:** Already separates Rust-vs-C comparisons by functional/order/spin; extends naturally to per-precision columns. Rayon-parallelized from Phase 4.
- **170 existing kernel sub-crates:** Layout preserved. Each already has a unique Cargo.toml; only source files regenerate.

### Established Patterns

- **Per-family dispatch enums (`LdaFunctional`, `GgaFunctional`, `MggaFunctional` with `from_id`, `deferred::is_deferred`):** Established in Phase 4. Must become `LdaFunctional<F>` etc. Constructors stay id-based; `F` threads through via type parameter.
- **Buffer convention: `Array<f64>` with SoA interleaved layout matching libxc:** Becomes `Array<F>` with the same layout. No buffer reshuffling; only element type changes.
- **Oracle comparison via approx crate (`relative_eq!`, `ulps_eq!`) with per-family tolerance:** Scales directly — just add per-precision tolerance rows to the existing fixture tables.
- **Workspace profile: `lto = "thin"`, edition 2024, MSRV 1.85+:** Unchanged. Phase 9 owns build-time optimization; Phase 7 should not introduce new profile tweaks.
- **`Drop` hygiene from Phase 5 (FUNC-06):** `Functional<F>::drop` pattern inherits unchanged per-precision; no new cleanup semantics.

### Integration Points

- **Public crate root (`src/lib.rs`):** Add `pub enum Precision { F64, F32, F16 }` and the runtime-dispatching `Functional` facade. This is the boundary between generic internals and non-generic external API — downstream agents must understand this is where monomorphizations get unified.
- **`src/api/builder.rs`:** Existing `FunctionalBuilder` gains `.precision(Precision)` method. Default comes from env var at `.build()` time.
- **`src/error.rs`:** Add `Error::BackendPrecisionUnsupported { backend, precision }`, `Error::InvalidPrecisionEnvVar(String)`, `Error::PrecisionMismatch { expected, got }` variants.
- **`src/kernel/launch.rs`:** Kernel launcher becomes `launch_lda<F: Float, B: Backend>(...)`; backend selection (CPU/CUDA/HIP/WGPU) and precision (F) are orthogonal generic parameters. Dispatch matrix per D-10 enforced here with compile-time + runtime guards.
- **`verify/Cargo.toml`:** No new dependencies; bindgen/cmake/libxc-master vendor stays. New test files: `low_precision_consistency.rs` for Track 2.
- **`compat/` (Phase 6's C FFI module):** **No changes from D-04b.** C symbols stay f64-only. This is a simplifying outcome.

### Non-Obvious Constraints

- **CubeCL's `#[cube]` generic patterns have known pitfalls** (see `docs/manual/Cubecl/cubecl_error_solution_guide/mismatched types.md`): `ExpandElementTyped<T>` is what the macro actually manipulates post-expansion, not raw `F`. Some helper traits (e.g. `Exp`) need explicit bounds beyond `F: Float`. Researcher must enumerate which bounds are needed.
- **Phase 9 (Reduce Kernel Build Time) overlaps directly.** Tripling monomorphizations can 3× compile time in the worst case. Phase 9's feature-gating approach (`--features gga`, `--features all-kernels`) may need to extend to precision dimensions if build time regresses. Plan ahead: measure compile time before and after D-01/D-02 regeneration, compare against Phase 9 baseline.
- **f16 subnormal threshold (~6.1e-5)** is above many polynomial coefficients in GGA/MGGA functionals. Some functionals may silently flush-to-zero at f16 for specific coefficient combinations — this is a correctness concern that Track 2 (D-06) will surface via the WARN-only kxc/lxc bands.
- **CubeCL 0.9.0 f16 support varies by backend:** cubecl-cpu has f16 (software); cubecl-cuda requires compute capability ≥6.0 with native f16; cubecl-hip f16 status less documented; cubecl-wgpu f16 depends on `SHADER_F16` extension. D-10's conservative matrix sidesteps most of this for v1.

</code_context>

<specifics>
## Specific Ideas

- **Generator-first execution order:** Update `tools/translate_*.py` (D-01) and regenerate kernel-math by hand first (small surface) to validate the generic `<F: Float>` pattern before regenerating the 170 bulk kernel sub-crates. This de-risks the translator change cheaply.
- **Calibration harness precedes band locking:** D-07 bands must be validated against measured LDA_X worst-case before they're committed as hard CI gates. Plan 07-04 should include a calibration step: regenerate LDA_X at f32 and f16, measure relative error vs f64 across the Phase 4 test-point grid (H, Li, BrOH, BrOH+), then adjust D-07 values up or down before lock.
- **Env var parsing: strict, not permissive.** `LIBXC_RS_PRECISION=F64` (uppercase), `= f32 ` (whitespace), `=double` (libxc-ism) all return `Error::InvalidPrecisionEnvVar(value)`. Exact match on lowercase `f64` | `f32` | `f16` only. Unset env var defaults to `Precision::F64`.
- **`Precision` enum Display/FromStr:** Implement `FromStr` for parsing env var; `Display` lowercase. Matches the env var grammar one-to-one.

</specifics>

<deferred>
## Deferred Ideas

### Postponed to v2 (out of scope for Phase 7)

- **f16 on CUDA / HIP / WGPU backends** — deferred per D-10; only CPU gets f16 in v1. GPU f16 requires native hardware support checks and additional test infrastructure.
- **bf16 precision** — not requested by user; CubeCL supports it but user's stated envs are f64/f32/f16 only. Track as future PREC-FEAT if requested later.
- **Mixed-precision evaluation within a single call** — e.g. f64 exc + f32 derivatives. Out of scope; would require kernel-internal precision boundaries and is not consistent with the stated "one precision per evaluate()" model.
- **Runtime precision switching without reconstruction** — `Functional::set_precision(Precision::F32)` mid-lifetime is intentionally omitted. Precision is set at `new()` / `build()` and fixed; users that want a different precision construct a new `Functional`.
- **Per-precision Cargo feature gating** — feature gating was rejected in D-05 for the default path; reserved as a contingency escape hatch if compile time becomes intractable (see Claude's Discretion).
- **Per-precision C FFI entry points** — rejected in D-04b; C ABI stays 1:1 with libxc at f64. If Fortran/C callers request f32/f16 later, handle as a v2 extension with a new ADR.

### Reviewed Todos (not folded)

None — no matching todos at discussion time.

</deferred>

---

*Phase: 07-gpu-backends-and-performance*
*Context gathered: 2026-04-24*
