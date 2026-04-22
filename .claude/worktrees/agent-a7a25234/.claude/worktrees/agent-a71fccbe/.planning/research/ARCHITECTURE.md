# Architecture Patterns

**Domain:** Exchange-correlation functional library (DFT computational chemistry)
**Researched:** 2026-04-09
**Confidence:** HIGH (based on direct source analysis of vendored libxc 7.0.0 + detailed design document)

## How libxc Is Actually Structured

Understanding the C architecture is essential because libxc_rs must reproduce its behavior exactly (10^-12 relative error). The Rust architecture must mirror the same data flow while improving type safety and adding GPU support.

### libxc's Four Architectural Layers

```
Layer 4: Public API     xc_gga_exc(), xc_gga(), xc_gga_new()
                        |
Layer 3: Dispatch       gga.c -- xc_gga_new() selects unpol/pol x order
                        mix_func.c -- mixed/hybrid accumulation
                        |
Layer 2: Work Template  work_gga.c + work_gga_inc.c
                        Grid-point loop, density screening, input clamping
                        Calls into func_exc_unpol() / func_vxc_pol() etc.
                        |
Layer 1: Kernel         maple2c/gga_exc/gga_c_pbe.c
                        Pure math: reads rho/sigma, writes to output via +=
                        Auto-generated from Maple CAS; 266 files, ~4M lines total
```

**Key insight**: Layers 2-3 are identical across all functionals within a family. The `work_gga.c` template provides the grid loop and is `#include`-ed by every GGA functional. The only per-functional code is the maple2c kernel (Layer 1) and the `xc_func_info_type` metadata declaration.

### libxc's Component Boundaries

| Component | Files | Responsibility |
|-----------|-------|---------------|
| **Public API** | `xc.h` (367 lines) | 85 extern functions, all structs/enums/constants |
| **Dispatch** | `lda.c`, `gga.c`, `mgga.c` | Per-family entry points; spin/order routing; calls work template or mix_func |
| **Mixing** | `mix_func.c` | Hybrid/mixed functional accumulation: iterates aux functionals, scales by coefficient, sums |
| **Work templates** | `work_{lda,gga,mgga}_inc.c` | Grid-point loop with density threshold screening, input clamping, kernel invocation |
| **Kernels** | `maple2c/{family}_{exc,vxc}/*.c` | Per-functional math: 266 files, each with up to 10 functions (5 orders x 2 spins) |
| **Registry** | `functionals.c`, `funcs_key.c`, `funcs_{lda,gga,mgga}.c` | ID-to-info lookup, name-to-number mapping |
| **Metadata** | Per-functional `.c` files (e.g., `gga_c_pbe.c`) | `xc_func_info_type` static declarations, parameter structs, init functions |
| **Utilities** | `util.c`, `util.h` | Dimension setup, special math functions, constants |
| **Special math** | `bessel.c`, `faddeeva.c`, `expint_e1.c` | Bessel functions, Faddeeva, exponential integrals used by specific functionals |

### libxc's Data Flow (Single Functional Evaluation)

```
Input: rho[np*dim_rho], sigma[np*dim_sigma]
  |
  v
xc_gga_new(func, order, np, rho, sigma, &out)
  |
  +-- Sanity check: order valid? output pointers match flags?
  +-- Initialize: zero all output buffers (memset)
  |
  +-- Is func->info->gga != NULL?  (direct kernel path)
  |     YES: select func->info->gga->{unpol,pol}[order]
  |          -> work_gga_{order}_{spin}()
  |               for ip in 0..np:
  |                 dens = total_density(rho, ip)
  |                 if dens < threshold: skip
  |                 clamp rho, sigma to thresholds
  |                 call func_{order}_{spin}(p, ip, my_rho, my_sigma, &out)
  |                   // maple2c kernel: pure arithmetic, writes via +=
  |
  +-- Is func->mix_coef != NULL?  (mixed functional path, e.g. B3LYP)
        YES: xc_mix_func(func, np, rho, sigma, ..., outputs...)
             for i in 0..n_func_aux:
               evaluate aux_func[i] into scratch
               output += mix_coef[i] * scratch
```

**Critical detail**: The two paths are NOT mutually exclusive. A functional can have BOTH a direct kernel AND auxiliary functionals. The direct kernel runs first, then mix_func accumulates auxiliary contributions on top.

### Spin-Dependent Kernel Dispatch

Each functional provides up to 10 kernel functions organized as:

```
xc_{family}_funcs_variants {
    unpol[5]: [exc_unpol, vxc_unpol, fxc_unpol, kxc_unpol, lxc_unpol],
    pol[5]:   [exc_pol,   vxc_pol,   fxc_pol,   kxc_pol,   lxc_pol]
}
```

The dispatcher selects `variants.{unpol,pol}[order]` and calls it. Null entries mean that derivative order is not implemented.

### Maple2c Kernel Structure

Each kernel file contains functions like `func_exc_unpol(p, ip, rho, sigma, out)` that:

1. Extract parameters from `p->params` (cast from `void*`)
2. Declare many temporary variables (`t1`, `t2`, ..., `t113`)
3. Compute pure arithmetic using `POW_1_3`, `my_piecewise3`, `M_CBRT3`, etc.
4. Write results via `out->zk[ip] += tzk0` (accumulation, not assignment)

The largest files exceed 11,000 lines (e.g., `gga_c_pbe.c` with 4th-order derivatives). Total maple2c code: ~4 million lines across 266 files.

---

## Recommended Rust Architecture

### Component Diagram

```
libxc_rs/
|
+-- model/          Domain types: Family, Kind, Spin, FunctionalId, etc.
|                    DEPENDS ON: nothing
|
+-- error/          LibxcRsError (thiserror v2)
|                    DEPENDS ON: model/
|
+-- meta/           FunctionalMeta: &'static data for each of 649 functionals
|                    DEPENDS ON: model/
|
+-- registry/       Lookup tables: ID->Meta (O(1)), Name->ID (O(log n))
|                    DEPENDS ON: model/, meta/
|
+-- dims/           Dimension calculation: family x spin -> component counts
|                    DEPENDS ON: model/
|
+-- input/          LdaInput, GgaInput, MggaInput (validated buffer wrappers)
|                    DEPENDS ON: model/, dims/, error/
|
+-- output/         LdaOutput, GgaOutput, MggaOutput + OutputMask bitflags
|                    DEPENDS ON: model/, dims/, error/
|
+-- math/           Mathematical core: #[cube] building blocks
|   +-- power.rs        pow_1_3, safe_cbrt, pow_2_3, pow_4_3, pow_5_3
|   +-- threshold.rs    piecewise3, piecewise5, clamp_density
|   +-- constants.rs    M_CBRT3, M_CBRTPI, KF_CONST, RS_CONST, etc.
|   +-- spin.rs         to_total_zeta, spin_scaling, clamp_zeta
|   +-- special.rs      erf_approx, erfc_approx, LambertW, dilogarithm
|   +-- polynomial.rs   poly_eval (Horner), rational_eval
|   +-- dft.rs          wigner_seitz_rs, reduced_gradient_s, dimensionless_alpha
|                    DEPENDS ON: nothing (leaf module; CubeCL #[cube] only)
|
+-- kernel/         CubeCL kernel functions (per-functional math)
|   +-- launch.rs       Kernel launch wrapper: backend selection, buffer mgmt
|   +-- shared/         Kernel-level shared code: thresholds, spin transform
|   +-- lda/            ~43 LDA kernel files
|   +-- gga/            ~130 GGA kernel files
|   +-- mgga/           ~75 MGGA kernel files
|                    DEPENDS ON: math/
|
+-- eval/           Evaluation orchestration
|   +-- dispatch.rs     Route by family/order/spin to correct kernel
|   +-- mix.rs          Mixed/hybrid accumulation (port of mix_func.c)
|   +-- workspace.rs    Reusable scratch buffers for mixed functionals
|                    DEPENDS ON: kernel/, input/, output/, model/, dims/
|
+-- func/           Functional instance: lifecycle, configuration
|   +-- lifecycle.rs    Functional::new(), Drop
|   +-- config.rs       Threshold/parameter setters
|   +-- params.rs       Per-functional parameter computation
|                    DEPENDS ON: registry/, eval/, model/, error/
|
+-- hybrid/         Hybrid-specific queries: CAM, NLC, aux functionals
|                    DEPENDS ON: func/, model/
|
+-- gpu/            GPU buffer management (feature-gated)
|   +-- buffer.rs       GpuBuffer<R> with dirty tracking
|   +-- pool.rs         Buffer pool for reuse
|   +-- backend.rs      Backend selection + f64 capability check
|   +-- evaluator.rs    GpuEvaluator
|                    DEPENDS ON: eval/, kernel/
|
+-- api/            High-level ergonomic API
|   +-- builder.rs      FunctionalBuilder
|   +-- batch.rs        BatchEvaluator with workspace
|                    DEPENDS ON: func/, eval/, gpu/
|
+-- compat/         C compatibility layer: extern "C" for 85 public functions
|                    DEPENDS ON: api/, func/ (unsafe boundary)
|
+-- lib.rs          Public re-exports
```

### Component Communication Rules

| From | To | Communication Pattern |
|------|-----|----------------------|
| `api/` | `func/` | Direct method calls; builder creates Functional |
| `func/` | `registry/` | Static lookup: `REGISTRY_BY_ID[id]` returns `&'static FunctionalMeta` |
| `func/` | `eval/` | Calls `dispatch::evaluate()` with functional, input, output |
| `eval/dispatch` | `kernel/` | Calls specific kernel launch function by family/order/spin |
| `eval/mix` | `eval/dispatch` | Recursively evaluates auxiliary functionals |
| `kernel/*.rs` | `math/` | Inline calls to `#[cube]` math functions (zero-cost at compile time) |
| `gpu/` | `kernel/launch` | Passes GPU client + handles for device-side execution |
| `compat/` | `api/` + `func/` | Thin unsafe wrappers translating C types to Rust |

**No component may depend upward.** `math/` and `model/` are leaf modules with zero dependencies. `kernel/` depends only on `math/`. This ensures the most critical code (numerical kernels) has the fewest dependencies.

### Data Flow: Complete Evaluation Path

```
                    User Code
                       |
                       v
              +------------------+
              |  api/builder.rs  |  FunctionalBuilder::new(PBE).spin(Polarized).build()
              +------------------+
                       |
                       v
              +------------------+
              |  func/           |  Functional { meta, spin, dims, thresholds, ext_params, params }
              +------------------+
                       |
         functional.evaluate(&input, order, &mut output)
                       |
                       v
              +------------------+
              |  eval/dispatch   |  Match on (family, order, spin) -> kernel function pointer
              +------------------+
                       |
               +-------+--------+
               |                |
          Direct kernel    Mixed/hybrid?
               |                |
               v                v
     +------------------+  +------------------+
     | kernel/launch.rs |  |  eval/mix.rs     |  For each aux: dispatch + accumulate
     +------------------+  +------------------+
               |                |
               v                v
     +------------------+  (recursive dispatch to kernel/launch)
     | CubeCL Runtime   |
     | - cubecl-cpu     |
     | - cubecl-cuda    |
     | - cubecl-wgpu    |
     +------------------+
               |
               v
     Grid-point loop (inside kernel):
       for ip in 0..np:
         total_dens = rho[ip] (+ rho[ip+1] if polarized)
         if total_dens < threshold: skip
         clamp inputs to thresholds
         call func_{order}_{spin}(params, ip, rho, sigma, out)
           // Pure arithmetic using math/ building blocks
           // Writes via out.zk[ip] += result  (accumulation)
```

### Data Flow: GPU Path

```
Host Memory                          Device Memory (GPU)
-----------                          -------------------
rho: &[f64]  --client.create()-->    rho_handle
sigma: &[f64] --client.create()-->   sigma_handle
                                     output_handles (client.empty())
                                          |
                                     CubeCL kernel launch
                                     CubeCount: ceil(np/256)
                                     CubeDim: 256
                                          |
                                     Each thread: one grid point
                                     threshold check -> clamp -> kernel math -> accumulate
                                          |
output: &mut [f64] <--client.read()-- output_handles
```

**Transfer minimization**: For repeated evaluations on the same grid (common in SCF iterations), input buffers stay resident on the GPU. Only output buffers are read back. The `GpuBuffer` struct tracks dirtiness to avoid redundant uploads.

---

## Patterns to Follow

### Pattern 1: Static Registry with Compile-Time Completeness

**What:** All 649 functional metadata entries are `const`/`static` Rust data. The registry is a sparse array indexed by raw ID (O(1) lookup) plus a sorted name array (O(log n) binary search).

**When:** Always. This replaces libxc's linked-list approach with zero-allocation lookup.

**Why:** Eliminates runtime registration, startup I/O, and the possibility of missing functionals. The compiler enforces that every referenced ID has a metadata entry.

### Pattern 2: Work Template as Kernel Launch Wrapper

**What:** libxc's `work_gga_inc.c` pattern (grid loop + threshold + clamp + kernel call) maps to a single `kernel/launch.rs` function per family. The per-functional kernel plugs into this wrapper.

**When:** Every evaluation. The launch wrapper handles all boilerplate; the kernel handles only math.

**Why:** 266 kernel files must NOT each implement their own grid loop. Centralizing the loop in the launch wrapper means threshold bugs are fixed once, not 266 times.

```rust
// Pseudocode for the launch pattern
fn launch_gga_kernel<R: Runtime>(
    client: &ComputeClient<R::Server>,
    kernel_fn: fn(params, ip, rho, sigma, &mut out),  // per-functional
    func: &Functional,
    np: usize,
    rho: &[f64],
    sigma: &[f64],
    output: &mut GgaOutput,
) {
    // Zero outputs
    // Upload inputs (or use resident buffers)
    // Launch CubeCL kernel with grid-point parallelism
    // Inside kernel: threshold check, clamp, call kernel_fn, accumulate
}
```

### Pattern 3: Accumulation Semantics for Mixed Functionals

**What:** All kernel writes use `+=` (accumulation), never `=` (assignment). Output buffers are zeroed before evaluation. For mixed functionals like B3LYP, each component functional writes its weighted contribution additively.

**When:** Every evaluation. Even non-mixed functionals use `+=` for consistency.

**Why:** This is how libxc works. Changing to assignment would break mixed functional composition. The zero-then-accumulate pattern ensures deterministic results regardless of evaluation order.

### Pattern 4: Separation of Metadata from Parameters

**What:** `FunctionalMeta` is static (lives in `.rodata`). `Functional` holds mutable runtime state (ext_params, thresholds, computed params). Many functionals share the same kernel but differ only in parameters (e.g., 13 PBE variants differ only in beta/gamma/B values).

**When:** Functional construction. The `gga_c_pbe.c` pattern shows 13 `xc_func_info_type` declarations all pointing to the same `work_gga` kernel but with different default parameter arrays.

**Why:** Avoids duplicating kernel code for parametric families. One kernel file + N parameter sets = N functionals.

### Pattern 5: CubeCL Unified Kernel with Zero-Cost Math Core

**What:** All math building blocks are `#[cube]` functions that CubeCL inlines at compile time. The compiled kernel is flat arithmetic with no function call overhead.

**When:** All kernels use `math::pow_1_3()`, `math::piecewise3()`, etc. instead of inlining the math.

**Why:** Source-level organization with zero runtime cost. CubeCL's expansion mechanism produces identical IR whether the function is called or inlined by hand.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Trait Objects for Kernel Dispatch

**What:** Using `dyn FunctionalKernel` trait objects to dispatch to per-functional kernels.

**Why bad:** Introduces vtable indirection in the innermost loop (per-grid-point). CubeCL cannot compile trait objects to GPU code. The dispatch must happen BEFORE the grid loop, selecting a concrete kernel function, not inside it.

**Instead:** Use a match statement or function pointer array indexed by (functional_id, order, spin) to select the concrete kernel before launching the grid loop.

### Anti-Pattern 2: Per-Functional Grid Loops

**What:** Each of the 266 kernel files implementing its own grid-point loop with threshold checking.

**Why bad:** Any bug in threshold logic must be fixed in 266 places. Any behavioral change (e.g., new clamping rule) requires touching every kernel file.

**Instead:** One grid loop per family in `kernel/launch.rs`. Kernels receive pre-clamped inputs and write to pre-zeroed outputs. The kernel function signature is `fn(params, ip, rho, sigma, out)` -- single grid point, no loop.

### Anti-Pattern 3: f32 Fallback

**What:** Silently using f32 when f64 is unavailable on the GPU.

**Why bad:** Produces results with ~7 digits of precision instead of ~15. DFT calculations may fail to converge or produce physically wrong results. Users have no way to know their results are degraded.

**Instead:** Return `LibxcRsError::DeviceCapabilityMismatch` if the device does not support f64. Let the user explicitly choose CPU fallback.

### Anti-Pattern 4: Runtime Functional Registration

**What:** Building a HashMap of functionals at startup from configuration files or lazy initialization.

**Why bad:** Adds startup latency, requires synchronization for thread safety, introduces failure modes (missing files, parse errors). libxc uses static C arrays; the Rust version should be at least as fast.

**Instead:** Static arrays + const evaluation. The compiler verifies completeness. Lookup is O(1) by ID.

---

## Suggested Build Order

The build order follows dependency chains. Each layer depends only on layers below it.

### Layer 0: Foundation (no dependencies)

```
model/          Domain enums and newtypes
error/          Error types (depends on model/)
```

**Rationale:** Everything else depends on these types. They are trivial to implement and test.

### Layer 1: Static Data (depends on Layer 0)

```
meta/           FunctionalMeta struct definition
registry/       Lookup tables (649 entries)
dims/           Dimension calculation
```

**Rationale:** These are pure data with no runtime behavior. Can be validated by counting entries and checking dimension values against libxc's `util.c`.

### Layer 2: I/O (depends on Layers 0-1)

```
input/          Input bundle types with validation
output/         Output bundle types with OutputMask
```

**Rationale:** Input/output types define the evaluation interface. Must be stable before kernels are written.

### Layer 3: Math Core (leaf dependency)

```
math/           #[cube] building blocks
```

**Rationale:** Math core has NO dependencies on any other module. It is the most critical code for numerical correctness. Must be thoroughly tested before kernels consume it. Can be developed in parallel with Layers 0-2.

### Layer 4: Canary Kernel (depends on Layers 2-3)

```
kernel/shared/      Threshold, spin transform, output mask
kernel/launch.rs    Launch wrapper for one family (LDA)
kernel/lda/lda_x.rs First kernel: simplest LDA exchange
verify/             Oracle FFI to validate against C libxc
```

**Rationale:** This is the make-or-break validation point. If LDA_X produces bit-accurate results through CubeCL on CPU, the approach is validated. If it fails, the entire CubeCL strategy must be reconsidered. Do this BEFORE translating 265 more kernels.

### Layer 5: Bulk Kernel Translation (depends on Layer 4)

```
kernel/lda/*        All LDA kernels (~43 files)
kernel/gga/*        All GGA kernels (~130 files)
kernel/mgga/*       All MGGA kernels (~75 files)
```

**Rationale:** The largest work item by volume (~4M lines of C to translate). Each kernel is independent; multiple can be translated in parallel. Each requires oracle validation. Start with LDA (simplest) then GGA then MGGA (most complex). Math core extraction happens concurrently -- when a pattern appears in multiple kernels, factor it into `math/`.

### Layer 6: Orchestration (depends on Layer 5)

```
eval/dispatch.rs    Route to correct kernel
eval/mix.rs         Mixed functional accumulation
eval/workspace.rs   Scratch buffer management
```

**Rationale:** Cannot be completed until kernels exist. The dispatch table needs all kernel function pointers. Mix logic needs at least a few hybrid functionals to test against.

### Layer 7: Instance Management (depends on Layer 6)

```
func/               Functional struct, lifecycle, ext_params
hybrid/             Hybrid queries (CAM, NLC, aux)
```

**Rationale:** The `Functional` struct is the public-facing runtime object. It composes registry lookup, dimension calculation, parameter management, and evaluation dispatch. All sub-systems must be ready.

### Layer 8: GPU + API + Compat (depends on Layer 7)

```
gpu/                GPU buffer management, backend selection
api/                Builder, BatchEvaluator
compat/             C FFI layer (85 extern "C" functions)
```

**Rationale:** These are thin layers on top of the core library. GPU support is feature-gated and does not block the CPU-only path. C compat is mechanical wrapping.

### Dependency Graph Summary

```
Layer 0: model, error
   |
Layer 1: meta, registry, dims
   |
Layer 2: input, output          Layer 3: math (independent)
   |                                |
   +----------------+---------------+
                    |
Layer 4: kernel/shared + launch + lda_x canary + verify
                    |
Layer 5: kernel/lda, kernel/gga, kernel/mgga (bulk)
                    |
Layer 6: eval (dispatch, mix, workspace)
                    |
Layer 7: func, hybrid
                    |
Layer 8: gpu, api, compat
```

---

## Scalability Considerations

| Concern | At 1K grid points | At 100K grid points | At 10M grid points |
|---------|-------------------|---------------------|---------------------|
| Memory | Trivial (~KB) | Moderate (~MB for 4th-order MGGA polarized: 477 components x 100K x 8 bytes = 382 MB) | GPU required; CPU memory ~38 GB for full 4th-order output |
| CPU throughput | Overhead dominates | Kernel math dominates; cache effects matter | Memory bandwidth limited; consider tiling |
| GPU transfer | Not worth it (transfer > compute) | Break-even; depends on kernel complexity | Strongly GPU-favored; transfer amortized |
| Mixed functional | Negligible scratch allocation | Scratch buffers ~MB; workspace reuse essential | Workspace reuse critical; consider fusing aux evaluations |

**Key threshold:** GPU evaluation becomes beneficial above ~10K-50K grid points for most functionals. Below that, CPU evaluation is faster due to transfer overhead.

---

## Sources

- Direct source analysis: `libxc-master/src/xc.h`, `gga.c`, `work_gga.c`, `work_gga_inc.c`, `functionals.c`, `util.h`, `gga_c_pbe.c`, `maple2c/gga_exc/gga_c_pbe.c` (HIGH confidence)
- Design document: `docs/design/libxc_rs_detailed_design.md` v2.0 (HIGH confidence)
- CubeCL documentation: `docs/manual/Cubecl/` (MEDIUM confidence -- CubeCL 0.9.0 specifics need runtime validation)
