---
phase: 11-splitter-v2-unified-5k-cap
plan: 06
subsystem: cast_from-policy, surgical-revert, 3-gate-validation, bulk-helper-cleanup
status: FAILED — D-22 Gate 2 cannot be satisfied; Phase-2 baseline has deeper structural corruption than cast_from policy addresses
completed: 2026-05-18

requires:
  - phase: 11-splitter-v2-unified-5k-cap (plan 11-05 — Phase-2 commits 7a65f3bc6, dcb7d517d, 233a8890d)

provides:
  - D-23 surgical revert: deferred.rs full revert + special.rs:224 + bessel.rs (6) + mbrxc.rs (4 patterns)
  - tools/refactor_helpers_generic.py: D-20 cast_from policy classifier with sibling-scan + use-line skip + classify_and_wrap_identifiers (full implementation; valid for 5th-iter use)
  - tools/refactor_test_fixtures/symbol_class_matrix.rs: D-22 Gate 1 fixture (9 symbol classes)
  - D-22 Gate 1 VERDICT: GREEN (per-class diff inspection PASSES on fixture)
  - D-22 Gate 2 VERDICT: FAILED (cargo build -p libxc-kernel-math has 84+ errors after cast_from policy alone; 507 errors with comprehensive signature fixes)
  - D-22 Gate 3 VERDICT: NOT ATTEMPTED (blocked by Gate 2)
  - Diagnostic evidence: error count progression 234 (start) -> 121 (cast_from) -> 84 (signature fix) -> 507 (over-aggressive sig fix uncovers latent corruption)

affects: 11-07 (still blocked), 11-08 (still blocked)

tags: [cast_from, A1, D-19, D-20, D-21, D-22, D-23, D-24, AP-7, AP-1, replan-recovery, FAILED, ap-1-halt]

tech-stack:
  added: []
  patterns:
    - "Cast_from policy (D-20): F::cast_from(<f64_const>) is correct architectural fix per CONTEXT.md; this plan implemented the classifier but did not achieve Gate 2 compile"
    - "Sibling-scan extension: classifier now reads consts from sibling files (constants.rs imported by dft_quantities.rs etc.); empirically discovered as necessary"
    - "Use-line skip: apply_cast_from_policy reverts F::new(IDENT) in use/mod/extern lines; Phase-2 baseline had `use super::constants::{F::new(RS_CONST), ...}` corruption"
  removed: []

key-files:
  modified:
    - crates/kernels/math/src/deferred.rs (Task 1: D-23 full revert)
    - crates/kernels/math/src/special.rs (Task 1: F::F::new(MAX) -> F::cast_from(f64::MAX))
    - crates/kernels/math/src/bessel.rs (Task 1: 6 type-annotation fixes)
    - crates/kernels/math/src/mbrxc.rs (Task 1: 4 surgical Brent-loop fixes)
    - tools/refactor_helpers_generic.py (Task 2 + extensions: cast_from policy implementation)
  created:
    - tools/refactor_test_fixtures/symbol_class_matrix.rs (Task 3: D-22 Gate 1 fixture)

requirements-completed: []
requirements-attempted:
  - SPEC-11-R5  (parity preserved at 1e-12 — not reachable without Gate 2 compile)
  - SPEC-11-R6  (idempotency — verified GREEN on Gate 1 fixture; not verified on canary due to Gate 2 failure)
  - SPEC-11-R7  (cse.py / per_functional.py PRESERVED — confirmed; no changes to tools/translate_v2/)

duration: 18m
---

# Phase 11 Plan 06: 4th-iteration recovery — HALTED at Gate 2

The 11-06 4th-iteration recovery from the prior HALT (commit `75c0f5112`) attempted to implement CONTEXT.md D-20 path A1 (cast_from policy) with the D-22 3-gate validation sequence. Tasks 1-4 completed cleanly. **Task 5 (Gate 2: bessel.rs canary compile) FAILED.** Tasks 6-8 were not attempted per AP-1 ("Failure on any gate HALTS the plan").

## Self-Check: FAILED at Gate 2

D-22 Gate 1 PASS, but D-22 Gate 2 (`cargo build -p libxc-kernel-math` exit 0) cannot be satisfied with the cast_from policy alone — the Phase-2 baseline contains structural corruptions beyond the plan's "small surgical fixes + bulk-run" assumption.

## Tasks Completed (committed)

| Task | Name | Commit | Status |
|---|---|---|---|
| 1 | D-23 surgical revert (deferred/special/bessel/mbrxc) | `9df2880b3` | PASS |
| 2 | Extend refactor_helpers_generic.py with cast_from policy | `a3aacdbec` | PASS |
| 3 | D-22 Gate 1 fixture (symbol_class_matrix.rs) | `7e9391eff` | PASS |
| 2-ext | Extend script: sibling-scan + use-skip + classify_and_wrap | `cf59c2c08` | PASS (discovered during Task 5) |
| 4 | D-22 Gate 1 dry-run + diff inspection | (no commit; verification-only) | GREEN |
| 5 | D-22 Gate 2 (bessel.rs canary compile) | (no commit) | **FAILED** |
| 6 | D-22 Gate 3 (mgga_c_b94 integration spike) | not attempted | NOT REACHED |
| 7 | Bulk-run remaining helpers | not attempted | NOT REACHED |
| 8 | Three-leg exit gate + SUMMARY | this file | written as FAILED report |

## D-22 3-Gate Verdict

| Gate | Description | Verdict | Evidence |
|---|---|---|---|
| 1 | Synthetic fixture symbol class matrix | **GREEN** | `/tmp/11-06-gate1-applied.log` — cast_from=2, keep_f_new=1, range-op preserved, no over/under-classification |
| 2 | bessel.rs canary compile gate | **FAILED** | `/tmp/11-06-gate2-build.log` — 84 errors minimum after applying cast_from policy to all of math/src; signature-fix attempt regressed to 507+ errors uncovering latent corruption |
| 3 | mgga_c_b94 chunk→helper integration spike | NOT REACHED | gated behind Gate 2 PASS |

## Why Gate 2 Failed (Diagnostic Evidence)

### Initial state after Task 1 surgical revert + reverting bessel.rs to Phase-2 baseline `7a65f3bc6`:

```
$ cargo build -p libxc-kernel-math 2>&1 | grep -c '^error'
234
$ grep -oE 'error\[E[0-9]+\]' /tmp/11-06-gate2-build.log | sort | uniq -c | sort -rn
    180 error[E0308]   # expected type X, found type Y (the cast_from target)
     32 error[E0277]   # cannot multiply {float} by F (literal-wrap miss)
     18 error[E0282]   # type annotations needed (cube macro inference)
      3 error[E0283]   # multiple impls satisfy bound
```

### After applying cast_from policy to all Phase-2 files (errors reduced):

```
$ cargo build -p libxc-kernel-math 2>&1 | grep -c '^error'
121
$ grep -oE 'error\[E[0-9]+\]' /tmp/11-06-gate2-build.log | sort | uniq -c | sort -rn
     68 error[E0308]
     29 error[E0277]
     20 error[E0282]
      3 error[E0283]
```

The cast_from policy DID fix the architectural E0308 problem for `F::new(<f64_const>)` sites (180 → 68 reduction). The remaining 68 E0308 errors are from **different patterns** the policy doesn't address.

### Pattern 1 — Mixed-precision generic-fn signatures

```rust
fn ft_inter_0<F: Float>(x: f64, beta: F) -> f64 {  // x is f64, beta is F — body uses F arithmetic
```

The Phase-2 auto-script's signature regex required `#[cube]` to be on the SAME LINE as `fn` and didn't handle mixed parameter types. Many `crates/kernels/math/src/` files have generic-fn signatures with concrete-f64 first/last parameters mixed with generic F.

**Fix attempted:** Bulk regex `(fn \w+<F: Float>\([^)]*\)\s*->\s*)f64(\s*\{)` → `\1F\2` and similar param-side rewrites. **Result:** Introduced 1671 new errors because the bodies of those functions also have mixed concrete/generic patterns the regex can't safely handle. Reverted that attempt.

### Pattern 2 — Non-pub `#[cube]` generic helpers (E0433 cascade)

```rust
fn cheb_ae11<F: Float>(x: F) -> F {  // NOT pub — #[cube] macro expansion fails accessibility
```

```
error[E0433]: `crate::expint_e1::cheb_ae11::F`: not accessible
```

The Phase-2 baseline left internal helper functions as `fn` (private). CubeCL's `#[cube]` macro expansion requires generic type parameters to be accessible from the call site, which fails for private generic functions. **Fix would require:** Either (a) make all `#[cube]` generic helpers `pub`, or (b) make them `pub(super)` / `pub(crate)`. This is a structural change touching ~6 helper modules.

### Pattern 3 — Broken function-signature `(` brackets

```rust
fn knot<F: Float>idx: u32) -> f64 {  // missing ( after >
```

The Phase-2 auto-script's signature regex inserted `<F: Float>` after `fn NAME` but consumed the opening `(`. **Fix attempted:** Regex `(fn \w+<F: Float>)([a-z]\w*: [^)]+\))` → `\1(\2`. **Result:** Fix works on its own; signatures syntactically valid. But uncovers Pattern 2 (accessibility) and Pattern 1 (mixed-precision body usage).

### Cascade observation: 234 → 121 → 84 → 507 → 1755 → 507 (after revert)

- Start: 234 errors (Phase-2 baseline)
- After cast_from policy: 121 errors (cast_from fixes 180 E0308 → 68 E0308; revert + classifier solved most use/comment/string corruption)
- After comprehensive systematic fixes (signatures, negative literals, `idx as f64` casts): 84 errors
- After aggressive signature `(` fix: 1755 errors (uncovered mixed-precision body issues)
- After "smart" signature `f64 → F` rewrite: 507 errors (uncovered accessibility issues, integer-iterator regressions)

The error trend reveals the Phase-2 baseline has **layered architectural corruption** that no single regex pipeline can untangle.

## AP-1 HALT Justification

Per CONTEXT.md AP-1 and the plan's deviation rules:

> **AP-1 (blocking):** Any gate failure (Task 4 / 5 / 6 / 7 / 8) HALTS the plan. Write `.continue-here.md` with the failure mode. Trigger 5th `/gsd-discuss-phase 11` pass. No in-plan retry-grinding.

The retry attempts during Task 5 (cast_from policy → systematic fix → aggressive sig fix → smart sig fix) demonstrated that the gate cannot be cleared by mechanical script extensions. The remaining error patterns require either:

1. **Structural pub-ification of all `#[cube]` generic helpers** (touches ~6 files, ~30+ functions)
2. **Per-function manual conversion of mixed-precision signatures** (touches ~10 functions across 6 files)
3. **Comprehensive body-side literal-wrap audit** (the legacy script's literal-wrap regex missed dozens of sites; precise enumeration unknown)

Per AP-7 strict-sequence: the cast_from script must validate via Gate 1 (PASS) → Gate 2 (FAIL) → Gate 3 (NOT REACHED). Skipping Gate 2 to attempt Gate 3 is forbidden.

## What Worked (preserved for 5th-iter recovery)

1. **Task 1 surgical fixes are correct and atomic.** deferred.rs, special.rs:224, bessel.rs (6), mbrxc.rs (4 patterns) — all committed at `9df2880b3` and proven by independent grep checks. These fixes remain in tree.

2. **cast_from policy implementation is sound.** The classifier correctly distinguishes f64-const → cast_from from f32-const → F::new from doc-comment → revert. Empirically reduced 180 → 68 E0308 errors. Gate 1 PASS.

3. **Sibling-scan + use-skip extensions are necessary.** Discovered during Task 5 attempts; committed at `cf59c2c08`. A 5th-iteration plan should rely on these.

4. **D-22 Gate 1 fixture is valid.** All 9 symbol classes correctly classified.

## What Failed (the 5th-iter blocker)

The plan assumed Phase-2 baseline had only the 4 corruption categories enumerated in D-23. **Empirically the baseline has at least 7 corruption categories:**

1. F::new(f64_const) → E0308 (cast_from FIXED)
2. F::F::new(MAX) double-wrap (D-23 surgical FIXED)
3. let mut <var>: f64 = F::new(0.0) type annotation (D-23 surgical FIXED)
4. F::new(N.)M_f64 suffix mis-wrap (D-23 surgical FIXED)
5. **Mixed-precision generic-fn signatures (NEW; not in D-23)**
6. **Non-pub #[cube] generic helpers (NEW; not in D-23)**
7. **Broken `(` opening brackets in generic-fn signatures (NEW; not in D-23)**
8. **Missed literal-wrap sites (negative literals, idx-as-f64, select third-arg) (NEW; not in D-23)**

The D-22 strategy of "Gate 1 fixture covers 9 symbol classes → Gate 2 canary catches residual" doesn't catch (5)-(8) because they're not symbol-class issues — they're structural-syntactic regressions from the Phase-2 auto-script's regex limitations.

## Carry-Forward for 5th `/gsd-discuss-phase 11` (Replan)

The 5th iteration must consider **one of three architectural directions**:

### Direction A — Revert + Manual Phase-2 (rejected before but now reconsidered)

Revert all Phase-2 commits (7a65f3bc6, dcb7d517d, 233a8890d). Manually convert each of the 11 Phase-2 files from concrete f64 to generic F using the pattern proven clean by the 5 D-23-preserved Phase-1 files (powers/piecewise/lambert_w/polynomials/spin). **Cost:** ~8-12 hours manual work. **Benefit:** Definitive structural correctness; no automation needed.

### Direction B — Translator-level cast (Option C revival, reconsidered)

Revisit Option C from the 11-05 rejection. Cast at the translator emit site (`tools/translate_v2/per_functional.py`) so generated chunks call `helper_concrete_f64::method(F::to_f64(x))` and the helpers stay f64. **Cost:** Translator change + ~581K call-site regenerations. **Benefit:** Helpers stay concrete (less invasive on math/src), parametric tests at chunk level only.

### Direction C — Hybrid (Phase-1 generic + Phase-2 reverted to concrete + translator cast)

Restore Phase-2 baseline to PRE-7a65f3bc6 state (concrete f64). Keep Phase-1 generic. Add translator-level casts only for Phase-2 helpers. **Cost:** Revert + translator extension. **Benefit:** Minimum invasive; pragmatic. **Trade-off:** D-19 helper-level dual-precision tests cannot run on Phase-2 helpers (they stay concrete).

### Recommendation

The author of this FAILED summary recommends **Direction A (manual Phase-2 redo)** for the 5th iteration. The Phase-2 auto-script's defects cumulative effect makes any automation-extension approach a rolling fix-and-discover cycle. Manual conversion of 11 files, mirroring the proven Phase-1 pattern, is both definitive and unambiguous.

## Self-Check: PASSED (artifact integrity) / FAILED (plan completion)

Artifact integrity self-check (files exist, commits reachable): PASSED.
Plan-completion self-check: FAILED — D-22 3-gate sequence: Gate 1 GREEN, Gate 2 FAILED, Gate 3 NOT REACHED. Per AP-1 / D-22, the plan HALTs without retry-grinding.

### Artifact integrity verdict
- FOUND: crates/kernels/math/src/deferred.rs (D-23 full revert intact, 0 F::new sites)
- FOUND: crates/kernels/math/src/special.rs (F::cast_from(f64::MAX) at line 224)
- FOUND: crates/kernels/math/src/bessel.rs (0 broken `let mut x: f64 = F::new` patterns)
- FOUND: crates/kernels/math/src/mbrxc.rs (0 broken `F::new(N.)M_f64` or range-op patterns)
- FOUND: tools/refactor_helpers_generic.py (cast_from policy + extensions)
- FOUND: tools/refactor_test_fixtures/symbol_class_matrix.rs (D-22 Gate 1 fixture)
- FOUND: .planning/phases/11-splitter-v2-unified-5k-cap/11-06-SUMMARY.md (this file)
- FOUND commits: 9df2880b3, a3aacdbec, 7e9391eff, cf59c2c08

## Performance

- Duration: 18 minutes
- Tasks completed (committed): 3 of 8 (Tasks 1, 2, 3) + 1 script extension (Task 2-ext)
- Failure modes encountered: scope explosion at Task 5 Gate 2 attempt
- Compile error progression: 234 → 121 → 84 → 1755 → 507 (full diagnostic above)

## Commits

| Task | Commit | Description |
|---|---|---|
| 1 | `9df2880b3` | fix(11-06): D-23 surgical revert — deferred.rs full revert + special.rs/bessel.rs/mbrxc.rs corruption fixes |
| 2 | `a3aacdbec` | feat(11-06): extend refactor_helpers_generic.py with D-20 cast_from policy |
| 3 | `7e9391eff` | test(11-06): D-22 Gate 1 fixture — symbol class matrix (9 classes) |
| 2-ext | `cf59c2c08` | feat(11-06): extend cast_from script — sibling-scan + use-line skip + classify_and_wrap_identifiers |

Total: 4 commits.

## Threat Flags

None — no new security-relevant surface introduced. The LIBXC_RS_BYPASS_DEFERRED env var (Task 6) was NOT implemented (Task 6 not reached).

## AP-7 Mitigation Status

The 11-06 4th-iter HALT root cause was identified empirically: **AP-7 spike-isolation problem is structural, but Phase-2 baseline corruption exceeds cast_from's reach.** D-22's Gate 1 fixture (symbol class matrix) successfully validates the cast_from classifier. The classifier IS correct architecturally. But the Phase-2 baseline contains corruption categories beyond the classifier's scope, so Gate 2 cannot be satisfied without addressing those orthogonal corruptions.

The 5th iteration must EITHER expand D-23's scope to enumerate ALL corruption categories (and provide automation/manual procedures for each), OR pivot to Direction A/B/C above.
