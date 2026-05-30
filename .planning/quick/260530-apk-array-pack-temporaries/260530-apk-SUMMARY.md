---
quick_id: 260530-apk
slug: array-pack-temporaries
date: 2026-05-30
status: complete
outcome: SUCCESS — generators array-pack temporaries; both flat (Array::<f64>) and chunked (Array::<F>) paths regen + compile clean. Mass regen deferred to user (heavy/OOM-prone).
duration: ~1 session (inline, jobs=1)
commits:
  - 851b043df4   # feat(translate): array-pack scalar temporaries into a scratch Array (260530-apk)
follow_up:
  - MASS REGEN NOT RUN. The generator change is committed but the ~253K-file kernel tree is NOT regenerated. See "Regen runbook" below. Mass regen + full recompile is an OOM-prone, multi-hour, user-driven operation.
  - Regen prerequisite (pre-existing, unrelated breakage): tools/kernel_routing.py::_model_path points at src/model/{family}_functional.rs, but the model files moved to crates/libxc-core/src/model/ during the Phase 10 workspace split. Either `ln -sfn ../crates/libxc-core/src/model src/model` before regen, or fix _model_path. Worked around with a temp symlink during validation (removed).
  - GPU-local-memory note: dense per-body indexing sizes each array exactly (no waste). Acceptable for f64/CPU correctness target; revisit if GPU occupancy becomes a concern for very large kernels.
  - Chunked generic emit still emits `F::new(0.0)` literals that warn "falling back to f32" (pre-existing, from _wrap_f64_literals_v2 — NOT introduced here). Compiles fine; worth a separate cleanup.
---

# Quick Task 260530-apk — Array-pack scalar temporaries

## What changed

The Maple2c translators emitted one `let tN = expr;` per intermediate — thousands
of distinct scalar SSA locals per kernel/chunk. rustc liveness/regalloc scale
super-linearly in simultaneously-live locals, so these are the dominant
kernel-crate compile cost. The generators now collapse them into a single
comptime-sized scratch array.

**New file:** `tools/translate_v2/array_pack.py` — pure helper:
- `compute_packed(parsed, exclude) -> (index_map, length)`: packs pure-numeric
  (`^t\d+$`), non-bool, non-excluded temps; assigns DENSE 0-based slots.
- `remap(expr, index_map)`: rewrites packed `tN` → `t[slot]`; leaves bools,
  tuple-input params, and ambient ids (`rho`, `param_*`, `M_*`) untouched.
- `decl(elem, len, indent)`: `let mut t = Array::<elem>::new({len}usize);`
- `emit_line(var, expr, index_map, indent)`: `t[slot] = expr;` for packed, else
  `let var = expr;`.
- Bool detection reuses `cse._is_bool_rhs` (identical classification to the CSE
  partitioner).

**Flat path** (`generate_function` in all three translators): one
`Array::<f64>::new(Nusize)` per kernel; output-write vars and bools stay `let`.

**Chunked path** (`_cse_chunk_part` in `per_functional.py`): one
`Array::<F>::new(Nusize)` per `<F: Float>` chunk. Tuple-INPUT params stay bare
`F` (not packed); bools stay `let`; OUTPUT temps are packed and the in-chunk
return expression indexes them (`(t[0], t[1])`) while the caller-side
`wrapper_bind` keeps the bare names it binds. Existing `_wrap_f64_literals_v2`
F-wrapping runs first, then `remap` (wrapping never emits a `tN` token, so the
order is safe).

## Key technical findings (load-bearing)

1. **cubecl 0.10 removed the frontend `LocalArray`.** The objective specified
   `LocalArray::<f64, N>::new()` — that does NOT exist in cubecl 0.10 (it was a
   0.9 frontend type). The 0.10 local-scratch idiom is `Array::<T>::new(#[comptime]
   length: usize)` with index assignment via `ListMut` (cubecl-core
   `frontend/container/array/base.rs`). Confirmed against the pinned 0.10.0.
2. **Size literal must be explicit `usize`.** Bare `Array::<f64>::new(114)` fails
   `E0277: i32: Into<usize>` under the `#[cube]` macro (one error per kernel).
   Canonical form is `Array::<u32>::new(2usize)` (cubecl-core
   runtime_tests/index.rs). Fix: emit `new(Nusize)`.
3. **Index literals are fine bare.** `order[0] = 0;` compiles in `#[cube]`
   (runtime_tests/index.rs) — no suffix needed on the index.
4. **Dense per-body indexing, not original Maple numbers.** The chunk path
   indices are GLOBAL/sparse; preserving them sized a 2-live-value tail chunk at
   `Array::new(1902)`. Dense slots size it at 2. FP op order is unaffected.

## Validation (jobs=1, nightly)

- Flat: regenerated `gga_c_pbe` → `Array::<f64>::new(68usize)`, dense `t[0..]`,
  bool `t33` kept as `let`, `piecewise3::<f64>(t33, t[34]*…)` mixes bare bool +
  array refs. `cargo +nightly check -p libxc-kernel-gga_c_pbe` → **Finished, exit 0**
  (all 10 output modules).
- Chunked: real generated generic chunk (`lda_c_pw_erf ... chunk243<F: Float>`)
  with `Array::<F>::new(2usize)`, bare `F` inputs, bool `let t44`, F-wrapped
  literals, `(t[0], t[1])` return — compiled as a throwaway module inside the
  already-built pbe crate → **Finished, exit 0**.
- `array_pack.py --selftest` and `cse.py --selftest` pass.
- The kernel tree was restored to its committed state after validation; only the
  5 generator files changed.

## Regen runbook (USER-DRIVEN — heavy / OOM-prone)

The committed change only updates the GENERATORS. The ~253K-file kernel tree is
unchanged. To propagate:

```bash
# 0. one-time prerequisite (model files moved in the Phase 10 split):
ln -sfn ../crates/libxc-core/src/model src/model      # or fix kernel_routing._model_path

# 1. translate (Python, low RAM) — rewrites unsharded per-functional subcrates:
PYTHONPATH=tools python3 tools/maple_to_kernels.py translate --family all
#    (or per family: --family gga | lda | mgga; --dry-run to preview)

# 2. re-shard the oversized functionals:
PYTHONPATH=tools python3 tools/maple_to_kernels.py split --family all

# 3. compile-GATE INCREMENTALLY, per-crate, jobs=1 (NEVER the whole tree at once —
#    concurrent rustc OOMs this box; a single small kernel crate ~4 min):
cargo +nightly check -p <one-kernel-crate> -j1

# 4. per-family f64 oracle parity (see memory reference_per_family_oracle_command).
```

Do NOT run concurrent cargo checks (observed an exit-137 OOM when two ran at
once). Treat per-`-p` compile as the entry gate before declaring any functional
done.
