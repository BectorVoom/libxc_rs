---
phase: 5
slug: functional-lifecycle-and-hybrid-properties
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-24
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Derived from 05-RESEARCH.md §Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` + `approx 0.5.1` for scalar comparisons |
| **Config file** | `Cargo.toml` (workspace) + `verify/Cargo.toml` (FFI oracle dev-deps) |
| **Quick run command** | `cargo test -p libxc_rs --lib functional` |
| **Full suite command** | `cargo test --workspace` |
| **Oracle round-trip** | `cargo test -p libxc_rs-verify --test metadata_oracle` |
| **Estimated runtime** | ~5 s (unit), ~2 min (oracle round-trip), ~10 min (full release) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p libxc_rs --lib functional` — unit tests on the `Functional` handle, `FunctionalParams` trait, hybrid queries. No FFI compilation. Fast enough for every commit.
- **After every plan wave:** Run `cargo test -p libxc_rs --lib` (all unit tests) + `cargo test -p libxc_rs-verify --test metadata_oracle --test hybrid_type_oracle` (FFI round-trip). Ensures metadata hasn't drifted and HybridType classification matches.
- **Before `/gsd-verify-work`:** Full workspace `cargo test --workspace --release` must be green — includes the 10,312-ish existing kernel oracle tests plus Phase 5 new tests.
- **Max feedback latency:** 5 seconds for unit, 120 seconds for wave merge.

---

## Per-Task Verification Map

*Populated by the planner with concrete task IDs + wave assignments. Each row binds a task to the automated test command that verifies it.*

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 05-01-* | 01 | 1 | — (metadata snapshot / libxc-sys factoring) | — | N/A | integration | `cargo test -p libxc_rs-verify --test metadata_oracle` | ❌ Wave 0 | ⬜ pending |
| 05-02-* | 02 | 2 | FUNC-01, FUNC-02, FUNC-03, FUNC-05 | — | N/A | unit | `cargo test -p libxc_rs --lib functional` | ❌ Wave 0 | ⬜ pending |
| 05-03-* | 03 | 3 | FUNC-04, FUNC-06, HYB-01, HYB-02, HYB-03, HYB-04 | — | N/A | integration | `cargo test -p libxc_rs-verify --test mixed_oracle --test hybrid_oracle --test hybrid_type_oracle` | ❌ Wave 0 | ⬜ pending |

*Per-requirement detail is expanded in 05-RESEARCH.md §Validation Architecture →  Phase Requirements → Test Map (10 REQ-IDs, 30+ rows).*

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Tests and infrastructure that must exist before execution can proceed:

- [ ] `verify/tests/metadata_oracle.rs` — D-04 round-trip: every `FunctionalMeta` field compared vs fresh `xc_func_init` for every id (all 649). Core sign-off test.
- [ ] `verify/tests/hybrid_type_oracle.rs` — HYB-01 three-way compare: Rust port `classify_hybrid` vs snapshotted `meta.hybrid_type` vs live `xc_hyb_type(t)` across 649 IDs.
- [ ] `verify/tests/mixed_oracle.rs` — FUNC-04 / HYB-04 integration: B3LYP (GGA Vxc), CAM-B3LYP (ext-param sweep), HSE, mgga_c_b94_hyb (MGGA Vxc), wB97X.
- [ ] `verify/tests/hybrid_oracle.rs` — HYB-02 / HYB-03 coefficient queries: CAM omega/alpha/beta for CAM family; NLC (b, C) for vv10 / vv10-like.
- [ ] `src/functional/lifecycle.rs` `#[cfg(test)] mod tests` — `Functional::new` happy path, unknown-id error, dims by family+spin, default ext_params applied, B3LYP 4-aux shape, aux depth ≤ 2, drop no-panic.
- [ ] `src/functional/config.rs` `#[cfg(test)] mod tests` — set/get ext_param by name, by index, bulk `set_ext_params` length validation, `ExtParamNotFound` on unknown name, threshold setter round-trip.
- [ ] `src/functional/params.rs` `#[cfg(test)] mod tests` — `NoParams` used for zero-ext_param functionals, downcast via `as_any().downcast_ref::<T>()` succeeds in dispatch, default round-trip across all 229.
- [ ] `src/functional/hybrid.rs` `#[cfg(test)] mod tests` — `classify_hybrid` matches snapshot on all 649 IDs; `exx_coefficient` for B3LYP == 0.20; `cam_coefficients` returns `None` for non-CAM; `nlc_coefficients` returns `None` for non-NLC; `auxiliary_functionals` yields `(id, weight)` pairs.
- [ ] Extended `src/eval/workspace.rs` tests — `gga_scratch_mut` / `mgga_scratch_mut` produce correct-length slices for polarized + unpolarized (mirror existing `lda_scratch_*` shape tests).
- [ ] Extended `src/eval/mix.rs` tests — `evaluate_mixed_gga` / `evaluate_mixed_mgga` single-aux weight=1.0 equivalence to direct `dispatch_*` (mirror existing `mixed_single_aux_weight_1_matches_dispatch`).
- [ ] Framework install: **none needed** — `approx` already in `verify/Cargo.toml` dev-deps, `#[test]` is built-in.

---

## Oracle Comparison Dimensions

For `verify/tests/metadata_oracle.rs` (D-04):

| Dimension | Comparison | Tolerance |
|-----------|------------|-----------|
| `id` | exact u16 | 0 |
| `name` | `&'static str` equality | exact |
| `kind`, `family` | enum discriminant | exact |
| `flags` | bitflags bitwise == | exact |
| `default_density_threshold` | f64 | exact |
| `references[i].{citation, doi, bibtex, key}` | `&'static str` | exact — xtask copies C strings verbatim |
| `ext_params[i].{name, description, default_value}` | str + f64 | exact |
| `auxiliaries[i].{id, weight}` | (FunctionalId, f64) | exact |
| `hybrid_terms[i].{kind, coefficient, omega}` | (enum, f64, f64) | exact |
| `hybrid_type` | enum discriminant | exact (both from `xc_hyb_type`) |
| `nlc_params` | `Option<(f64, f64)>` | exact |
| `max_order` | `DerivativeOrder` | exact (derived from flags) |

For evaluation oracle tests (mixed_oracle, hybrid_oracle): energy (zk) `rel_err <= 1e-12`, Vxc `rel_err <= 1e-10`, higher orders per VERIFY-05..07.

---

## Sampling Strategy for 229 FunctionalParams Impls

Three-tier sampling gives full 229-coverage with ~5 test functions:

1. **Default-params oracle sweep** (automatic, covers all 229): loop over `all_functional_ids()`, construct `Functional::new(id, Unpolarized)`, assert raw ext_params length + per-index equality with `meta.ext_params[i].default_value`.
2. **Perturbation oracle sweep** (automatic, covers ext-param-bearing subset): pseudo-random id-seeded perturbation, set via `set_ext_param_by_index(0, perturbed)`, evaluate + compare against libxc oracle with identical ext_params set.
3. **Hand-written targeted tests** (~10–15): CAM-B3LYP, B3LYP (4-aux), mgga_c_b94_hyb, HSE (range-sep), wB97X (range-sep+mix), PBE0, B2PLYP (PT2 double-hybrid).

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| No memory leaks on hybrid Drop | FUNC-06 | Requires valgrind, not a standard `cargo test` dep | `valgrind --leak-check=full cargo test --release functional::lifecycle::tests::drop_hybrids_ok` |

*All other phase behaviors have automated verification via `cargo test`.*

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5 s (unit) / 120 s (wave merge)
- [ ] `nyquist_compliant: true` set in frontmatter (flip after execution verifies all rows)

**Approval:** pending
