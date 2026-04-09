# Phase 4: Bulk Kernel Translation - Context

**Gathered:** 2026-04-10
**Status:** Ready for planning

<domain>
## Phase Boundary

Translate all 262 maple2c C kernel files (42 LDA + 130 GGA + 90 MGGA, plus 4 special _vxc files) to Rust `#[cube]` functions preserving exact floating-point operation order. Wire each functional into the dispatch layer and verify every functional against the libxc oracle through all applicable derivative orders and both spin modes. Phase 2's LDA_X canary kernel established the canonical translation pattern; this phase applies it to the remaining 261 functionals.

</domain>

<decisions>
## Implementation Decisions

### Translation Approach
- **D-01:** Fully manual hand-translation of each maple2c C file to Rust `#[cube]` functions, following the LDA_X pattern established in Phase 2 (D-12). No automated translator tool.
- **D-02:** One Rust file per functional, matching the 1:1 correspondence with maple2c source files (e.g., `lda_c_vwn.c` -> `lda_c_vwn.rs`). Each file contains all derivative order x spin mode combinations.
- **D-03:** Each functional gets its own launch wrapper file (matching the `launch_lda_x.rs` pattern from Phase 2).
- **D-04:** The 4 special `_vxc` files (`lda_xc_tih`, `gga_x_lb`, `mgga_x_2d_prp10`, `mgga_x_tb09`) are translated alongside their family batches, not deferred.

### Large Kernel Handling
- **D-05:** Translate massive MGGA kernels (up to 100K lines) as-is, faithfully following the maple2c source. If CubeCL compilation fails or produces unacceptable compile times, split into sub-kernels per derivative order as a fallback.
- **D-06:** Test the largest MGGA kernel (`mgga_c_rmggac`, 100K lines) as the FIRST MGGA translation to surface compilation limit risks immediately. If it compiles and runs, the rest will too.

### Verification Strategy
- **D-07:** Per-family batch test files: `lda_oracle.rs`, `gga_oracle.rs`, `mgga_oracle.rs` in the verify/ crate. Each iterates over all functionals in the family, testing across applicable derivative orders and spin modes using the 4 test systems (H, Li, BrOH, BrOH+).
- **D-08:** Each functional must pass oracle verification before moving to the next. Translation is only "done" when it passes. No deferred failure fixing.
- **D-09:** Each derivative order tested independently. For a functional supporting up to 4th order, run 5 separate tests (exc, vxc, fxc, kxc, lxc). Each checks all output fields at that level against the appropriate tolerance tier (VERIFY-03 through VERIFY-07).
- **D-10:** Tolerance tiers per requirements: energy (exc) <= 10^-12, VXC <= 10^-10, FXC <= 10^-8, KXC <= 10^-6, LXC <= 10^-4.

### Translation Ordering
- **D-11:** Family order: LDA (42 files) -> GGA (130 files) -> MGGA (90 files). Simplest-first matches the roadmap's plan structure and natural dependency chain (GGA may use LDA sub-expressions, MGGA may use GGA/LDA).
- **D-12:** Dispatch wiring happens per-functional: each translated functional is immediately wired into the dispatch match statement and verified before moving to the next. No batch-then-wire approach.

### Claude's Discretion
- Module structure under `kernel/gga/` and `kernel/mgga/` (flat vs grouped by sub-family)
- Whether to add a `kernel/mod.rs` re-export strategy or keep modules internal
- How to organize the per-family oracle test files (parametric test macros, test helper utilities)
- Whether the launch wrapper pattern needs adaptation for GGA/MGGA (additional input arrays: sigma, lapl, tau)
- Commit granularity during translation (per-functional vs small batches of related functionals)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Kernel Translation Pattern
- `src/kernel/lda/lda_x.rs` -- The canonical LDA_X translation showing exact variable name preservation, FP operation order, and `#[cube]` function structure
- `src/kernel/lda/launch_lda_x.rs` -- Launch wrapper pattern: buffer creation, kernel dispatch, result readback
- `docs/design/libxc_rs_detailed_design.md` Section 9.9 -- kernel/ module structure, per-family organization

### Translation Sources
- `libxc-master/src/maple2c/lda_exc/` -- 42 LDA kernel C files to translate
- `libxc-master/src/maple2c/gga_exc/` -- 130 GGA kernel C files to translate
- `libxc-master/src/maple2c/mgga_exc/` -- 90 MGGA kernel C files to translate
- `libxc-master/src/maple2c/lda_vxc/lda_xc_tih.c` -- Special LDA _vxc file
- `libxc-master/src/maple2c/gga_vxc/gga_x_lb.c` -- Special GGA _vxc file
- `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c` -- Special MGGA _vxc file
- `libxc-master/src/maple2c/mgga_vxc/mgga_x_tb09.c` -- Special MGGA _vxc file
- `libxc-master/src/util.h` -- Macro definitions (POW_1_3, my_piecewise3, M_CBRT3, etc.)

### Dispatch and Evaluation Infrastructure
- `src/eval/dispatch.rs` -- Existing match-based dispatch for LDA_X; extend for all functionals
- `src/eval/mix.rs` -- Mixed functional accumulation logic
- `src/kernel/launch.rs` -- Backend selection, buffer management, CubeCount/CubeDim

### Verification
- `docs/design/libxc_rs_detailed_design.md` Section 17 -- Oracle verification plan, error metrics, tolerance thresholds
- `verify/build.rs` -- cmake + bindgen oracle infrastructure
- `verify/src/` -- Existing verification harness

### Input/Output Contracts
- `src/input/mod.rs` -- LdaInput, GgaInput, MggaInput validation and buffer layout
- `src/output/mod.rs` -- LdaOutput, GgaOutput, MggaOutput with Option<&mut [f64]> semantics
- `src/dims/mod.rs` -- Dimensions struct for buffer size calculation

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/kernel/lda/lda_x.rs`: 10 kernel functions (5 orders x 2 spins) -- the exact pattern to replicate for all 262 functionals
- `src/kernel/lda/launch_lda_x.rs`: Launch wrapper with `BufArg` enum for optional output buffers
- `src/kernel/launch.rs`: `cpu_client()`, `create_input_buffer()`, `create_zero_output_buffer()`, `read_output_buffer()`, `calculate_launch_config()`
- `src/math/`: All mathematical building blocks (`pow_1_3`, `piecewise3`, `piecewise5`, `erf`, `erfc`, `safe_cbrt`, constants) ready for use in kernels
- `src/eval/dispatch.rs`: Match-based dispatch scaffold ready to extend

### Established Patterns
- `#[cube(launch_unchecked)]` with `ABSOLUTE_POS` and bounds check
- Exact maple2c variable name preservation (`t2`, `t3`, ..., `tzk0`, `tvrho0`)
- Output accumulation via `+=` semantics (`zk[ip] += tzk0`)
- Numeric literal translation: `0.2e1` -> `2.0`, `0.3e1` -> `3.0`
- Macro mapping: `POW_1_3(x)` -> `pow_1_3(x)`, `my_piecewise3(c, x1, x2)` -> `piecewise3(c, x1, x2)`

### Integration Points
- `src/kernel/mod.rs`: Needs `pub mod gga;` and `pub mod mgga;` added
- `src/eval/dispatch.rs`: Needs `dispatch_gga()` and `dispatch_mgga()` functions alongside existing `dispatch_lda()`
- `src/lib.rs`: May need re-exports updated as kernel modules grow
- `verify/`: New test files for GGA and MGGA oracle verification

</code_context>

<specifics>
## Specific Ideas

- Preserve exact maple2c variable names for traceability back to C source (established in Phase 2 D-12)
- The LDA_X translation in `src/kernel/lda/lda_x.rs` is the gold-standard reference -- every subsequent translation should follow this exact structure
- Test the 100K-line `mgga_c_rmggac` first among MGGAs to validate CubeCL can handle massive kernels before committing to translating all 90

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 04-bulk-kernel-translation*
*Context gathered: 2026-04-10*
