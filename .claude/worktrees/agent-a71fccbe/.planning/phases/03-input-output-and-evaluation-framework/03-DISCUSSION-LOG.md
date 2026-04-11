# Phase 3: Input/Output and Evaluation Framework - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-04-09
**Phase:** 03-input-output-and-evaluation-framework
**Areas discussed:** Input bundle design, Output bundle design, Dispatch architecture, Mixed functional accumulation

---

## Input Bundle Design

| Option | Description | Selected |
|--------|-------------|----------|
| Borrowed only (&[f64]) | Zero-copy views over caller-owned slices, matches libxc pointer semantics | ✓ |
| Borrowed + owned enum | Cow-like enum supporting both &[f64] and Vec<f64> | |
| Generic over storage | Generic T: AsRef<[f64]> for flexible storage types | |

**User's choice:** Borrowed only (&[f64])
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| At construction | Validate immediately on LdaInput::new(), invalid inputs never reach kernel | ✓ |
| At evaluation time | Validate when evaluate() is called, simpler structs | |
| Both (belt and suspenders) | Validate at construction AND re-check at evaluation | |

**User's choice:** At construction
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Single flat slice | Interleaved SoA [rho_a_0, rho_b_0, rho_a_1, ...] matching libxc convention | ✓ |
| Separate per-spin slices | (rho_a, rho_b) separately, cleaner API but mismatches libxc layout | |
| Struct of arrays | Separate named fields per component, explicit but too many fields | |

**User's choice:** Single flat slice
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Store np explicitly | np stored as field, needed for launch config and output sizing | ✓ |
| Derive from buffer length | np = rho.len() / dims.rho, saves one field | |

**User's choice:** Store np explicitly
**Notes:** None

---

## Output Bundle Design

| Option | Description | Selected |
|--------|-------------|----------|
| OutputMask drives Option fields | Option<&mut [f64]> per derivative, OutputMask determines which are Some | ✓ |
| OutputMask as separate parameter | All fields always &mut [f64], OutputMask controls kernel writes | |
| Order-based factory only | No explicit OutputMask bitflags, Option pattern IS the mask | |

**User's choice:** OutputMask drives Option fields
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Caller-provided &mut [f64] | Caller allocates, library validates sizes, zero allocation | ✓ |
| Library-allocated Vec<f64> | Factory methods allocate owned output | |
| Both modes via enum | Support both caller-provided and library-allocated | |

**User's choice:** Caller-provided &mut [f64]
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Kernel checks Option, skips None | Each derivative checks buffer existence before writing | ✓ |
| Pre-filter at dispatch | Dispatch selects kernel variant based on OutputMask | |
| You decide | Claude chooses based on CubeCL constraints | |

**User's choice:** Kernel checks Option, skips None
**Notes:** None

---

## Dispatch Architecture

| Option | Description | Selected |
|--------|-------------|----------|
| Match-based dispatch | match (family, order, spin) routes to specific kernel function | ✓ |
| Trait-based dispatch | KernelEvaluator trait per functional | |
| Function pointer table | FunctionalMeta stores fn pointers indexed by (order, spin) | |

**User's choice:** Match-based dispatch
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Method on Functional struct | functional.evaluate_lda(&input, order, &mut output) | ✓ |
| Free function in eval/ | eval::evaluate_lda(meta, params, ...) standalone functions | |
| You decide | Claude chooses based on integration with launch.rs | |

**User's choice:** Method on Functional struct
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Build dispatch scaffold now | Full match structure, LDA_X populated, others return error | ✓ |
| Minimal dispatch for LDA_X | Only LDA dispatch, GGA/MGGA added in Phase 4 | |
| Registry-driven dispatch | FunctionalMeta gains dispatch_fn field | |

**User's choice:** Build dispatch scaffold now
**Notes:** None

---

## Mixed Functional Accumulation

| Option | Description | Selected |
|--------|-------------|----------|
| Pre-allocated workspace | Scratch buffers pre-allocated, reused across auxiliaries | ✓ |
| Allocate per evaluation | Fresh allocation per mixed evaluation call | |
| Arena allocator | Bump allocator arena with per-auxiliary slices | |

**User's choice:** Pre-allocated workspace
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Largest-family sizing | Workspace sized for MGGA superset, LDA/GGA use subset | ✓ |
| Per-auxiliary sizing | Exact-sized buffers per auxiliary family | |
| You decide | Claude chooses based on mix_func.c reference | |

**User's choice:** Largest-family sizing
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Direct kernel call, no workspace | Non-mixed bypass workspace entirely, zero heap allocation | ✓ |
| Workspace always present | All evaluations use workspace uniformly | |
| You decide | Claude chooses cleanest path for EVAL-04 | |

**User's choice:** Direct kernel call, no workspace
**Notes:** None

---

## Claude's Discretion

- Exact struct field layout for input/output bundles
- OutputMask bitflag values
- Internal structure of eval/dispatch.rs
- How dispatch handles unimplemented kernels
- EvaluationWorkspace internal data structure
- Convenience factory methods alongside caller-provides model

## Deferred Ideas

None -- discussion stayed within phase scope
