---
title: Audit error/ and math/ module placement before workspace-modular-split phase
date: 2026-05-07
priority: medium
---

# Audit error/ and math/ module placement

Before the workspace-level modular split phase can be planned, two modules
in `src/` need an explicit destination decision. Neither came up in the
/gsd-explore session that produced the target architecture, so this todo
exists to resolve them.

## What to decide

### `src/error/`

Default position: lives in `libxc-core` (both `eval` and `compat` need to
construct and match `LibxcRsError`).

Counter-position: extract as a standalone `libxc-error` micro-crate so
that downstream consumers who only want the error type don't have to pull
in `libxc-core`'s metadata tables (which are large generated artifacts).

Decision criteria:
- How many distinct error variants are there? (Recent commit
  `1d5d25f6 feat(06-02a): LibxcRsError::discriminant() with 24-variant
  exhaustive table` says 24.)
- Does any kernel-* crate need to construct errors? If yes, either they
  depend on `libxc-core` (heavy) or on `libxc-error` (cheap).
- Is there a stable C-ABI errno mapping that lives close to error
  variants? If yes, the FFI codegen lives in `libxc-compat` regardless.

### `src/math/`

Default position: delete or fold into existing `kernel-math` crate.

Counter-position: the module exists for a reason — check `mod.rs` for
what's actually exported and whether anything outside kernels uses it.

Decision criteria:
- Read `src/math/mod.rs` and grep for `use crate::math::` across `src/`
  and `crates/`.
- If only kernels use it: merge into `kernel-math`.
- If `eval` or `api` uses it for host-side reference computation: keep
  as a small module (in `libxc-eval` or root).

## Output

A two-line decision log that the modular-split planner can read:

```
error/: <decision> (<reason>)
math/:  <decision> (<reason>)
```

Add to the workspace-modular-architecture note when resolved.
