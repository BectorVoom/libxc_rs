---
phase: 05-functional-lifecycle-and-hybrid-properties
reviewed: 2026-04-28T03:54:13Z
depth: standard
files_reviewed: 31
files_reviewed_list:
  - Cargo.toml
  - libxc-sys/Cargo.toml
  - libxc-sys/build.rs
  - libxc-sys/src/lib.rs
  - src/error/mod.rs
  - src/eval/dispatch.rs
  - src/eval/gga_dispatch/mod.rs
  - src/eval/mgga_dispatch/mod.rs
  - src/eval/mix.rs
  - src/eval/mod.rs
  - src/functional/config.rs
  - src/functional/evaluate.rs
  - src/functional/hybrid.rs
  - src/functional/lifecycle.rs
  - src/functional/mod.rs
  - src/functional/params.rs
  - src/functional/params_gga.rs
  - src/functional/params_lda.rs
  - src/functional/params_mgga.rs
  - src/lib.rs
  - src/meta/mod.rs
  - src/model/mod.rs
  - verify/Cargo.toml
  - verify/build.rs
  - verify/src/oracle_ffi.rs
  - verify/tests/hybrid_oracle.rs
  - verify/tests/hybrid_type_oracle.rs
  - verify/tests/lda_oracle.rs
  - verify/tests/metadata_oracle.rs
  - verify/tests/mixed_oracle.rs
  - xtask/Cargo.toml
  - xtask/src/generate_metadata.rs
  - xtask/src/main.rs
findings:
  blocker: 7
  warning: 12
  total: 19
status: issues_found
---

# Phase 5: Code Review Report

**Reviewed:** 2026-04-28T03:54:13Z
**Depth:** standard
**Files Reviewed:** 31
**Status:** issues_found

## Summary

Phase 5 introduces the `Functional` runtime handle with eager recursive auxiliary
construction, the `FunctionalParams` plumbing trait + ~80 per-functional impls,
hybrid classification (`classify_hybrid`), CAM/NLC query methods, mixed-evaluation
accumulators (`evaluate_mixed_lda_functional` / `evaluate_mixed_gga` /
`evaluate_mixed_mgga`), the `libxc-sys` workspace crate factoring out C linkage,
and re-routes verify FFI tests through it.

The core architecture is sound and the Pitfall mitigations called out in the
plan (per-aux family gating, Pitfall 5; deferred-id construction, Pitfall 7;
empty propagation rules guard) are implemented correctly. However the review
surfaces multiple **BLOCKER** classes of defects:

1. **xtask `generate-metadata` is broken** (writes structurally invalid Rust
   that will not compile when run, and silently emits empty stub files). Phase
   5 explicitly defers populating the metadata, but the tool that is supposed
   to populate it is not actually capable of doing so. The phase has *not*
   produced a usable metadata generator, and the `#[ignore]`-d oracle tests
   that depend on it will never be unignorable until this is fixed.
2. **Numerical-correctness liabilities** in `evaluate_mixed_gga` /
   `evaluate_mixed_mgga` from passing entire scratch slices to `add_opt`
   (which silently truncates on length mismatch instead of erroring). When a
   future change accidentally reshapes scratch, this will produce wrong
   results without any test failure.
3. **MGGA mixed accumulator does not honor parent's `NEEDS_LAPLACIAN`/
   `NEEDS_TAU` gating** — only the aux's flags are consulted, which violates
   `mix_func.c` semantics when the parent wants to drop those derivatives
   regardless of aux contributions.
4. **`set_ext_param_by_index` panic path** when `ext_params` is `None` —
   silent crate panic instead of typed error.
5. **Multiple `expect()`/`panic!` paths in production dispatch macros** that
   trigger when output handles are missing for higher-order tiers; the
   surrounding plumbing currently always allocates them, but the safety net
   between unrelated callers is brittle.
6. **`xc_func_init` return code is not checked** in `verify/tests/hybrid_oracle.rs`
   `ffi_cam` / `ffi_exx`, and metadata oracle `xc_func_end` is called even
   when init fails (leak / use-after-free risk).
7. **Workspace metadata mismatch** — `libxc_rs` Cargo.toml has all per-functional
   `libxc-kernel-mgga-*` crates as `[dev-dependencies]` instead of regular
   dependencies; the phase did not add the `libxc-kernel-gga-*` crates as
   dependencies anywhere visible. Compile-time wiring is fragile.

The pure-Rust correctness (`classify_hybrid`, `propagate_to_aux`, deferred-id
constructor success) is verified by the snapshot-equivalence test, but only
because the snapshot is empty — the test will become trivial-passing once
metadata is populated only because every functional snapshot says
`Semilocal`. The "rust port matches snapshot for all 649" test does NOT
exercise hybrid logic in any meaningful way today.

---

## BLOCKERS

### CR-01: `xtask generate-metadata` produces invalid Rust, will overwrite production files with stubs (BLOCKER)

**File:** `xtask/src/generate_metadata.rs:37-48,56-80`

**Issue:**
1. `collect_all_functionals()` returns `Vec::new()` unconditionally (line 38-47).
   The comment says "This is a placeholder — in reality, we'd iterate
   `all_functional_ids()`", but this loop is *not* implemented. The function
   then asserts `!functionals.is_empty()` at the call site (line 14) — so
   running `cargo xtask generate-metadata` will **always** fail at the
   `ensure!` call, masking that the tool itself is unusable.
2. `write_generated_rs` (line 56-80) emits invalid Rust:
   ```rust
   pub(crate) const FUNC_{name}: FunctionalMeta = FunctionalMeta {
       id: FunctionalId({}),
       name: "{}",
       // ... populated by full implementation
   };
   ```
   The `// ... populated by full implementation` comment leaves the struct
   missing 11 required fields. This will not parse as Rust. If the
   `ensure!` in `run()` were ever skipped (or a single functional gets
   added), this would corrupt `src/meta/generated.rs` (which is a real
   committed file) with a non-compiling stub, breaking the whole crate.
3. `write_generated_hybrid_rs` and `write_generated_propagation_rs` write
   the file path **unconditionally** even when `functionals` is empty — so
   the safety net in `run()` (which `ensure!`s non-empty) is the only thing
   protecting the committed files. If anyone refactors that ordering, all
   three generated files become empty stubs and commit damage.
4. Phase 5 explicitly relies on this tool to populate `meta.hybrid_terms`,
   `meta.auxiliaries`, `meta.nlc_params`, etc. The `#[ignore]` oracle tests
   in `verify/tests/hybrid_oracle.rs` and `verify/tests/mixed_oracle.rs`
   are gated on running this tool. The tool **cannot** produce the data
   they require — it is a structurally non-functional placeholder
   masquerading as a deferred completion. Plan 05-01's "deferred metadata
   population" is not actually deferred, it is *abandoned* by the current
   tooling.

**Fix:** Either (a) remove the `xtask generate-metadata` subcommand and the
ignored oracle tests until a working implementation exists (so reviewers and
downstream agents do not assume a tool is in-progress), or (b) implement the
FFI introspection loop:
```rust
fn collect_all_functionals() -> Result<Vec<SnapshotMeta>> {
    let mut out = Vec::new();
    for &id in libxc_rs::registry::all_functional_ids() {
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let rc = unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
        if rc != 0 { continue; }
        // Snapshot info.flags, info.references, info.ext_params, info.aux_ids,
        // xc_hyb_type(&t), xc_hyb_cam_coef(&t,...), t.nlc_b, t.nlc_C ...
        out.push(snapshot_func_type(id, &t)?);
        unsafe { xc_func_end(&mut t); }
    }
    Ok(out)
}
```
And replace `write_generated_rs`'s placeholder string with a real emitter
that writes all 14 `FunctionalMeta` fields.

---

### CR-02: `evaluate_mixed_gga` / `evaluate_mixed_mgga` silently truncate on length mismatches via `add_opt` (BLOCKER)

**File:** `src/eval/mix.rs:202-212` (definition), and every `add_opt(...)` call in `evaluate_mixed_gga`/`evaluate_mixed_mgga` (lines 394-463, 551-685)

**Issue:** `add_opt` takes the smaller of `dst.len()` and `src.len()` and silently
clamps:
```rust
fn add_opt(dst: Option<&mut [f64]>, coeff: f64, src: &[f64]) {
    if let Some(d) = dst {
        let n = d.len().min(src.len());
        for i in 0..n { d[i] += coeff * src[i]; }
    }
}
```
Callers in `evaluate_mixed_gga` pass the **entire scratch slice** (e.g.
`scratch.vrho` — which is sized for the workspace's MGGA-superset dims, not
the GGA-aux's actual output count for this `np`). Today this *happens to
work* because (a) the workspace allocates with `Dimensions::mgga(spin)` and
that has `vrho == 2` (polarized) which equals GGA's `vrho == 2`; (b) all
allocations multiply by `np`. But the contract is silently load-bearing on
this coincidence:

- If the workspace ever sizes scratch with a different `np` than the input
  (which already would trigger `WorkspaceMismatch` — but the check uses
  `==` and could be relaxed in future), `add_opt` would *silently* compute
  partial results without erroring.
- The pre-existing `evaluate_mixed_lda` (lines 109-194) does NOT use
  `add_opt` — it slices the scratch by `..zk_len`, `..vrho_len`, etc. and
  uses `add_to_mix` (which `debug_assert_eq!`s lengths). The two
  accumulator paths have **different correctness contracts**: `add_to_mix`
  panics in dev / requires equal length; `add_opt` never errors.
- `evaluate_mixed_mgga` MGGA-aux path passes `scratch.v2rho2`,
  `scratch.v2rhosigma`, etc. (length `dims.{field} * np` from the
  *workspace's* MGGA dims) into `add_opt(output.{field}.as_deref_mut(),
  ...)`. The output field lengths are validated only by the OutputBufferSize
  check inside dispatch — once that passes for the aux, this is fine, but
  the lack of explicit length matching (or even debug_assert) means a
  future bug in scratch sizing would corrupt accumulator output.

**Why this is a BLOCKER:** the 10^-12 oracle precision target depends on
exact accumulation. A length-min truncation that omits even one trailing
component (e.g., the σ-σ' cross derivative of a polarized GGA) will produce
silent wrong answers that pass unit tests (which are unpolarized at np=4)
but fail the oracle suite once it is unblocked. The defect is currently
hidden by metadata being empty (no aux ever runs); it activates the moment
metadata is populated.

**Fix:**
1. Replace `add_opt` with an explicitly-sized version that takes a `len`
   parameter and panics or returns an error on mismatch:
   ```rust
   fn add_opt_n(dst: Option<&mut [f64]>, coeff: f64, src: &[f64], len: usize)
       -> Result<(), LibxcRsError>
   {
       if let Some(d) = dst {
           if d.len() != len || src.len() < len {
               return Err(LibxcRsError::OutputBufferSizeMismatch {
                   field: "mixed accumulator", expected: len, actual: d.len(),
               });
           }
           for i in 0..len { d[i] += coeff * src[i]; }
       }
       Ok(())
   }
   ```
2. Compute the per-family per-field lengths once at the top of each
   `evaluate_mixed_*` function (mirroring lines 247-251 of
   `evaluate_mixed_lda_functional`) and thread them through.

---

### CR-03: `evaluate_mixed_mgga` ignores parent functional's `NEEDS_LAPLACIAN`/`NEEDS_TAU` flags (BLOCKER)

**File:** `src/eval/mix.rs:617-685`

**Issue:** The MGGA-aux path consults only the **aux's** flags
(`aux.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN)`) to decide
whether to populate `vlapl`/`vtau` on the aux's output, and to gate the
accumulation back into the parent. The parent's own flags are never
consulted. `mix_func.c:184-305` (referenced in the doc comment) gates BOTH
ways — the parent's `vlapl`/`vtau` slots are filled only if the parent
needs them. If the parent functional doesn't need laplacian (no
`NEEDS_LAPLACIAN`) but an aux does, the parent will still receive
contributions to `vlapl` because the loop does
`add_opt(output.vlapl.as_deref_mut(), ...)` whenever the aux is laplacian-
bearing. The caller passed `output.vlapl = Some(buffer)` because that's
the API contract for asking for that derivative — but the parent's
declared flags should determine whether such accumulation is meaningful.

This produces a subtle correctness drift: a hybrid MGGA built from a
laplacian-needing aux + a non-laplacian-needing aux would leak vlapl
contributions out of the first aux without the second contributing to it,
giving wrong values for the laplacian-derivative of the *combined*
functional.

**Fix:** Add a check at the top of the MGGA-aux branch:
```rust
let parent_needs_lapl = functional.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
let parent_needs_tau  = functional.meta.flags.contains(FunctionalFlags::NEEDS_TAU);
let needs_lapl = aux.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN) && parent_needs_lapl;
let needs_tau  = aux.meta.flags.contains(FunctionalFlags::NEEDS_TAU)       && parent_needs_tau;
```
Or better: validate at the top of `evaluate_mixed_mgga` that the aux flag
union equals the parent's flag union, and reject inconsistent combinations
with a typed error (matching libxc's `xc_func_set_dens_threshold` lineage).

---

### CR-04: `set_ext_param_by_index` will panic when `ext_params` is `None` and bounds-check passes only because `count==0` (BLOCKER)

**File:** `src/functional/config.rs:91-107`

**Issue:** The flow is:
```rust
pub fn set_ext_param_by_index(&mut self, idx: usize, val: f64) -> Result<(), LibxcRsError> {
    let count = self.meta.ext_params.len();
    if idx >= count { return Err(...); }                         // ← only catches idx >= count
    let mut new_vals: Vec<f64> = self
        .ext_params
        .as_deref()
        .map(|s| s.to_vec())
        .unwrap_or_default();                                    // ← Vec::new() if ext_params is None
    new_vals[idx] = val;                                         // ← panics if vec is empty
    self.set_ext_params(&new_vals)
}
```
If `meta.ext_params` is non-empty (count>0) but `self.ext_params` is `None`
(impossible in current `Functional::new`, but the invariant is not
locally enforced), `new_vals` is an empty `Vec` and `new_vals[idx] = val`
on line 105 panics out of bounds.

The invariant "ext_params is `Some` iff `meta.ext_params` is non-empty" is
maintained by the constructor but not a type-level guarantee — any future
code path that mutates `self.ext_params` to `None` (e.g., a `Clear` API) will
introduce a panic here. Defensive code should be added.

Also note the symmetric issue in `ext_param` (line 39-50): `arr[i]` on
line 43 will panic if `meta.ext_params` lists more entries than
`self.ext_params` has. Same invariant gap.

**Fix:**
```rust
let mut new_vals = self
    .ext_params
    .as_deref()
    .map(|s| s.to_vec())
    .unwrap_or_else(|| {
        self.meta.ext_params.iter().map(|s| s.default_value).collect()
    });
if new_vals.len() != count {
    return Err(LibxcRsError::ExtParamCountMismatch {
        id: self.meta.id, expected: count, actual: new_vals.len()
    });
}
new_vals[idx] = val;
self.set_ext_params(&new_vals)
```

---

### CR-05: `xc_func_init` return code ignored in verify FFI helpers — undefined behavior on failure (BLOCKER, FFI safety)

**File:** `verify/tests/hybrid_oracle.rs:23-38`, `verify/tests/mixed_oracle.rs:92-95`,
       `verify/tests/metadata_oracle.rs:14-15`

**Issue:**
```rust
fn ffi_cam(id: u16) -> (f64, f64, f64) {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) };  // ← rc discarded
    let (mut o, mut a, mut b) = (0.0_f64, 0.0_f64, 0.0_f64);
    unsafe { xc_hyb_cam_coef(&t, &mut o, &mut a, &mut b) };               // ← reads zeroed memory
    unsafe { xc_func_end(&mut t) };                                       // ← may double-free / use-after-free
    (o, a, b)
}
```
If `xc_func_init` returns non-zero (failure), `t` is left in the zeroed
state with NULL internal pointers. Calling `xc_hyb_cam_coef` and
`xc_func_end` on a zeroed/half-initialized `xc_func_type` is undefined
behavior in libxc and will likely crash the test process with no
indication of which functional failed. `metadata_oracle.rs:14-15` does
`assert_eq!(rc, 0, ...)` (good!) but `hybrid_oracle.rs:25,33` and
`mixed_oracle.rs:93,147` do not. (`mixed_oracle.rs:93,147` does check
`assert_eq!(rc, 0, ...)`; the bug is only in `hybrid_oracle.rs`.)

The same pattern repeats in `ffi_exx`. These are test-only files but
test crashes propagate into CI, masking other test failures. With the
larger 649-id loop in `metadata_oracle.rs::aux_ids_match_ffi_for_hybrids`
(line 68-75) the loop body silently ignores all failures (the placeholder
body is `// Deferred...` with no FFI calls — but if the body is filled in
without the rc check, this becomes a fleet-wide UB risk).

**Fix:**
```rust
let rc = unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) };
assert_eq!(rc, 0, "xc_func_init failed for id={id}");
```
Apply at every FFI call site in all five verify test files.

---

### CR-06: Pure-Rust crate `libxc_rs` has *all* MGGA kernel crates as `[dev-dependencies]` — runtime depends on dev configuration (BLOCKER, build correctness)

**File:** `Cargo.toml:16-127`

**Issue:** The root `[dependencies]` block (lines 6-14) only pulls in
`libxc-kernel-math`, `libxc-kernel-lda`, `libxc-kernel-gga`, `libxc-kernel-mgga`
(the family-aggregate crates). The 95 per-functional MGGA sub-crates
(`libxc-kernel-mgga-1a` through `libxc-kernel-mgga-37b`) and 58 per-functional
GGA sub-crates are listed under `[dev-dependencies]` (lines 19-127). Yet
`src/eval/mgga_dispatch/batch*.rs` and `src/eval/gga_dispatch/batch*.rs`
(the dispatch helpers) reference these crates' kernel functions directly.

This means:
- Running `cargo test --workspace` works (dev-dependencies are pulled in).
- Running `cargo build` or `cargo run` (release / library mode) produces a
  library that **does not link** the per-functional kernels — calling
  `dispatch_mgga(MggaXTpss, ...)` from a downstream consumer will fail to
  resolve symbols.
- `cargo check -p libxc_rs` will likely fail because dispatch sites resolve
  imports via dev-deps that are not in scope for the library compilation.

The mismatch is a Phase 4-era issue carried into Phase 5 (Phase 5 didn't
introduce it), but it's exposed because Phase 5 is the first phase whose
public API (`Functional::evaluate_mgga`) routes through these crates from
non-test code. If `cargo build -p libxc_rs` succeeds today it's only
because the parent crate functions never resolve those modules in
non-`#[cfg(test)]` code paths — but the dispatch table inside
`src/eval/mgga_dispatch/batch*.rs` is *not* test-only.

**Fix:** Move the 95 MGGA + 58 GGA kernel crates from `[dev-dependencies]`
to `[dependencies]`. Verify via
`cargo check -p libxc_rs --no-default-features` that the library builds.

---

### CR-07: `dispatch_gga` / `dispatch_mgga` macros call `.expect()` for handles that the surrounding code claims to allocate, but the proof is structural (BLOCKER, defensive)

**File:** `src/eval/gga_dispatch/mod.rs:125-184` (10 `.expect()` calls in `ten_arm_dispatch_gga!`),
       `src/eval/mgga_dispatch/mod.rs:120-138` (5 `.expect()` calls in `mgga_zero_scalar_unpol_dispatch!`)

**Issue:** The 10-arm GGA dispatch macro and the MGGA macro use `.expect()`
on `Option<&Handle>`:
```rust
let h = $ctx.zk.expect("zk handle missing for Exc+ order on exc-bearing functional");
```
The contract is that `dispatch_gga` (the outer entry point) allocates
all higher-order handles when `order >= DerivativeOrder::Vxc` (lines
371-417 of gga_dispatch). When that contract holds, these `.expect()`s
never trigger. But:

1. The macro is `#[allow(unused_macros)] pub(crate) use ten_arm_dispatch_gga;`
   and re-exported. Any future use site that doesn't go through the outer
   `dispatch_gga` (e.g. a test helper, a future Phase-6 caller) and forgets
   to allocate handles will get a *panic* instead of a typed error.
2. The "vxc-only case" (`GgaXLb`) goes through a *different* batch helper
   (`batch8d::dispatch_gga_x_lb`) that the review didn't load, but the
   `.expect("zk handle missing for Exc+ order on exc-bearing functional")`
   suggests the same macro is reused for vxc-only kernels — which would
   panic immediately because `zk_handle` is `None` for `GgaXLb`.

The dispatch entry point at lines 362-367 sets:
```rust
let zk_handle = if functional.has_exc() {
    Some(create_zero_output_buffer(...))
} else { None };
```
So `zk_handle` is `None` for `GgaXLb`. But then any 10-arm macro invocation
for `GgaXLb` will hit `expect("zk handle missing...")` — and the dispatch
table's line 521 routes `GgaFunctional::GgaXLb => batch8d::dispatch_gga_x_lb`.
Whether this triggers depends on how `batch8d::dispatch_gga_x_lb` is
implemented (we did not load it). If it uses a vxc-only macro variant,
fine. If it forwards to `ten_arm_dispatch_gga!`, it panics on order==Vxc.

**Fix:** Replace `.expect(msg)` with `.ok_or_else(|| LibxcRsError::KernelLaunchFailed {
reason: msg.to_string() })?` (or `OutputBufferSizeMismatch`). Library
code MUST NOT panic on caller mistakes — this is foundational to
`Send + Sync` thread-safe usage and to the FFI compatibility goal.

The same pattern is in `mgga_zero_scalar_unpol_dispatch!` lines 120-138.

---

## WARNINGS

### WR-01: `propagate_to_aux` collects parent ext_params into a `Vec<f64>` per call — wastes alloc on every call, and "snapshot" is incomplete

**File:** `src/functional/lifecycle.rs:109-149`

**Issue:** The function clones the parent's `ext_params` into a `Vec` snapshot
to break borrow conflicts with the `&mut self.auxiliaries[...]` read in
the loop body. This is fine but every `set_ext_params(...)` -> `propagate_to_aux()`
re-clones; for the tight inner loop of a hybrid update, this is O(N_rules)
allocations. Not a correctness bug but a wart.

More importantly: the snapshot is taken once at the top, but if a rule
updates an aux which itself has further descendants whose propagation
should flow through, the cascading update is not run. Phase 5 plan
restricts depth to ≤ 2 (D-17), so two-level propagation is enough for
the current spec — but `propagate_to_aux` does NOT itself recurse into
the aux to call `aux.propagate_to_aux()` after `aux.set_ext_param`. This
means a parent → aux → grandaux propagation chain (depth 2) will not
re-fire grandaux propagation rules when the parent updates. If any
hybrid functional has a nested rule structure (CAM-B3LYP-style omega
threading two levels deep), the deep state will be stale.

**Fix:** Either (a) document and assert that `PROPAGATION_RULES` only ever
target depth-1 auxes (xtask invariant), or (b) recurse:
```rust
aux.set_ext_param(...)?;
aux.propagate_to_aux()?;  // cascade to grandaux, with depth bound 2
```

### WR-02: `Functional::cam_coefficients` two-term branch hardcodes ordering "ErfSr/YukawaSr/GaussianSr first, Fock second"

**File:** `src/functional/hybrid.rs:128-148`

**Issue:** The match arm only matches `(SR, Fock)` pairs but not the
reversed `(Fock, SR)`. `classify_hybrid` (lines 65-73) treats
`(Fock, ErfSr)` as `Mixture`, so cam_coefficients returns `None` for
that case — but a real-world CAM functional with Fock listed first
would silently return `None` for both queries instead of computing
omega/alpha/beta.

If xtask's metadata snapshot does not enforce the canonical ordering
(SR first, Fock second), real CAM-style hybrids will be misclassified
as `Mixture`. The libxc convention IS SR-first / Fock-second per
hybrids.c, but the validation that snapshot ordering matches happens
nowhere in this code.

**Fix:** Either (a) accept both orderings and pick the correct
coefficient by `kind`, or (b) add an xtask-time canonicalization that
reorders terms before snapshotting and a runtime debug_assert for the
ordering invariant.

### WR-03: `classify_hybrid` does not match `(Pt2, Fock)` reverse ordering for double-hybrid

**File:** `src/functional/hybrid.rs:65-73`

Same issue as WR-02 but for the double-hybrid case. `(Fock, Pt2)`
returns `Mixture` instead of `DoubleHybrid`. This appears intentional
("range-separated short-range term first, full-range Fock term
second") but the symmetry is undocumented and not validated by
xtask. Single-line comment says "single-term PT2 is unusual but not
forbidden; treat as Mixture so the call site does not surface a
misleading classification" but this only covers the 1-term case.

### WR-04: `evaluate_mixed_lda_functional` rejects non-LDA aux but `evaluate_mixed_gga` rejects MGGA aux — inconsistent; should reject GGA aux inside LDA parent for symmetric reasons

**File:** `src/eval/mix.rs:259-264, 466-470`

**Issue:** `evaluate_mixed_lda_functional` checks `aux.meta.family != Family::Lda`
and rejects with `UnsupportedFunctional { reason: "non-LDA auxiliary inside LDA parent" }`.
This is correct for the libxc semantics. But the rejection text /
behavior is asymmetric with `evaluate_mixed_gga`'s explicit "MGGA inside
GGA" check — `evaluate_mixed_lda_functional` rejects ANY non-LDA aux
silently as "non-LDA", lumping GGA and MGGA together. Future debug
output won't tell you which family was wrong. Cosmetic but in a
surface that's already error-paranoid.

### WR-05: `evaluate_mixed_lda_functional` validates spin/np match but not for `evaluate_mixed_gga`/`evaluate_mixed_mgga` aux that share the parent's input

**File:** `src/eval/mix.rs:228-235, 335-342, 495-502`

The workspace validation matches input. But the LDA aux inside a GGA
parent constructs `LdaInput::new(input.rho(), input.np(), input.spin())`
(line 372). For *polarized* GGA inputs, `input.rho()` has length
`2 * np` — exactly LDA's expected polarized rho length, so this is
correct. But the construction can fail (Result), and the caller
forwards `?`. If `LdaInput::new` ever validates against
`spin_dim * np * extra_factor` differently, this is silently broken.
Add a debug_assert that the LDA dim matches the GGA rho buffer dim,
or run the check upfront.

### WR-06: `Functional::new` propagation sequence ordering is fragile

**File:** `src/functional/lifecycle.rs:80-97`

**Issue:** The constructor:
1. Builds aux Vec (recursive).
2. Builds the `Functional` with default ext_params and the trait-object params.
3. Calls `propagate_to_aux()` AFTER assignment.

If a parent's `params.set_ext_params` (called in `set_ext_params`, but
NOT here in the constructor) modifies derived scalars, those derived
scalars will not influence the propagation. Today the default ext_params
flow through `meta.ext_params[i].default_value` directly into both
`self.ext_params` and `self.params` (via `construct_params`), so they
agree. But if a future custom `FunctionalParams` impl recomputes derived
scalars only on `set_ext_params`, the constructor will skip that
recomputation, leaving derived state stale. Recommend calling
`self.params.set_ext_params(self.ext_params.as_deref().unwrap_or(&[]))`
after construction to force the recomputation hook.

### WR-07: `evaluate_lda` deferred-id behavior ("construct succeeds but evaluate fails") relies on `LdaFunctional::from_id` returning Err

**File:** `src/functional/evaluate.rs:41-54`

This is documented Pitfall 7. The branch `auxiliaries.is_empty()` calls
`LdaFunctional::from_id(self.meta.id)?`. For deferred ids
(LDA_C_PK09=554, ...), `from_id` returns `UnsupportedFunctional` and
the `?` propagates. Good. But: if a future refactor adds the deferred
id to `LdaFunctional` enum, this branch will silently start *running*
the kernel — and tests expecting `UnsupportedFunctional` (e.g.
`evaluate_lda_deferred_id_returns_unsupported`) will fail. The contract
is fragile because the "deferred" status lives in two unrelated places
(the `from_id` match and `is_deferred` in `kernel-lda::deferred`). If
they drift, behavior diverges. Centralize deferral logic.

### WR-08: `propagate_to_aux` clones an empty `Vec` when there are no rules — non-zero overhead path even on the common case

**File:** `src/functional/lifecycle.rs:111-112`

`let ext_snapshot: Option<Vec<f64>> = self.ext_params.as_deref().map(|s| s.to_vec());`
allocates even when `PROPAGATION_RULES.iter().filter(|r| r.parent_id == id)`
yields zero matches (the common case for ALL non-hybrid functionals).
Move the filter into a peek and only allocate the snapshot if at least
one rule matches. Minor but called from every `Functional::new`.

### WR-09: `metadata_oracle.rs::snapshot_from_ffi` is unreachable code with a placeholder return

**File:** `verify/tests/metadata_oracle.rs:12-40`

The function exists but is never called (its only call site, line 50, is
commented out). Yet it allocates an `xc_func_type`, calls `xc_func_init`,
and then ignores the result. If ever uncommented without finishing the
"placeholder" assignments (line 25-35: `kind: Kind::Exchange // placeholder`),
the resulting `FunctionalMeta` snapshot will be wrong for every functional
and the `metadata_round_trip_all_649` test will produce 649 false
positives. Remove or finish the placeholder.

### WR-10: `evaluate_mixed_mgga` MGGA-aux body has dead `let _ = (needs_lapl, needs_tau, needs_both);` while branches still use them

**File:** `src/eval/mix.rs:653`

```rust
let _ = (needs_lapl, needs_tau, needs_both);
dispatch_mgga(...)
```
The `let _` is intended to silence "unused variable" warnings for the
Kxc/Lxc branches that aren't yet wired. But the variables ARE used in
the accumulation block at lines 662-686. The `let _ = (...)` is
misleading and stale — it should be removed (the variables are now
load-bearing).

### WR-11: `add_to_mix` debug_assert downgrades from a mismatch error to a release-build silent-truncation

**File:** `src/eval/mix.rs:42-46`

```rust
pub fn add_to_mix(dst: &mut [f64], coeff: f64, src: &[f64]) {
    debug_assert_eq!(dst.len(), src.len(), "...");
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d += coeff * *s;
    }
}
```
In release mode, `debug_assert_eq!` is a no-op. The `zip` iterator
silently truncates to `min(dst.len(), src.len())`. Same class of issue
as CR-02 but in a smaller scope. Convert to `assert_eq!` (always-on) or
return a `Result`.

### WR-12: `LdaXParams::new` allows arbitrary alpha (including NaN/inf) — no input validation

**File:** `src/functional/params_lda.rs:50-53`

```rust
pub fn new(alpha: f64) -> Self {
    let raw = Box::<[f64]>::from([alpha]);
    Self { alpha, raw }
}
```
No `is_finite()` check. A caller building `LdaXParams::new(f64::NAN)`
will produce a Functional that yields NaN energies — correctness
depends on caller discipline. Consider `Result<Self, LibxcRsError>` or
clamp/validate. Same applies to `set_ext_params` in
`params_lda::LdaXParams::set_ext_params` (line 71-82).

---

## Cross-cutting observations

- **Operation-order discipline (CLAUDE.md constraint "Maple2c formula
  translations must preserve floating-point operation order"):** The
  `add_to_mix` and `add_opt` accumulators iterate through auxiliaries in
  `Vec` order. The order is determined by `Functional::new`'s aux
  recursion, which iterates `meta.auxiliaries` in slice order. As long
  as xtask emits `meta.auxiliaries` in the same order libxc uses
  internally (mix_func.c iteration order), bit-equivalence holds. There
  is **no test** that asserts this ordering, and the `#[ignore]`-d
  oracle tests cannot expose drift while metadata is empty. Add a
  three-way ordering test as part of CR-01's xtask fix.

- **`libxc-sys` build.rs:** Looks correct. Does not validate that
  `bindgen` actually generated the expected symbols (e.g.,
  `xc_hyb_cam_coef`, `xc_hyb_exx_coef`); a future libxc version that
  renames or removes a symbol will produce a vague "unresolved symbol"
  link error. Consider `bindings.write_to_file(...)?` followed by a
  smoke test that the file contains expected names. The `allowlist_*`
  is broad (`xc_.*`/`XC_.*`), so this is best-effort.

- **`libxc-sys/src/lib.rs:** `#![allow(... clippy::all)]` is too broad.
  `clippy::all` masks real safety lints in the include!d FFI bindings;
  specifically allow only the lints bindgen needs (`non_snake_case`,
  `non_camel_case_types`, `non_upper_case_globals`, `dead_code`).

- **`Functional::Drop` (lifecycle.rs:152-159):** Correctly empty
  per D-15. Documented intent prevents future drift. ✓

- **`Functional` `Send + Sync` guarantees:** Compile-time enforced via
  `assert_send_sync::<Functional>()` in `mod.rs:81-89`. All `Box<dyn
  FunctionalParams + Send + Sync>` storage. ✓ — but note that the
  trait *bound* `FunctionalParams: Send + Sync` is on the trait
  definition, while the field type in `Functional` is just
  `Box<dyn FunctionalParams>`. This means the field type is
  `Box<dyn FunctionalParams + 'static>` (no Send+Sync), and Send+Sync
  on Functional comes from the *trait* bound implying it. This works
  in stable Rust (trait bound auto-applies via the dyn implementation)
  but is non-obvious. Recommend annotating
  `Box<dyn FunctionalParams + Send + Sync>` explicitly to avoid future
  drift if the trait bound is ever relaxed.

- **`error/mod.rs` is well-structured and fully tested.** Send+Sync
  asserted at line 152-155. All 22 variants have descriptive `#[error]`
  attributes. ✓

- **`classify_hybrid`'s 649-id sweep test passes trivially because
  every snapshotted `meta.hybrid_terms` is `&[]`.** This is the test
  that the doc comment claims "verifies the agreement at test time".
  Once metadata is populated, the test will actually exercise the
  classifier — but until then, the test provides no signal. Combined
  with CR-01 (xtask broken), there is currently no path to surface
  real classifier drift.

---

_Reviewed: 2026-04-28T03:54:13Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
