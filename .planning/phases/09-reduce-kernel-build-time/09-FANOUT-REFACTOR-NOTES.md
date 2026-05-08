---
phase: 09-reduce-kernel-build-time
artifact: fanout-refactor-notes
date: 2026-05-08
git_head_at_start: a4ca30daa942fcb821a0b8b1538d2f5d8bf5f817
trigger: user-pivoted off 09-07 cargo-test loop because cargo check kept being killed by the 10-min Bash timeout under RAM pressure
reference: docs/manual/Cubecl/cubecl_macro_fanout_manual.md
---

# Phase 9 — CubeCL Macro Fan-out Refactor (Mid-09-07 Pivot)

## Why this document exists

Plan 09-07 ("oracle parity sweep at strict 1e-12") is the verification gate
for SPEC §Acceptance Criteria item 6. While running its first cargo step
the build harness on this RAM-constrained machine hit a sustained
killed-by-timeout loop:

| Run | Ended at | Reason |
|---|---|---|
| `cargo check -p libxc_rs-verify --tests` (#1) | 10 min | Bash 10-min cap; 28 lines logged |
| (#2) | 10 min | sccache ConnectionRefused (server died), 184 lines logged |
| (#3) | 10 min | sccache server shut down again, 209 lines logged |
| (#4 without sccache) | 10 min | Bash cap, 184 lines logged |

Cumulative observation: zero compile errors surfaced across the four runs
(no Heaviside / kernel-mgga-29 / Phase 4 Plan 03 issues). The bottleneck
is purely RAM pressure during CubeCL `#[cube]` proc-macro expansion of the
~3,800 generated kernel files across 168 sub-crates. The user explicitly
invoked the standing failure protocol (memory: `feedback_kernel_build_failure.md`):

> "If test will fail, please refactor and generate kernel(reduce one per
> kernel code amount) by tools following to docs/manual/Cubecl/
> cubecl_macro_fanout_manual.md."

This document captures the refactor.

## Audit baseline

Pre-refactor counts across `crates/kernel-{gga,mgga,lda}-*/`:

| Metric | Value |
|---|---|
| sub-crates | 168 |
| `.rs` files under `src/` | 3,843 |
| total bytes | 397 MB |
| `#[cube(launch_unchecked)]` declarations | 3,369 |
| plain `#[cube]` helpers | **0** |
| `#[derive(CubeType / CubeLaunch)]` derives | 0 |

The split-helper population:

| Filename pattern | Count |
|---|---|
| `<order>_<spin>_part<N>(_<suffix>)*.rs` (GGA + MGGA) | 1,396 |
| of which GGA | 395 |
| of which MGGA | 1,001 |

## Manual-rule-to-finding mapping

| Manual reference | Finding | Severity |
|---|---|---|
| §4.3 / §23 ("Use `#[cube(launch)]` only for true entry kernels.") | 3,369 launchable kernels for ~150 functionals × 5 orders × 2 spins ≈ 1,500 logical entries — every helper is launchable. | High |
| §5 ("Reduce the Number of Launch Kernels") | 1,396 split-part files emit a launch wrapper each, even though dispatch only invokes the un-split symbol. | High |
| §10 ("Avoid Too Many Tiny `#[cube]` Helpers") | Inverse of the codebase: the codebase has zero helpers — every fragment is a launch. | High (different polarity) |
| §11 (`CubeType` / `CubeLaunch` derives) | Already minimal (zero derives). | Pass |
| §13 (Element-type generic explosion) | f64-only — no per-numeric-type duplication. | Pass |

## Dispatch impact analysis

Confirmed `_partN` files are dead code from the host-launch perspective:

```bash
$ grep -rE "_part[0-9]" src/eval/{gga,mgga}_dispatch/  # → no matches
```

The MGGA dispatch additionally returns
`UnsupportedDerivativeOrder` for Fxc / Kxc / Lxc orders
(`src/eval/mgga_dispatch/mod.rs:174-182`), which is exactly the order set
where `_partN` files exist for MGGA. So all 1,001 MGGA `_partN` launch
wrappers and ~all 395 GGA `_partN` launch wrappers were never reachable
from the host.

This validates the safety of demoting them to plain `#[cube]` without
touching dispatch.

## Refactor delivered

### 1. One-shot in-tree transform

`tools/shrink_part_fanout.py` walks every
`crates/kernel-{gga,mgga,lda}-*/src/<func>/<order>_<spin>_part<N>(_<suffix>)*.rs`
file and replaces the line `#[cube(launch_unchecked)]` with `#[cube]`.

Run output:

```
Inspected 1396 _partN.rs files
Modified: 1396 files (1396 #[cube(...)] annotation lines)
  by family: gga = 395
  by family: mgga = 1001
```

### 2. Translator patches (regen-stable)

To prevent the next regen pass from re-introducing the launch-surface
explosion, `generate_function()` in each translator now selects the
attribute by `fn_suffix`:

```python
cube_attr = '#[cube]' if fn_suffix.startswith('_part') else '#[cube(launch_unchecked)]'
```

Patches:
- `tools/translate_mgga.py:639-651, 678` (split path)
- `tools/translate_gga.py:653-672, 692` (split path)
- `tools/translate_lda_v2.py:475-490, 506` (split path)

The non-split entry kernels (`generate_incremental_function` / fn_suffix==``)
remain `#[cube(launch_unchecked)]` — they are the genuine host-launch
entry points and per the manual §22 should stay launchable.

### 3. Side artifacts kept in tree

Authored before the pivot and ready for the eventual 09-07 resumption:

- `verify/tests/parity_phase09.rs` — strict 1e-12 sweep over the 25 deferred
  GGA functionals + 5-functional MGGA spot-check; per-tuple `PARITY_TUPLE:`
  log lines feed the post-processor.
- `tools/build_parity_report.py` — converts a `cargo test` log into the
  `09-07-PARITY-REPORT.md` markdown table.

## Net fanout impact

| Metric | Before | After | Δ |
|---|---|---|---|
| `#[cube(launch_unchecked)]` decls | 3,369 | 1,973 | −41 % |
| Plain `#[cube]` helpers | 0 | 1,396 | new |
| Total `#[cube*]` proc-macro invocations | 3,369 | 3,369 | unchanged |

Plain `#[cube]` skips the host-side launch wrapper generation
(per manual §4.2). Host-side launch boilerplate proc-macro work drops by
roughly the ratio of demoted decls — i.e. **about 41 % less launch
boilerplate per build**. Per-kernel IR-builder work is unchanged.

The expected outcome on the RAM-constrained 24 GB box is: each kernel
sub-crate's peak proc-macro RAM during compile decreases (because launch
wrappers contribute Type-3 proc-macro work — `KernelDefinition`,
launcher struct, host wrappers — that we no longer emit for 1,396 of the
fragments). Whether this is enough to let `cargo check --workspace` finish
inside the 10-min Bash budget is a measurement question for the next
session.

## Verification status

| Check | State |
|---|---|
| Tools pass `python3 -c "import tools.shrink_part_fanout"` import-check | n/a — single-file script, dry-run+apply both succeed |
| Translators produce equivalent output for non-split functionals | not measured — visual diff on `generate_function` shows only the cube-attribute line is conditional |
| `cargo check -p libxc-kernel-mgga-23` (single-crate) | NOT RUN — user explicitly stopped all compile |
| `cargo check -p libxc_rs-verify --tests` (full) | NOT RUN — same reason |
| Plan 09-07 parity sweep | NOT RUN — same reason |

This deliberately **does not** carry a compile-verified seal of approval.
The next session should run a single-crate cargo check on the largest
kernel sub-crate (`crates/kernel-gga-14` ≈ 3.5 MB or
`crates/kernel-mgga-28` ≈ 3.5 MB) as a smoke test before re-attempting the
full workspace check.

## Files touched

- 1,396 modified `_partN.rs` kernel files (cube-attribute swap only)
- `tools/shrink_part_fanout.py` (NEW)
- `tools/translate_mgga.py` (patched `generate_function`)
- `tools/translate_gga.py` (patched `generate_function`)
- `tools/translate_lda_v2.py` (patched `generate_function`)
- `verify/tests/parity_phase09.rs` (NEW — pending compile check)
- `tools/build_parity_report.py` (NEW — paired with parity_phase09.rs)
- `.planning/phases/09-reduce-kernel-build-time/09-FANOUT-REFACTOR-NOTES.md` (this file)

## What's next

1. **Verify the refactor compiles** — single-crate check on one of the
   largest kernel sub-crates first; if green, escalate to the workspace.
2. **Commit the refactor as a stand-alone change** — atomic, easily revertable
   if a regression surfaces. Suggested subject:
   `refactor(09): demote split-helper kernels to plain #[cube] per fanout manual`.
3. **Resume Plan 09-07** — re-run the parity sweep; if it now fits inside
   memory and time budget, capture the report and close the phase
   acceptance gate.
4. **(optional) Measure** — `cargo build --timings` before/after on a
   single kernel crate to quantify the proc-macro RAM and wall-clock
   savings, capture in a follow-on note.
