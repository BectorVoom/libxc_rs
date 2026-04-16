# Phase 2: Math Core and CubeCL Substrate - Context

**Gathered:** 2026-04-09
**Status:** Ready for planning

<domain>
## Phase Boundary

All mathematical building blocks implemented as `#[cube]` functions, validated against known values and libm references, CubeCL CPU backend integrated, and the LDA_X canary kernel producing bit-accurate f64 results through all derivative orders and both spin modes. This phase establishes the CubeCL compute substrate and translation pattern that Phase 4 uses for all 270 kernel files.

</domain>

<decisions>
## Implementation Decisions

### CubeCL Integration
- **D-01:** Add cubecl with only the `cpu` feature in Phase 2. GPU backends (cuda, hip, wgpu) are feature-gated and deferred to Phase 7. This keeps compile times down and avoids GPU toolchain requirements during math core development.
- **D-02:** Trust the CubeCL CPU backend directly for testing -- no separate plain-Rust reference implementations of `#[cube]` functions. Tests run math functions through CubeCL CPU and compare against hardcoded known values and libm sweeps.

### Module Organization
- **D-03:** Flat `src/math/` module with submodules: `powers.rs`, `piecewise.rs`, `constants.rs`, `spin.rs`, `erf.rs`, `dft_quantities.rs`, `polynomials.rs`. All functions are `#[cube]`-annotated. Matches design doc Section 9.5.
- **D-04:** LDA_X canary kernel lives at `src/kernel/lda/lda_x.rs`, following design doc Section 9.9 structure from day one. Creates the `kernel/` module hierarchy that Phase 4 populates with all 270 kernels.

### erf/erfc Implementation
- **D-05:** Use Cephes/libm-style piecewise rational approximation for erf and erfc. Port the approach from libm's `erf.c` -- multiple coefficient sets per interval.
- **D-06:** Target full f64 precision (~1e-15 relative error) for erf/erfc. No precision shortcuts -- errors in erf can amplify through derivative chains in range-separated functionals.

### Math Core Testing
- **D-07:** Tests live inline as `#[cfg(test)]` at the bottom of each math submodule (e.g., `src/math/powers.rs` contains its own tests).
- **D-08:** Test against both hardcoded known values (hand-computed / Wolfram Alpha: `cbrt(-8)==-2`, `erf(1)==0.8427007929...`) AND libm sweep tests comparing across ranges of inputs (e.g., erf for x in -6..6 at 1000 points). Add `libm` as a dev-dependency for sweep comparison.
- **D-09:** Cross-backend consistency testing (MATH-10: CPU vs GPU producing identical results) is deferred to Phase 7 when GPU backends are available. Phase 2 verifies correctness on CubeCL CPU only.

### LDA_X Canary Kernel
- **D-10:** Translate all derivative orders through 4th (exc, vxc, fxc, kxc, lxc) from `libxc-master/src/maple2c/lda_exc/lda_x.c`. The maple2c file has `maple2c_order 4` -- translate the complete file.
- **D-11:** Include both unpolarized (`func_*_unpol`) and polarized (`func_*_pol`) spin modes. Validates spin handling early per success criteria SC-3.
- **D-12:** Manual hand-translation of `lda_x.c` to Rust `#[cube]` functions, preserving exact variable names (`t2`, `t3`, ...) and floating-point operation order. This establishes the canonical translation pattern that Phase 4 follows for all 270 files.
- **D-13:** Build full launch infrastructure in `kernel/launch.rs` -- backend selection, buffer management, CubeCount/CubeDim calculation, dispatch traits. Front-loads the foundation rather than building a minimal wrapper that gets rewritten.

### Claude's Discretion
- Exact CubeCL `ComputeClient` initialization pattern and lifetime management
- CubeCount/CubeDim calculation strategy (elements per workgroup)
- Whether `poly_eval` and `rational_eval` use const generics or slices for coefficient arrays
- Internal organization of kernel/launch.rs (traits, structs, helper functions)
- libm dev-dependency version selection

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Mathematical Core Specification
- `docs/design/libxc_rs_detailed_design.md` Section 7 -- Complete math core design: all function signatures, boundaries, testing strategy, abstraction avoidance
- `docs/design/libxc_rs_detailed_design.md` Section 7.2 -- Core component specifications (powers, piecewise, constants, spin, erf, polynomials, DFT quantities)
- `docs/design/libxc_rs_detailed_design.md` Section 7.3 -- Boundary table: what belongs in math core vs functional-specific kernels

### CubeCL Kernel Infrastructure
- `docs/design/libxc_rs_detailed_design.md` Section 9.9 -- kernel/ module structure, launch.rs responsibilities, per-family organization
- `docs/design/libxc_rs_detailed_design.md` Section 12 -- GPU design with cubecl: backend abstraction, kernel granularity, launch patterns
- `docs/design/libxc_rs_detailed_design.md` Section 12.9 -- How math core is shared across backends

### CubeCL Documentation
- `docs/manual/Cubecl/cubecl_3d_dft.md` -- Confirms f64 usage in #[cube] kernels, provides launch patterns
- `docs/manual/Cubecl/Cubecl_multi_ compute.md` -- Multi-compute patterns
- `docs/manual/Cubecl/cubecl_reduce_sum.md` -- Reduction patterns

### Translation Source
- `libxc-master/src/maple2c/lda_exc/lda_x.c` -- The canary kernel source to translate
- `libxc-master/src/util.h` -- Defines POW_1_3, my_piecewise3, M_CBRT3 macros and math helpers

### Oracle Verification
- `docs/design/libxc_rs_detailed_design.md` Section 17 -- Oracle verification plan, error metrics, tolerance thresholds
- `verify/build.rs` -- Existing cmake + bindgen setup for oracle comparison

### Performance Considerations
- `docs/design/libxc_rs_detailed_design.md` Section 14.3 -- Mathematical core performance guarantee (inlining via #[cube])
- `docs/design/libxc_rs_detailed_design.md` Section 14.4 -- Operation ordering and numerical equivalence

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/model/mod.rs`: Family, Spin, DerivativeOrder enums -- used by kernel dispatch and dimension calculation
- `src/dims/mod.rs`: Dimensions struct -- needed for buffer size validation in launch wrappers
- `src/error/mod.rs`: LibxcRsError enum -- extend with GPU/kernel error variants
- `src/registry/mod.rs`: lookup_by_id returns FunctionalMeta -- needed for ext_params in canary kernel
- `verify/build.rs`: cmake + bindgen infrastructure -- extend for LDA_X oracle comparison tests
- `Cargo.toml`: Workspace with verify/ and xtask/ already configured

### Established Patterns
- Edition 2024, `#![deny(warnings)]` enforced
- Module-per-directory structure (model/, meta/, registry/, error/, dims/)
- thiserror v2 at library boundary
- Generated code committed to repo (from Phase 1 xtask)

### Integration Points
- `src/lib.rs`: Needs `pub mod math;` and `pub mod kernel;` added
- `Cargo.toml`: Needs cubecl dependency with cpu feature
- `verify/`: Oracle comparison tests for LDA_X canary validation
- `src/meta/generated.rs`: Contains ext_params for lda_x (alpha parameter)

</code_context>

<specifics>
## Specific Ideas

- Preserve exact maple2c variable names (t2, t3, t4, ..., tzk0, tvrho0) in the Rust translation for traceability back to C source
- The lda_x.c canary uses `lda_x_params` struct with an `alpha` parameter -- this is the ext_params mechanism that needs to flow through the kernel launch
- libm's erf.c source is the reference for the piecewise rational approximation -- multiple intervals with different polynomial coefficients for small, medium, and large x

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 02-math-core-and-cubecl-substrate*
*Context gathered: 2026-04-09*
