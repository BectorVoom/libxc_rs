---
phase: 10-workspace-level-modular-split
plan: 03
subsystem: infra
tags: [cargo, workspace, refactor, libxc-compat, cdylib, facade, oracle]

requires:
  - phase: 10-02
    provides: libxc-eval (orchestration + kernel deps + feature machinery)
provides:
  - "crates/libxc-compat: extern-C shim (rlib+cdylib+staticlib), deps core+eval, nothing depends on it (SC-4)"
  - "Root libxc_rs reduced to a thin facade preserving the public surface line-for-line (SC-5)"
  - "4-link feature chain finalized; compat default-features=false keeps the umbrella kernel-free"
  - "Per-family oracle parity matches the Wave-0 baseline exactly (SC-6/SC-7): LDA/GGA/MGGA all pass at 1e-12"
affects: [milestone-v1.0]

tech-stack:
  added: []
  patterns:
    - "cdylib FFI crate pinned default-features=false + self-forwarding features so the facade dep stays kernel-free"

key-files:
  created:
    - crates/libxc-compat/Cargo.toml
    - crates/libxc-compat/src/lib.rs
  modified:
    - src/lib.rs (thin facade: re-exports core/eval/compat; deps shrunk to 3 leaf crates)
    - Cargo.toml (compat dep default-features=false; members; root deps shrunk)
    - verify/tests/{lda,mgga}_oracle.rs (deferred imports via facade)
    - crates/libxc-eval/src/eval/mgga_dispatch/mod.rs (fix: $crate macro metavar)
  deleted:
    - src/main.rs (Hello-world vestige)
    - src/compat/ (moved to libxc-compat)

key-decisions:
  - "compat pins libxc-eval default-features=false + forwards oracle-* via its own [features]; root deps compat default-features=false. The plan's compat Cargo.toml (eval default features) would force all 306 kernels into every umbrella check via feature-unification → OOM, defeating the kernel-free gate. cargo tree proves root --no-default pulls 0 kernels with compat present."
  - "Facade preserves the Phase-6 c_layout re-export (pub use libxc_compat::c_layout::{xc_func_info_type, xc_func_type}) the plan's facade spec had omitted."

patterns-established:
  - "SC-5 surface preservation verified by diff: zero removed paths, only intentional deferred-path delta"

requirements-completed: []

duration: ~1h (assistant) + USER-RUN oracle (incl. one re-run after the macro fix)
completed: 2026-05-26
---

# Phase 10 / Plan 03: Extract libxc-compat + Thin Facade Summary

**Extracted the extern-C shim into crates/libxc-compat (cdylib/staticlib), reduced root to a thin facade preserving the public surface line-for-line, finalized the OOM-safe 4-link feature chain, and confirmed per-family oracle parity matches the Wave-0 baseline exactly (LDA/GGA/MGGA all pass at 1e-12).**

## Performance
- **Duration:** ~1h assistant + USER-RUN oracle (with one re-run after a macro fix)
- **Completed:** 2026-05-26
- **Tasks:** 3 (Tasks 1-2 in one commit; Task 3 USER-RUN gate; + 1 fix commit)
- **Files modified:** 24 (Tasks 1-2) + 2 (fix)

## Accomplishments
- `crates/libxc-compat` (rlib+cdylib+staticlib): deps core+eval, nothing depends on it — SC-4 via `cargo tree --invert`
- Root facade preserves the public surface — SC-5 surface diff: ZERO removed paths, only the intentional `libxc_rs::deferred` delta
- 4-link chain `verify → libxc_rs → libxc-eval → kernels` resolves; compat default-features=false keeps the umbrella kernel-free (`cargo tree -p libxc_rs --no-default` = 0 per-functional kernels)
- SC-1 (4 crates), SC-2/3/4 (cargo tree), SC-5 (surface + per-`-p`), SC-8 (deny warnings) all green
- **SC-6/SC-7:** per-family oracle parity matches Wave-0 baseline EXACTLY — LDA 2/2, GGA 2/2, MGGA 2/2 at 1e-12

## Task Commits
1. **Tasks 1+2 (extract compat + facade + verify repoint)** — `91f69f298f` (feat)
2. **Fix: mgga_dispatch macro metavar** — `247932a619` (fix)
3. **Task 3 (USER-RUN oracle parity)** — logs committed with this SUMMARY

## Decisions Made
- **compat Cargo.toml OOM-fix (plan deviation):** the plan deps libxc-eval WITHOUT default-features=false. Since root deps compat, cargo feature-unification would force all 306 kernels into every `cargo check -p libxc_rs` → OOM, defeating the kernel-free gate the user chose. Fixed: compat pins `libxc-eval default-features=false` + its own `[features]` forwarding (default = all families for a functional cdylib); root deps compat `default-features=false`. Proven kernel-free via cargo tree.
- **Facade c_layout re-export preserved:** Phase 6 added `pub use compat::c_layout::{xc_func_info_type, xc_func_type}` (line 41) which the plan's facade spec omitted. Preserved as `pub use libxc_compat::c_layout::{...}` (SC-5).
- **10 compat files, not 7:** Phase 6 added hybrid/info/library.rs; moved all 10.

## Deviations from Plan
1. **[Plan OOM bug] compat eval dep needs default-features=false + feature forwarding** (above) — else the umbrella check pulls all 306 kernels.
2. **[Plan gap] facade must preserve the c_layout re-export** (Phase-6 addition).
3. **[Latent 10-02 bug, caught here] mgga_dispatch macro `$crate::` metavar mangled by the 10-02 repoint sed** → `$libxc_core::` (invalid). Latent behind `#[cfg(feature=oracle-mgga)]`; surfaced as a BUILD failure (not parity regression) on the first USER-RUN MGGA oracle. Fixed (commit 247932a619); see memory `feedback_sed_repoint_corrupts_macro_dollar_crate`.

**Total deviations:** 3 (all forced/correctness). The MGGA "failure" was a build bug, not a numerical regression — parity is byte-identical to baseline.

## Issues Encountered
- Same sed-corruption-of-`$crate`-in-macros bit twice: compat macros.rs (caught inline at the 10-03 umbrella check) and eval mgga_dispatch (latent, surfaced at the USER-RUN MGGA oracle). Both fixed; gotcha recorded to memory.

## Next Phase Readiness
- Phase 10 COMPLETE: all 4 crates exist, layering enforced by compiler boundaries, public surface + oracle parity preserved.
- The libxc-compat cdylib (`libxc_rs.so`/`.a`) is excluded from default-members (its all-kernel link is OOM-heavy — build deliberately with `cargo build -p libxc-compat`).

---
*Phase: 10-workspace-level-modular-split*
*Completed: 2026-05-26*
