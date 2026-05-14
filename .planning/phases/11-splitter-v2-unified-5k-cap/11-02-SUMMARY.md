---
phase: 11-splitter-v2-unified-5k-cap
plan: 02
subsystem: tooling (Maple→Rust kernel codegen)
tags: [splitter-v2, cse, codegen, per-functional-subcrates]
requires: [P11-INV-1, P11-INV-A1]
provides: [P11-INV-2, P11-INV-5, P11-INV-6, P11-INV-A1]
affects: [tools/translate_v2/, tools/translate_lda_v2.py, tools/translate_gga.py, tools/translate_mgga.py, tools/maple_to_kernels.py, tools/audit_subcrate_collapse.sh]
tech-stack:
  added: []
  patterns: [cse-compute-line-partitioning, per-functional-subcrate-emitter, family-adapter-orchestration]
key-files:
  created:
    - tools/translate_v2/__init__.py
    - tools/translate_v2/cse.py
    - tools/translate_v2/emit.py
    - tools/translate_v2/per_functional.py
  modified:
    - tools/translate_lda_v2.py
    - tools/translate_gga.py
    - tools/translate_mgga.py
    - tools/maple_to_kernels.py
    - tools/audit_subcrate_collapse.sh
key-decisions:
  - "CSE pass operates on the C compute_lines list (RESEARCH Option C) — no Maple AST, no Rust syn pass."
  - "build_dependency_graph/transitive_deps COPIED verbatim into cse.py, not imported — the translators import cse.py at top level, so importing back would be a circular import."
  - "Shared per_functional.py orchestrator with a FamilyAdapter instead of 3 hand-copied emit_per_functional bodies — cuts duplication risk."
  - "maple_to_kernels.py translate drives the translators DIRECTLY in-process (emit_per_functional) — regen_phase09.py is bypassed (stale + clean-slate-incompatible); user-approved Option A."
requirements-completed: [P11-INV-2, P11-INV-5, P11-INV-6, P11-INV-A1]
duration: ~3h
completed: 2026-05-14
---

# Phase 11 Plan 02: Splitter v2 Tooling Summary

CSE-aware compute-line partitioner + per-functional-subcrate nested-by-output
emitter + three translators rewired to a 4500-line `SPLIT_THRESHOLD`, with
`maple_to_kernels.py` rewired to drive the translators directly. Pure `tools/`
work — no kernel tree mutated, no cargo run (that is plan 11-03).

**3 tasks, 9 files (4 created, 5 modified), all selftests + acceptance criteria green.**

## Task 1 — CSE pass (`tools/translate_v2/cse.py`, 386 lines)

`partition_compute_lines()` walks the C `compute_lines` list (RESEARCH **Option C**
confirmed) and cuts it into deterministic ≤4500-line chunks with planned D-02
tuple in/out signatures.

- Knob values used (RESEARCH "CSE Detection Heuristic"): `CHUNK_MAX_LINES = 4500`,
  `MIN_REVERSE_DEPS = 5`, `MIN_CHAIN_LENGTH = 50`, `MAX_TUPLE_ARITY = 16`.
- `build_dependency_graph` + `transitive_deps` were **copied verbatim** (with a
  provenance comment), not imported. Reason: the three translators import
  `translate_v2.cse` at module top level; `cse.py` importing back from
  `translate_lda_v2` would be a circular import that fails during partial module
  init. The plan explicitly sanctioned this fallback.
- Bool intermediates (`tN = tM <= thresh`) are detected by a top-level comparison
  operator with no ternary `?`, and forced to stay whole within one chunk (a bool
  cannot pass through an `F` tuple). `--selftest` asserts bool-no-cross.
- Chunk ids are strict 0-based sequence indices; tuple inputs sorted by first-use
  line. `--selftest` asserts double-call determinism + cap respected. **Exits 0.**

## Task 2 — emitter (`tools/translate_v2/emit.py`, 262 lines)

Owns the filesystem layout for the D-10 per-functional-subcrate target with the
D-04 nested-by-output `src` tree (q02 `mgga_c_b94` golden spec):
`subcrate_dir`, `emit_cargo_toml`, `emit_lib_rs`, `emit_single_output`,
`emit_chunked_output`. Single `_write` helper = the D-LOCK-D idempotency surface
(one trailing newline, no per-line trailing whitespace).

- `--selftest` emits a synthetic functional into a tempdir, asserts the q02
  layout + strings, **emits a second time and diffs — byte-identical. Exits 0.**

## Task 3 — wire-up, driver, audit

**Translators** (`translate_lda_v2.py` / `translate_gga.py` / `translate_mgga.py`):
- `SPLIT_THRESHOLD` 6000 → **4500** + D-LOCK-B comment in each.
- Import `translate_v2.{cse,emit,per_functional}`.
- New `emit_per_functional(c_file, func_name, family, is_vxc_only, split_threshold)`
  in each — builds a `FamilyAdapter` and delegates to
  `per_functional.emit_functional`. Reuses each translator's own primitives
  (`parse_body`/`parse_function_body`, `generate_function`, `split_by_output_array`,
  `merge_small_splits`, `build_dependency_graph`, `transitive_deps`,
  `translate_line`/`translate_expr`).

**`tools/translate_v2/per_functional.py`** (NEW, 410 lines — additive package
module): the family-agnostic orchestrator. `emit_functional()` runs the
(level, spin) loop, the single-file-vs-split decision, the per-output-component
sub-split, the **CSE hook** (a lone over-cap single output → `partition_compute_lines`
→ D-02 chunks), and builds the `<output>/mod.rs` wrapper-of-parts. The
`FamilyAdapter` normalises the small per-translator signature differences (LDA
`build_dependency_graph` 2-tuple/no-`is_pol`; GGA/GGA output_writes are tuples
vs LDA `OutputWrite` objects; `LEVEL_ORD` vs `LEVEL_ORDER`). `--selftest` covers
the single-file and nested-by-output layouts. **Exits 0.**

**`maple_to_kernels.py`**: `translate` now drives the translators **directly
in-process** (`discover_maple_sources` + `translate_family` →
`mod.emit_per_functional`), iterating Maple sources under
`libxc-master/src/maple2c/{family}_{exc,vxc}/`. `DEFAULT_SPLIT_THRESHOLD`
100000 → **4500**; `DEFAULT_TARGET_MAX` marked obsolete (the `split` subcommand
is retired in 11-06). `--dry-run translate --family lda` discovers 43 sources
and exits clean.

**`audit_subcrate_collapse.sh`**: added the family-level-crate-absence check;
now reports BOTH failure classes (numbered subcrates AND family crates) before
`exit 1`. Against the current tree it correctly exits 1 (27 numbered + 6 family
artifacts still present — 11-03 deletes them).

## Deviations from Plan

### [Rule 1 — stale plan interface] Old LDA chunked-scratch path does not exist
The plan's `<interfaces>` block (and Task 3 step 4) said to DELETE
`chunk_single_output_split` / `_generate_chunk_helper` / `_generate_chunked_wrapper`
/ `_build_scratch_replacer` from `translate_lda_v2.py` L571-702. `grep` confirms
**none of these functions exist** — the q03 wrong-ABI chunker WIP was stashed and
never committed (see 11-01-SUMMARY D4). The plan's L571-702 region is actually
`detect_shared_preamble` / `detect_incremental_deltas` / `translate_file`. No-op;
the acceptance criterion (`grep -c == 0`) is trivially satisfied. All Task-3 line
numbers in the plan were navigated by content, not line, as a result.

### [Rule 4 — architectural, user-approved] regen_phase09.py pipeline gap
Plan 11-03 STEP 2 routes `maple_to_kernels.py translate` → `regen_phase09.py` to
produce the per-functional tree. `regen_phase09.py` **cannot do this** and is in
no plan's `files_modified`: (1) it scans the pre-q07 `crates/kernel-*` path
prefix (already stale), (2) it does *in-place replacement of pre-existing*
functional dirs — but 11-03 STEP 1 deletes everything first, so it would
discover nothing. Surfaced to the user; **Option A approved**: rewire
`maple_to_kernels.py` to drive the translators directly. `regen_phase09.py` is
now dead code (cleanup deferred to 11-06).

### [additive] tools/translate_v2/per_functional.py not in plan files_modified
Added a third `translate_v2` package module to host the shared emission
orchestration. Strictly additive, selftested; chosen over 3 hand-copied
`emit_per_functional` bodies to cut duplication risk.

## Issues Encountered / Risk Flagged for 11-03

**D-02 CSE-chunk ABI is not fully spike-validated.** The Wave-0 spike proved only
`#[cube] fn f<F: Float>(x: F, y: F) -> (F, F)`. Real translator output for an
over-cap single output references *ambient* identifiers — `rho0`/`rho1`/`sigma*`/
`lapl*`/`tau*` (pol loads), `rho[ip]`-style indexing (unpol), `param_*`, and the
`f64` `dens_threshold`/`zeta_threshold`. `emit_cse_chunked_output` threads ambient
identifiers through as explicit chunk `F` args as a **best-effort** design, but
the mixed-`F`/`f64` and array-indexing cases are NOT spike-validated. If 11-03's
`audit_kernel_size.py --strict` trips or a CSE-chunked subcrate fails to compile,
that is the documented **11-02 ↔ 11-03 retune loop** (11-03 STEP-3 CHECKPOINT),
not a silent failure. Most functionals will not hit the CSE path at all (it
triggers only for a single output component still >4500 lines after the
per-output cut).

## Verification

| Check | Result |
|-------|--------|
| `cse.py --selftest` | exit 0 |
| `emit.py --selftest` | exit 0 (double-emit byte-identical) |
| `per_functional.py --selftest` | exit 0 |
| 3 translators import cleanly | PASS |
| `SPLIT_THRESHOLD = 4500` ×3 | 1 each |
| `audit_subcrate_collapse.sh` vs current tree | exit 1, both failure classes reported |
| `DEFAULT_SPLIT_THRESHOLD = 4500` in driver | 1 |
| forbidden env (`CARGO_BUILD_JOBS`/`RUST_MIN_STACK`/`--jobs`) in modified files | none |

No cargo build/test in this plan (pure tooling). First cargo verification is 11-03.

## Next

Ready for **11-03** — D-10a clean-slate restructure: delete the 27 numbered
subcrates + 3 family crates, run `maple_to_kernels.py translate --family all`
(now wired to splitter v2) to regenerate ~264 per-functional subcrates, rewrite
root `Cargo.toml`, regenerate the dispatch tree. 11-03's STEP-3 CHECKPOINT is the
proving gate for the D-02 CSE-chunk ABI flagged above.
