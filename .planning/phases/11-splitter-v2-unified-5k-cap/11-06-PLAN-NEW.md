---
phase: 11-splitter-v2-unified-5k-cap
plan: 06
type: execute
wave: 2
depends_on: [11-05]
files_modified:
  - tools/translate_v2/cse.py
  - crates/kernels/math/src/piecewise.rs
  - crates/kernels/math/src/powers.rs
  - crates/kernels/math/src/erf.rs
  - crates/kernels/math/src/lambert_w.rs
  - crates/kernels/math/src/bspline.rs
  - crates/kernels/math/src/br89.rs
  - crates/kernels/math/src/bessel.rs
  - crates/kernels/math/src/dft_quantities.rs
  - crates/kernels/math/src/spin.rs
  - crates/kernels/math/src/integrate.rs
  - crates/kernels/math/src/polynomials.rs
  - crates/kernels/math/src/mbrxc.rs
  - crates/kernels/math/src/special.rs
  - crates/kernels/math/src/expint_e1.rs
autonomous: true
requirements: []
user_setup: []

must_haves:
  truths:
    - "CSE pass in cse.py correctly handles D-02 Option A literal wrapping (F::new for all f64 literals)"
    - "All 38 math/src/ helpers updated to generic signature matching D-02 Option A choice from 11-05"
    - "Math crate test code API drift fixed (CubeCL 0.10 ArrayArg::from_raw_parts signature)"
    - "Math crate tests compile without errors"
  artifacts:
    - path: "tools/translate_v2/cse.py"
      provides: "D-16 AST emit visitor for literal wrapping"
    - path: "crates/kernels/math/src/"
      provides: "38 helpers refactored to generic <F: Float> (per D-02 Option A locked in 11-05)"
  key_links:
    - from: "cse.py AST visitor"
      to: "helper signatures"
      via: "literal-wrap detection for generic F"
      pattern: "F::new"

---

<objective>
**What:** Fix math crate test-API drift from CubeCL 0.10 and consolidate translator f64-literal wrapping per the D-02 ABI choice locked in 11-05 (Option A: generic helpers).

**Purpose:** Prepare the translator to emit correctly for the full 266-subcrate regen (11-07). This is the second part of the D-02 validation: helpers are refactored in 11-05; translator emit logic and test drift are fixed here.

**Output:** Math crate compiles, translator ready for full regen.
</objective>

<execution_context>
@.planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md (D-16, D-05)
@.planning/phases/11-splitter-v2-unified-5k-cap/11-04-SUMMARY.md (four-layer bug structure)
@.planning/quick/260515-q01-cse-chunk-arity-cap-12/SPIKE-FINDINGS.md
</execution_context>

<context>
@CLAUDE.md
@.planning/ROADMAP.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Fix ArrayArg::from_raw_parts API drift (CubeCL 0.10)</name>
  <files>
    crates/kernels/math/src/dft_quantities.rs
    crates/kernels/math/src/powers.rs
    crates/kernels/math/src/polynomials.rs
    crates/kernels/math/src/bspline.rs
    crates/kernels/math/src/erf.rs
    crates/kernels/math/src/br89.rs
    crates/kernels/math/src/bessel.rs
    crates/kernels/math/src/expint_e1.rs
    crates/kernels/math/src/integrate.rs
    crates/kernels/math/src/special.rs
    crates/kernels/math/src/piecewise.rs
    crates/kernels/math/src/mbrxc.rs
    crates/kernels/math/src/lambert_w.rs
    crates/kernels/math/src/spin.rs
  </files>
  <action>
Update all ~165 `ArrayArg::from_raw_parts` call sites in `#[cfg(test)]` test code from the CubeCL 0.9 signature to the CubeCL 0.10 signature.

**Old (0.9):** `ArrayArg::from_raw_parts::<f64>(&handle, n, 1)`  
**New (0.10):** `ArrayArg::from_raw_parts(handle, n)` (by-value handle, 2 args)

**Process:** For each file, use `grep -n "from_raw_parts" <file>` to find all call sites, then batch-replace. Key changes:
- Remove turbofish `::<f64>`
- Change `&handle` to `handle` (by-value)
- Remove the third argument `, 1`

**Verify:** `cargo test -p libxc-kernel-math --lib -- --nocapture` must pass all tests.
  </action>
  <verify>
    <automated>cargo test -p libxc-kernel-math --lib -- --nocapture 2>&1 | tail -20</automated>
  </verify>
  <done>All 165 `from_raw_parts` call sites updated; math tests compile and pass.</done>
</task>

<task type="auto">
  <name>Task 2: Implement D-16 AST literal-wrapping visitor in cse.py</name>
  <files>tools/translate_v2/cse.py</files>
  <action>
Extend `cse.py` with an AST-level visitor that emits the correct f64-literal wraps for chunks calling generic helpers (D-02 Option A, locked in 11-05).

**What to add:**
1. After `partition_compute_lines()` returns chunk boundaries, add an AST visitor pass that:
   - Walks each chunk's Rust tokens/AST
   - Detects all f64 float literals (`2.5`, `0.123e-1`, `2e-21`)
   - Detects named-constant references (`M_PI`, `M_CBRT3`, etc.)
   - Emits `F::new(<literal>)` for each (since helpers expect `F` args, not f64)

2. Integration point: This visitor runs **after** `translate_expr` converts Maple lines to Rust expressions, **before** the chunk function body is written to file.

3. Mark or directly emit wrapped form in the returned chunk (exact mechanism depends on how `per_functional.py`'s `emit_cse_chunked_output` is structured — may need light refactoring there too).

**Verify:**
- `python3 tools/translate_v2/cse.py --selftest` exits 0 (idempotency)
- A test regen of one functional (`--family lda --function lda_x`) produces chunks with `F::new(2.5)` visible in generated `.rs` files

**Rationale per D-16:** The q01 brief showed that literal wrapping is correctness-critical; consolidating it into one AST pass (not multiple regexes in per_functional.py) reduces fragmentation and makes future maintenance clearer.
  </action>
  <verify>
    <automated>python3 tools/translate_v2/cse.py --selftest && echo "PASS: cse.py selftest"</automated>
  </verify>
  <done>D-16 AST visitor implemented in cse.py; idempotency test passes; chunk emission includes correct literal wrapping.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| CubeCL 0.10 API | `ArrayArg::from_raw_parts` signature change — tests use new API. Type system enforces correctness. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-11-06-01 | I | CubeCL 0.10 signature compatibility | mitigate | Verify and test via `cargo test -p libxc-kernel-math --lib` |
| T-11-06-02 | R | D-16 AST visitor correctness | mitigate | `--selftest` idempotency gate ensures determinism |
</threat_model>

<verification>
- [ ] All `from_raw_parts` calls updated to CubeCL 0.10 signature
- [ ] `cargo test -p libxc-kernel-math --lib` passes
- [ ] `cse.py --selftest` exits 0
- [ ] Test regen shows `F::new` wrapping in generated chunks
</verification>

<success_criteria>
1. Math crate tests pass with CubeCL 0.10 API.
2. CSE emit logic handles D-02 Option A literal wrapping.
3. No regression in prior waves (11-01..03).
4. Translator ready for full 266-subcrate regen (11-07).
</success_criteria>

<output>
After completion, create `.planning/phases/11-splitter-v2-unified-5k-cap/11-06-SUMMARY.md`
</output>
