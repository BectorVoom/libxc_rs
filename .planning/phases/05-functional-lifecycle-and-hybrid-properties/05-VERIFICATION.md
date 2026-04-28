---
phase: 05-functional-lifecycle-and-hybrid-properties
verified: 2026-04-28T00:00:00Z
status: gaps_found
score: 2/5 must-haves verified
overrides_applied: 0
gaps:
  - truth: "Functional::new(id, spin) returns a fully initialized instance with correct metadata, dimensions, thresholds, and default ext_params"
    status: partial
    reason: "Functional::new itself is correctly implemented (metadata/dims/thresholds/ext_params initialize correctly for the current skeleton). However the 'fully initialized' clause includes ext_param defaults and aux construction — both are structurally correct but produce empty aux lists because meta.auxiliaries is &[] for all 649 entries. More critically, ext_param defaults from meta.ext_params are also &[] for all 649 entries (xtask never populated them). For functionals that should have ext_params (e.g. CAM-B3LYP), ext_params is None at construction time because meta.ext_params is empty — so the initialized values are wrong (not the documented defaults). The constructor code is correct; the data it reads is placeholder-only."
    artifacts:
      - path: "src/meta/generated.rs"
        issue: "All 649 entries have ext_params: &[], auxiliaries: &[], hybrid_terms: &[], nlc_params: None, hybrid_type: HybridType::Semilocal — placeholder values throughout. xtask generate-metadata was never run against a working libxc build."
      - path: "xtask/src/generate_metadata.rs"
        issue: "collect_all_functionals() returns Vec::new() unconditionally (explicit placeholder comment at line 44-45). The ensure!(!functionals.is_empty()) guard prevents file corruption but also means the tool always fails when invoked. It is structurally non-functional."
    missing:
      - "Implement collect_all_functionals() to iterate libxc_rs::registry::all_functional_ids() and call xc_func_init for each ID"
      - "Implement write_generated_rs() to emit fully-populated FunctionalMeta literals (all 14 fields including references, ext_params, hybrid_terms, auxiliaries, nlc_params)"
      - "Run cargo xtask generate-metadata from workspace root and commit the populated generated.rs, generated_hybrid.rs, generated_propagation.rs"

  - truth: "External parameters can be set/get by name or index, and modifying ext_params triggers recomputation of derived parameters"
    status: partial
    reason: "The setter/getter API exists and the code paths are correctly implemented. However: (1) Since meta.ext_params is &[] for all 649 functionals, every functional constructs with ext_params = None. The setters and getters are exercised against empty ext_param specs and produce correct empty-array behavior — but they cannot be tested against any real non-trivial values (CAM omega/alpha, LDA alpha, etc.) because the metadata is unpopulated. (2) CR-04 BLOCKER: set_ext_param_by_index has a latent panic path — when meta.ext_params.len() > 0 but self.ext_params is None (theoretically impossible today but not type-enforced), new_vals is an empty Vec and new_vals[idx] panics. (3) GGA/MGGA params impls are all zero-ext_param scaffolds — no real derived-scalar recomputation is wired for CAM/CAMY/LC/LCY functionals."
    artifacts:
      - path: "src/functional/config.rs"
        issue: "set_ext_param_by_index: new_vals built via unwrap_or_default() — if ext_params is None while count > 0, new_vals[idx] panics (CR-04)"
      - path: "src/functional/params_gga.rs"
        issue: "All 106 GGA impls have ext_param_count = 0 (zero-ext_param scaffolds); no CAM/CAMY/LC/LCY ext-param-bearing impls with real derived-scalar recomputation"
      - path: "src/functional/params_mgga.rs"
        issue: "All 95 MGGA impls are zero-ext_param scaffolds"
    missing:
      - "Fix set_ext_param_by_index to use meta.ext_params defaults as fallback instead of unwrap_or_default() (see CR-04 fix in REVIEW.md)"
      - "Once metadata is populated: implement real ext-param-bearing GGA/MGGA params structs for CAM/CAMY/LC/LCY functionals"

  - truth: "Hybrid functionals correctly report their HybridType, CAM coefficients (omega, alpha, beta), and NLC coefficients (b, C)"
    status: failed
    reason: "The API surface exists and classify_hybrid is correctly implemented as a pure-Rust port of xc_hyb_type. However it cannot correctly classify any hybrid functional because meta.hybrid_terms is &[] for all 649 entries. classify_hybrid(&[]) always returns Semilocal, so every functional reports HybridType::Semilocal regardless of its actual classification. The rust_port_matches_snapshot_for_all_649 test passes trivially (Semilocal == Semilocal for all 649). cam_coefficients() returns None for every functional because hybrid_type() returns Semilocal (the method gates on non-Semilocal type). nlc_coefficients() returns None because meta.nlc_params is None for all 649. The phase goal for this success criterion is structurally in place but effectively a no-op shell — no hybrid functional actually reports correct hybrid type, CAM coefficients, or NLC coefficients against the real libxc values."
    artifacts:
      - path: "src/meta/generated.rs"
        issue: "hybrid_type: HybridType::Semilocal for all 649 entries; hybrid_terms: &[] for all 649; nlc_params: None for all 649"
      - path: "src/meta/generated_hybrid.rs"
        issue: "HYBRID_TYPES table is empty (comment says 'Populated by cargo xtask generate-metadata' but xtask was never run)"
      - path: "xtask/src/generate_metadata.rs"
        issue: "Non-functional placeholder; cannot produce populated hybrid type data"
    missing:
      - "xtask generate-metadata must be implemented and run to populate meta.hybrid_terms, meta.hybrid_type, meta.nlc_params for all 649 functionals"
      - "Once populated: remove #[ignore] from three_way_hybrid_type_matches_for_all_649, b3lyp_exx_coefficient_matches_ffi, cam_b3lyp_cam_coefficients_match_ffi, vv10_nlc_coefficients_match_ffi tests"

  - truth: "Auxiliary functionals for mixed/hybrid functionals are recursively constructed and iterable"
    status: failed
    reason: "The recursive construction code is correctly wired in Functional::new (lines 66-78 of lifecycle.rs). PROPAGATION_RULES table iteration is implemented. However meta.auxiliaries is &[] for all 649 functionals — so the loop is a no-op for every functional. No hybrid functional (B3LYP, CAM-B3LYP, HSE03, etc.) actually constructs its auxiliary subtree. The test empty_metadata_aux_is_empty explicitly documents this: 'assert_eq!(f.meta.auxiliaries.len(), 0)' — the comment says 'After xtask populates B3LYP's 4-aux structure... this assert must be updated to expect 4'. The PROPAGATION_RULES table is also empty so ext_param propagation is a no-op. The verify oracle tests for FUNC-04/HYB-04 (mixed_oracle.rs) are all #[ignore]d pending metadata population."
    artifacts:
      - path: "src/meta/generated.rs"
        issue: "auxiliaries: &[] for all 649 entries"
      - path: "src/meta/generated_propagation.rs"
        issue: "PROPAGATION_RULES is empty (comment: 'Populated by cargo xtask generate-metadata')"
      - path: "verify/tests/mixed_oracle.rs"
        issue: "All 6 oracle tests are #[ignore]d; none can verify real aux construction until metadata is populated"
    missing:
      - "xtask generate-metadata must populate meta.auxiliaries and PROPAGATION_RULES for all 649 functionals"
      - "After population: update empty_metadata_aux_is_empty to assert B3LYP has 4 auxiliaries"
      - "After population: remove #[ignore] from mixed_oracle.rs tests"

  - truth: "Drop implementation cleans up all resources without leaks"
    status: partial
    reason: "Drop is implemented as an explicit no-op (lifecycle.rs:152-159) per design intent D-15. The drop_hybrids_ok test constructs and drops 10 representative hybrid candidates. However, since meta.auxiliaries is empty for all 649 functionals, no recursive auxiliary subtrees are ever constructed — so the test only validates the trivial single-level Drop path. The recursive Drop path (which matters for real hybrids with 4-6 aux functionals) is not exercised at all. The code is correct but untested against realistic hybrid depth. Also note CR-06: kernel crates (libxc-kernel-mgga-*) are in [dev-dependencies] rather than [dependencies], which means evaluate_mgga routing in non-test build context is potentially broken — any drop after an MGGA evaluation call in a real binary would exercise this path."
    artifacts:
      - path: "src/functional/lifecycle.rs"
        issue: "drop_hybrids_ok test exercises only empty-aux construction; auxiliary slice always length 0 for all candidates (metadata not populated)"
    missing:
      - "After xtask metadata population: update drop_hybrids_ok to assert aux lists are non-empty for hybrid candidates, then verify drops are panic-free"
      - "Fix CR-06: move per-functional MGGA/GGA kernel sub-crates from [dev-dependencies] to [dependencies] in root Cargo.toml"
---

# Phase 5: Functional Lifecycle and Hybrid Properties — Verification Report

**Phase Goal:** Users can construct a Functional instance by ID, configure external parameters and thresholds, query hybrid properties, and evaluate any of the 649 functionals through the Functional struct
**Verified:** 2026-04-28
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Functional::new(id, spin) returns fully initialized instance with correct metadata, dimensions, thresholds, and default ext_params | ? PARTIAL | Constructor code correct; data it reads is all placeholder (ext_params = &[], auxiliaries = &[] for all 649). Real ext_param defaults never initialized because xtask was not run. |
| 2 | External parameters can be set/get by name or index; modifying ext_params triggers recomputation of derived params | ? PARTIAL | API surface exists and is structurally correct. CR-04 panic path in set_ext_param_by_index. All GGA/MGGA params impls are zero-ext_param scaffolds with no real derived-scalar recomputation. Cannot test against real values because metadata is unpopulated. |
| 3 | Hybrid functionals correctly report HybridType, CAM coefficients (omega, alpha, beta), and NLC coefficients (b, C) | FAILED | classify_hybrid(&[]) always returns Semilocal. All 649 functionals report Semilocal. cam_coefficients() / nlc_coefficients() return None for all functionals. rust_port_matches_snapshot passes trivially (Semilocal == Semilocal). No real hybrid classification occurs. |
| 4 | Auxiliary functionals for mixed/hybrid functionals are recursively constructed and iterable | FAILED | meta.auxiliaries = &[] for all 649 functionals. Recursive construction loop is a no-op for every functional. B3LYP has 0 auxiliaries, not 4. PROPAGATION_RULES is empty. All mixed_oracle.rs tests are #[ignore]d. |
| 5 | Drop implementation cleans up all resources without leaks | ? PARTIAL | Drop is explicit no-op; drop_hybrids_ok test passes but only exercises empty-aux path (trivial single-level drop). Recursive Drop path for realistic hybrid depth never tested. CR-06: MGGA kernel sub-crates in dev-dependencies may cause non-test build failures. |

**Score:** 2/5 truths verified (PARTIAL counts as 0.5; two truths are hard FAILED)

The two truths that are structurally VERIFIED (no gaps):
- The FunctionalParams trait + NoParams blanket impl + ~239 per-functional impls are landed and wired (FUNC-05 code surface complete but data is empty)
- Functional struct with lifecycle/config/params submodules, dispatch migration to &dyn FunctionalParams, GgaScratch/MggaScratch materialized (FUNC-01/02/03 code structure complete but data is empty)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `src/functional/hybrid.rs` | classify_hybrid + CamCoefficients/NlcCoefficients + 6 query methods | EXISTS/SUBSTANTIVE | 9 required symbols present at lines 26, 36, 48, 84, 97, 112, 156, 165, 173 |
| `src/functional/evaluate.rs` | evaluate_lda/gga/mgga routers | EXISTS/SUBSTANTIVE | All 3 routing methods at lines 34, 58, 82; auxiliaries.is_empty() branch present |
| `src/functional/lifecycle.rs` | Functional::new + Drop + propagate_to_aux | EXISTS/SUBSTANTIVE | All present; aux loop is correct code but no-op due to empty metadata |
| `src/functional/config.rs` | 6 ext_param setters/getters + 4 threshold setters | EXISTS/SUBSTANTIVE | All 10 methods present; CR-04 panic path in set_ext_param_by_index |
| `src/functional/params.rs` | FunctionalParams trait + NoParams | EXISTS/SUBSTANTIVE | trait at line 18, NoParams present |
| `src/functional/params_lda.rs` | 38 LDA FunctionalParams impls | EXISTS/SUBSTANTIVE | 38 impls (6 concrete, 32 zero-ext_param scaffolds) |
| `src/functional/params_gga.rs` | 106 GGA FunctionalParams impls | EXISTS/SUBSTANTIVE | 106 impls, all zero-ext_param scaffolds (no CAM/CAMY bearing impls) |
| `src/functional/params_mgga.rs` | 86+ MGGA FunctionalParams impls | EXISTS/SUBSTANTIVE | 95 impls, all zero-ext_param scaffolds |
| `xtask/src/generate_metadata.rs` | FFI-driven metadata snapshot tool | EXISTS/STUB | File exists (115 lines) but collect_all_functionals() returns Vec::new() unconditionally — tool always fails at ensure!() guard when run |
| `src/meta/generated.rs` | 649 fully-populated FunctionalMeta entries | EXISTS/STUB | 649 entries with ext_params: &[], auxiliaries: &[], hybrid_terms: &[], nlc_params: None, hybrid_type: Semilocal for every entry |
| `src/meta/generated_hybrid.rs` | HybridType table for all IDs | EXISTS/STUB | File exists (9 lines); HYBRID_TYPES = &[] empty |
| `src/meta/generated_propagation.rs` | PropagationRule table | EXISTS/STUB | File exists (9 lines); PROPAGATION_RULES = &[] empty |
| `verify/tests/hybrid_type_oracle.rs` | HYB-01 three-way oracle comparison | EXISTS/HOLLOW | Always-on test rust_port_matches_snapshot_for_all_649 passes trivially (Semilocal == Semilocal for all 649). FFI comparison test is #[ignore]d. |
| `verify/tests/hybrid_oracle.rs` | HYB-02/03 CAM + NLC coefficient tests | EXISTS/HOLLOW | 3 of 5 tests are #[ignore]d; 2 always-on tests check None returns for lda_x/non-nlc (trivially correct, no real hybrid data) |
| `verify/tests/mixed_oracle.rs` | FUNC-04/HYB-04 mixed oracle | EXISTS/HOLLOW | All 6 tests are #[ignore]d pending metadata population; also has CR-05 (xc_func_init rc not checked in ffi_cam/ffi_exx helpers) |
| `verify/tests/metadata_oracle.rs` | D-04 round-trip oracle | EXISTS/STUB | Test exists but snapshot_from_ffi() call site is commented out; would compare against empty skeleton even if uncommented |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `xtask/src/generate_metadata.rs` | `libxc-sys` (xc_func_init/end) | unsafe FFI calls | NOT_WIRED | collect_all_functionals() returns empty Vec; no FFI calls are actually made |
| `src/functional/lifecycle.rs::Functional::new` | `src/meta/generated_propagation.rs::PROPAGATION_RULES` | iter rules, filter by parent_id | PARTIAL | Code is wired (line 113: PROPAGATION_RULES.iter().filter()); table is empty so wiring is a no-op |
| `Functional::evaluate_lda/gga/mgga` | `evaluate_mixed_lda_functional/gga/mgga` | if auxiliaries.is_empty() | PARTIAL | Routing code is correct; always takes the direct-dispatch branch because auxiliaries is always empty |
| `src/eval/mix.rs::evaluate_mixed_gga` per-aux family branch | `mix_func.c:170-308` semantics | match aux.meta.family | PARTIAL_BUGGY | Code structure present; CR-02 (add_opt silent truncation) + CR-03 (parent NEEDS_LAPLACIAN/NEEDS_TAU flags not consulted) — semantic deviation from mix_func.c reference |
| `src/functional/lifecycle.rs::Functional::new` | `src/registry/mod.rs::lookup_by_id` | returns &'static FunctionalMeta | WIRED | Confirmed at line 32: lookup_by_id(id.raw())? |
| `dispatch_lda/gga/mgga` | `FunctionalParams::as_any().downcast_ref::<T>()` | per-arm downcast | PARTIAL | Only LdaX arm actually downcasts; GGA/MGGA arms have no ext-param-bearing downcast yet (no real params data) |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|--------------|--------|-------------------|--------|
| `Functional::hybrid_type()` | meta.hybrid_type | generated.rs constant | No — all entries are Semilocal placeholder | STATIC |
| `Functional::cam_coefficients()` | meta.hybrid_terms[i] | generated.rs constant | No — all entries are &[] | STATIC |
| `Functional::nlc_coefficients()` | meta.nlc_params | generated.rs constant | No — all entries are None | STATIC |
| `Functional::auxiliary_functionals()` | self.auxiliaries (built from meta.auxiliaries) | generated.rs constant | No — meta.auxiliaries is &[] for all 649 | STATIC |
| `Functional::new` ext_params initialization | meta.ext_params[i].default_value | generated.rs constant | No — all entries are &[] | STATIC |

### Behavioral Spot-Checks

| Behavior | Command/Check | Result | Status |
|----------|--------------|--------|--------|
| Functional::new(lda_x_id, Unpolarized) initializes correctly | Test new_lda_x_unpolarized_succeeds in lifecycle.rs | Passes (code correct; meta.id=1 maps to correct name) | PASS |
| B3LYP has 4 auxiliary functionals | empty_metadata_aux_is_empty test assert: meta.auxiliaries.len() == 0 | Asserts 0, not 4 — comment documents this is wrong placeholder | FAIL |
| hybrid_type(b3lyp) == HybridType::Hybrid | classify_hybrid(meta.hybrid_terms) for id 402 | Returns Semilocal (hybrid_terms is &[]) | FAIL |
| cam_coefficients(cam_b3lyp) returns Some(omega,alpha,beta) | cam_coefficients() for CAM-B3LYP | Returns None (hybrid_type() is Semilocal) | FAIL |
| rust_port_matches_snapshot_for_all_649 passes | Always-on test in hybrid_type_oracle.rs | Passes trivially — both sides are Semilocal for all 649 | TRIVIAL PASS (no real signal) |
| set_ext_param_by_index(valid_idx, val) succeeds | Unit test in config.rs (with empty ext_params) | Test passes only because count=0 prevents the panic path from being hit | PASS (not exercising real path) |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|---------|
| FUNC-01 | 05-02 | Functional::new constructs instance with correct metadata, dims, thresholds, ext_params | PARTIAL | Constructor code correct; ext_params defaults all empty (no real values) |
| FUNC-02 | 05-02 | External parameter management: set/get by name, index, bulk | PARTIAL | API exists; CR-04 panic path; all real ext_param specs are empty so no actual param management exercised |
| FUNC-03 | 05-02 | Threshold configuration: density, zeta, sigma, tau | VERIFIED | All 4 setters present at config.rs:129-141; write-through confirmed |
| FUNC-04 | 05-03 | Auxiliary functional initialization for hybrid/mixed functionals (recursive) | BLOCKED | Recursion code correct; meta.auxiliaries = &[] so no real aux ever constructed |
| FUNC-05 | 05-02 | FunctionalParams trait for per-functional computed parameters | PARTIAL | Trait exists; 239 impls present but all GGA/MGGA impls are zero-ext_param scaffolds |
| FUNC-06 | 05-03 | Drop implementation cleans up resources | PARTIAL | Drop is no-op + test passes; only trivial empty-aux path tested |
| HYB-01 | 05-01/05-03 | HybridType classification | BLOCKED | classify_hybrid code correct; snapshot data all Semilocal; real classification impossible |
| HYB-02 | 05-03 | CAM coefficient extraction (omega, alpha, beta) | BLOCKED | cam_coefficients() always returns None (hybrid_type always Semilocal) |
| HYB-03 | 05-03 | NLC coefficient extraction (b, C) | BLOCKED | nlc_coefficients() always returns None (meta.nlc_params all None) |
| HYB-04 | 05-03 | Auxiliary functional iteration (IDs and weights) | BLOCKED | auxiliary_functionals() always returns &[] (meta.auxiliaries all empty) |

**Orphaned requirements check:** All 10 requirement IDs declared across plans (FUNC-01 through FUNC-06, HYB-01 through HYB-04) are accounted for above. No orphaned requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `xtask/src/generate_metadata.rs` | 38-47 | `collect_all_functionals()` returns `Vec::new()` — always returns empty result | BLOCKER | xtask generate-metadata is non-functional; all metadata population impossible |
| `xtask/src/generate_metadata.rs` | 56-80 | `write_generated_rs` emits invalid Rust with `// ... populated by full implementation` — missing 11 struct fields | BLOCKER | If the ensure! guard is bypassed, would corrupt generated.rs with non-compiling stubs |
| `src/meta/generated.rs` | 1-10390 | All 649 entries have `hybrid_type: HybridType::Semilocal`, `ext_params: &[]`, `auxiliaries: &[]`, `hybrid_terms: &[]`, `nlc_params: None` | BLOCKER | Every metadata query returns placeholder values; hybrid/mixed functionality is effectively disabled |
| `src/functional/config.rs` | 91-107 | `set_ext_param_by_index`: `new_vals = self.ext_params.as_deref().map(...).unwrap_or_default()` then `new_vals[idx] = val` — panics if ext_params is None but count > 0 | BLOCKER | Latent panic in library API path (currently not reachable because all ext_params are empty, but will activate when metadata is populated) |
| `verify/tests/hybrid_oracle.rs` | 25, 34 | `xc_func_init` return code discarded in `ffi_cam` and `ffi_exx` — subsequent FFI calls on zeroed struct are UB | BLOCKER (FFI safety) | Test-tier UB; if init fails, xc_hyb_cam_coef and xc_func_end called on null-initialized struct |
| `Cargo.toml` | 16-127 | ~153 per-functional MGGA/GGA kernel sub-crates in `[dev-dependencies]` instead of `[dependencies]` | BLOCKER | `cargo build -p libxc_rs` (non-test mode) cannot link dispatch arms; evaluate_* from downstream binary callers would fail at link time |
| `src/eval/mix.rs` | 202-212 | `add_opt` silently truncates on length mismatch via `.min(src.len())` | BLOCKER | Mixed accumulator correctness is fragile; will produce silent wrong answers once metadata enables real aux evaluation |
| `src/eval/mix.rs` | 617-619 | `evaluate_mixed_mgga` only consults aux flags for NEEDS_LAPLACIAN/NEEDS_TAU; parent flags ignored | BLOCKER | Deviates from mix_func.c semantics; incorrect vlapl/vtau accumulation for hybrids where parent does not need laplacian but aux does |

### Human Verification Required

No items require human verification — the failures identified are programmatically observable from the source code.

### Gaps Summary

The root cause of all BLOCKER-class failures for success criteria 1, 3, 4, and 5 is a single upstream gap: **xtask generate-metadata was never implemented and never run**. The tool (xtask/src/generate_metadata.rs) exists as a 115-line placeholder where `collect_all_functionals()` returns `Vec::new()` unconditionally, and the file-writing functions are partial stubs. As a result, all three generated files (generated.rs, generated_hybrid.rs, generated_propagation.rs) contain empty placeholder data for all 649 functionals.

The consequences cascade through every hybrid/mixed feature:
- All 649 functionals report `HybridType::Semilocal` (wrong for ~80 hybrid functionals)
- No functional has any ext_params, auxiliaries, hybrid_terms, or nlc_params
- Every hybrid query method returns the trivial empty/None/Semilocal value
- All mixed-evaluation oracle tests are #[ignore]d
- The always-on drift test passes trivially because both sides of the comparison return Semilocal

The code architecture is sound — the constructor, dispatch, config, hybrid query, and mixed evaluation code paths are all correctly implemented and would produce correct results if the metadata tables were populated. But the phase goal "Users can construct a Functional instance by ID, configure external parameters and thresholds, query hybrid properties, and evaluate any of the 649 functionals through the Functional struct" is not achieved in any meaningful sense for the hybrid/mixed subset of functionals (success criteria 3 and 4).

Two additional independent blockers exist regardless of metadata state:
- CR-04: panic path in `set_ext_param_by_index` (activates when metadata is populated)
- CR-06: MGGA/GGA kernel sub-crates in `[dev-dependencies]` instead of `[dependencies]` (breaks non-test builds right now)

Success criteria that are VERIFIED without qualification:
- The code structure for FUNC-05 (FunctionalParams trait) is complete and `&dyn FunctionalParams` dispatch is wired
- FUNC-03 (threshold setters) is fully implemented and testable today
- The Drop implementation is correct per D-15

---

_Verified: 2026-04-28_
_Verifier: Claude (gsd-verifier)_
