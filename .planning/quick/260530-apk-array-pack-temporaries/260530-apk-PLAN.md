---
quick_id: 260530-apk
slug: array-pack-temporaries
date: 2026-05-30
status: complete
---

# Quick Task 260530-apk: Array-pack scalar temporaries in the kernel generators

## Objective

Refactor the kernel code generators so the emitted CubeCL kernels aggregate the
thousands of individual scalar temporaries (`let t1 = ...; let t2 = ...;`) into a
single comptime-sized scratch array, to relieve the super-linear rustc
liveness/regalloc cost of the dense Maple2c temporaries.

## Contract

1. One scratch array per kernel/chunk body; each `let tN = expr;` becomes an
   index assignment into it.
2. Boolean temporaries (comparison RHS, e.g. `tN = tM <= thresh`) stay as
   individual `let` bindings — a float array cannot hold a bool.
3. Preserve FP operation order (only the storage slot changes, never arithmetic).

## Tasks

1. Shared helper `tools/translate_v2/array_pack.py` — packed-set + dense index
   map + `remap`/`emit_line`/`decl`. Reuse `cse._is_bool_rhs` for bool detection.
2. Wire into the flat path: `generate_function` in `translate_gga.py`,
   `translate_lda_v2.py`, `translate_mgga.py` (`Array::<f64>::new`).
3. Wire into the chunked path: `_cse_chunk_part` in
   `translate_v2/per_functional.py` (`Array::<F>::new`), preserving the existing
   `_wrap_f64_literals_v2` F-wrapping, excluding tuple-input params, remapping
   packed outputs in the return tuple.
4. Validate by regen + `cargo +nightly check -p` of a representative flat
   functional and a representative generic chunk.
