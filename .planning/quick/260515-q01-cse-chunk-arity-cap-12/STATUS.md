# 260515-q01 — Partial landing, superseded by Phase 11 replan

**Status (2026-05-15):** Partial. Commit 1 of 3 landed; commits 2 (full regen) and 3 (STATE.md row) not made. The Phase 11 replan supersedes the open work.

## What landed

- **Commit `5c379dc25`** — three emit fixes in `tools/translate_v2/`:
  1. `cse.py` — `MAX_TUPLE_ARITY 16→12` (CubeCL 0.10 `CubeType` derive ceiling)
  2. `per_functional.py` — `_wrap_f64_literals` post-pass wraps `0.123e1`-style literals as `F::new(...)`
  3. `per_functional.py` — single-output chunks emit `-> F` (scalar) instead of `-> (F,)`

  Plus: `crates/kernels/math/tests/spike_cse_emit_q01.rs` (positive-regression tests for the chosen idioms), this directory's `BRIEF.md` and `SPIKE-FINDINGS.md`.

## Why we stopped (per the q01 executor's checkpoint)

The three emit fixes are individually correct (validated by the spike). But the regen surfaced three NEW bug families on the full kernel tree:

- **Family A** — Fix 2's regex misses named f64 constants (`M_PI`, `M_CBRT3`, …) and integer-mantissa scientific notation (`2e-21`).
- **Family B (architectural)** — `crates/kernels/math/src/` has 38 concrete-f64 helpers (`pow_1_3`, `piecewise3`, `f64::sqrt`, …), 0 generic over `<F: Float>`. Chunks emit `<F: Float>` per D-02 but call those helpers — fundamentally incompatible under CubeCL 0.10. No emit-pass surgery fixes this.
- **Family C** — `param_*` identifiers referenced in chunk bodies but not threaded as chunk args (documented existing risk in `per_functional.py:23-29`).

Family B is the architectural blocker. q01's brief carved `crates/kernels/math/src/` out as out-of-scope, so the q01 executor halted per protocol.

## Disposition

- **Commit `5c379dc25` stays landed.** Independently valid. Validated by the spike. The replan should build on it.
- **The "Quick Tasks Completed" row in STATE.md is NOT being added** yet — q01 is partial. Once the replan confirms the three fixes are kept, q01 can be marked complete with `5c379dc25` as the closing commit.
- **No `crates/kernels/` regen happened.** The kernel tree is at the 11-03 clean-slate output (`97d6347be`) — still carries 1118 wide-tuple chunks. The replan owns the regen decision.

## Where Phase 11 picks up

- Pause notice: `.planning/phases/11-splitter-v2-unified-5k-cap/.continue-here.md`
- STATE.md status: "Phase 11 PAUSED at 11-04 Task 1A — architectural blocker, replan required"
- Next command: `/gsd-plan-phase 11` (or `/gsd-discuss-phase 11` first to lock the D-02 disposition)

## Pyright warnings flagged during commit 1 (for the replan's awareness)

- `per_functional.py:43-44` — `Import "translate_v2" could not be resolved`. Pre-existing path-resolution noise (the imports work when running from the project root); cosmetic.
- `per_functional.py:426:75` — `"group" is not a known attribute of None`. Pyright Optional-strictness flag on `re_assign.match(...).group(1)` inside an `if re_assign.match(...) and ... .group(...)` guard. Runtime behavior is correct because the `and` short-circuit precedes; Pyright doesn't model the cross-call relationship. Cosmetic.
- `per_functional.py:175,190,220,343,421,500` — unused-variable warnings (`_wrapper_args`, `_ph`, `_vo`, `_bind`, `_args`, `compute`). Style noise from underscored placeholders; preserve or rename in the replan's emit-pass cleanup pass.
