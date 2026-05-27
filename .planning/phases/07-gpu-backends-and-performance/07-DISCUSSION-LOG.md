# Phase 7: GPU Backends and Performance — Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `07-CONTEXT.md` — this log preserves the alternatives considered.

**Date:** 2026-05-27
**Phase:** 07-gpu-backends-and-performance
**Mode:** Re-scope update — CONTEXT.md existed (2026-04-24, updated 2026-05-07, multi-precision premise) and was rewritten for f64-only after the user chose "Re-scope & update."
**Areas discussed:** GPU verify/hardware strategy, Runtime threading strategy, Backend selection surface, Benchmark suite + perf targets

> **Predecessor superseded (preserved in git history):** the prior discussion log (2026-04-24 / 2026-05-07) reshaped Phase 7 into a multi-precision compute substrate — generic `<F: Float>` kernels, a `Precision` enum, `LIBXC_RS_PRECISION`, two-track f32/f16 verification, and a backend×precision matrix (old D-01..D-14). f32/f16 was re-deferred milestone-scale on 2026-05-23 (the kernels are f64-concrete: 2491 files `&Array<f64>`, 0 generic). The backend-selection decisions (old D-12/13/14) were carried forward into the f64-only re-scope; everything precision-related was dropped. The full original log is in git history prior to this commit.

---

## Pre-discussion: handling the existing CONTEXT.md

| Option | Description | Selected |
|--------|-------------|----------|
| Re-scope & update | Rewrite for f64-only GPU backends; drop precision decisions, keep backend machinery | ✓ |
| View current first | Show existing decisions before deciding | |
| Skip — keep as-is | Leave the f32-premised CONTEXT unchanged | |

**User's choice:** Re-scope & update.

---

## Area 1 — GPU Verify / Hardware Strategy

### Q1: Completion bar for GPU backends
| Option | Description | Selected |
|--------|-------------|----------|
| Wire all 3, verify locally runnable | All three compile-gate + verify whichever runs on the 860M | |
| One backend, fully verified | Pick the single locally-runnable backend; scaffold the other two as compile stubs | ✓ |
| Compile-and-wire only, defer GPU runtime | All 3 compile + plumbing + benches; no GPU runtime verification | |

### Q2: Local verification target backend
| Option | Description | Selected |
|--------|-------------|----------|
| Let research probe & decide | Probe vulkaninfo + ROCm, pick whichever runs f64 | |
| WGPU (Vulkan) | Most likely to launch; exercises the f64-capability check | |
| HIP (ROCm) | Native f64 in HW on gfx1152; ROCm support unproven | ✓ |

### Q3: Scale of GPU-vs-CPU correctness (SC#1, 1e-14)
| Option | Description | Selected |
|--------|-------------|----------|
| Representative subset per family | ~2 LDA + 2 GGA + 2 MGGA oracle witnesses; dodges all-281 OOM | ✓ |
| All tested functionals | Full roster; OOM; chunked multi-day USER-run | |
| Single smoke functional | One functional GPU-vs-CPU smoke | |

### Q4: `>5x`-on-ROCM throughput target (SC#2 / PERF-02)
| Option | Description | Selected |
|--------|-------------|----------|
| Reinterpret as hardware-gated, don't block | Measure but don't gate on >5x | |
| Drop from phase scope | Remove SC#2/PERF-02; future task | ✓ |
| Keep as hard gate | Require >5x (blocks the phase) | |

**Notes:** HIP chosen as the single fully-verified backend; CUDA + WGPU become compile-stubs. Researcher flagged: cubecl-hip 0.10 on gfx1152 is unproven (may need `HSA_OVERRIDE_GFX_VERSION`) → drives the D-04 spike-first gate.

---

## Area 2 — Runtime Threading Strategy

### Q1: How to thread the cubecl Runtime (CpuRuntime-concrete → backend-selectable)
| Option | Description | Selected |
|--------|-------------|----------|
| Runtime match at the launch boundary | Generic-over-R dispatch + buffer helpers; Backend→R match in ONE launch wrapper | ✓ |
| Full generic-over-R end-to-end | Thread <R> through Functional/BatchEvaluator; heaviest monomorphization | |
| cfg-gated parallel dispatch | Duplicate launch path per backend | |

### Q2: Where the threading change lands (generators emit CpuRuntime)
| Option | Description | Selected |
|--------|-------------|----------|
| Let research map call-sites, then choose | Map generated call-site shape, pick regen-vs-localized by build/OOM cost | ✓ |
| Change generators + full regen | Update translate_lda + generate_{gga,mgga}_dispatch; regen ~133 files | |
| Localize to non-generated launch layer | Confine to launch.rs/dispatch.rs if call-sites route through a stable wrapper | |

### Q3: When the GPU client is resolved
| Option | Description | Selected |
|--------|-------------|----------|
| Once per Functional at build()/new() | Resolve + store + reuse across evaluate() | ✓ |
| Per evaluate() call | Re-create client each evaluation | |

### Q4: De-risking HIP-on-gfx1152 + cubecl-0.10 generic launch
| Option | Description | Selected |
|--------|-------------|----------|
| Spike feasibility first in research | Single-kernel HIP f64 launch + generic monomorphization check before the refactor | ✓ |
| Assume feasible, plan the refactor now | Handle surprises as execution deviations | |

---

## Area 3 — Backend Selection Surface

### Q1: Cargo feature shape
| Option | Description | Selected |
|--------|-------------|----------|
| CPU always-on + additive cuda/hip/wgpu | Keep cubecl/cpu unconditional; GPU backends opt-in | |
| cpu-as-default-feature (old D-12 shape) | cpu becomes a feature; default includes it; additive GPU features | ✓ |

### Q2: CUDA + WGPU stub depth (vs GPU-04/SC#3 WGPU f64 check)
| Option | Description | Selected |
|--------|-------------|----------|
| WGPU wired enough to probe f64; CUDA light stub | WGPU instantiates + probes SHADER_FLOAT64; CUDA = #[cfg] variant + typed error, no dep | ✓ |
| Both CUDA + WGPU fully wired (deps included) | Both pull cubecl backend deps; CUDA needs toolkit | |
| Both light stubs (variant + typed error, no deps) | Only CPU + HIP real; drops GPU-04/SC#3 | |

### Q3: Placement across layered crates
| Option | Description | Selected |
|--------|-------------|----------|
| Research maps it; core-type/eval-client split | Backend type+parsing+errors in libxc-core; client construction in libxc-eval | ✓ |
| All backend code in libxc-eval | Keep everything next to cubecl | |

---

## Area 4 — Benchmark Suite + Perf Targets

### Q1: Which functionals to benchmark
| Option | Description | Selected |
|--------|-------------|----------|
| Representative few per family | lda_x + a GGA + one MGGA; within build limits | ✓ |
| Configurable set, default representative | Harness takes a list; defaults to representative | |
| All functionals | Full roster; OOM | |

### Q2: libxc C baseline for PERF-01 (within 1.5x)
| Option | Description | Selected |
|--------|-------------|----------|
| Reuse verify/'s bindgen libxc in the bench | Call libxc C via existing FFI in criterion; direct ratio | ✓ |
| Offline recorded baseline fixture | Compare against stored libxc timings | |
| Measure ratio manually, document it | criterion tracks Rust only; ratio documented | |

### Q3: PERF-05 zero-alloc verification
| Option | Description | Selected |
|--------|-------------|----------|
| Allocation-counting test gate | Counting allocator/dhat asserts zero allocs on hot path | ✓ |
| Manual audit + documentation | Review + document; no gate | |
| Defer PERF-05 verification | Design goal only | |

### Q4: GPU-05 resident buffers
| Option | Description | Selected |
|--------|-------------|----------|
| Internal resident buffers, benched not public | GpuBuffer<R> internal; resident.rs/transfer.rs measure | ✓ |
| Public resident-batch API | Expose a resident/batch evaluator | |
| Defer resident-buffer optimization | Naive upload→eval→readback | |

---

## Claude's Discretion

- f64-unsupported error variant name/shape.
- `GpuBuffer<R>` API internals.
- Counting-allocator mechanism for PERF-05.
- Bench batch sizes beyond the PERF anchors.

## Deferred Ideas

- `>5x` GPU throughput (datacenter f64 GPU); CUDA runtime verification (no NVIDIA); WGPU kernel-correctness; full-roster GPU sweep; the entire f32/f16/generic-`<F>` multi-precision program; mixed precision / per-precision C FFI; DSP/AIE-ML accelerator.
