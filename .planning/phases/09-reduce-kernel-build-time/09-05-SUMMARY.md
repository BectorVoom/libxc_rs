---
phase: 09-reduce-kernel-build-time
plan: 05
subsystem: build-optimization
tags: [audit, deferred-gga, build-optimization, kernel-coverage, script-driven]

requires:
  - phase: 09-reduce-kernel-build-time
    provides: "Translator threshold raised 5K→18K + 212 single-sub-crate kernels regenerated (09-04); 35 multi-sub-crate functionals SKIPPED for D-09 preservation but pre-existing layout already complies with the cap."
provides:
  - "tools/audit_deferred_gga.py — reproducible script-driven audit for the 25 deferred GGA functionals (CONTEXT D-12)"
  - "tools/test_audit_deferred_gga.py — 7 unit tests covering canonical-list resolution + 5 gap-detection behaviors + multi-sub-crate aggregation"
  - "Per-functional coverage report: all 25 deferred GGAs at status OK across exc/vxc/fxc/kxc/lxc × {pol, unpol}"
  - "Forward-guard re-verified: 0 commented `// pub mod` entries, 0 `#[cfg(feature = \"order-*\")]` attributes anywhere under crates/kernel-gga*/src/"
  - "SPEC Req 1, Req 2, Req 3 all hold post-09-04 with no further mod.rs/lib.rs edits required"
affects: [09-06, 09-07]

tech-stack:
  added: []
  patterns:
    - "Script-driven audit: walks crates/kernel-gga*/src/<functional>/ with file-name regex classification, mod.rs `pub mod` aggregation, and lib.rs export check"
    - "Canonical list calibrated by maple2c-source LXC polarized-block body line count (>= 6,000 maple2c lines = canonical 25)"
    - "Multi-sub-crate aggregation: UNION of (level, spin) tuples observed across every sub-crate that owns a directory for the functional"

key-files:
  created:
    - "tools/audit_deferred_gga.py"
    - "tools/test_audit_deferred_gga.py"
    - ".planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md"
    - "log/09-05-deferred-gga-audit.json"
    - "log/09-05-task1-tests.log"
    - "log/09-05-audit-pass1.log"
    - "log/09-05-task2-audit-final.log"
    - "log/09-05-commented-pubmod-check.log"
    - "log/09-05-order-feature-gate-check.log"
    - "log/09-05-all-orders-feature-gate-check.log"
    - "log/09-05-cap-forward-guard.log"
    - "log/09-05-cap-violations.log"
    - "log/09-05-profile-check.log"
  modified:
    - ".gitignore (added __pycache__/, *.pyc; correctness fix per Rule 2)"

key-decisions:
  - "Calibrate canonical-list threshold at 6,000 maple2c LXC-polarized body lines — exactly reproduces the 25 RESEARCH.md-named deferred GGAs"
  - "Use UNION semantics across sub-crates for coverage aggregation — matches the live multi-sub-crate distribution pattern (e.g., gga_c_ft97 across kernel-gga-1a/-1b/-1c/-1d)"
  - "No mod.rs/lib.rs edits required — Plan 09-04's regen + the pre-existing multi-sub-crate distribution already satisfy SPEC Req 1 boundaries"

patterns-established:
  - "Script-driven phase audit pattern: tests-first → CLI script with --strict gate → JSON+markdown reports under log/ and .planning/phases/<phase>/"
  - "Canonical-list calibration via maple2c source measurement: parse `#ifndef XC_DONT_COMPILE_LXC` polarized block to derive deferred-set membership without running the translator"
  - "UNION-based coverage aggregation across letter-suffix sub-crates honors D-09 (sub-crate boundaries are stable; coverage is sliced across them, not consolidated)"

requirements-completed: [SPEC-09-R1]

duration: ~60min
completed: 2026-04-29
---

# Phase 9 Plan 05: Deferred-GGA Coverage Audit Summary

**Script-driven audit (tools/audit_deferred_gga.py) confirms all 25 historically-deferred GGA functionals have full derivative-order coverage post-09-04, with zero forbidden `// pub mod` comments and zero `#[cfg(feature = "order-*")]` attributes — SPEC Req 1 satisfied without any mod.rs/lib.rs edits.**

## Performance

- **Duration:** ~60 min (single executor in parallel worktree)
- **Started:** 2026-04-29T~22:00Z (worktree base reset + plan kickoff)
- **Completed:** 2026-04-29T~13:02Z (UTC; current locale clock)
- **Tasks:** 2 (Task 1 audit script TDD + Task 2 audit run + acceptance verification)
- **Files modified:** 3 created tools (audit + test + .gitignore tweak), 13 audit/log artifacts, 1 markdown audit report

## Accomplishments

- **Audit tool authored**: `tools/audit_deferred_gga.py` (538 lines) with `load_canonical_list()`, `audit_functional()`, and `main()` entry points; --json-out, --md-out, --strict, --repo-root CLI flags.
- **Test suite (TDD)**: `tools/test_audit_deferred_gga.py` covers all 7 behaviors specified in the plan — canonical-list resolution, happy path, orphan-file detection, commented `// pub mod` detection, FORBIDDEN_GATE detection, missing-lib-export detection, multi-sub-crate aggregation. All pass on first GREEN attempt.
- **Canonical list calibrated**: empirical analysis of `libxc-master/src/maple2c/gga_exc/*.c` polarized-LXC block body lines yields exactly 25 functionals at threshold ≥ 6,000 lines; reproduces the 6 RESEARCH.md-named functionals (`gga_c_ft97`, `gga_x_wpbeh`, `gga_c_pbe_erf_gws`, `gga_c_optc`, `gga_c_q2d`, `gga_c_revtca`) and adds the 19 unnamed ones.
- **Audit run (--strict)**: 25 functionals, 25 OK, 0 GAP, 0 FORBIDDEN_GATE.
- **Coverage report**: `.planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md` lists every functional with status, sub-crate distribution, and the 10 covered (level × spin) tuples.
- **Boundary checks**: 0 commented `// pub mod` entries, 0 `cfg(feature = "order-*")` matches, 0 files > 20K lines (forward-guard cap held), 0 sub-crate Cargo.toml has `[profile.*]`.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED**: `a20ce792` — `test(09-05): add failing tests for tools/audit_deferred_gga.py`
2. **Task 1 GREEN**: `8cfe3dc4` — `feat(09-05): implement tools/audit_deferred_gga.py`
3. **Task 2**: `b94ffc37` — `chore(audit): add post-09-04 deferred-GGA coverage audit (D-12)`

(TDD plan: RED + GREEN cycle for Task 1; no REFACTOR commit was needed because the GREEN implementation already satisfies all 7 tests cleanly.)

## Canonical 25 Deferred GGAs

The script's `load_canonical_list()` produces this set (sorted alphabetically):

| # | Functional | Sub-crates |
|---|---|---|
| 1 | `gga_c_acgga` | kernel-gga-9c |
| 2 | `gga_c_acggap` | kernel-gga-8a, -8b, -8c |
| 3 | `gga_c_ft97` | kernel-gga-1a, -1b, -1c, -1d |
| 4 | `gga_c_gapc` | kernel-gga-4a, -4b, -4c, -4d, -4e, -4f |
| 5 | `gga_c_gaploc` | kernel-gga-5a, -5b, -5c, -5d, -5e, -5f |
| 6 | `gga_c_hcth_a` | kernel-gga-7d |
| 7 | `gga_c_optc` | kernel-gga-6a, -6b, -6c |
| 8 | `gga_c_pbe_erf_gws` | kernel-gga-3a, -3b, -3c, -3d |
| 9 | `gga_c_pbeloc` | kernel-gga-11 |
| 10 | `gga_c_pw91` | kernel-gga-13 |
| 11 | `gga_c_q2d` | kernel-gga-7a, -7b, -7c |
| 12 | `gga_c_regtpss` | kernel-gga-12 |
| 13 | `gga_c_revtca` | kernel-gga-3e |
| 14 | `gga_c_sg4` | kernel-gga-10a, -10b |
| 15 | `gga_c_sogga11` | kernel-gga-12 |
| 16 | `gga_c_zpbeint` | kernel-gga-13 |
| 17 | `gga_c_zvpbeint` | kernel-gga-11 |
| 18 | `gga_c_zvpbeloc` | kernel-gga-12 |
| 19 | `gga_x_ft97` | kernel-gga-14 |
| 20 | `gga_x_hjs` | kernel-gga-10c, -10d |
| 21 | `gga_x_hjs_b88_v2` | kernel-gga-9a, -9b |
| 22 | `gga_x_lcgau` | kernel-gga-11 |
| 23 | `gga_x_wpbeh` | kernel-gga-2a, -2b, -2c, -2d |
| 24 | `gga_xc_b97` | kernel-gga-13 |
| 25 | `hyb_gga_xc_wb97` | kernel-gga-8d |

Eleven of these (rows 2, 3, 4, 5, 7, 8, 11, 14, 20, 21, 23) are the same eleven multi-sub-crate GGAs that 09-04's `log/09-04-skipped-multi-subcrate.log` documented as SKIPPED for D-09 preservation. Their pre-existing distribution (Plan 09-03's bin-packing) already satisfies the audit — every (level × spin) tuple is wired through one of the owning sub-crates' mod.rs and the functional is exported from each owning sub-crate's lib.rs.

## Files Created/Modified

### Created
- `tools/audit_deferred_gga.py` — audit script (~538 lines).
- `tools/test_audit_deferred_gga.py` — 7-test unittest suite.
- `.planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md` — markdown coverage report (25 × OK).
- `log/09-05-deferred-gga-audit.json` — machine-readable report (`canonical_count: 25`, every report `status: "OK"`).
- `log/09-05-task1-tests.log` — `python3 -m unittest tools.test_audit_deferred_gga -v` (Ran 7 tests, OK).
- `log/09-05-audit-pass1.log` — first audit run (no --strict).
- `log/09-05-task2-audit-final.log` — final --strict audit run, exit 0.
- `log/09-05-commented-pubmod-check.log` — empty (no commented `// pub mod`).
- `log/09-05-order-feature-gate-check.log` — empty (no `order-*` feature gates).
- `log/09-05-all-orders-feature-gate-check.log` — empty (no `all-orders` feature gates).
- `log/09-05-cap-forward-guard.log` — `wc -l` of every kernel `.rs` file (4,293 entries).
- `log/09-05-cap-violations.log` — empty (no files > 20K lines).
- `log/09-05-profile-check.log` — empty (no sub-crate `[profile.*]`).

### Modified
- `.gitignore` — added `__pycache__/` and `*.pyc` (Rule 2 correctness fix; tools/__pycache__/ pre-existed and would otherwise have shown up untracked on every Python invocation).

### Not modified (intentionally)
- Zero `crates/kernel-gga-*/src/**/mod.rs` files
- Zero `crates/kernel-gga-*/src/lib.rs` files

The audit found NO gaps — Plan 09-04's regen + the pre-09-04 layout already satisfy SPEC Req 1 boundaries. This is consistent with 09-04's note that the SKIPPED 35 multi-sub-crate functionals "were already complying with the 20K cap before the regen since the largest pre-regen file was 16,703 lines."

## Decisions Made

### D-DEV-05-A: Calibrate canonical-list threshold at 6,000 maple2c LXC-polarized body lines

**Context:** RESEARCH.md names 6 deferred GGAs but only asserts a count of 25. The remaining 19 are not enumerated by name in any planning artifact. The plan's recommended approach was a translator `--dry-run` mode, but no such mode exists in `tools/translate_gga.py`.

**Decision:** Calibrate via maple2c source measurement. Each `libxc-master/src/maple2c/gga_exc/<functional>.c` file has two `#ifndef XC_DONT_COMPILE_LXC` blocks — the second is the polarized derivative computation, which is the largest in nearly all deferred functionals. Measuring its body line count and applying threshold ≥ 6,000 lines yields exactly 25 functionals (vs. 28 at ≥ 5,500, 26 at ≥ 5,800, 25 at ≥ 6,200, 23 at ≥ 6,800). The 6 RESEARCH-named functionals are all present at threshold 6,000.

**Trade-off:** This is calibrated to the current libxc 7.0.0 maple2c source. If a future libxc upgrade adds new functionals or changes block sizes, `LXC_POL_BODY_LINE_THRESHOLD` may need re-calibration. The script's `load_canonical_list()` asserts cardinality 25 explicitly so any drift surfaces as a hard error rather than a silent miscount.

### D-DEV-05-B: Use UNION semantics for multi-sub-crate coverage

**Context:** Per CONTEXT §"Established Patterns", a single deferred functional may live across multiple letter-suffix sub-crates (e.g., `gga_c_ft97` distributed across `kernel-gga-1a/-1b/-1c/-1d` after Plan 09-03's bin-packing). Each sub-crate's `mod.rs` lists only the `_partN` files the bin-packer placed there.

**Decision:** Coverage is the UNION of (level, spin) tuples observed across every sub-crate that owns a directory for the functional. The same UNION rule applies for the lib.rs export check — `pub mod <functional>;` need only appear uncommented in at least ONE owning sub-crate's lib.rs.

**Impact:** Honors D-09 (sub-crate boundaries are not consolidated). The audit reports each sub-crate where the functional lives; users can spot-check distribution by reading the markdown report's "Sub-crates" column.

### D-DEV-05-C: No mod.rs/lib.rs edits — coverage already satisfied

**Context:** The plan anticipates the audit may find gaps requiring fixes (orphan_file → add `pub mod`; commented_pub_mod → uncomment; FORBIDDEN_GATE → delete cfg attribute; missing_lib_export → add export). The Wave 1 (09-04) summary noted that 35 multi-sub-crate functionals were SKIPPED for regen, raising concern that those might surface as audit gaps.

**Decision:** Run the audit; let the data drive any edits. Empirically: 25/25 OK, 0 GAP, 0 FORBIDDEN_GATE. No edits required. The skipped multi-sub-crate functionals' pre-existing layout (built by Plan 09-03's bin-packer with full `mod.rs` and `lib.rs` wiring) already satisfies SPEC Req 1.

**Trade-off:** None. The forward-guard cap, commented-`// pub mod` check, and feature-gate check all pass cleanly on the post-09-04 tree.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added __pycache__/*.pyc to .gitignore**
- **Found during:** Task 1 GREEN — staging `tools/audit_deferred_gga.py` revealed `tools/__pycache__/test_audit_deferred_gga.cpython-314.pyc` as untracked, alongside a pre-existing `tools/__pycache__/translate_lda.cpython-314.pyc` from prior work.
- **Issue:** Project `.gitignore` did not list `__pycache__/` or `*.pyc`. CLAUDE.md mandates "never leave generated files untracked" (post-commit deletion check / generated-files convention). Pre-existing tools/__pycache__ entries had been silently dropped from git status because no .gitignore rule covered them.
- **Fix:** Added the two lines to `.gitignore` and committed with the RED test commit.
- **Files modified:** `.gitignore` (+4 lines).
- **Verification:** `git status --short` after every Python invocation now shows only intended changes.
- **Committed in:** `a20ce792` (test RED commit, alongside the test file).

---

**Total deviations:** 1 (Rule 2 — Missing Critical, gitignore correctness fix)
**Impact on plan:** Trivial. The audit work itself ran exactly as planned with no scope creep.

## Issues Encountered

### Non-issue: 11 multi-sub-crate GGAs from 09-04's SKIPPED log all pass

The user's `<critical_user_conventions>` flagged that 35 multi-sub-crate functionals were SKIPPED in 09-04 (per `log/09-04-skipped-multi-subcrate.log`), raising the concern that some of those might surface as audit gaps in this plan. Of the 35 skipped, 11 are deferred GGAs (in the canonical 25). All 11 audit at status `OK` because:

- Plan 09-03's bin-packer left every functional's directory `mod.rs` correctly listing each `_partN_*` file actually placed in that sub-crate.
- Each owning sub-crate's `lib.rs` has an uncommented `pub mod <functional>;` line.
- The UNION across the multi-sub-crate distribution covers all 10 (level × spin) tuples for every functional.
- No file in the multi-sub-crate set is over 20,000 lines (largest is < 17K from 09-04's data; the 09-05 cap-forward-guard log confirms 0 violations across the entire workspace).

So no 09-04 regression exists. The `<critical_user_conventions>` allowance for "Deferred to follow-up" / PARTIAL audit was not needed.

## User Setup Required

None — no external service configuration introduced or required.

## Next Phase Readiness

### Ready for Plan 09-06 (cargo check verification)

- All 25 deferred GGAs are wired with full derivative-order coverage; `cargo check` should reach every `pub mod` entry.
- No FORBIDDEN_GATE or commented `// pub mod` blocks compilation.
- Forward-guard cap (≤ 20K lines) is held — no oversize file will OOM the proc-macro.
- Profile single-source preserved; sccache + `incremental = false` settings unchanged.
- Per CONTEXT D-13: 09-06 should redirect output to `log/cargo-check-09-final.log` per project convention.

### Ready for Plan 09-07 (oracle parity sweep)

- Audit verifies the deferred 25 expose every (level × spin) tuple — 09-07's parity sweep can iterate over them deterministically.
- Translator regen (09-04) preserved operation order per D-08; 1e-12 oracle parity should hold by transitivity for the 212 single-sub-crate functionals.
- For the 11 multi-sub-crate deferred GGAs (untouched by 09-04 per D-09): pre-regen byte-identity to HEAD confirms parity is unaffected.

### Notes

- `tools/audit_deferred_gga.py --strict` is reusable as a regression gate for any future translator/regen pass. Embed in CI later if desired.
- The 25 canonical functionals are calibrated against libxc 7.0.0 maple2c sources at `LXC_POL_BODY_LINE_THRESHOLD = 6000`. Future libxc upgrades may need this constant re-tuned (the script's hard cardinality assert will catch drift).

## Self-Check: PASSED

- All 3 task commits exist in git log: `a20ce792`, `8cfe3dc4`, `b94ffc37` — verified.
- `tools/audit_deferred_gga.py` and `tools/test_audit_deferred_gga.py` present.
- `.planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md` exists with `## Coverage Status` section listing 25 functional rows, all `OK`.
- `log/09-05-deferred-gga-audit.json` parses; `canonical_count == 25`, every report `status == "OK"`.
- `log/09-05-task1-tests.log` ends with `OK` (Ran 7 tests).
- `log/09-05-task2-audit-final.log` reports `OK=25, GAP=0, FORBIDDEN_GATE=0` — exit 0 under --strict.
- `grep -rE '^[[:space:]]*//[[:space:]]*pub mod' crates/kernel-gga*/src/` returns nothing.
- `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"order-' {} +` returns nothing.
- `find crates -path '*kernel-gga*/src/*' -name '*.rs' -exec grep -l 'cfg(feature *= *"all-orders' {} +` returns nothing.
- `find crates/kernel-lda crates/kernel-gga* crates/kernel-mgga* -path '*/src/*' -name '*.rs' -exec wc -l {} + | awk 'NF==2 && $2 != "total" && $1 > 20000 {n++} END {exit n}'` exits 0.
- `grep -l '^\[profile\.' crates/*/Cargo.toml` returns nothing.
- `git status --porcelain` is empty (working tree clean before SUMMARY commit).

---
*Phase: 09-reduce-kernel-build-time*
*Plan: 05*
*Completed: 2026-04-29*
