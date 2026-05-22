---
phase: 11-splitter-v2-unified-5k-cap
plan: 14
subsystem: infra
tags: [g6, cubecl-010, launch-abi, umbrella, dispatch-glue, migration, per-p-entry-gate, deny-warnings]

requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 09)
    provides: "the PASSING verify-canary g1 test that hand-writes the correct cubecl-0.10 launch ABI (source of truth) + the von Weizsäcker τ-clamp wiring to preserve through regen"
provides:
  - "libxc_rs umbrella lib compiles clean under cubecl 0.10: cargo check -p libxc_rs --lib EXIT 0 (3031 → 0 errors), jobs=1, peak RSS ~536 MB"
  - "GGA+MGGA dispatch generators migrated to the 0.10 launch ABI (durable — regen preserves it); LDA dispatch.rs + launch.rs hand-migrated"
  - "dispatch_gga gained the params: &dyn FunctionalParams arg, making it consistent with dispatch_mgga/dispatch_lda and the callers"
  - "G-2 / plan 11-12 (full-649 f32 oracle) is now UNBLOCKED — the umbrella compiles"
affects: [11-12, 11-13, cubecl-migration, dispatch, compat]

tech-stack:
  added: []
  patterns:
    - "cubecl-0.10 launch ABI: ArrayArg::from_raw_parts(handle, len) 2-arg handle-by-value; bare scalars (no ScalarArg wrapper); launch_unchecked returns () (no Result-chain)"
    - "Ctx holds &Handle → .clone() the reference to satisfy 0.10's by-value handle (cheap Arc bump; original survives for read_output_buffer)"
    - "Generated-file fixes live in the GENERATOR template so a regen preserves them (mirrors 11-09)"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-14-MIGRATION-LOG.md
  modified:
    - tools/generate_gga_dispatch.py
    - tools/generate_mgga_dispatch.py
    - src/eval/gga_dispatch/mod.rs
    - src/eval/mgga_dispatch/mod.rs
    - src/eval/dispatch.rs
    - src/kernel/launch.rs
    - src/compat/raw_handle.rs

key-decisions:
  - "ABI lock-in (Task 1 checkpoint): adopt the canary-and-crate-source-confirmed 0.10 forms (from_raw_parts 2-arg; bare scalars; launch returns (); drop ScalarArg import) — user-selected"
  - "Handle ownership: Ctx structs hold &Handle but 0.10 from_raw_parts takes owned Handle → .clone() through the reference (not in the plan's transform list; discovered during migration)"
  - "dispatch_gga params arg: added in the GENERATOR (mirrors dispatch_mgga) rather than removing the arg from callers — fixes 3 pre-existing E0061 WITHOUT touching out-of-scope mix.rs/evaluate.rs"
  - "DEVIATION: 4 of the 3031 errors were NOT launch-ABI (3 dispatch_gga signature + 1 read_one Result drift); all fixed in 11-14's own files"
  - "DEVIATION (out-of-scope file): as_initialized_mut dead_code under deny(warnings) — pre-existing, newly exposed; #[allow(dead_code)] (non-destructive) rather than HALT; flagged for compat-layer review"

patterns-established:
  - "cubecl-0.9→0.10 launch-ABI migration recipe (arrays/scalars/launch-return/import) verified against a passing canary + installed crate source before bulk edit"
  - "Per-`-p` umbrella cargo check as the ENTRY gate (memory project_phase11_structural_without_compile) — surfaced not just launch-ABI but signature + dead_code drift the partial 3031 count masked"

requirements-completed: []

duration: ~2h (incl. 3 user-driven jobs=1 gate runs)
completed: 2026-05-22
---

# Phase 11 / Plan 14: cubecl-0.10 umbrella launch-ABI migration (G-6) Summary

**The `libxc_rs` umbrella lib now compiles clean under cubecl 0.10 — `cargo check -p libxc_rs --lib` EXIT 0, 3031 → 0 errors — unblocking G-2 / plan 11-12.**

## Performance

- **Duration:** ~2h wall-clock (migration + 3 user-driven `--jobs 1` gate runs)
- **Completed:** 2026-05-22
- **Tasks:** 3/3 (Task 1 ABI checkpoint, Task 2 migrate+regen, Task 3 entry gate)
- **Files modified:** 137 (5 source + 132 regenerated) + later fixes
- **Peak RSS at gate:** ~536 MB (jobs=1; runs #2/#3 — run #1 was ~30 GB building all 281 kernel deps cold)

## Accomplishments

- **Migrated the launch ABI** across GGA+MGGA dispatch generators + hand-written `dispatch.rs`/`launch.rs`: `from_raw_parts(handle, len)` 2-arg with `.clone()` (Ctx holds `&Handle`, 0.10 takes owned); bare scalars (no `ScalarArg`); `launch_unchecked` returns `()` (dropped `.map_err(..)?`); deleted `ScalarArg` imports. **Durable** — the 0.10 forms live in the generators; the regenerated 132 files inherit them.
- **Preserved the 11-09 von Weizsäcker τ-clamp** verbatim through the MGGA regen (`tau_von_weizsacker` present in generator + emitted `mod.rs`).
- **Drove 3031 → 0** across three jobs=1 gate runs, fixing residuals the partial 3031-count had masked (all in 11-14's own files):
  - `read_output_buffer` (launch.rs): `client.read_one()` returns `Result` in 0.10 → `.expect(...)` (matches the canary). [E0308]
  - `dispatch_gga`: added the `params: &dyn FunctionalParams` arg in the generator (mirrors `dispatch_mgga`/`dispatch_lda`); fixed 3 caller E0061 without touching out-of-scope `mix.rs`/`evaluate.rs`.
  - 3 `map_*_launch_err` helpers orphaned by dropping the Result-chains → removed (+ unused `LaunchError` imports), in the generators + `dispatch.rs`. [dead_code under `#![deny(warnings)]`]
  - `as_initialized_mut` (compat) pre-existing dead_code, newly exposed → `#[allow(dead_code)]` (out-of-scope deviation, flagged).

## Deviations

1. **Handle `.clone()`** — not in the plan's transform list; required because the Ctx structs hold `&Handle` while 0.10 `from_raw_parts` takes an owned `Handle`.
2. **4 of 3031 not launch-ABI** — the plan assumed all 3031 were launch-ABI; 4 were a `read_one` Result drift + a `dispatch_gga` signature gap. Fixed in-scope.
3. **`as_initialized_mut` (out-of-scope file)** — `src/compat/raw_handle.rs` is not in `files_modified`; a one-line `#[allow(dead_code)]` was needed to clear `deny(warnings)`. Non-destructive; preserves intended compat API. Flagged for compat-layer review (is a mutable C op missing its wiring?).

## Self-Check: PASSED

- `cargo check -p libxc_rs --lib --jobs 1` → **exit 0**, `error[` count **0** (`/tmp/11-14-fix2-check.log`).
- Source-level: 0 stale `from_raw_parts::<f64>` / `ScalarArg` / `map_*_launch_err` across generators + emitted + hand-written; `tau_von_weizsacker` preserved; `dispatch_gga` 6-arg.
- `.cargo/config.toml` untouched; no monolithic `cargo build`; `crates/kernels/*` untouched.

## Follow-ups (not G-6 scope)

- **`crates/kernels/math/src/{piecewise,powers,polynomials,erf,dft_quantities}.rs`** carry the same 0.9 launch+readback drift in their `#[cfg(test)] mod tests` host drivers — test-gated, so the `--lib` gate doesn't reach them, but `cargo test` / the 11-10 sweep / 11-12 oracle will. Need the same migration there.
- **`as_initialized_mut`** — confirm whether a mutable C entry point should be calling it (else it stays `#[allow(dead_code)]`).

## Unblocks

- **G-2 / plan 11-12** (full-649 f32 oracle) — the umbrella now compiles, the precondition for the oracle path.
- Plan 11-13 (G-5 closure) still also depends on 11-10 (G-3 sweep) + 11-12.
