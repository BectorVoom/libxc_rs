# 260515-q01 — Fix three emit bugs in translate_v2/ for CubeCL 0.10

**Original framing (superseded):** Just cap CSE chunk arity at ≤12. → That fix is necessary but not sufficient. The spike revealed two more emit bugs that were masked by the wide-tuple trait-bound short-circuit.

**Spike findings:** `.planning/quick/260515-q01-cse-chunk-arity-cap-12/SPIKE-FINDINGS.md` — three emit bugs reproduced bare under `#[cube]` 0.10, two candidate fix idioms verified compiling. Use that as the empirical reference; this brief is the execution scope.

**User decision (recorded 2026-05-15):** Path 1 — enlarge q01 to fix all three; one agent, full fix + regen.

---

## Scope (three emit fixes + full regen + verification)

### Fix 1 — chunk-arity cap ≤12 (already in working tree, uncommitted)

`tools/translate_v2/cse.py:32`:
```python
-MAX_TUPLE_ARITY = 16            # cubecl-macros tuple arity cap (drop to 8 if needed)
+MAX_TUPLE_ARITY = 12            # CubeCL 0.10 CubeType derive ceiling (tuples <=12)
+                                # Verified 2026-05-15 via E0277 against 13/14/15-tuples
+                                # in 19 subcrates (11-04 Task 1B halt). Do NOT raise
+                                # above 12 without re-validating CubeType implementations.
```

Confirm `cse.py:229`'s `arity_forced = (inputs_arity >= MAX_TUPLE_ARITY or outputs_open >= MAX_TUPLE_ARITY)` semantics actually produce chunks with **≤12 outputs** after the constant change (the spike confirms 12 is at-ceiling and compiles; >12 fails). If there's an off-by-one such that chunks can reach 13, drop to 11 instead.

### Fix 2 — wrap f64 literals as `F::new(<lit>)` in chunk bodies

`tools/translate_v2/per_functional.py` and/or `tools/translate_v2/emit.py` — wherever the body line emit happens (the executor flagged `per_functional.py:217` `body.append(f"    let {var} = {rust_expr};")` as the likely site).

Current behavior emits raw Rust expressions like:
```rust
let t2622 = 0.21687162600603479684e-1 * t2621;
```
where `t2621: F` and `t2622: F`. CubeCL 0.10 fails this with `E0277: cannot multiply {float} by F` because f64 literals don't auto-coerce to generic `F: Float` under `#[cube]`.

**Fix:** wrap every numeric f64 literal in chunk-body expressions as `F::new(<literal>)`. Target shape:
```rust
let t2622 = F::new(0.21687162600603479684e-1) * t2621;
```

**Empirical validation:** see Q4a/b/c in the spike — `F::new(<lit>) * x` and `x / F::new(<lit>)` and `F::new(<lit>) + x` all compile bare under cubecl 0.10. The positive-regression tests now live at `crates/kernels/math/tests/spike_cse_emit_q01.rs::{F_new_literal_mul, F_new_literal_div, F_new_literal_add}` — preserve these as forward regression coverage.

**Implementation note:** the emit pass needs a regex / AST transform over the expression RHS to detect float-literal tokens (`\b\d+\.\d+(?:e[+-]?\d+)?\b` or similar) and wrap them as `F::new(...)`. Pure-integer literals (`0`, `1`, `2`) used as array indices or `u32` cube-builtins should NOT be wrapped — only `{float}` literals in arithmetic with `F`-typed operands. Verify by running the spike + the canary builds after.

**Concern to verify during emit-fix dev:** scientific notation literals like `0.21687162600603479684e-1` — does `F::new()` accept arbitrary precision f64 literals, or is there a precision-loss concern at the proc-macro boundary? Test with the canary build; if precision-loss flags, document and proceed (D-03a precision/operation-order policy is the relevant decision).

### Fix 3 — emit `-> F` (scalar) for single-output chunks instead of `-> (F,)`

Same files. When the CSE pass produces a chunk with `len(outputs) == 1`, the current emit produces:
```rust
#[cube]
pub fn ..._chunk1079<F: Float>(...) -> (F,) {
    let tv4rho3sigma6 = ...;
    let tv4rho3sigma8 = tv4rho3sigma6;
    (tv4rho3sigma8,)
}
```

This fails because `let tv4rho3sigma8 = tv4rho3sigma6;` inside `-> (F,)` triggers the macro's return-type inference: it expects `tv4rho3sigma8` to be `(NativeExpand<F>,)` but the RHS is `NativeExpand<F>`. The same error occurs for ANY `let` inside `-> (F,)` (not just bare-identifier aliases — verified by the spike's Q2b).

**Fix:** when a chunk's output set has cardinality 1, emit:
```rust
#[cube]
pub fn ..._chunk1079<F: Float>(...) -> F {
    let tv4rho3sigma6 = ...;
    let tv4rho3sigma8 = tv4rho3sigma6;
    tv4rho3sigma8
}
```

Drop the `(F,)` return type, drop the `(..., )` trailing-comma return-expression. **The chunk callsite must also be updated** — wherever `(x,) = chunk1079(...)` is emitted, change to `x = chunk1079(...)`. The callsite is also emitted by `per_functional.py`/`emit.py`; both sides of the call must move together.

**Empirical validation:** spike's `scalar_return_alias` and `scalar_return_compute` — both compile.

### Full regen — all 266 per-functional subcrates

Run the regen via `tools/maple_to_kernels.py` (preferred entry) or the three per-family translators (`translate_lda_v2.py`, `translate_gga.py`, `translate_mgga.py`) — whichever is the documented entry. Read CLAUDE.md / 11-02-SUMMARY.md if the entry-point convention is documented.

**RAM SAFETY:** the regen itself doesn't compile kernels, so it's RAM-cheap. But if you batch a `cargo build --workspace` afterwards you will OOM. Run cargo builds **sequentially, one `-p` at a time**, with `.cargo/config.toml`'s committed `jobs = 1`. Never set `CARGO_BUILD_JOBS` higher.

### Canary per-`-p` compile gates (must all exit 0)

```bash
cargo build -p libxc-kernel-gga_c_acggap     # was the first 11-04 Task 1B failure
cargo build -p libxc-kernel-lda_c_pw_erf     # widest LDA offender (121 wide chunks pre-fix)
cargo build -p libxc-kernel-mgga_c_revtpss   # widest MGGA (208 wide chunks pre-fix); PHASE11_WORST_CASE entry
cargo build -p libxc-kernel-gga_x_pbe        # PHASE11_SMOKE entry — narrow chunks only, must compile cleanly
cargo build -p libxc-kernel-lda_x            # PHASE11_SMOKE entry — simplest LDA
```

If any canary still fails:
- E0277 `(F × ≥13): CubeType` → Fix 1 incomplete; lower `MAX_TUPLE_ARITY`.
- E0277 `Mul<F> for {float}` (or `/`, `+`, `-`) → Fix 2 missed a literal; expand the regex/AST coverage.
- E0308 `expected (NativeExpand<F>,), found NativeExpand<F>` → Fix 3 missed a 1-tuple path; expand the chunk-signature special-case.
- Anything else → STOP, return CHECKPOINT REACHED with the cargo error.

### Invariant gates (must all pass post-regen)

```bash
python3 tools/audit_kernel_size.py --strict   # exit 0, 0 unexcepted >5K-line files
bash tools/audit_cube_launch.sh               # D-13 per-design budget still passes
bash tools/audit_subcrate_collapse.sh         # no family-level crates resurrected
bash tools/audit_dispatch_tree.sh             # exit 0 (no batchN refs)
```

### Idempotency (P11-INV-6)

Re-run the regen against the same functionals a second time. `git diff` must be empty.

### Static rescan (must be 0 wide chunks)

```bash
grep -rl --include='*.rs' -E '\(F, F, F, F, F, F, F, F, F, F, F, F, F' crates/kernels/ | wc -l
# Expected: 0
```

---

## Commit sequence (up to 3 commits, atomic per concern)

The original brief asked for one atomic commit. Given the enlarged scope, split into:

1. **`feat(q01): fix three emit bugs in translate_v2/ for CubeCL 0.10`** — only `tools/translate_v2/{cse,emit,per_functional}.py` changes + the spike file at `crates/kernels/math/tests/spike_cse_emit_q01.rs` + this BRIEF.md + SPIKE-FINDINGS.md. The emit-only commit; no kernel regen yet. (Allows clean bisect if a later canary fails.)

2. **`feat(q01): regenerate all 266 per-functional subcrates with fixed emit`** — the regen output (`crates/kernels/{lda,gga,mgga}/*/src/...`). Pure tool output; no hand edits.

3. **`docs(q01): close q01 — update STATE.md Quick Tasks Completed table`** — appends:
   ```
   | 260515-q01 | Fix three emit bugs in translate_v2/ for CubeCL 0.10 (cse arity cap 16→12, F::new literal wrap, scalar return for 1-output chunks); re-emit all 266 per-functional subcrates | 2026-05-15 | <commit> | [260515-q01-cse-chunk-arity-cap-12](.planning/quick/260515-q01-cse-chunk-arity-cap-12/) |
   ```

If splitting feels artificial, you may collapse to 1 or 2 commits — preserve the atomic-bisectability property regardless.

---

## Out of scope (flag, do NOT fix in q01)

- **`crates/kernels/math/src/{powers,polynomials,dft_quantities,...}.rs` `#[cfg(test)]` `from_raw_parts` API drift** — the spike surfaced this (165 errors when building any math-crate test). It's a pre-existing 11-03 (or earlier CubeCL 0.10 upgrade) carry-forward. The `ArrayArg::from_raw_parts` signature changed in cubecl 0.10 from `::<f64>(&handle, n, 1)` to `(handle: Handle, length: usize)`. This blocks ALL math-crate tests including `spike_tuple_return_cube` — but it does NOT block kernel-subcrate compiles (which is what 11-04 cares about). Document this in 11-03's SUMMARY carry-forward list or open a follow-up quick task; do NOT touch `crates/kernels/math/src/` in q01.

- **Phase 11 plan structure** — leave 11-04-PLAN.md, the wave dispatch, and the .continue-here.md alone. The execute-phase orchestrator will resume 11-04 Task 1B with a fresh continuation agent after q01 commits.

---

## Critical constraints (DO NOT VIOLATE)

- **DO NOT modify `.cargo/config.toml`.** D-07/D-08/D-09 invariant. Committed `jobs = 1` is the source of truth; user restores the cap by hand. Uncapped builds OOM (exit 137) on this 30 GB machine.
- **Run cargo builds sequentially, one at a time.** No concurrent cargo invocations.
- **Do NOT hand-edit any file under `crates/kernels/`.** All fixes go through the translators/splitter (D-LOCK-D / P11-INV-6 idempotency). If you find yourself opening a generated `.rs` file in `crates/kernels/{lda,gga,mgga}/` to edit it, STOP — the bug is in the emit, not the output.
- **Do NOT touch Phase 11 plans, SUMMARYs, ROADMAP.md plan rows, or `.continue-here.md`.**
- **Do NOT raise `MAX_TUPLE_ARITY` above 12** unless you've also confirmed CubeCL's `CubeType` is implemented for wider tuples in the linked version (currently 0.10.0).
- **The spike file `crates/kernels/math/tests/spike_cse_emit_q01.rs` is a positive-regression test for the chosen idioms** — keep it intact (it's checked into commit 1).

## When complete

Stop. Do not attempt to resume Phase 11 plan 11-04 — the execute-phase orchestrator will dispatch a fresh continuation agent for 11-04 Task 1B once q01 commits.

If you reach any of the STOP triggers (canary fails with an unexpected error class, invariant gate fails, idempotency check produces non-empty diff, scope creep beyond translate_v2/ + crates/kernels/ regen output), return a structured CHECKPOINT REACHED with the failure details — do not self-resolve.
