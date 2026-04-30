# Phase 7: GPU Backends and Performance — Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `07-CONTEXT.md` — this log preserves the alternatives considered.

**Date:** 2026-04-24
**Phase:** 07-gpu-backends-and-performance
**Original scope:** "GPU Backends and Performance — ROCM/HIP/WGPU backends, GPU buffer management, benchmarks, performance targets" (from ROADMAP.md).
**Revised scope:** User directive reshaped Phase 7 to include precision refactor because "wgpu does not currently support f64" and precision should be switchable between f64/f32/f16 via environment variables. Scope grows from 3 plans to ~6-8 plans (D-09).

**Areas discussed:** Codegen emission model; Precision selection surface; Oracle + verification strategy; Phase shape + backend × precision matrix; Prerequisite doc amendments.

---

## Pre-discussion: Scope Conflict Surfaced

Before starting the gray-area discussion, Claude flagged a significant conflict between the user's request and locked project documents:

- PROJECT.md constraint `Precision: f64 only`.
- REQUIREMENTS.md Out of Scope entry `f32 evaluation mode`.
- Phase 7 Success Criterion #4 `WGPU backend returns a typed error if the device lacks f64 support`.
- GPU-06 requirement `f64-only precision policy; no silent f32 fallback`.

Claude presented three interpretations of the user's request:
1. Literal: genericize everything; accept f32/f16 violate 10^-12 oracle.
2. WGPU-only path: f64 everywhere + separate f32/f16 WGPU path.
3. Storage-only: kernels stay f64, buffers downcast at boundary.

**User clarified:** Option 2 (oracle-comparison paths stay end-to-end f64; f32/f16 are real-world operation implementations; f64 remains default).

**User correction:** Kernels can be regenerated from `tools/translate_*.py` scripts; the 3,937 f64-referencing files are generated, not hand-written. This unlocked the "single generic `<F: Float>` emission" approach as tractable.

---

## Area 1 — Codegen Emission Model

### Q1.1: How should `tools/translate_*.py` emit kernel code?

| Option | Description | Selected |
|--------|-------------|----------|
| Single generic `<F: Float>` (Recommended) | One generic kernel set; translators emit `F::ln`, `Array<F>`, `F::new(literal)`. Matches CubeCL's `erf.md` canonical example. | ✓ |
| Parallel concrete emissions per precision | Three crate sets (f64/f32/f16) generated separately; 170 → 510 sub-crates, conflicts with Phase 9 build-time goals. | |
| Concrete with cfg-driven `Precision` alias | One-precision-per-build via `type Precision = F64;` + feature flag. Contradicts env-var runtime selection. | |
| Hybrid: generic math core, concrete kernels | kernel-math generic, 270 functional kernels stay concrete with parallel f32/f16 copies. Unjustified complexity given no evidence of #[cube] generic limits. | |

**User's choice:** Single generic `<F: Float>` emission.
**Notes:** Confirmed by CubeCL docs (`erf.md` shows canonical `fn apply_erf<F: Float>(x: Line<F>) -> Line<F>` pattern).

### Q1.2: Genericization scope — which code becomes `<F>` and which stays `f64`?

| Option | Description | Selected |
|--------|-------------|----------|
| Only `#[cube]` kernels (Recommended) | Genericize cube functions only; Rust host-side code (lambert_w, oracle harness, xtask) stays f64. Minimal public API change. | |
| Everything touching a kernel value | Kernels, buffer types, dispatch enums, input/output bundles, `Functional<F>`, `BatchEvaluator<F>` all generic. Most consistent, highest change surface. | ✓ |
| Kernels + buffers only, API stays concrete | Runtime `Precision` enum inside Functional dispatches to generic kernels; public API unchanged. | |

**User's choice:** Everything touching a kernel value.
**Notes:** Implies public API is NOT fully generic — a runtime-dispatching facade is needed to reconcile with env-var selection (Claude flagged this; accepted).

### Q1.3: How are numeric constants handled?

| Option | Description | Selected |
|--------|-------------|----------|
| `F::new(literal)` cast at use site (Recommended) | Translator wraps every f64 literal: `let a = F::new(0.12345);`. | ✓ |
| Per-precision const tables | Generator emits `consts_f64::PP0`, `consts_f32::PP0`, `consts_f16::PP0` modules; kernels pick via generic bound. | |
| Keep constants as f64, cast inside kernel body | One canonical const table; casts localized to kernel bodies (near option 1 but with a shared top-level module). | |

**User's choice:** `F::new(literal)` cast at use site.
**Notes:** f16 subnormal threshold (~6.1e-5) is a latent risk; flagged as a calibration concern during execution. Some polynomial coefficients may silently flush-to-zero at f16.

---

## Area 2 — Precision Selection Surface

### Q2.1: How does the user actually pick a precision?

| Option | Description | Selected |
|--------|-------------|----------|
| Env var reads into enum-dispatch facade (Recommended) | Public `Functional` (non-generic) reads env var at construction, stores one of three monomorphizations. | |
| Env var + Cargo features (hybrid) | Features gate compile-in; env var selects among compiled-in set. | |
| Env var + builder override | Env var default; `FunctionalBuilder::precision()` per-instance override; public `Precision` enum. | ✓ |
| Pure builder, env var only as fallback | Builder is primary; env var is fallback only. | |

**User's choice:** Env var + builder override.
**Notes:** Public `Precision` enum is exposed at crate root. Per-instance override preserves flexibility for mixed-precision workflows.

### Q2.2a: How do C FFI callers pick precision? (First attempt — pre-clarification)

| Option | Description | Selected |
|--------|-------------|----------|
| Env var only for C callers (Recommended) | Existing `xc_func_init` reads env var; drop-in libxc semantics. | |
| New `xc_func_init_precision(id, spin, prec)` | libxc_rs-specific C extension beyond 1:1 libxc mapping. | |
| Per-precision parallel entry points | Triple the extern "C" surface (85 × 3 = 255 symbols). | |

**User rejected this question and clarified:** "C FFI is f64-only; precision is only used for Rust callers. No env var consulted from C FFI, no new C symbols, no precision parameter."

**Resolved decision (locked as D-04b in CONTEXT.md):** C FFI path is f64-only. This simplifies the `compat/` module design and keeps the oracle harness (which uses C FFI) on f64 without special-casing.

### Q2.2b: Are all three precisions always compiled in, or feature-gated? (Asked after C FFI clarification)

| Option | Description | Selected |
|--------|-------------|----------|
| Always all three compiled (Recommended) | No feature gates on precision; binary ~3× larger for kernel code; simpler dispatch. | ✓ |
| Feature-gated precisions | `default = ["precision-f64"]`, opt-in `precision-f32`, `precision-f16`. Adds `#[cfg]` to dispatch sites. | |

**User's choice:** Always all three compiled.
**Notes:** Feature-gating reserved as a contingency escape hatch if compile time regresses intractably (noted under Claude's Discretion in CONTEXT.md).

---

## Area 3 — Oracle + Verification Strategy

### Q3.1: How do we verify f32/f16 correctness, given libxc C oracle is f64-only?

| Option | Description | Selected |
|--------|-------------|----------|
| Two-track: f64 to libxc, f32/f16 to f64 result (Recommended) | Track 1 tight bounds vs libxc; Track 2 low-precision vs Rust f64 at per-precision bands. Isolates precision error from translation error. | ✓ |
| Per-precision oracle bands against libxc | Every precision directly vs libxc with own tolerance. Mixes error sources; requires calibrating ~15 bands per family × order. | |
| f64 gated; f32/f16 smoke-tested | Only f64 full regression; f32/f16 ~5 representative functionals at loose bands. Weakest guarantee. | |

**User's choice:** Two-track verification.
**Notes:** Track 1 = f64 vs libxc (unchanged Phase 4 tolerances). Track 2 = f32/f16 vs Rust-f64 (per-precision bands).

### Q3.2: What per-precision tolerance bands for Track 2?

| Option | Description | Selected |
|--------|-------------|----------|
| Conservative per-order bands (Recommended) | f32: exc 1e-6 → lxc 1e-2. f16: exc 1e-3, kxc/lxc WARN-only. Order-indexed to match IEEE conditioning amplification. | ✓ |
| Flat per-precision bands | Single band per precision (f32 <=1e-6 for all; f16 <=1e-3 for all). Fails on 3rd/4th-order derivatives. | |
| Start WARN-only, harden later | All Track 2 WARN-only in Phase 7; bands locked in a follow-up. | |

**User's choice:** Conservative per-order bands.
**Notes:** Bands are first-order estimates; must be calibrated against measured LDA_X worst-case before Phase 7 locks them as hard CI gates (noted in Specifics).

### Q3.3: Which functionals get Track 2 coverage?

| Option | Description | Selected |
|--------|-------------|----------|
| All 649 at all three precisions (Recommended) | Full regression sweep. Test matrix ~3× current; parallelizable via rayon. | ✓ |
| Representative subset at f32/f16 | ~20 canary functionals at f32/f16; f64 full sweep. | |
| Family-representative | One functional per family × order at f32/f16. | |

**User's choice:** All 649 at all three precisions.
**Notes:** Preserves Phase 4's "all 649 oracle-verified" discipline. GPU × precision coverage limited by D-10's matrix; Track 2 is primarily CPU-run.

---

## Area 4 — Phase Shape + Backend × Precision Matrix

### Q4.1: How should Phase 7 be shaped to fit the precision refactor?

| Option | Description | Selected |
|--------|-------------|----------|
| Enlarge Phase 7: precision refactor + backends + perf (Recommended) | ~6-8 plans in a single expanded phase. Keeps precision and GPU work coupled (WGPU is the reason for f32/f16). | ✓ |
| Split into Phase 7a (precision) + Phase 7b (backends/perf) | Cleaner checkpoints; each phase ~3-4 plans. Risk: WGPU motivation isn't validated until 7b. | |
| Insert Phase 6.5 before current Phase 7 | Decimal-phase insert preserves Phase 7's existing scope. Adds one phase to project count. | |

**User's choice:** Enlarge Phase 7.
**Notes:** Suggested 8-plan decomposition documented in CONTEXT.md D-09. Planner may refine.

### Q4.2: Which backend × precision combinations are committed?

| Option | Description | Selected |
|--------|-------------|----------|
| CPU all 3; CUDA all 3; HIP all 3; WGPU f32+f16 (Recommended) | Most comprehensive; f16 on every backend. | |
| Conservative: CPU all 3; CUDA/HIP f64+f32; WGPU f32 only | f16 CPU-only in v1; WGPU gets minimum viable precision. | ✓ |
| Everything everywhere, emulate when missing | Software-emulate absent hardware precisions. Complex to validate. | |

**User's choice:** Conservative matrix.
**Notes:** Unsupported combo → `Error::BackendPrecisionUnsupported { backend, precision }` at `Functional::new()`. f16 on CUDA/HIP/WGPU deferred to v2.

---

## Prerequisite Doc Amendments

### Q5.1: How are PROJECT.md / REQUIREMENTS.md amendments handled?

| Option | Description | Selected |
|--------|-------------|----------|
| Plan 07-01 = doc amendments first (Recommended) | Docs-only plan runs before any code work; researcher/planner see updated docs. | ✓ |
| Amend before Phase 7 starts (pre-phase) | Standalone transition step; adds a gate. | |
| Defer doc amendment until verification | Fastest start; contradictions visible mid-phase. | |

**User's choice:** Plan 07-01 = doc amendments first.
**Notes:** Specific PROJECT.md + REQUIREMENTS.md edits documented in CONTEXT.md D-11. Six new requirement IDs (PREC-01..06) added and mapped to Phase 7.

### Q5.2: Anything else to capture before writing CONTEXT.md?

| Option | Description | Selected |
|--------|-------------|----------|
| Nothing more — ready for context | Proceed to write CONTEXT.md and DISCUSSION-LOG.md. | ✓ |
| I want to discuss more | Surface an additional area. | |

**User's choice:** Ready for context.
**Notes:** Phase 9 (build time) interaction captured in CONTEXT.md code_context — 3× monomorphization may require Phase 9 revisit if compile time regresses.

---

## Claude's Discretion

Areas where the user left Claude flexibility (documented in CONTEXT.md):

- **Tolerance calibration** — D-07 bands are first-order estimates; planner/executor measures LDA_X worst-case before locking as hard CI gates.
- **`F: Float` bound specifics** — Whether to use `cubecl::prelude::Float` directly or a local trait alias bundling `bytemuck::Pod` and other required derived bounds.
- **`Precision` enum placement** — `src/model/` vs `src/lib.rs` is a naming choice for the planner.
- **Feature-gate escape hatch** — If compile time regresses intractably, planner may re-open D-05 and introduce precision feature flags as a follow-up plan.

## Deferred Ideas (captured from discussion)

None surfaced during discussion that weren't already in REQUIREMENTS.md v2 / Out of Scope. User's response to the "anything else to capture" question was "Nothing more."

Scope-creep ideas that could have appeared (e.g. bf16, mixed-precision evaluation, runtime precision switching without reconstruction) are explicitly documented in CONTEXT.md `<deferred>` for future-phase consideration.
