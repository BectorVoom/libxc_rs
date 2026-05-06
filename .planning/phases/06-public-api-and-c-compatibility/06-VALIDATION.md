---
phase: 6
slug: public-api-and-c-compatibility
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-05-06
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Detailed rationale and signal inventory live in `06-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` + `cargo test` (no external test framework) |
| **Config file** | None — Cargo conventions; `verify/Cargo.toml` defines verify-tier deps |
| **Quick run command** | `cargo test -p libxc_rs api:: compat::` |
| **Full suite command** | `cargo test --workspace --no-fail-fast` |
| **Estimated runtime** | ~30s quick / ~5–10min full (includes verify/ oracle suite) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p libxc_rs api:: compat::`
- **After every plan wave:** Run `cargo test --workspace --no-fail-fast`
- **Before `/gsd-verify-work`:** Full suite green + `cargo build -p libxc_rs --release && nm target/release/liblibxc_rs.so | grep -c '^.* T xc_'` ≥ 85 + `gcc -fsyntax-only -Wall -Werror include/xc.h`
- **Max feedback latency:** ~30 seconds

---

## Per-Task Verification Map

> Tasks are filled in by `gsd-planner` in step 8. The map below seeds the requirement → command coverage; planner extends it with concrete `Task ID`s once plans are written.

| Plan (anticipated) | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|--------------------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 06-01 (Layer-3 API) | 1 | API-01 | — | Builder validation surfaces `LibxcRsError` for unknown id, bad ext_param | unit | `cargo test -p libxc_rs api::builder::tests` | ❌ W0 | ⬜ pending |
| 06-01 | 1 | API-02 | — | BatchEvaluator round-trips ≥100 evaluations without realloc | unit | `cargo test -p libxc_rs api::batch::tests::workspace_reuse_no_realloc` | ❌ W0 | ⬜ pending |
| 06-01 | 1 | API-02 | — | `np > np_max` returns `BatchOverflow` | unit | `cargo test -p libxc_rs api::batch::tests::overflow_returns_error` | ❌ W0 | ⬜ pending |
| 06-01 | 1 | API-03 | — | `EvaluateInput::dispatch` for LdaInput on LDA Functional bit-equivalent to `evaluate_lda` | unit | `cargo test -p libxc_rs api::evaluate::tests::lda_dispatch_bit_equivalent` | ❌ W0 | ⬜ pending |
| 06-01 | 1 | API-03 | — | `LdaInput` on GGA Functional returns `FamilyMismatch` | unit | `cargo test -p libxc_rs api::evaluate::tests::family_mismatch_lda_input_gga_func` | ❌ W0 | ⬜ pending |
| 06-02 (compat lifecycle + setters) | 2 | COMPAT-02 | T-06-02 | Opaque `xc_func_type` size-of == 0 (compile-time) | unit | `cargo test -p libxc_rs compat::c_layout::tests::opaque_size_zero` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-02 | T-06-02 | `Family::Lda as i32 == XC_FAMILY_LDA == 1` and parallel for all flags | unit | `cargo test -p libxc_rs compat::c_layout::tests::repr_constants_match_libxc` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | — | Lifecycle round-trip: alloc → init → exc → end → init → exc → end → free | integration | `cargo test --test compat_smoke lifecycle_round_trip` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | T-06-04 | Re-init drops previous Functional (no leak) | unit (+ miri opt) | `cargo test -p libxc_rs compat::raw_handle::tests::reinit_drops_previous` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | — | Threshold setter propagates to all aux (Pitfall 4) | unit | `cargo test -p libxc_rs functional::config::tests::threshold_propagates_to_aux` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | — | `XC_EXT_PARAMS_DEFAULT = -999998888` substitution | unit | `cargo test -p libxc_rs compat::legacy_eval::tests::ext_params_default_marker` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | T-06-05 | Forced panic returns `LIBXC_RS_PANIC` errno (no UB) | unit | `cargo test -p libxc_rs compat::macros::tests::catch_panic_returns_errno` | ❌ W0 | ⬜ pending |
| 06-02 | 2 | COMPAT-01 | — | `xc_rs_last_error_code` / `xc_rs_last_error_message` round-trip | integration | `cargo test --test compat_smoke errno_round_trip` | ❌ W0 | ⬜ pending |
| 06-03 (compat evaluate + integration) | 3 | COMPAT-01 | — | All 85 extern "C" symbols exported (build smoke) | smoke | `nm target/release/liblibxc_rs.so \| grep -c '^.* T xc_'` ≥ 85 | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-01 | — | All 33 evaluate functions callable, return 0, fill output | integration | `cargo test --test compat_smoke evaluate_all_orders` | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-01 | T-06-03 | NULL output pointer skips that derivative (libxc parity) | integration | `cargo test --test compat_smoke null_skips_derivative` | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-01 | — | Discovery: `xc_number_of_functionals == 649`, `xc_functional_get_number("lda_x") == 1` | integration | `cargo test --test compat_smoke discovery_matches_registry` | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-01 | — | Hybrid coefficients: B3LYP / CAM-B3LYP CAM coefficients oracle parity | integration | `cargo test --test compat_smoke hybrid_oracle_b3lyp` | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-02 | — | C header compiles under `gcc -fsyntax-only -Wall -Werror` | smoke | `gcc -fsyntax-only -Wall -Werror include/xc.h` | ❌ W0 | ⬜ pending |
| 06-03 | 3 | COMPAT-03 | — | No `unsafe` outside `compat/`, `kernel/launch.rs`, `kernel/buffer.rs` | static | `! find src -name '*.rs' -not -path '*/compat/*' -not -path '*/kernel/launch.rs' -not -path '*/kernel/buffer.rs' \| xargs grep -l 'unsafe '` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Wave 0 (test infrastructure stubs that unlock the per-task tests above):

- [ ] `src/api/builder.rs` body — currently 2-line placeholder; extend to support D-A1 builder shape
- [ ] `src/api/batch.rs` body — currently 2-line placeholder; extend to support D-A2 workspace shape
- [ ] `src/api/evaluate.rs` — file does not exist; create with `EvaluateInput` sealed trait per D-A3
- [ ] `src/api/mod.rs` — register new submodules (`pub mod evaluate;`)
- [ ] `src/compat/{c_layout,raw_handle,ids,legacy_eval,removed}.rs` bodies — currently placeholders
- [ ] `src/compat/{info,hybrid,library,errno,macros}.rs` — files do not exist; create
- [ ] `src/compat/mod.rs` — register new submodules
- [ ] `src/error/mod.rs` extension — add `BatchOverflow`, `UninitializedHandle`, `Panicked`, plus `discriminant()` mapping for errno
- [ ] `include/xc.h` — file does not exist; create (~250 lines)
- [ ] `verify/tests/compat_smoke.rs` — file does not exist; needed for FFI integration tests
- [ ] Phase-5 `src/functional/config.rs` threshold setters — extend to walk `self.auxiliaries` (Pitfall 4 fix)

*All gaps are net-new code; no existing test infrastructure modifications needed.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Drop-in replacement against a real C/Fortran DFT consumer | COMPAT-01 / COMPAT-02 | Requires an external DFT code (e.g., NWChem, PySCF) and is out of scope for the in-repo test suite | Document in `docs/compat-integration.md`; smoke-link a hello-world `.c` against `liblibxc_rs.so` to confirm header + symbol resolution |
| Behavior under `valgrind`/`miri` for the compat lifecycle round-trip | COMPAT-01, COMPAT-03 | Optional — runs in Phase 7 if available toolchain supports it | `cargo +nightly miri test compat::raw_handle::tests::reinit_drops_previous` (best-effort; miri may reject FFI) |

*All other phase behaviors have automated verification via `cargo test`.*

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s for quick command
- [ ] `nyquist_compliant: true` set in frontmatter (after planner expands per-task IDs)

**Approval:** pending
