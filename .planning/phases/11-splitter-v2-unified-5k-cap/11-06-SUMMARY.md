---
phase: 11-splitter-v2-unified-5k-cap
plan: 06
subsystem: math-helpers-syntax-cleanup, translator-confirm, three-leg-gate
status: FAILED — Task 1 leg 1 blocked by architectural mismatch between Phase 2 (11-05) automated refactor and CubeCL Float trait API
completed: 2026-05-18
tags: [serena-mcp, syntax-cleanup, three-leg-gate, D-02, D-14, D-15, D-16, D-18, ARCHITECTURAL-HALT, AP-1, AP-6]

# Dependency graph
requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 11-05)
    provides: D-02 Option A refactoring spike — 38 helpers in 16 files transformed via Phase 1 manual + Phase 2 automated script; syntax errors deferred to 11-06

provides:
  - HALT-with-evidence finding: Phase 2 automated refactor in 11-05 introduced ~508 architectural errors NOT classified in the 11-06 plan's three "syntax error" categories
  - Concrete blocker: CubeCL Float trait `fn new(val: f32) -> Self` cannot accept `f64` named constants — `F::new(SQRT_DBL_EPSILON: f64)` is a type error and there is no automated-script regex that fixes this
  - Proposed forward path (for re-discussion): use `F::cast_from(<f64 const>)` (cubecl-core `Cast` trait) instead of `F::new(<f64 const>)` to preserve f64 precision through generic helpers

affects: 11-07, 11-08

tech-stack:
  added: []
  patterns:
    - "BLOCKER: F::new(val: f32) -> Self in CubeCL Float trait cannot construct f64 from f64 named constants — auto-refactor that wrapped `F::new(SQRT_DBL_EPSILON)` etc. produces ~447 E0308 'expected f32, found f64' errors"
    - "PATH FORWARD candidate (not chosen yet — requires user decision): F::cast_from(f64_const) via cubecl-core 0.10 Cast trait at line 14 of cast.rs"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-06-SUMMARY.md (this FAILED summary)
  modified: []

key-decisions:
  - "HALT per AP-1 / D-15: gate failure halts the replan, surfaces failure mode to user, triggers a third /gsd-discuss-phase pass. No in-plan retry-grinding."

patterns-established:
  - "Per AP-6 reframed: 'structural completion is not real completion' — a 515-error compile leg is a leg-1 HARD FAIL regardless of how many of the plan's 3 named categories were addressable"

requirements-completed: []  # SPEC-11-R5, SPEC-11-R6, SPEC-11-R7 NOT completed — plan halted at Task 1 Step 5 (first per-`-p` compile check)

# Metrics
duration: 30min (investigation + halt-decision + summary writing; no commits)
completed: 2026-05-18
---

# Phase 11 Plan 06: HALT — Three-leg gate cannot proceed; Phase 2 automated refactor (11-05) was based on an invalid `F::new(val: f32)` premise

**`cargo build -p libxc-kernel-math` exits with 515 errors. ~508 of those errors are NOT in any of the plan's three named "syntax error" categories — they are a fundamental architectural mismatch between the Phase 2 automated refactor and the CubeCL Float trait API. Per AP-1/D-15, this plan HALTs and surfaces to the user.**

## Self-Check: FAILED

**Leg 1 of three-leg gate (`cargo build -p libxc-kernel-math` exit 0):** FAILED — 515 errors.

**Legs 2 and 3:** NOT ATTEMPTED — Task 1 done criterion (helper crate compiles cleanly) is unachievable, so legs 2 and 3 are blocked.

**Idempotency check:** NOT ATTEMPTED — same reason.

## Performance

- **Duration:** ~30 min (investigation + halt-decision + this SUMMARY)
- **Started:** 2026-05-18 (continuation of 11-05 work)
- **Halted at:** Task 1 Step 5 (first per-`-p` compile check)
- **Tasks completed:** 0 of 3
- **Files modified:** 0 (this SUMMARY only — no kernel/helper edits committed)

## What was investigated (and what was found)

### Step 0 — AP-2 pre-flight check: PASS

`.cargo/config.toml` is unchanged from the locked baseline. The three expected lines are present:

```
jobs = 1
target-dir = "/home/user/Documents/workspace/libxc_rs/.cache/cargo-target"
RUST_MIN_STACK = "67108864"
```

`grep -E '^(jobs = 1$|target-dir = "/home/user/Documents/workspace/libxc_rs/\.cache/cargo-target"$|RUST_MIN_STACK = "67108864"$)' .cargo/config.toml | wc -l` → `3` (exact match expected).

### Step 1 — Error inventory: plan's 3 categories vastly under-counted vs. reality

The plan's Task 1 Step 2 inventory was run as specified. Results compared with the **actual `cargo build -p libxc-kernel-math` error categorization (515 total errors)**:

| Plan category | Plan estimate | grep-inventory count | rustc error count by class |
|---|---|---|---|
| Cat 1: `F::new([A-Z_]+)` corruption (>1 char) | "~150+" | 148 grep sites | **N/A — see "Architectural finding" below** |
| Cat 2a: `for _ in 0.F::new(.500)` | 1 site | 1 site (`mbrxc.rs:154`) | 1 rustc syntax error |
| Cat 2b: `F::new(0.)0_f64` | unspecified | 1 site (`mbrxc.rs:153`) | 1 rustc syntax error |
| Cat 2b'**: `F::new(3.)0_f64` / `F::new(1.)0_f64` (related malformation, not in plan) | NOT IN PLAN | 3 sites (`mbrxc.rs:145`) | 3 rustc syntax errors |
| Cat 2c: truncated exponents | "SHORT list" | 2 sites (`erf.rs:29-30`) | N/A — `e-0` is parseable Rust |
| Cat 3: `ArrayArg::from_raw_parts::<TYPE>(_, _, 1)` | "~165" | 60 sites in src/ | gated by `#[cfg(test)]` — does not surface in lib build |
| **Cat A** (NEW — architectural): `F::new(f64_const)` mismatch | NOT IN PLAN | ~148 sites | **447 E0308 "expected f32, found f64" + 11 E0277 cascade** |
| **Cat B** (NEW): bare f64 literals added to F | NOT IN PLAN | dozens | 9 E0277 `NativeExpand<F>: From<{float}>` + 5 E0277 `cannot multiply {float} by F` + 4 related |
| **Cat C** (NEW): `let mut b2: f64 = F::new(0.0);` (broken type annotations) | NOT IN PLAN | dozens | 18 E0282 `type annotations needed` |
| **Cat D** (NEW): `F::F::new(MAX)` (double-wrap, `f64::MAX` corrupted) | NOT IN PLAN | 1 site (`special.rs:224`) | 2 E0425 `cannot find value MAX` |
| **Cat E** (NEW): `F::new(IDENT)` in `deferred.rs` where the file is NOT generic | NOT IN PLAN | 32 sites | 2 E0433 `cannot find type F` + cascade |
| **Cat F** (NEW): string-literal corruption `"... F::new(17.)5K lines ..."` | NOT IN PLAN | several | string literal is valid Rust but semantically wrong (originally `"17.5K"`) |

```
$ cargo build -p libxc-kernel-math 2>&1 | grep -E '^error' | sort | uniq -c | sort -rn
    447 error[E0308]: mismatched types               <-- the dominant blocker
     18 error[E0282]: type annotations needed
     11 error[E0277]: the trait bound `f32: From<f64>` is not satisfied
      9 error[E0277]: the trait bound `NativeExpand<F>: From<{float}>` is not satisfied
      6 error[E0308]: arguments to this function are incorrect
      5 error[E0277]: cannot multiply `{float}` by `F`
      3 error[E0283]: type annotations needed
      3 error: expected one of `)`, `,`, `.`, `?`, or an operator, found `0_f64`
      2 error[E0433]: cannot find type `F` in this scope
      2 error[E0425]: cannot find value `MAX` in this scope
      2 error[E0277]: the trait bound `NativeExpand<f64>: From<NativeExpand<F>>` is not satisfied
      2 error[E0277]: the trait bound `NativeExpand<F>: From<NativeExpand<f64>>` is not satisfied
      2 error[E0277]: cannot add `F` to `{float}`
      1 error[E0277]: the trait bound `f32: From<NativeExpand<f64>>` is not satisfied
      1 error[E0220]: associated type `F` not found for `F`
      1 error: expected one of `.`, `;`, `?`, `else`, or an operator, found `0_f64`
      1 error: could not compile `libxc-kernel-math` (lib) due to 515 previous errors
```

**Plan's 3-category model accounted for ~7 errors out of 515 (1.4%).**

## Architectural finding (Rule 4 — requires user decision)

### The premise that broke

The Phase 2 automated refactor in 11-05 (`tools/refactor_helpers_generic.py`) was built on the assumption that this rewrite is mechanical:

```rust
const SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-8;
// old:    if y < 2.0 * SQRT_DBL_EPSILON
// new:    if y < F::new(2.0) * F::new(SQRT_DBL_EPSILON)
```

The script wrapped every f64 literal AND every named identifier in `F::new(...)`. For literals like `2.0`, this works (CubeCL's Float manual §8.3 shows `F::new(comptime!(1.25f32))` as the idiom — f32-precision literals are intended). For **f64 named constants like `SQRT_DBL_EPSILON: f64`, this fails** because the CubeCL Float trait declares:

```rust
// /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cubecl-core-0.10.0/src/frontend/element/float.rs:75
pub trait Float: ... {
    fn new(val: f32) -> Self;   //  <-- val is f32, not f64
}
```

The full Float trait surface offers no `Float::new_f64` or `From<f64>` constructor on F-implementations. Passing `f64` to `F::new` is a compile-time E0308 mismatch.

The cleanup the plan calls "syntax cleanup" cannot resolve this — the auto-script's premise was wrong, not its regex. The plan's truths line:

> "F::new(IDENT) corruptions (where the auto-script wrapped a constant name like SLATEC/W/C/API/**SQRT_DBL_EPSILON** as if it were a literal) are reverted to bare identifiers"

is itself wrong: `SQRT_DBL_EPSILON` IS a real f64 constant used inside the actual `<F: Float>` generic functions (e.g., `bessel.rs:218`, `bessel.rs:243`). Reverting `F::new(SQRT_DBL_EPSILON)` → `SQRT_DBL_EPSILON` leaves bare `f64` mixed with `F` — also a type error (`F * f64` doesn't typecheck). Both directions break.

This is **exactly the architectural issue 11-05's spike was supposed to validate before bulk-applying the refactor**. The 11-05 SUMMARY noted "syntax errors deferred to 11-06" but did not surface that the underlying API contract is incompatible with f64-precision named constants — that finding is new here.

### What an actual forward path looks like (proposed; needs user decision)

CubeCL 0.10 does have an API that constructs an `F` from an arbitrary `CubePrimitive` (including `f64`):

```rust
// /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cubecl-core-0.10.0/src/frontend/element/cast.rs:14-15
pub trait Cast: CubePrimitive {
    fn cast_from<From: CubePrimitive>(value: From) -> Self;
    // ...
}
impl<P: CubePrimitive> Cast for P { ... }   // blanket impl
```

So the refactor pattern that **could** preserve f64 precision is:

```rust
// before (broken — E0308):  F::new(SQRT_DBL_EPSILON)
// after  (preserves f64):   F::cast_from(SQRT_DBL_EPSILON)
```

`F::cast_from(<f64 const>)` is the idiomatic CubeCL way to convert a non-F primitive into an F runtime value while letting CubeCL emit the appropriate cast IR. When F=f64, this is identity (no precision loss). When F=f32, this is a narrowing cast (precision-only — per D-03, f32 is not oracle-gated).

**This is NOT in the plan and is NOT what the auto-script did.** The fix would require:

1. **Re-run a smarter script** that distinguishes `F::new(IDENT)` where IDENT is a defined f64 const (→ `F::cast_from(IDENT)`) vs IDENT is doc-comment text (→ bare `IDENT`) vs IDENT is something else (case-by-case).
2. **Inline f32-precision literals where possible** — many constants in helpers (like `2.5641895835477562869480794515` in `special.rs:207`) ARE used as Chebyshev coefficients that exceed f32 mantissa precision; these need `F::cast_from(f64 const)` not `F::new(f32 literal)`.
3. **Fix `deferred.rs`** which is NOT a generic-over-F file at all — `pub fn is_deferred(id: u16) -> bool` — the auto-script never should have run on it. All `F::new(...)` wraps and string-literal corruption in `deferred.rs` need to be reverted.
4. **Fix non-script regressions** like the `let mut b2: f64 = F::new(0.0)` type-annotation corruption (the auto-script preserved `f64` annotations while changing the RHS to `F`).

This is a substantially different scope than 11-06's "Serena MCP syntax cleanup" framing. It needs Phase-level planning, not in-plan execution.

## Why HALT (per AP-1, D-15, project memory)

Per `.continue-here.md` / CONTEXT.md decision D-15:

> "Gate failure halts the replan, writes `.continue-here.md` documenting the failure mode, triggers a third `/gsd-discuss-phase 11` pass. No in-plan retry-grinding — that was the failure mode of 11-04 pre-pause."

Per project memory `project_phase11_structural_without_compile.md`:

> "Phase 11 declared structural completion 3 times without compile gates ... make per-`-p` cargo compile a phase ENTRY gate, not an exit gate"

This is the entry gate working as designed. It found that the prior plan (11-05) declared completion without a per-`-p` compile gate, the deferred compile gate fails at 515 errors, and the failure mode is architectural (not "syntax cleanup"). Continuing in-plan would be exactly the failure mode AP-1 forbids.

Per project critical constraint #4 in the executor's brief:

> "STRUCTURAL COMPLETION IS NOT REAL COMPLETION ... If any leg fails: STOP, do NOT commit a 'complete' SUMMARY.md, report the failure with the exact compiler/test output, and ask for direction."

Direction is needed at the architectural level (Option A is salvageable via `F::cast_from`, or it isn't — and if not, re-visit Option C).

## What was NOT done (to avoid muddling the picture)

In the interest of producing a clean failure report rather than a partial-fix-with-residual-blockers:

- **NO Serena MCP transformations were applied.** Serena was never started; the failure was diagnosed by direct rustc output before any edits.
- **NO `tools/cleanup_helper_syntax.py` was written.** The plan's fallback script path is not reached.
- **NO edits to `crates/kernels/math/src/*.rs`.** All 14 listed files are unchanged. `git status` is clean except for this SUMMARY.
- **NO edits to `tools/translate_v2/{cse,per_functional}.py`.** D-16 confirmation was not exercised because Task 1 didn't reach it.
- **NO env-var bypass at `MggaFunctional::from_id`.** Task 3 was not started.
- **NO commits to other-than-this-SUMMARY paths.** Per `git commit --only -- <path>` discipline.

This is intentional — a halt at the entry gate, with the architectural finding surfaced, is more useful than 4-8 hours of partial cleanup that still leaves the compile broken.

## Three-leg gate verdict

| Leg | Command | Status | Evidence |
|---|---|---|---|
| 1 | `cargo build -p libxc-kernel-math` | **FAIL** | 515 errors; primary class is E0308 `expected f32, found f64` from `F::new(f64_const)` |
| 1' (mgga_c_b94) | `cargo build -p libxc-kernel-mgga_c_b94` | NOT ATTEMPTED | Depends on helper crate compiling |
| 2 | `cargo build -p libxc_rs` | NOT ATTEMPTED | Depends on leg 1' |
| 3a (smoke) | `LIBXC_RS_BYPASS_DEFERRED=1 cargo test --test parity_phase11 phase11_smoke` | NOT ATTEMPTED | Depends on leg 2 |
| 3b (worst-case) | `LIBXC_RS_BYPASS_DEFERRED=1 cargo test --test parity_phase11 phase11_worst_case` | NOT ATTEMPTED | Depends on leg 2 |
| 4 (idempotency) | `python3 tools/maple_to_kernels.py translate --family mgga --func mgga_c_b94 && git status` | NOT ATTEMPTED | Depends on the rest |

## Files Created/Modified

- `.planning/phases/11-splitter-v2-unified-5k-cap/11-06-SUMMARY.md` — this FAILED summary.

(No source code or tooling files were modified — see "What was NOT done" above.)

## Decisions Made

- **HALT per AP-1 / D-15.** The plan's premise (3 fixable syntax-error categories) is incomplete; 99% of compile errors are out-of-scope for the plan as written. Continuing would be in-plan retry-grinding against the architectural mismatch — explicitly forbidden by AP-1.

## Deviations from Plan

### Architectural-level deviation (Rule 4 — STOPPED rather than auto-fix)

**1. [Rule 4 - Architectural] `F::new(val: f32)` API contract incompatible with auto-script's `F::new(f64_const)` wraps**

- **Found during:** Task 1 Step 5 (first per-`-p` compile check)
- **Issue:** The Phase 2 auto-script (`tools/refactor_helpers_generic.py`, run in 11-05) wrapped every f64 literal AND every named identifier in `F::new(...)`. For literals this works (CubeCL idiom is f32-precision constants per Float manual §8.3). For f64 named constants (`SQRT_DBL_EPSILON`, `LOG_DBL_MAX`, `TWO_DBL_MIN`, `TWO_SQRT2_SQRT_DBL_EPSILON`, `RS_CONST`, `KF_CONST`, `ERX`, `PI_TWO_THIRDS`, `POW_32PI_TWO_THIRDS`, and ~10 others) it fails: `F::new(f64) -> Self` produces 447 E0308 "expected f32, found f64" errors.
- **Why not auto-fix:** The fix is not a regex pattern. Each `F::new(IDENT)` site needs case-by-case classification: doc-comment (revert), defined-f64-const (rewrite to `F::cast_from(IDENT)`), defined-f32-const (keep `F::new`), or non-generic-file (revert entirely). This is architectural — equivalent to re-running the spike with a corrected wrapping policy. Per AP-1: no in-plan retry-grinding.
- **Proposed forward path:** Re-discuss in `/gsd-discuss-phase 11` whether to (a) extend `tools/refactor_helpers_generic.py` with the `F::cast_from` policy and re-run on the 11 auto-pass files, OR (b) reconsider Option C (cast at call-site in translator, leaving helpers as f64).
- **Files NOT modified (to avoid muddling):** all 14 helper files listed in plan, all `tools/translate_v2/` files, `src/model/mgga_functional.rs`, `verify/tests/parity_phase11.rs`.

**Total deviations:** 1 architectural-level halt (Rule 4). No auto-fixes attempted — all fixes are downstream of the architectural decision.

## Issues Encountered

The plan as written cannot achieve its done-criteria because the issue space is ~70× larger and qualitatively different from what the plan classifies. Specifically:

- **Plan's truths line on `F::new(IDENT)` reversion is itself wrong** — reverting `F::new(SQRT_DBL_EPSILON)` to bare `SQRT_DBL_EPSILON` leaves an f64 in a context expecting F. Both `F::new(f64)` and bare `f64` fail to typecheck in an `<F: Float>` body. The correct fix needs `F::cast_from(SQRT_DBL_EPSILON)`, which is NOT in the plan.

- **`deferred.rs` is not a generic-over-F file.** It's a registry module with `pub fn is_deferred(id: u16) -> bool`. The auto-script in 11-05 corrupted it anyway (`F::new(DEFERRED_LDA_FUNCTIONALS).iter()`, `"kxc_pol (F::new(17.)5K lines..."` in string literals, `F::new(LDA)`/`F::new(MGGA)`/`F::new(ID)` in doc-comments). This is a Phase 2 scope leak that the 11-06 plan did not anticipate.

- **`special.rs:224` has `result = F::F::new(MAX);`** — the auto-script double-wrapped `f64::MAX` into a nonsensical token. This is one of 2 E0425 errors.

- **`bessel.rs` has 6 `let mut b2: f64 = F::new(0.0);`** — broken type annotation; the auto-script changed the RHS but not the LHS.

- **`mbrxc.rs:145` has `F::new(3.)0_f64`** (3 occurrences) — the auto-script's regex misfired on `3.0_f64` literal suffix, turning `3.0_f64` into `F::new(3.)0_f64`. This is similar to the plan's Cat 2a/2b but not the same pattern.

These are not "syntax cleanup" — they are systematic auto-refactoring breakage with diverse failure modes.

## Next Phase Readiness

**NOT READY for 11-07 regen** under the current Option A path. Helper crate does not compile (515 errors).

### Recommended path forward (for `/gsd-discuss-phase 11` re-planning, 3rd iteration)

1. **Architectural decision (Rule 4):** User picks between:
   - **A1 (Option A retry with corrected wrapping):** Extend `tools/refactor_helpers_generic.py` with the `F::cast_from(<f64 const>)` rule. Re-run on the 11 auto-pass files. Manually inspect & fix non-script regressions (`deferred.rs`, `special.rs:224`, `bessel.rs` type-annotations, `mbrxc.rs:145` literal-suffix bug).
   - **A2 (Option A retry with f32 helper constants):** Demote all helper-module named constants from f64 to f32. Loses oracle precision in the helper math layer — likely violates CLAUDE.md 1e-12 oracle gate.
   - **C (Option C revival):** Leave helpers as f64, add cast-at-call-site in translator emit. Re-locks D-02 from Option A back to Option C.
   - **Hybrid:** Keep Phase 1 manual files generic (they have no f64 named constants), revert the 11 Phase 2 auto-pass files to f64, apply Option C at translator level only for the post-Phase-1 helpers.

2. **Once architectural decision is locked:** Re-write 11-06 with the corrected wrapping policy + a real cleanup script + the existing three-leg gate.

3. **Per AP-6 (reframed under AP-1):** The new 11-06 MUST include `cargo build -p libxc-kernel-math` as an entry-gate, executed BEFORE proceeding to 11-07. The current 11-06 has this gate, but the prior plan (11-05) did not — and that is the root cause of the current halt.

### Files state at halt

- `git status` is clean except for `11-06-SUMMARY.md` (this file).
- `.cargo/config.toml` is unchanged from baseline (AP-2 confirmed via pre-flight).
- All 14 helper files listed in plan are unchanged from their 11-05 state.
- No translator changes, no source-model changes.

---
*Phase: 11-splitter-v2-unified-5k-cap*
*Plan: 06*
*Status: FAILED (halted at Task 1 Step 5 per AP-1/D-15)*
*Halted: 2026-05-18*
