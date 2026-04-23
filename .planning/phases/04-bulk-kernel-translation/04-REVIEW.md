---
phase: 04-bulk-kernel-translation
reviewed: 2026-04-24T00:00:00Z
depth: standard
files_reviewed: 48
files_reviewed_list:
  - crates/kernel-lda/src/deferred.rs
  - crates/kernel-lda/src/lib.rs
  - crates/kernel-mgga/src/deferred.rs
  - src/error/mod.rs
  - src/eval/dispatch.rs
  - src/eval/gga_dispatch/batch12.rs
  - src/eval/gga_dispatch/batch13.rs
  - src/eval/gga_dispatch/batch14.rs
  - src/eval/gga_dispatch/batch15.rs
  - src/eval/gga_dispatch/batch16.rs
  - src/eval/gga_dispatch/batch17.rs
  - src/eval/gga_dispatch/batch18.rs
  - src/eval/gga_dispatch/batch19.rs
  - src/eval/gga_dispatch/batch20.rs
  - src/eval/gga_dispatch/batch21.rs
  - src/eval/gga_dispatch/batch22.rs
  - src/eval/gga_dispatch/batch4g.rs
  - src/eval/gga_dispatch/batch5g.rs
  - src/eval/gga_dispatch/batch6d.rs
  - src/eval/gga_dispatch/batch8d.rs
  - src/eval/gga_dispatch/mod.rs
  - src/eval/mgga_dispatch/batch17.rs
  - src/eval/mgga_dispatch/batch21.rs
  - src/eval/mgga_dispatch/batch23.rs
  - src/eval/mgga_dispatch/batch28.rs
  - src/eval/mgga_dispatch/batch29.rs
  - src/eval/mgga_dispatch/batch30.rs
  - src/eval/mgga_dispatch/batch33.rs
  - src/eval/mgga_dispatch/batch34.rs
  - src/eval/mgga_dispatch/batch35.rs
  - src/eval/mgga_dispatch/mod.rs
  - src/eval/mix.rs
  - src/eval/mod.rs
  - src/kernel/mod.rs
  - src/lib.rs
  - src/model/gga_functional.rs
  - src/model/lda_functional.rs
  - src/model/mgga_functional.rs
  - src/model/mod.rs
  - tools/generate_gga_dispatch.py
  - tools/generate_gga_roster.py
  - tools/generate_mgga_roster.py
  - verify/Cargo.toml
  - verify/src/lib.rs
  - verify/tests/gga_oracle.rs
  - verify/tests/lda_oracle.rs
  - verify/tests/mgga_oracle.rs
  - xtask/src/main.rs
  - xtask/src/verify_phase_4.rs
findings:
  critical: 0
  warning: 3
  info: 9
  total: 12
status: issues_found
---

# Phase 4: Code Review Report

**Reviewed:** 2026-04-24T00:00:00Z
**Depth:** standard
**Files Reviewed:** 48
**Status:** issues_found

## Summary

Phase 4 introduces the Rust dispatch layer (`dispatch_lda`, `dispatch_gga`,
`dispatch_mgga`) plus per-functional oracle-comparison tests for the
libxc 7.0.0 reimplementation. The invariants called out in the review
context are upheld:

- **B1 (FunctionalId::from_raw is authoritative external path)**: All
  cross-crate consumers (`verify/tests/*.rs`, batch dispatch stubs,
  xtask verify_phase_4) use `FunctionalId::from_raw`. The one instance of
  tuple-literal construction (`FunctionalId({id})`) in `xtask/src/main.rs`
  writes into the library's own auto-generated `src/meta/generated.rs`
  where `pub(crate)` visibility is satisfied, and is not a violation.
- **B3 (no shared *FunctionalParams for GGA/MGGA)**: The only
  `*FunctionalParams` struct is the LDA-specific `LdaFunctionalParams`
  that existed pre-Phase 4 (it carries just `alpha` for Slater
  exchange). GGA and MGGA dispatch keep per-functional scalars inline at
  the batch-level launch helpers; no shared GGA/MGGA params struct was
  introduced.
- **W5 (has_exc filesystem-driven)**: `LdaFunctional::has_exc`,
  `GgaFunctional::has_exc`, and `MggaFunctional::has_exc` all key on
  a single vxc-only variant each (`LdaXcTih`, `GgaXLb`, `MggaXTb09`)
  that corresponds to a kernel directory lacking `exc_unpol.rs` /
  `exc_pol.rs`. The oracle harness explicitly decouples `has_exc()`
  from libxc's `FLAGS_HAVE_EXC` (see `mgga_oracle.rs:397-398`).
- **W7 (deferred MGGA via is_deferred)**: `MggaFunctional::from_id`
  defers to `libxc_kernel_mgga::deferred::is_deferred` (line 127 of
  `src/model/mgga_functional.rs`) before the main match, and
  `verify/tests/mgga_oracle.rs` uses the same helper (line 496) — there
  is a single source of truth.
- **W8 (no clap in xtask)**: `xtask/src/main.rs` uses a hand-rolled
  `std::env::args()` parser with a simple `match` on command strings.
  `clap` is not introduced.

The issues below are non-invariant-violating quality items. None block
the phase from merging.

## Warnings

### WR-01: `mgga_zero_scalar_unpol_dispatch!` reports the wrong `FunctionalId` on the unreachable-guard branch

**File:** `src/eval/mgga_dispatch/mod.rs:108-113, 157-162`
**Issue:** Inside the macro, the polarized-mode guard and the
Fxc/Kxc/Lxc guard both construct `FunctionalId::from_raw(1)` (LDA_X)
for the error payload:
```rust
return Err(LibxcRsError::UnsupportedFunctional {
    id: crate::model::FunctionalId::from_raw(1).expect("valid id"),
    reason: "MGGA polarized dispatch deferred...",
});
```
In practice these branches are unreachable at Phase 4 because
`dispatch_mgga` rejects polarized spin and high orders before calling
into any batch helper. But if the macro ever gets exercised directly
(e.g., a future refactor that inlines a dispatch helper outside
`dispatch_mgga`), the error would misreport the offending functional as
`lda_x`. Users debugging test output would be misled.
**Fix:** Thread the functional id through the macro so the error
reports it correctly, or delete the guards entirely since
`dispatch_mgga` already enforces them:
```rust
// Option A — accept an id argument:
macro_rules! mgga_zero_scalar_unpol_dispatch {
    (
        $ctx:expr, $order:expr, $spin:expr,
        [$($exc_u:tt)::+], [$($vxc_u:tt)::+],
        $functional_name:literal,
        $id:expr
    ) => {{ ... }}
}

// Option B — delete the two guards (they duplicate dispatch_mgga's).
```

### WR-02: `xtask verify-phase-4` loses per-family structure when a family test panics

**File:** `xtask/src/verify_phase_4.rs:91-128`
**Issue:** `run_family` calls `cargo test`, parses the structured
summary lines, then bails via `bail!("{test_binary} exited with status …")`
if `!out.status.success()`. That means when (say) `lda_oracle` panics
but `gga_oracle` and `mgga_oracle` would have reported clean, the
harness returns the first error and never invokes the remaining
families. Downstream users relying on `print_phase_4_summary` to see a
cross-family matrix will only get the LDA block, even though the GGA
and MGGA results are obtainable and informative.
**Fix:** Capture the per-family `FamilyReport` even on failure and
surface the aggregated report before returning a non-zero exit status,
e.g. move `bail!` to after `run_phase_4_verification` returns — or
collect failures into `Phase4Report::failed_families: Vec<String>` and
keep going:
```rust
let lda = run_family("lda_oracle").ok();
let gga = run_family("gga_oracle").ok();
let mgga = run_family("mgga_oracle").ok();
// assemble report, then exit non-zero if any are None
```

### WR-03: LDA oracle harness counts vxc-only functionals as `tested` without comparing any derivative

**File:** `verify/tests/lda_oracle.rs:238-240, 401-405, 407-410`
**Issue:** For a vxc-only functional like `lda_xc_tih`
(`has_exc()==false`), the outer test guard at line 401
(`functional.has_exc() && (flags & FLAGS_HAVE_EXC) == 0`) evaluates to
`false && …` = `false`, so it does NOT skip and falls through to
`compare_lda_functional`. `compare_lda_functional` immediately returns
`Ok(())` at line 239 because `flags & FLAGS_HAVE_EXC == 0`. The loop
then increments `tested += 1` at line 408 — but no Rust-vs-C
comparison was actually done. The comment at lines 231–237 explains why
the oracle cannot be invoked for these functionals (it `exit(1)`s), so
skipping is the right runtime behavior, but the accounting advertises
them as tested.
**Fix:** Categorize vxc-only functionals with no oracle exc support as
`SkippedNoOracle` (new counter) rather than `tested`. Or, have
`compare_lda_functional` return an enum `CompareOutcome::Tested |
SkippedNoOracle` (as the GGA and MGGA harnesses already do) and update
the counting accordingly. Keeps summary lines honest.

## Info

### IN-01: `src/eval/gga_dispatch/mod.rs` top-level module allows three cargo-warning lints

**File:** `src/eval/gga_dispatch/mod.rs:23`
**Issue:** The module-level attribute
`#![allow(clippy::too_many_arguments, clippy::single_match, unused_mut, unused_variables)]`
blankets the whole dispatch module, including the `dispatch_gga` entry
function where `unused_variables` and `unused_mut` would usefully flag
wiring bugs. Batch submodules already carry the same allow-list at
their own top level, so loosening the parent module is redundant.
**Fix:** Trim to `#![allow(clippy::too_many_arguments,
clippy::single_match)]` on the parent and let batch files keep their
`unused_imports` / `unused_variables` allow (they're needed for the
no-scalar stubs).

### IN-02: `dispatch_gga` macro argument missing functional name for better panic messages

**File:** `src/eval/gga_dispatch/mod.rs:125, 129, ...`
**Issue:** The `ten_arm_dispatch_gga!` macro uses
`.expect("zk handle missing for Exc+ order on exc-bearing functional")`
style panics when an Option handle is None. These are unreachable in
practice because `dispatch_gga` allocates them based on `order`. If
they ever fire (e.g. during refactors) the message does not include
which functional triggered it. The MGGA macro (`mgga_zero_scalar_unpol_dispatch!`)
already takes a `$functional_name:literal` — adopt the same pattern.
**Fix:** Accept and weave `$functional_name:literal` into each
`.expect(...)` message:
```rust
let zk_arg = || {
    let h = $ctx.zk.expect(concat!(
        "zk handle missing for ", $functional_name, " at Exc+ order"
    ));
    unsafe { ArrayArg::from_raw_parts::<f64>(h, $ctx.zk_len, 1) }
};
```

### IN-03: `tools/generate_gga_dispatch.py` vxc-only branch is dead code

**File:** `tools/generate_gga_dispatch.py:684-698`
**Issue:** The `if has_exc == 0:` branch emits a fallback
`UnsupportedFunctional` stub, but the preceding `if scalars:` branch
already captures the only vxc-only functional (`gga_x_lb`, which has
3 scalars). The comment on line 685-686 acknowledges this: "We know it
has scalars, so we actually fall into the scalar branch above". The
branch has never been reached. It is harmless but misleading to future
readers.
**Fix:** Either delete the branch and add a `# gga_x_lb is covered by
the scalar stub path` comment, or explicitly emit a different reason
string for vxc-only so the dead branch becomes reachable if a
future zero-scalar vxc-only kernel is added.

### IN-04: Duplicate per-field default constants in `dispatch.rs` hardcoded parameters

**File:** `src/eval/dispatch.rs:1043-1062, 1067-1081`
**Issue:** `launch_lda_c_chachiyo` and `launch_lda_c_chachiyo_mod`
duplicate the same set of 6 magic numbers. The comment on line 1067
says "lda_c_chachiyo_mod uses the same par_chachiyo defaults"; a shared
`const` array would make that relationship explicit and avoid drift.
Similarly `launch_lda_xc_1d_ehwlrg` is called three times (lines
196–207) with three different hardcoded parameter tuples, one for each
of the `LdaXc1dEhwlrg{1,2,3}` variants — those could also live in a
shared `const` array.
**Fix:** Hoist the hardcoded defaults into module-level `const` arrays
(or a small helper function) so the per-variant assignments become
a single source of truth. Example:
```rust
const CHACHIYO_PAR: [f64; 6] = [
    -0.007772675, -0.01554535,
    27.4203609, 20.4562557,
    27.4203609, 20.4562557,
];
```
and pass it to both launchers.

### IN-05: `compare_lda_functional` returns `Ok(())` as both "clean" and "skipped"

**File:** `verify/tests/lda_oracle.rs:238-240, 331`
**Issue:** The function returns `Ok(())` in two semantically different
cases: (1) after all derivative orders matched within tolerance (line
331), and (2) when the oracle has no EXC support and the function
early-returns without comparing anything (line 239). Combined with
WR-03, this makes the tested/skipped counters hard to reason about.
**Fix:** Return an enum (or add a second return channel) distinguishing
"compared, all within tolerance" from "skipped, cannot compare".
GGA/MGGA harnesses already use an `enum CompareResult` — adopt the
same shape for LDA for consistency.

### IN-06: `LdaFunctional` variant comments are doc-style but scattered inline

**File:** `src/model/lda_functional.rs:29-67`
**Issue:** Variant-level comments (`// 588`, `// 4`) are side comments
rather than `///` doc strings. For the GGA enum (`src/model/gga_functional.rs:40-304`)
and the MGGA enum (`src/model/mgga_functional.rs:62-112`) the same
comments are written as `///` doc-strings. Mixing styles within the
same crate is a small readability drift.
**Fix:** Promote the LDA variant comments to `///` doc lines to match
the GGA/MGGA enums:
```rust
/// `hyb_lda_xc_bn05` (libxc id 588)
HybLdaXcBn05,
/// `lda_c_1d_csc` (libxc id 18)
LdaC1dCsc,
```

### IN-07: `oracle_mgga_all_with_opts` does not null-pointer-guard the LXC tier

**File:** `verify/src/lib.rs:623-638`
**Issue:** The oracle function gates LXC on `flags & FLAGS_HAVE_LXC != 0`
and calls `xc_mgga_lxc` — but it passes plain `.as_mut_ptr()` for all 35
LXC output buffers without any per-level nullptr guard. The LDA and
GGA oracle paths use `np_()` / explicit `if has_fxc` selectors to pass
`null_mut()` for unsupported derivative levels (see lines 288-294 for
GGA). If a functional has HAVE_LXC flag set but some internal
sub-derivative path is missing, this could mismatch the LDA pattern.
In practice, MGGA lxc is all-or-nothing in libxc so this matches
libxc's semantics today, but the inconsistency with the GGA/LDA
handling is worth noting.
**Fix:** Document the MGGA all-or-nothing LXC contract inline, or
adopt the same `has_*` selectors for symmetry:
```rust
// MGGA's xc_mgga_lxc requires *all* 4th-order buffers or *none* —
// unlike GGA where sub-orders can be nulled. We gate the whole call.
```

### IN-08: `mgga_oracle.rs` TOL_FXC/TOL_KXC/TOL_LXC carry `#[allow(dead_code)]`

**File:** `verify/tests/mgga_oracle.rs:204-209`
**Issue:** Three tolerance constants are annotated `#[allow(dead_code)]`
because Phase 4 scope limits the MGGA oracle harness to Exc+Vxc arms.
The constants are kept for eventual Fxc+ expansion but currently
unused. That's fine, but the allow annotation is load-bearing — a
future developer removing one of these constants during cleanup could
inadvertently ship a regression where the oracle silently skipped an
Fxc comparison.
**Fix:** Either wire in at least one Fxc check for a zero-scalar
functional (to keep the constant live), or move the constants into a
`mod unused_until_follow_up { … }` block with a single allow(dead_code)
attribute so their intent is scoped:
```rust
#[allow(dead_code)]
mod follow_up_tiers {
    pub const TOL_FXC: f64 = 1e-8;
    pub const TOL_KXC: f64 = 1e-6;
    pub const TOL_LXC: f64 = 1e-4;
}
```

### IN-09: `src/lib.rs` top-level `#![deny(warnings)]` combined with wide `#![allow(...)]`

**File:** `src/lib.rs:1-7`
**Issue:** The crate roots `#![deny(warnings)]` then immediately
`#![allow(clippy::excessive_precision, clippy::needless_late_init,
clippy::too_many_arguments)]`. `deny(warnings)` is broad — it will
also promote unrelated future lint categories (e.g.
`dead_code`, `unused_imports`) to errors. Any such lint triggered by
dispatch scaffolding would fail compilation rather than surface as
a warning first. Given the Phase 4 stubs return `UnsupportedFunctional`
with unused `_ctx`/`_order`/`_spin` parameters, the module-level
`#![allow(unused_imports, unused_variables, ...)]` is what is keeping
the build green in batch files. That's fragile.
**Fix:** Replace `#![deny(warnings)]` with the narrower
`#![deny(rust_2018_idioms, future_incompatible, nonstandard_style)]`
or similar, and lean on CI to fail on `warnings` via
`RUSTFLAGS="-D warnings"` instead of bending the crate-root lint
policy. That keeps intent explicit and leaves future lints as
warnings during local development.

---

_Reviewed: 2026-04-24T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
