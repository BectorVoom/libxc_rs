---
title: Target workspace architecture for libxc_rs
date: 2026-05-07
context: Captured during /gsd-explore session on workspace-level modular refactor
---

# Target workspace architecture for libxc_rs

## Driving concern

Coupling between `eval/` orchestration and `model/` + `registry/` types is the
problem to solve. Today, type definitions and metadata live in the same crate
as compute orchestration, so there is no compiler-enforced boundary preventing
the data layer from importing the compute layer. The refactor's load-bearing
principle is:

> **`libxc-core` contains no compute.** Dependencies flow one way only:
> `libxc-eval` -> `libxc-core`. Never the reverse.

If a change tries to add a `use libxc_eval::...` to anything inside
`libxc-core`, that is a design failure, not a missing import.

## Target shape

```
crates/
  libxc-core/        # model, meta, registry, input, output, layout, dims
  libxc-eval/        # eval, functional, kernel-glue, workspace
  libxc-compat/      # extern "C" shim (cdylib)
  libxc-kernel-*/    # existing 170 kernel-{lda,gga,mgga}-* crates (unchanged)
  kernel-math/       # existing (unchanged)
libxc_rs (root)      # api/ facade only — re-exports curated public surface
```

### Crate responsibilities

- **`libxc-core`** — pure data. The 649-functional metadata table, registry
  lookups, domain enums (`Family`, `Spin`, `Kind`, `Derivative`, `Precision`),
  bitflags (`OutputMask`, `FunctionalFlags`), I/O bundle types
  (`LdaInput`/`GgaInput`/`MggaInput` and outputs), layout descriptors, and
  dimension calculation. **No CubeCL imports. No compute logic.**
- **`libxc-eval`** — orchestration. Dispatch routing, mixed-functional
  accumulation, kernel launch glue, scratch/workspace planning, functional
  lifecycle and parameter management. Depends on `libxc-core` and the
  `kernel-*` crates.
- **`libxc-compat`** — extern "C" ABI shim. The 85 C entry points, errno
  handling, cdylib build, opaque handle types, libxc constant assertions.
  Depends on `libxc-eval` and `libxc-core` but is **never** depended on
  by them.
- **`libxc_rs` (root)** — thin facade. Builder pattern, `BatchEvaluator`,
  curated public re-exports. Most users depend only on this crate.

## Modules whose homes are not yet decided

- `src/error/` — likely belongs in `libxc-core` since both `eval` and
  `compat` need to construct/match `LibxcRsError`. Open question: should
  it be a separate `libxc-error` micro-crate to avoid forcing every
  consumer to pull `libxc-core`'s metadata tables? See pending todo
  `audit-error-math-placement`.
- `src/math/` — currently a thin module; the heavy lifting is in the
  existing `kernel-math` crate. Likely either deleted (if redundant) or
  absorbed into `kernel-math`.

## Risks worth flagging early (for the planning phase)

- **Generated files cross crate boundaries.** `meta/generated.rs`,
  `meta/generated_propagation.rs`, `registry/generated.rs`, and
  `compat/ids.rs` are likely build-script outputs. Splitting their
  destinations across crates affects `xtask` codegen flow and incremental
  rebuild ordering. See research question logged for this phase.
- **Re-export churn.** Every `use libxc_rs::model::Spin` callsite becomes
  `use libxc_rs_core::model::Spin` (or a re-export through the root).
  The root crate should re-export the curated public surface so external
  consumers see a stable import path.
- **Collides with active 06-02a work.** Phase 06 is currently extracting
  the C compatibility layer. Best executed at a quiet seam between
  feature phases (after 06 closes, before next milestone work).
