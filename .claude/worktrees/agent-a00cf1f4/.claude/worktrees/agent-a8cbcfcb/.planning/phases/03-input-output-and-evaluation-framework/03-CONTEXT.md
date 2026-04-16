# Phase 3: Input/Output and Evaluation Framework - Context

**Gathered:** 2026-04-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Type-safe I/O bundles validate buffer sizes, output masks control which derivatives are computed, and the dispatch/accumulation framework correctly routes evaluation for single and mixed functionals. Only LDA_X kernel is available (from Phase 2); the dispatch scaffold is built for all families but only LDA is populated. Mixed functional accumulation is implemented and tested with synthetic/mock auxiliaries.

</domain>

<decisions>
## Implementation Decisions

### Input Bundle Design
- **D-01:** Input bundles use borrowed slices only (`&[f64]`). No owned/Cow modes. Matches libxc's pointer semantics exactly with zero-copy, zero allocation.
- **D-02:** Buffer size validation happens at construction time. `LdaInput::new(rho, np, spin)` validates `rho.len() == np * dims.rho` immediately. Invalid inputs never reach the kernel. Evaluation is infallible after validation (ERR-03).
- **D-03:** Single flat interleaved SoA layout for polarized spin. For polarized: `[rho_a_0, rho_b_0, rho_a_1, rho_b_1, ...]`. Matches libxc convention exactly. Kernel indexes via `ip * dims.rho + component`.
- **D-04:** Input bundles store `np` (number of grid points) explicitly as a field, not derived from buffer length.

### Output Bundle Design
- **D-05:** OutputMask drives Option fields. Output bundles have `Option<&mut [f64]>` for each derivative level (zk, vrho, v2rho2, etc.). `OutputMask::VXC` means vrho must be `Some(...)`. Matches libxc's NULL-pointer semantics where passing NULL for a derivative level skips its computation.
- **D-06:** Output buffers are caller-provided `&mut [f64]`. The library does not allocate output buffers. Caller allocates and passes slices; output bundle validates sizes at construction. Zero allocation in the library path.
- **D-07:** Kernels check Option output fields and skip `None` derivatives. For each derivative level, the kernel checks if the buffer exists before writing. None fields are never touched.

### Dispatch Architecture
- **D-08:** Match-based dispatch in `eval/dispatch.rs`. `match (family, order, spin)` routes to the specific kernel function (e.g., `lda_x_vxc_unpol`). Simple, explicit, no trait machinery.
- **D-09:** Dispatch entry point is a method on the Functional struct: `functional.evaluate_lda(&input, order, &mut output)`. Phase 3 builds the dispatch logic that Phase 5 wraps in the full Functional lifecycle.
- **D-10:** Build the full dispatch scaffold now with LDA_X as the only populated arm. GGA/MGGA arms return `UnsupportedDerivativeOrder` (or similar) until Phase 4 fills them. Establishes the pattern early.

### Mixed Functional Accumulation
- **D-11:** EvaluationWorkspace pre-allocates scratch buffers sized for the largest auxiliary functional. Reused across auxiliary evaluations within a single evaluation call. One allocation at workspace creation, zero during evaluation (EVAL-03).
- **D-12:** Workspace scratch buffers are sized for MGGA (the superset family). LDA/GGA auxiliaries use only the fields they need; the rest is unused. Simple, avoids resizing.
- **D-13:** Non-mixed functionals bypass the workspace entirely. Dispatch detects non-mixed functional (no auxiliaries), calls kernel directly with user-provided output buffers. Zero heap allocation in the non-mixed hot path (EVAL-04). Workspace is only constructed for mixed functionals.

### Claude's Discretion
- Exact struct field layout for LdaInput/GgaInput/MggaInput (which fields beyond rho, sigma, lapl, tau)
- OutputMask bitflag values and whether to reuse FunctionalFlags or create a separate bitflags type
- Internal structure of eval/dispatch.rs (helper functions, intermediate types)
- How the dispatch scaffold handles "not yet implemented" kernels (error variant vs panic in debug)
- EvaluationWorkspace internal data structure (Vec<f64> per field vs single flat buffer with offsets)
- Whether to add convenience factory methods (e.g., `LdaOutput::for_order()`) alongside the caller-provides model

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Input/Output Module Specification
- `docs/design/libxc_rs_detailed_design.md` Section 9.6 -- input/ module responsibilities: LdaInput, GgaInput, MggaInput with validation
- `docs/design/libxc_rs_detailed_design.md` Section 9.7 -- output/ module responsibilities: OutputMask, Option<&mut [f64]> semantics, factory methods
- `docs/design/libxc_rs_detailed_design.md` Section 6 -- Domain model: Dimensions struct fields that input/output bundles validate against

### Evaluation Orchestration
- `docs/design/libxc_rs_detailed_design.md` Section 9.10 -- eval/ module: dispatch.rs, mix.rs, workspace.rs responsibilities
- `docs/design/libxc_rs_detailed_design.md` Section 10.2 -- LDA evaluation flow (non-mixed): validation, zeroing, per-point dispatch, accumulation
- `docs/design/libxc_rs_detailed_design.md` Section 10.3 -- Mixed functional evaluation flow: auxiliary iteration, scratch allocation, weighted accumulation

### Existing Infrastructure
- `src/kernel/launch.rs` -- Buffer create/read, CubeCount/CubeDim calc, cpu_client() (from Phase 2)
- `src/kernel/lda/lda_x.rs` -- LDA_X canary kernel: 10 functions (5 orders x 2 spin modes) showing the kernel signature pattern
- `src/dims/mod.rs` -- Dimensions struct with lda/gga/mgga constructors, total_output_components()
- `src/error/mod.rs` -- LibxcRsError with InputBufferSizeMismatch, OutputBufferSizeMismatch, FamilyMismatch, SpinMismatch variants
- `src/model/mod.rs` -- Family, Spin, DerivativeOrder, FunctionalFlags, Thresholds types

### libxc Reference Implementation
- `libxc-master/src/util.c` -- internal_counters_set_lda/gga/mgga functions (dimension calculation reference)
- `libxc-master/src/mix_func.c` -- Mixed functional accumulation logic (the reference for EVAL-02)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/kernel/launch.rs`: `cpu_client()`, `create_input_buffer()`, `create_zero_output_buffer()`, `read_output_buffer()`, `calculate_launch_config()` -- all needed for dispatch layer
- `src/dims/mod.rs`: `Dimensions` struct with all 70+ field dimensions -- used for input/output validation
- `src/kernel/lda/lda_x.rs`: 10 kernel functions showing the exact signature pattern (rho, zk, vrho, ..., alpha, dens_threshold, zeta_threshold)
- `src/error/mod.rs`: Buffer mismatch and family/spin mismatch error variants already defined

### Established Patterns
- CubeCL `#[cube(launch_unchecked)]` with ABSOLUTE_POS and bounds check pattern
- Kernel accumulation via `+=` (zk[ip] += tzk0, vrho[ip] += tvrho0)
- Edition 2024, `#![deny(warnings)]`
- Module-per-directory structure

### Integration Points
- `src/lib.rs`: Needs `pub mod input;`, `pub mod output;`, `pub mod eval;` added
- Kernel functions in `lda_x.rs` currently take individual `Array<f64>` params -- dispatch layer must bridge between input/output bundles and kernel arguments
- `Dimensions` is used both for input validation and output buffer sizing
- Mixed functional evaluation needs access to `FunctionalMeta.auxiliaries` (currently has placeholder data from Phase 1)

</code_context>

<specifics>
## Specific Ideas

No specific requirements -- open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 03-input-output-and-evaluation-framework*
*Context gathered: 2026-04-09*
