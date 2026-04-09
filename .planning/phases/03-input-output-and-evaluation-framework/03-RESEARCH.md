# Phase 3: Input/Output and Evaluation Framework - Research

**Researched:** 2026-04-09
**Domain:** Rust type-safe I/O bundles, bitflag-driven output masking, match-based dispatch, mixed functional accumulation
**Confidence:** HIGH

## Summary

Phase 3 builds the input/output bundle types with construction-time validation, the OutputMask bitflags for selecting derivative levels, and the dispatch/accumulation framework that routes evaluation calls to kernel functions. The existing codebase from Phases 1-2 provides solid foundations: `Dimensions` for buffer size computation, `LibxcRsError` with buffer mismatch variants, kernel launch infrastructure in `launch.rs`, and the LDA_X canary kernel with its 10 function signatures (5 orders x 2 spin modes).

The core challenge is bridging between the user-facing `LdaInput`/`LdaOutput` bundle types (which use `&[f64]` and `Option<&mut [f64]>`) and the CubeCL kernel functions (which take individual `Array<f64>` parameters). The dispatch layer must decompose bundles into individual buffer handles, launch the appropriate kernel, and handle output zeroing. For mixed functionals, a workspace pattern pre-allocates scratch buffers that accumulate weighted contributions from auxiliary functionals.

All design decisions are locked in CONTEXT.md (D-01 through D-13). The research focuses on implementation patterns, field layouts, and pitfalls specific to this phase rather than exploring alternatives.

**Primary recommendation:** Build three modules (`src/input/`, `src/output/`, `src/eval/`) with construction-time validation, bitflag-driven output masking, and a clean separation between non-mixed (zero-alloc direct dispatch) and mixed (workspace-based accumulation) evaluation paths.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Input bundles use borrowed slices only (`&[f64]`). No owned/Cow modes.
- **D-02:** Buffer size validation happens at construction time. Invalid inputs never reach the kernel.
- **D-03:** Single flat interleaved SoA layout for polarized spin. `[rho_a_0, rho_b_0, rho_a_1, rho_b_1, ...]`.
- **D-04:** Input bundles store `np` explicitly as a field, not derived from buffer length.
- **D-05:** OutputMask drives Option fields. Output bundles have `Option<&mut [f64]>` for each derivative level.
- **D-06:** Output buffers are caller-provided `&mut [f64]`. The library does not allocate output buffers.
- **D-07:** Kernels check Option output fields and skip None derivatives.
- **D-08:** Match-based dispatch in `eval/dispatch.rs`. `match (family, order, spin)` routes to the specific kernel function.
- **D-09:** Dispatch entry point is a method on the Functional struct: `functional.evaluate_lda(&input, order, &mut output)`.
- **D-10:** Build the full dispatch scaffold now with LDA_X as the only populated arm. GGA/MGGA arms return error until Phase 4.
- **D-11:** EvaluationWorkspace pre-allocates scratch buffers sized for the largest auxiliary functional.
- **D-12:** Workspace scratch buffers are sized for MGGA (the superset family).
- **D-13:** Non-mixed functionals bypass the workspace entirely. Zero heap allocation in the non-mixed hot path.

### Claude's Discretion
- Exact struct field layout for LdaInput/GgaInput/MggaInput (which fields beyond rho, sigma, lapl, tau)
- OutputMask bitflag values and whether to reuse FunctionalFlags or create a separate bitflags type
- Internal structure of eval/dispatch.rs (helper functions, intermediate types)
- How the dispatch scaffold handles "not yet implemented" kernels (error variant vs panic in debug)
- EvaluationWorkspace internal data structure (Vec<f64> per field vs single flat buffer with offsets)
- Whether to add convenience factory methods (e.g., `LdaOutput::for_order()`) alongside the caller-provides model

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| IO-01 | LdaInput, GgaInput, MggaInput structs with buffer size validation against Dimensions | Input bundle design pattern with `new()` constructor validating against `Dimensions` fields; error variants already exist in `LibxcRsError` |
| IO-02 | LdaOutput, GgaOutput, MggaOutput with Option<&mut [f64]> for NULL-pointer semantics | Output bundle design with `Option<&mut [f64]>` fields; dispatch layer decomposes to kernel args |
| IO-03 | OutputMask bitflags for selecting derivative levels to compute | Separate `OutputMask` bitflags type (not reusing `FunctionalFlags`) with values matching derivative order enum |
| IO-04 | SoA interleaved buffer layout matching libxc convention | Input bundles document layout convention; kernel indexing via `ip * dims.field + component` |
| IO-05 | MggaOutput supports all 70 derivative fields | All 70 fields enumerated from `Dimensions` struct; organized by derivative order |
| EVAL-01 | Dispatch routes evaluation calls to correct kernel based on family, order, spin | Match-based dispatch in `eval/dispatch.rs`; LDA_X populated, GGA/MGGA return error |
| EVAL-02 | Mixed functional accumulation: weighted sum of auxiliary functional results | `mix.rs` implementing `add_to_mix` pattern from libxc `mix_func.c`; per-field weighted accumulation |
| EVAL-03 | EvaluationWorkspace pre-allocates scratch buffers for mixed functional evaluation | Workspace sized for MGGA superset; scratch buffers reused across auxiliary evaluations |
| EVAL-04 | Non-mixed functionals require zero heap allocation in evaluation hot path | Non-mixed path bypasses workspace; dispatches directly to kernel with user-provided buffers |
| EVAL-05 | All hybrid/mixed functionals produce correct combined results | Tested with synthetic mock auxiliaries using known weights; verified against manual computation |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Tech stack**: Pure Rust + CubeCL 0.9.0; no C/Fortran in production path
- **Precision**: f64 only; energy relative error <= 10^-12 vs libxc oracle
- **Edition**: 2024 with `#![deny(warnings)]`
- **Clippy allows**: `excessive_precision`, `needless_late_init`, `too_many_arguments` (CubeCL macro expansion)
- **Dependencies**: bitflags 2.10, bytemuck 1.25, thiserror 2.0 (production); no ndarray, no nalgebra
- **Operation order**: Maple2c formula translations must preserve floating-point operation order
- **GPU precision**: No silent f32 fallback; typed error if device lacks f64 support

## Architecture Patterns

### Recommended Project Structure

```
src/
├── input/
│   └── mod.rs           # LdaInput, GgaInput, MggaInput with validation
├── output/
│   ├── mod.rs           # LdaOutput, GgaOutput, MggaOutput
│   └── mask.rs          # OutputMask bitflags
├── eval/
│   ├── mod.rs           # Re-exports
│   ├── dispatch.rs      # Match-based dispatch to kernel functions
│   ├── mix.rs           # Mixed functional accumulation (mix_func.c equivalent)
│   └── workspace.rs     # EvaluationWorkspace scratch buffer management
├── kernel/              # (existing)
│   ├── launch.rs        # (existing) Buffer management, CPU client
│   ├── lda/
│   │   ├── mod.rs
│   │   └── lda_x.rs     # (existing) 10 kernel functions
│   └── mod.rs
└── lib.rs               # Add: pub mod input; pub mod output; pub mod eval;
```

[VERIFIED: codebase inspection -- `src/kernel/`, `src/dims/`, `src/error/`, `src/model/` already exist]

### Pattern 1: Construction-Time Validation

**What:** Input and output bundles validate all buffer sizes in their `new()` constructor, returning `Result<Self, LibxcRsError>`. After construction succeeds, evaluation is infallible (ERR-03).

**When to use:** Every input/output bundle creation.

**Example:**
```rust
// Source: design doc Section 6.6 + CONTEXT.md D-01, D-02, D-04
pub struct LdaInput<'a> {
    rho: &'a [f64],
    np: usize,
    spin: Spin,
}

impl<'a> LdaInput<'a> {
    pub fn new(rho: &'a [f64], np: usize, spin: Spin) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::lda(spin);
        let expected = np * dims.rho as usize;
        if rho.len() != expected {
            return Err(LibxcRsError::InputBufferSizeMismatch {
                field: "rho",
                expected,
                actual: rho.len(),
            });
        }
        Ok(Self { rho, np, spin })
    }
}
```
[VERIFIED: `LibxcRsError::InputBufferSizeMismatch` exists in `src/error/mod.rs`]
[VERIFIED: `Dimensions::lda(spin)` returns correct rho dimension per spin mode]

### Pattern 2: Output Bundle with Option Fields

**What:** Output bundles use `Option<&'a mut [f64]>` for each derivative field. `None` means "don't compute this derivative". Validation checks that `Some(buf)` buffers have the correct size.

**Example:**
```rust
// Source: design doc Section 6.7 + CONTEXT.md D-05, D-06
pub struct LdaOutput<'a> {
    pub zk:     Option<&'a mut [f64]>,
    pub vrho:   Option<&'a mut [f64]>,
    pub v2rho2: Option<&'a mut [f64]>,
    pub v3rho3: Option<&'a mut [f64]>,
    pub v4rho4: Option<&'a mut [f64]>,
}

impl<'a> LdaOutput<'a> {
    pub fn new(
        zk: Option<&'a mut [f64]>,
        vrho: Option<&'a mut [f64]>,
        v2rho2: Option<&'a mut [f64]>,
        v3rho3: Option<&'a mut [f64]>,
        v4rho4: Option<&'a mut [f64]>,
        np: usize,
        spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::lda(spin);
        // Validate each Some buffer
        if let Some(ref buf) = zk {
            let expected = np * dims.zk as usize;
            if buf.len() != expected {
                return Err(LibxcRsError::OutputBufferSizeMismatch {
                    field: "zk", expected, actual: buf.len(),
                });
            }
        }
        // ... repeat for each field
        Ok(Self { zk, vrho, v2rho2, v3rho3, v4rho4 })
    }
}
```
[VERIFIED: `LibxcRsError::OutputBufferSizeMismatch` exists in `src/error/mod.rs`]

### Pattern 3: Dispatch Layer Bridge

**What:** The dispatch layer bridges between bundle types (user-facing) and CubeCL kernel functions (which take individual `Array<f64>` parameters). It creates CubeCL buffers from the input slices, launches the kernel, and reads results back.

**Example:**
```rust
// Source: existing launch.rs patterns + CONTEXT.md D-08
pub fn dispatch_lda(
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    alpha: f64,
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    let client = cpu_client();
    let np = input.np;
    let rho_handle = create_input_buffer(&client, input.rho);
    
    // Create output handles only for Some fields
    // Zero-initialize per T-02-06 (kernels use += accumulation)
    let zk_handle = create_zero_output_buffer(&client, np * 1); // dims.zk always 1
    
    let (cube_count, cube_dim) = calculate_launch_config(np);
    
    match (order, input.spin) {
        (DerivativeOrder::Exc, Spin::Unpolarized) => {
            unsafe {
                lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, np, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
                    ScalarArg::new(alpha),
                    ScalarArg::new(thresholds.density),
                    ScalarArg::new(thresholds.zeta),
                ).unwrap();
            }
        }
        // ... other (order, spin) combinations
        _ => return Err(LibxcRsError::UnsupportedDerivativeOrder { ... }),
    }
    
    // Read back results into caller's buffers
    if let Some(ref mut zk_buf) = output.zk {
        let result = read_output_buffer(&client, zk_handle, np);
        zk_buf.copy_from_slice(&result);
    }
    
    Ok(())
}
```
[VERIFIED: `launch_unchecked` pattern from `src/kernel/lda/lda_x.rs` line 26]
[VERIFIED: `ArrayArg::from_raw_parts`, `ScalarArg::new` patterns from `src/kernel/launch.rs` test]

### Pattern 4: Mixed Functional Accumulation

**What:** For mixed functionals, evaluate each auxiliary into scratch buffers, then accumulate `output[i] += weight * scratch[i]` for each derivative field. Matches `mix_func.c` `add_to_mix` pattern.

**Example:**
```rust
// Source: libxc-master/src/mix_func.c lines 51-54
fn add_to_mix(dst: &mut [f64], coeff: f64, src: &[f64]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d += coeff * *s;
    }
}
```
[VERIFIED: `mix_func.c` line 54: `for(ip = 0; ip < np; ip++) dst[ip] += coeff*src[ip];`]

### Anti-Patterns to Avoid

- **Trait-based dispatch:** Do NOT use trait objects or dynamic dispatch for kernel routing. Match-based dispatch (D-08) is explicit, zero-cost, and easy to extend in Phase 4.
- **Output buffer allocation by the library:** The library NEVER allocates output buffers (D-06). Caller always provides them.
- **Single monolithic output struct:** Do NOT create one output struct for all families. LdaOutput, GgaOutput, MggaOutput are separate types with different field sets.
- **Shared mutable state between dispatch and workspace:** Non-mixed path must be completely independent of workspace (D-13).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bitflags for OutputMask | Manual bit manipulation | `bitflags!` macro (bitflags 2.10) | Type safety, derive support, standard ecosystem pattern |
| Buffer size computation | Manual dimension arithmetic | `Dimensions::lda/gga/mgga(spin)` | Already implemented and tested in `src/dims/mod.rs` |
| Error types | Custom error structs | `LibxcRsError` variants via thiserror | Already defined with correct buffer mismatch variants |
| Buffer upload/readback | Manual bytemuck casting | `create_input_buffer`, `create_zero_output_buffer`, `read_output_buffer` | Already in `src/kernel/launch.rs` |

## Common Pitfalls

### Pitfall 1: Forgetting to Zero Output Buffers Before Kernel Launch

**What goes wrong:** Kernels use `+=` accumulation. If output buffers contain garbage, results are wrong.
**Why it happens:** CubeCL `client.empty()` returns uninitialized memory.
**How to avoid:** Always use `create_zero_output_buffer()` from `launch.rs`. This is already enforced by T-02-06.
**Warning signs:** Output values that differ wildly between runs.

[VERIFIED: `create_zero_output_buffer` in `src/kernel/launch.rs` uses `vec![0.0f64; n]`]

### Pitfall 2: Incorrect Buffer Size for Polarized Spin

**What goes wrong:** Polarized spin doubles or triples many buffer dimensions. A buffer sized for unpolarized will cause out-of-bounds access.
**Why it happens:** `dims.rho` is 1 for unpolarized, 2 for polarized. `dims.v2rho2` is 1 for unpolarized, 3 for polarized.
**How to avoid:** Always compute sizes from `Dimensions::lda/gga/mgga(spin)`. Never hardcode dimension values.
**Warning signs:** Buffer validation rejecting correctly-intended sizes.

[VERIFIED: `Dimensions::lda(Spin::Polarized)` returns `rho=2, v2rho2=3, v3rho3=4, v4rho4=5`]

### Pitfall 3: Mixed Functional Family Mismatch

**What goes wrong:** A mixed GGA functional may have LDA auxiliary functionals. When accumulating, the scratch buffer must be sized for the mixed functional's family (MGGA superset per D-12), but only the fields relevant to the auxiliary's family should be accumulated.
**Why it happens:** `mix_func.c` uses `is_gga(aux->info->family)` guards to conditionally accumulate sigma derivatives only for GGA+ auxiliaries.
**How to avoid:** In `mix.rs`, check the auxiliary's family before accumulating sigma/lapl/tau derivative fields. Only accumulate fields the auxiliary actually computes.
**Warning signs:** Zero values in sigma derivatives when a GGA auxiliary is evaluated through a mixed functional.

[VERIFIED: `mix_func.c` lines 180-191 shows family-gated accumulation]

### Pitfall 4: OutputMask vs DerivativeOrder Confusion

**What goes wrong:** `DerivativeOrder::Vxc` means "compute up to 1st derivatives" (energy + vxc). This is cumulative. But `OutputMask::VXC` is a single bit. The dispatch must interpret order as cumulative, not individual.
**Why it happens:** libxc always computes all orders up to the requested one. `func_vxc` computes both `zk` and `vrho`.
**How to avoid:** When `order = Vxc`, dispatch to the `_vxc_` kernel which computes both zk and vrho. The OutputMask can still be used to skip writing to None output fields.
**Warning signs:** Missing energy values when requesting VXC order.

[VERIFIED: `lda_x_vxc_unpol` in `src/kernel/lda/lda_x.rs` writes to both `zk[ip]` and `vrho[ip]`]

### Pitfall 5: Lifetime Conflicts in Output Bundle Construction

**What goes wrong:** `LdaOutput` has multiple `Option<&'a mut [f64]>` fields. Rust's borrow checker prevents creating multiple mutable references from the same allocation.
**Why it happens:** If caller tries to slice a single large buffer into sub-slices for different fields.
**How to avoid:** Caller allocates separate buffers for each derivative level. The API naturally guides this since each field is a separate `Option`.
**Warning signs:** Compilation errors about multiple mutable borrows.

### Pitfall 6: Scratch Buffer Reuse Without Zeroing

**What goes wrong:** In mixed functional evaluation, scratch buffers are reused across auxiliary evaluations. If not zeroed between auxiliaries, results from the previous auxiliary contaminate the next.
**Why it happens:** Optimization temptation to skip zeroing.
**How to avoid:** Zero scratch buffers before each auxiliary evaluation. The kernel uses `+=`, so starting from zero is required.
**Warning signs:** Mixed functional results that depend on auxiliary evaluation order.

[VERIFIED: `mix_func.c` line 148-149 shows `xc_mgga_vars_allocate_all` allocates fresh buffers; our scratch reuse must explicitly zero]

## Code Examples

### OutputMask Bitflags

```rust
// Source: design doc Section 6.7 + CONTEXT.md D-05
// Recommendation: Separate type from FunctionalFlags (different semantic domain)
use bitflags::bitflags;

bitflags! {
    /// Which derivative levels to compute in an evaluation call.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OutputMask: u8 {
        const EXC = 1 << 0;  // Energy density (order 0)
        const VXC = 1 << 1;  // 1st derivatives (order 1)
        const FXC = 1 << 2;  // 2nd derivatives (order 2)
        const KXC = 1 << 3;  // 3rd derivatives (order 3)
        const LXC = 1 << 4;  // 4th derivatives (order 4)
    }
}

impl OutputMask {
    /// Create mask from a DerivativeOrder (cumulative: VXC includes EXC)
    pub fn from_order(order: DerivativeOrder) -> Self {
        match order {
            DerivativeOrder::Exc => Self::EXC,
            DerivativeOrder::Vxc => Self::EXC | Self::VXC,
            DerivativeOrder::Fxc => Self::EXC | Self::VXC | Self::FXC,
            DerivativeOrder::Kxc => Self::EXC | Self::VXC | Self::FXC | Self::KXC,
            DerivativeOrder::Lxc => Self::EXC | Self::VXC | Self::FXC | Self::KXC | Self::LXC,
        }
    }
}
```
[ASSUMED: Exact bit values for OutputMask -- design doc shows same values but Claude's discretion allows variation]

### EvaluationWorkspace

```rust
// Source: design doc Section 6.9 + CONTEXT.md D-11, D-12, D-13
/// Pre-allocated scratch buffers for mixed functional evaluation.
/// Sized for MGGA (superset) so any auxiliary family's fields fit.
pub struct EvaluationWorkspace {
    /// Scratch output buffers for one auxiliary evaluation.
    /// Layout: contiguous Vec<f64> with known offsets for each derivative field.
    scratch: Vec<f64>,
    /// Number of grid points this workspace is sized for.
    np: usize,
    /// Spin mode.
    spin: Spin,
    /// MGGA dimensions (superset).
    dims: Dimensions,
}

impl EvaluationWorkspace {
    pub fn new(np: usize, spin: Spin) -> Self {
        let dims = Dimensions::mgga(spin);
        let total = dims.total_output_components() * np;
        Self {
            scratch: vec![0.0; total],
            np,
            spin,
            dims,
        }
    }

    /// Zero all scratch buffers before evaluating an auxiliary.
    pub fn zero_scratch(&mut self) {
        self.scratch.fill(0.0);
    }

    /// Get mutable slices into scratch buffer for each derivative field.
    /// Returns an LdaOutput/GgaOutput/MggaOutput view into the scratch.
    // ... field accessor methods with computed offsets
}
```
[ASSUMED: Single contiguous Vec vs per-field Vec is Claude's discretion; single Vec is recommended for cache locality]

### Dispatch Match Pattern

```rust
// Source: CONTEXT.md D-08, D-10
use crate::kernel::lda::lda_x::*;

pub fn dispatch_lda_kernel(
    client: &ComputeClient<CpuRuntime>,
    order: DerivativeOrder,
    spin: Spin,
    np: usize,
    rho_handle: &cubecl::server::Handle,
    output_handles: &LdaOutputHandles,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), LibxcRsError> {
    let (cube_count, cube_dim) = calculate_launch_config(np);
    
    match (order, spin) {
        (DerivativeOrder::Exc, Spin::Unpolarized) => {
            // Launch lda_x_exc_unpol
        }
        (DerivativeOrder::Vxc, Spin::Unpolarized) => {
            // Launch lda_x_vxc_unpol
        }
        // ... 8 more arms for all order x spin combinations
        _ => {
            return Err(LibxcRsError::UnsupportedDerivativeOrder {
                id: FunctionalId(1), // placeholder
                order,
                max: DerivativeOrder::Lxc,
            });
        }
    }
    Ok(())
}
```
[VERIFIED: LDA_X provides 10 functions: 5 orders (exc/vxc/fxc/kxc/lxc) x 2 spins (unpol/pol)]

### GGA/MGGA Input Bundles

```rust
// Source: design doc Section 6.6
pub struct GgaInput<'a> {
    rho: &'a [f64],    // np * dims.rho
    sigma: &'a [f64],  // np * dims.sigma
    np: usize,
    spin: Spin,
}

pub struct MggaInput<'a> {
    rho: &'a [f64],    // np * dims.rho
    sigma: &'a [f64],  // np * dims.sigma
    lapl: &'a [f64],   // np * dims.lapl
    tau: &'a [f64],    // np * dims.tau
    np: usize,
    spin: Spin,
}
```
[VERIFIED: `Dimensions` struct has `rho`, `sigma`, `lapl`, `tau` fields for input dimensions]

### MggaOutput All 70 Fields

The 70 output fields for MGGA are organized by derivative order:

| Order | Count (unpol) | Count (pol) | Fields |
|-------|--------------|-------------|--------|
| 0 | 1 | 1 | zk |
| 1 | 4 | 4 | vrho, vsigma, vlapl, vtau |
| 2 | 10 | 10 | v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2 |
| 3 | 20 | 20 | v3rho3, v3rho2sigma, ..., v3tau3 (20 fields) |
| 4 | 35 | 35 | v4rho4, v4rho3sigma, ..., v4tau4 (35 fields) |

Total: 1 + 4 + 10 + 20 + 35 = 70 fields. Each field is `Option<&'a mut [f64]>` where the slice length is `np * dims.<field>`.

[VERIFIED: `Dimensions` struct has exactly 70 output fields (counted from `src/dims/mod.rs`)]
[VERIFIED: `total_output_components()` for polarized MGGA returns 767 (the sum of all per-point component counts)]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| C NULL pointers for optional outputs | Rust `Option<&mut [f64]>` | This project | Type-safe, zero-cost abstraction over NULL semantics |
| C preprocessor macros for family dispatch | Rust `match` expressions | This project | Exhaustive checking, no silent fallthrough |
| C `malloc`/`free` for scratch buffers | Pre-allocated `EvaluationWorkspace` | This project | Amortized allocation, cache-friendly |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | OutputMask should be a separate bitflags type (not reusing FunctionalFlags) | Code Examples | Low -- both approaches work; separate type has cleaner semantics |
| A2 | Single contiguous Vec<f64> for workspace scratch is better than per-field Vecs | Code Examples | Low -- per-field Vecs work too; contiguous is better for cache but harder to index |
| A3 | `OutputMask::from_order()` should be cumulative (VXC includes EXC) | Code Examples | Medium -- if libxc ever supports computing VXC without EXC this would be wrong (but it doesn't) |

## Open Questions

1. **How should the dispatch layer handle the functional ID for non-LDA_X functionals?**
   - What we know: D-10 says GGA/MGGA arms return error. The dispatch currently only has LDA_X.
   - What's unclear: Should dispatch take a `FunctionalId` and match on it, or should each functional register a kernel function pointer?
   - Recommendation: For Phase 3, dispatch takes `FunctionalId` and has a single `match` arm for `LDA_X (id=1)`. Phase 4 will expand this. Match-based dispatch per D-08.

2. **Should output zeroing happen in the dispatch layer or at the kernel level?**
   - What we know: libxc zeros output buffers before evaluation. CubeCL kernels use `+=`. `create_zero_output_buffer()` zeros at buffer creation.
   - What's unclear: Whether caller-provided `&mut [f64]` output buffers should be zeroed by the dispatch layer before kernel launch, or whether the caller is responsible.
   - Recommendation: Dispatch layer zeros caller-provided output buffers (matching libxc behavior). This is part of the evaluation contract. Kernels assume zero-initialized outputs.

3. **How to handle the bridge between `Option<&mut [f64]>` output fields and CubeCL `Array<f64>` kernel parameters?**
   - What we know: Current LDA_X kernels take individual `Array<f64>` params for each output level. Higher-order kernels (e.g., `lda_x_fxc_unpol`) take `zk`, `vrho`, AND `v2rho2`.
   - What's unclear: When output.vrho is `None` but order is `Fxc`, the kernel still writes to vrho. We need a dummy buffer.
   - Recommendation: For `None` output fields at orders below the requested order, allocate a dummy zero buffer that the kernel writes to but whose results are discarded. This is cheap (just `np * dim` doubles) and avoids kernel branching. For `None` fields at the requested order, this is a user error (requesting Fxc but not providing v2rho2).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[cfg(test)]` + `cargo test` |
| Config file | Cargo.toml (already configured) |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| IO-01 | Input bundles validate buffer sizes | unit | `cargo test --lib input` | Wave 0 |
| IO-02 | Output bundles with Option None/Some | unit | `cargo test --lib output` | Wave 0 |
| IO-03 | OutputMask bitflags operations | unit | `cargo test --lib output::mask` | Wave 0 |
| IO-04 | SoA interleaved layout validation | unit | `cargo test --lib input` | Wave 0 |
| IO-05 | MggaOutput 70 fields construction | unit | `cargo test --lib output` | Wave 0 |
| EVAL-01 | Dispatch routes to correct kernel | integration | `cargo test --lib eval::dispatch` | Wave 0 |
| EVAL-02 | Mixed accumulation weighted sum | integration | `cargo test --lib eval::mix` | Wave 0 |
| EVAL-03 | Workspace pre-allocation | unit | `cargo test --lib eval::workspace` | Wave 0 |
| EVAL-04 | Non-mixed zero heap alloc | integration | `cargo test --lib eval::dispatch` (verify no alloc) | Wave 0 |
| EVAL-05 | Mixed functional combined results | integration | `cargo test --lib eval::mix` | Wave 0 |

### Wave 0 Gaps
- [ ] `src/input/mod.rs` -- input bundle types and validation tests
- [ ] `src/output/mod.rs` -- output bundle types and tests
- [ ] `src/output/mask.rs` -- OutputMask bitflags and tests
- [ ] `src/eval/dispatch.rs` -- dispatch tests with LDA_X
- [ ] `src/eval/mix.rs` -- mixed accumulation tests
- [ ] `src/eval/workspace.rs` -- workspace tests

## Sources

### Primary (HIGH confidence)
- Codebase: `src/dims/mod.rs` -- Dimensions struct with all 70+ output fields, tested against libxc util.c
- Codebase: `src/error/mod.rs` -- LibxcRsError with InputBufferSizeMismatch, OutputBufferSizeMismatch variants
- Codebase: `src/kernel/lda/lda_x.rs` -- 10 kernel functions showing exact signature pattern
- Codebase: `src/kernel/launch.rs` -- Buffer management, CPU client, launch config
- Codebase: `src/model/mod.rs` -- Family, Spin, DerivativeOrder, FunctionalFlags, Thresholds
- Codebase: `libxc-master/src/mix_func.c` -- Reference implementation for mixed functional accumulation
- Design doc: `docs/design/libxc_rs_detailed_design.md` Sections 6.6, 6.7, 6.9, 9.6, 9.7, 9.10, 10.2, 10.3

### Secondary (MEDIUM confidence)
- CONTEXT.md: 13 locked decisions (D-01 through D-13) from user discussion

### Tertiary (LOW confidence)
- None -- all findings verified against codebase or design documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all dependencies already in project (bitflags, thiserror, bytemuck, cubecl)
- Architecture: HIGH -- design doc + CONTEXT.md decisions fully specify the architecture
- Pitfalls: HIGH -- verified against actual kernel signatures and libxc source code

**Research date:** 2026-04-09
**Valid until:** 2026-05-09 (stable domain, no external dependency changes expected)
