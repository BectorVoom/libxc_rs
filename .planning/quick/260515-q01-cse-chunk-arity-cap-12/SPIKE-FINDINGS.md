# q01 Spike Findings — CubeCL 0.10 emit constraints (2026-05-15)

**Spike test:** `crates/kernels/math/tests/spike_cse_emit_q01.rs`
**Command:** `cargo build --tests -p libxc-kernel-math --test spike_cse_emit_q01`
**Verdict:** Three emit bugs in `tools/translate_v2/`, all decisively reproduced bare under `#[cube]` 0.10. Two candidate fix idioms verified compiling.

## Pattern test matrix

| # | Pattern | Test fn | Compiles? | Error |
|---|---------|---------|:---:|-------|
| Q1  | `let t = 0.173e2 * x;` (x:F)       | `q1_literal_times_F`        | ✗ | `cannot multiply {float} by F` (E0277) |
| Q1b | `let t = x / 0.5e2;`               | `q1b_literal_div_F`         | ✗ | `expected F, found floating-point number` (E0308) |
| Q1c | `let t = 0.25e1 + x;`              | `q1c_literal_add_F`         | ✗ | `cannot add F to {float}` (E0277) |
| Q2a | `let b = a; (b,)` inside `-> (F,)` | `q2a_one_tuple_bare_alias`  | ✗ | `expected (NativeExpand<F>,), found NativeExpand<F>` (E0308) |
| Q2b | `let b = a + a; (b,)` inside `-> (F,)` | `q2b_one_tuple_non_alias` | ✗ | **same error as Q2a** — issue is not bare-alias, **any `let` inside `-> (F,)` fails** |
| Q3a | `(F × 12)` return                  | `q3a_twelve_tuple`          | ✓ | (at ceiling) |
| Q3b | `(F × 13)` return                  | `q3b_thirteen_tuple`        | ✗ | `(F × 13): CubeType not satisfied` (E0277) — re-confirms `MAX_TUPLE_ARITY=12` |
| Q4a | `let t = F::new(0.173e2) * x;`     | `q4a_F_new_literal_times`   | ✓ | candidate fix for Q1 |
| Q4b | `let t = x / F::new(0.5e2);`       | `q4b_F_new_literal_div`     | ✓ | candidate fix for Q1b |
| Q4c | `let t = F::new(0.25e1) + x;`      | `q4c_F_new_literal_add`     | ✓ | candidate fix for Q1c |
| Q5  | `-> F { let b = a; b }`            | `q5_scalar_return_alias`    | ✓ | candidate fix for Q2a (drop the 1-tuple wrapping for single-output chunks) |
| Q5b | `-> F { let b = a + a; b }`        | `q5b_scalar_return_compute` | ✓ | candidate fix for Q2b |

## Why math/src/ compiles despite using the broken patterns

`crates/kernels/math/src/{bspline,dft_quantities,polynomials,...}.rs` use **concrete `f64`** parameter types — `fn wigner_seitz_rs(rho: f64) -> f64`, `let twox = 2.0 * x;`. Rust's `{float}` literal infers cleanly to `f64`. The bug only bites kernels that use **generic `<F: Float>`** per D-02, which is exclusively `crates/kernels/{lda,gga,mgga}/`.

## Concrete emit fixes (for the enlarged q01 / follow-up plan)

| # | Change | File(s) | Estimated lines | Blast radius |
|---|--------|---------|---:|--------------|
| 1 | `MAX_TUPLE_ARITY = 16 → 12` | `tools/translate_v2/cse.py:32` | 1 line | 19 subcrates with wide chunks |
| 2 | Wrap every f64 literal in chunk bodies as `F::new(<lit>)` | `tools/translate_v2/per_functional.py` and/or `tools/translate_v2/emit.py` body emit path | likely a regex/AST pass over emitted expressions | **all chunks with f64 literals — likely all 266 subcrates** |
| 3 | Emit single-output chunks as `-> F` (scalar) instead of `-> (F,)` | `tools/translate_v2/per_functional.py` chunk signature emit | ~3-5 lines (special-case `len(outputs)==1`) | every 1-output chunk — ubiquitous (HEAD's `lda_c_pw_erf` has 3669 chunk files, density per executor's count was 1046 1-tuples) |

## Path 1 risk assessment

The original q01 brief assumed blast radius = 19 subcrates (the wide-tuple set). The spike reveals blast radius = **all 266 per-functional subcrates** for fix 2, and a large majority for fix 3. Full regen is appropriate. This is no longer "quick" by GSD standards — it's a 1-day full re-emit + verification cycle.

The cse.py change is one prerequisite line; the body-emit fixes are the bulk.

## Spike file disposition

After validation, the spike's broken-pattern probes were removed (they'd otherwise wedge `cargo test -p libxc-kernel-math` permanently). The Q4/Q5 positive-regression tests remain in `crates/kernels/math/tests/spike_cse_emit_q01.rs` as compile-time regression coverage for the chosen emit idioms. The broken-pattern evidence is preserved in this findings doc.
