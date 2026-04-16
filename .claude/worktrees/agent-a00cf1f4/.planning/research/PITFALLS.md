<<<<<<< HEAD
# Domain Pitfalls

**Domain:** Rust reimplementation of libxc (C DFT exchange-correlation functional library) with CubeCL GPU compute
**Researched:** 2026-04-09

## Critical Pitfalls

Mistakes that cause rewrites, silent numerical corruption, or fundamental architecture failures.

### Pitfall 1: Floating-Point Operation Order Divergence Between C and Rust

**What goes wrong:** The maple2c-generated C code produces expressions like `t18 * t22 / 0.2e1 + t26 * t12 / 0.4e1 + 0.125e0 * t29 * t30 + t38`. The exact order of evaluation and rounding at each intermediate step determines the final bits. If the Rust translation reorders any sub-expression -- even `a * b + c` vs `c + a * b` -- the result differs at the ULP level, cascading through 4th-order derivatives into relative errors well above 10^-12.

**Why it happens:**
- Rust preserves IEEE 754 strict semantics by default (no implicit FMA contraction, no reassociation), which is good. But the translator (human or automated) may inadvertently parenthesize differently or split/merge temporaries.
- The C code uses Maple-numbered temporaries (t1, t2, ..., t89) with a specific linear dependency chain. Reorganizing into "cleaner" Rust expressions breaks bit-equivalence.
- Compiler optimizations: Rust's LLVM backend will NOT reorder by default (unlike `-ffast-math` in C). However, `target-cpu=native` can select different instruction sequences (x87 80-bit vs SSE2 64-bit), producing different rounding.

**Consequences:** Silent numerical corruption. Tests pass on one machine, fail on another. Derivatives amplify errors exponentially -- a 1-ULP error in energy becomes 10-100 ULP in 4th derivatives.

**Prevention:**
- Translate maple2c formulas mechanically: preserve every temporary variable, preserve exact operation order, preserve parenthesization. Do NOT simplify expressions.
- Build an automated translator (maple2c C -> Rust `#[cube]`) rather than hand-translating 270 files. Each file has up to 10 function variants (exc/vxc/fxc/kxc/lxc x unpol/pol), totaling ~2700 functions.
- Never compile the verify harness or libxc oracle with `-C target-cpu=native`. Use the default target to ensure identical instruction selection.
- Test every functional against the oracle at every derivative order, not just energy. The 10,312 regression tests with 4 test systems (H, Li, BrOH, BrOH+) must all pass.

**Detection:** Relative error exceeding 10^-12 in oracle comparison, especially in higher derivatives (fxc, kxc, lxc) while energy (exc) passes. This pattern is the hallmark of operation-order divergence.

**Phase relevance:** Phase 3 (kernel translation). Must be addressed before any kernel is translated.

---

### Pitfall 2: CubeCL `#[cube]` Macro Limitations Silently Breaking Kernel Translation

**What goes wrong:** CubeCL's `#[cube]` procedural macro has specific limitations that cause compilation failures or silent semantic changes:
1. **`if`-expressions as values** fail with `ExpandElementTyped` mismatches. The maple2c code uses `my_piecewise3(cond, x1, x2)` which is `cond ? x1 : x2` -- a ternary that returns a value.
2. **Method-style math calls** (`x.exp()`, `x.sqrt()`) fail. Must use `f64::exp(x)` associated-function style.
3. **Calling non-`#[cube]` functions** from within `#[cube]` functions fails silently or with cryptic errors.
4. **No standard library** inside `#[cube]`: no `assert!`, no `println!`, no heap allocation. The maple2c code uses `assert(p->params != NULL)`.

**Why it happens:** CubeCL's proc macro performs source-to-source transformation into a GPU IR. It does not understand arbitrary Rust; it understands a restricted subset. The macro generates `__expand_*` methods for known operations, but anything outside this subset produces confusing errors or wrong codegen.

**Consequences:** Compilation failures on 270 kernel files after translation. Worse: if the macro silently accepts incorrect code, you get wrong numerical results that are hard to trace back to a CubeCL codegen issue vs a translation error.

**Prevention:**
- Build a `my_piecewise3` / `my_piecewise5` helper as a `#[cube]` function using mutable assignment pattern:
  ```rust
  #[cube]
  fn piecewise3(cond: bool, x1: f64, x2: f64) -> f64 {
      let mut result: f64 = x2;
      if cond { result = x1; }
      result
  }
  ```
- Build all math helpers (`POW_1_3`, `POW_3_2`, `safe_cbrt`, `erf`, `erfc`) as `#[cube]` functions using associated-function style (`f64::sqrt(x)`, `f64::exp(x)`, etc.) before starting kernel translation.
- Write a comprehensive test suite for each helper function in isolation.
- Create a "CubeCL translation cookbook" documenting every maple2c pattern and its Rust `#[cube]` equivalent.

**Detection:** Compilation errors with `ExpandElementTyped` in the message. Also, test kernel helpers with known values before integrating into functional kernels.

**Phase relevance:** Phase 2 (CubeCL substrate / math building blocks). Must have all helpers correct before Phase 3.

---

### Pitfall 3: 100K-Line Kernels Exceeding GPU Compiler Limits

**What goes wrong:** The largest maple2c kernels are enormous: `mgga_c_rmggac.c` is 99,938 lines, `mgga_c_revtpss.c` is 90,750 lines. Each contains 10 function variants (5 derivative orders x 2 spin modes). A single polarized 4th-order MGGA kernel can have thousands of intermediate variables and tens of thousands of arithmetic operations. This may exceed:
1. CUDA PTX instruction limits (~2M instructions, but JIT compilation time becomes impractical well before that)
2. WGSL shader compilation timeouts
3. Rust compile times (monomorphization of deeply nested `#[cube]` expansions)
4. GPU register pressure causing excessive spilling and 10-100x slowdown

**Why it happens:** Maple's symbolic differentiator generates fully-expanded expressions without common subexpression elimination across derivative orders. Each derivative level re-derives all lower-order work. The 4th-order polarized MGGA output has up to 477 components.

**Consequences:** Some functionals simply will not compile for GPU. Others compile but run pathologically slowly due to register spilling. Build times for the full library become hours.

**Prevention:**
- Separate each derivative order into its own kernel function rather than one monolithic kernel. The C code already does this (`func_exc_unpol`, `func_vxc_unpol`, etc. are separate functions).
- Profile CubeCL compilation time on the 5 largest MGGA kernels early. If any exceed 30 seconds to compile, that is a red flag.
- Consider a tiered approach: compile LDA/GGA kernels first (small), then MGGA (large). Ship LDA/GGA GPU support first, add MGGA GPU support incrementally.
- Monitor GPU register usage via `--ptxas-options=-v` (CUDA) or equivalent. If register count exceeds 128 per thread, the kernel will be slow.
- For the truly enormous kernels (>50K lines), consider CPU-only execution as an acceptable fallback.

**Detection:** CubeCL compilation taking >10 seconds for a single kernel. GPU occupancy below 25%. Rust compiler OOM on large kernels.

**Phase relevance:** Phase 3 (kernel translation) and Phase 4 (GPU optimization). Must test the largest kernels early in Phase 3, not last.

---

### Pitfall 4: `cbrt` of Negative Numbers in CubeCL/GPU Context

**What goes wrong:** The maple2c code uses `cbrt(x)` extensively via `POW_1_3(x)` macro, including cases where `x` can be negative (e.g., `POW_1_3(rho[0])` where rho is always positive, but `cbrt(1+zeta)` and `cbrt(1-zeta)` where zeta ranges [-1, 1], making the argument negative). C's `cbrt(-8.0)` returns `-2.0`. However:
1. GPU hardware may not have a native `cbrt` instruction. The fallback `pow(x, 1.0/3.0)` returns NaN for negative x because `pow` treats the exponent as a real number.
2. CubeCL may or may not provide a `cbrt` intrinsic. If it maps to `pow(x, 1.0/3.0)`, every polarized functional silently produces NaN.

**Why it happens:** `cbrt` is a C99 function specifically designed to handle negative inputs. `pow` follows IEEE 754 rules where negative base with non-integer exponent is undefined (NaN). GPU math libraries often only provide `pow`, not `cbrt`.

**Consequences:** Every polarized functional (spin-resolved calculations) returns NaN for any system where spin-up density differs from spin-down density. This is nearly all real calculations.

**Prevention:**
- Implement `safe_cbrt` as a `#[cube]` function: `sign(x) * pow(abs(x), 1.0/3.0)`, but be careful about `x = 0.0` (must return 0.0, not -0.0).
- Verify this matches C's `cbrt` to full f64 precision by testing against the oracle for edge cases: x = 0.0, -0.0, very small negative, very small positive, large negative, subnormals.
- Pre-compute all constant cube roots (M_CBRT2 through M_CBRT9) as `const f64` literals with full precision rather than computing at runtime.

**Detection:** NaN values in polarized functional output. Automated test: run all polarized functionals and assert no NaN in output.

**Phase relevance:** Phase 2 (math building blocks). This is one of the first helpers to implement and verify.

---

### Pitfall 5: WGPU Backend Fundamentally Cannot Support f64

**What goes wrong:** WebGPU (WGSL) does not support f64 in the shader language specification. The `cubecl-wgpu` backend therefore cannot run f64 kernels. Since the project requires f64-only precision, the WGPU backend will either silently truncate to f32 (catastrophic) or fail to compile (acceptable if detected).

**Why it happens:** WebGPU was designed for web browsers where f64 support varies wildly across GPU hardware. The standard chose not to include it. This is a specification decision, not a bug, and will not change in the foreseeable future.

**Consequences:** If WGPU is the only available backend (e.g., on macOS without CUDA), users get either f32-precision results (catastrophically wrong for DFT) or no GPU acceleration at all.

**Prevention:**
- Feature-gate the WGPU backend and document that it is NOT suitable for production DFT calculations.
- At runtime, detect the backend and refuse to execute f64 kernels on WGPU. Return a typed error: `Error::F64NotSupported { backend: "wgpu" }`.
- Prioritize `cubecl-cuda` and `cubecl-hip` backends which support f64 natively.
- Consider removing WGPU from the default feature set entirely. The Cargo.toml currently has `features = ["cpu","wgpu"]` -- the `wgpu` feature should probably be opt-in only.
- For macOS users, `cubecl-cpu` (always available) is the correct fallback.

**Detection:** Any test producing relative error > 10^-6 is a sign of f32 truncation.

**Phase relevance:** Phase 1/2 (infrastructure). Must establish the f64 policy and backend gating before writing any kernels.

---

## Moderate Pitfalls

### Pitfall 6: Density Thresholding Mismatch

**What goes wrong:** libxc uses multiple thresholds (`dens_threshold`, `zeta_threshold`, `sigma_threshold`, `tau_threshold`) to skip evaluation or clamp values at very low densities. These thresholds affect both whether a grid point is evaluated AND how intermediate values are clamped. Getting any threshold wrong causes:
- Division by near-zero producing Inf/NaN
- Discontinuities in the energy surface causing SCF convergence failures
- Subtle errors only visible in specific molecular systems

**Why it happens:** The threshold logic is scattered across `util.h`, individual functional implementations, and the maple2c code itself (via `my_piecewise3` on `zeta_threshold`). It is not documented in one place.

**Prevention:**
- Extract all threshold logic from libxc C source into a specification document before implementing.
- Test with the H atom (unpolarized, very small density tails) and BrOH+ (polarized, asymmetric spin) specifically -- these exercise threshold edge cases.
- Match libxc's exact threshold defaults: `dens_threshold = 1e-14`, etc.

**Detection:** SCF convergence failures in downstream DFT codes. NaN/Inf in output arrays. Differences vs oracle concentrated at low-density grid points.

**Phase relevance:** Phase 2 (input bundles) and Phase 3 (kernel translation).

---

### Pitfall 7: Output Accumulation Semantics (`+=` vs `=`)

**What goes wrong:** libxc accumulates output via `+=` (e.g., `out->zk[ip] += tzk0`), not `=`. This is critical for mixed functionals (e.g., B3LYP = 0.2*HF + 0.8*Slater + 0.72*B88 + 0.81*LYP + 0.19*VWN). If any kernel uses `=` instead of `+=`, it overwrites contributions from previous functional components, producing wrong energies for every mixed/hybrid functional.

**Why it happens:** A translator naturally writes `output[i] = result` rather than `output[i] += result`. The `+=` semantics require the caller to zero the output buffer before the first evaluation, and all subsequent evaluations to accumulate.

**Consequences:** Every hybrid and mixed functional produces wrong results. Pure functionals (single component) appear correct, masking the bug until hybrids are tested.

**Prevention:**
- Make `+=` accumulation the ONLY output path in the kernel template. Never have a `=` path.
- Require output buffers to be zero-initialized before evaluation. Provide a `zero_output()` method.
- Test hybrid functionals early (B3LYP, PBE0) -- do not leave them until after pure functionals are "done."

**Detection:** B3LYP energy differs from oracle while PBE energy matches exactly.

**Phase relevance:** Phase 3 (kernel translation template design).

---

### Pitfall 8: Automated Translator Fragility with Maple2c Patterns

**What goes wrong:** Building an automated C-to-Rust translator for the maple2c code seems straightforward because the code is auto-generated and follows patterns. But the patterns have exceptions:
- Some functionals use `params->` access, others use `ext_params->`.
- Some use `my_piecewise3`, others use `my_piecewise5`.
- Deorbitalized functionals (`deorbitalize_1.c` through `deorbitalize_4.c`) have completely different structure.
- Some functionals reference auxiliary functional results.
- The `hyb_*` functionals have additional hybrid coefficient logic.
- Some have `#ifdef XC_DONT_COMPILE_EXC` guards, others have `#ifndef`.

An automated translator that handles 95% of cases will still fail on 30+ functionals, requiring manual fixes that may introduce the very errors the automation was meant to prevent.

**Why it happens:** Maple generates code, but the libxc build system wraps it with C preprocessor macros, struct access patterns, and functional-family-specific boilerplate that varies.

**Prevention:**
- Catalog ALL patterns in the maple2c corpus before building the translator. Run grep/regex analysis on all 270 files to find every macro, every struct access pattern, every conditional compilation guard.
- Build the translator incrementally: LDA first (simplest, ~20 files), then GGA (~50 files), then MGGA (~90 files). Validate each batch before proceeding.
- For each translated file, diff the Rust output against a "gold" reference (manual translation of a representative functional from each family) to catch translator regressions.

**Detection:** Translator producing code that compiles but fails oracle tests. Pattern: a new functional family fails while previous ones pass.

**Phase relevance:** Phase 3 (kernel translation). Must be the first task: analyze patterns, build translator, validate.

---

### Pitfall 9: External Parameter Mutation After Construction

**What goes wrong:** libxc allows users to modify external parameters (`ext_params`) after constructing a functional. Some parameters affect threshold computation, others change the functional form entirely (e.g., the fraction of exact exchange in a hybrid). If the Rust implementation pre-computes derived values at construction time and caches them, parameter mutation after construction produces stale results.

**Why it happens:** The C API is imperative -- you `xc_func_init`, then optionally `xc_func_set_ext_params`, then evaluate. The Rust API may optimize by pre-computing at init time.

**Prevention:**
- Either invalidate pre-computed state when `set_ext_params` is called, or compute derived values lazily at first evaluation.
- For the C compatibility layer (`extern "C"`), match libxc's exact lifecycle: init -> set_params -> evaluate.
- Test with a functional that has non-default `ext_params` (e.g., range-separated hybrids with user-specified omega).

**Detection:** Oracle comparison passes with default params, fails with custom params.

**Phase relevance:** Phase 4 (API and lifecycle management).

---

### Pitfall 10: GPU Thread Divergence in Piecewise Functions

**What goes wrong:** `my_piecewise3` and `my_piecewise5` are conditional branches. On GPU, threads in a warp that take different branches cause divergence, serializing execution. Some functionals use piecewise functions in the inner loop where the condition depends on per-grid-point data (e.g., `zeta >= zeta_threshold`), causing significant divergence.

**Why it happens:** DFT grid points span a wide range of densities. Some points have zeta near 0 (unpolarized), others near 1 (fully polarized). The piecewise conditions split the warp.

**Prevention:**
- Implement branch-free piecewise using `select` or arithmetic masking: `result = cond_f64 * x1 + (1.0 - cond_f64) * x2` where `cond_f64` is 0.0 or 1.0.
- The project already plans for "branch-free piecewise3/5" -- ensure this is actually implemented in the CubeCL substrate, not deferred.
- Measure GPU occupancy with and without branch-free piecewise on a representative MGGA functional.

**Detection:** GPU throughput less than 2x CPU throughput for polarized functionals (should be >5x per requirements).

**Phase relevance:** Phase 2 (math building blocks) for implementation, Phase 5 (optimization) for tuning.

---

## Minor Pitfalls

### Pitfall 11: Maple2c Constant Precision Loss

**What goes wrong:** The maple2c code uses constants like `0.1e1`, `0.2e1`, `0.125e0` which are exact in IEEE 754. But some functionals embed high-precision constants that were truncated by Maple's code generator. If these are re-typed or reformatted during translation, precision may change.

**Prevention:** Copy constants character-for-character from the C source. Do not "simplify" `0.1e1` to `1.0` (they are identical in IEEE 754, but the practice of "simplifying" constants may lead to accidentally simplifying non-trivial constants too).

**Phase relevance:** Phase 3 (kernel translation).

---

### Pitfall 12: Missing `#[inline(always)]` on Hot-Path Functions

**What goes wrong:** CubeCL `#[cube]` functions that call helper functions (like `piecewise3`, `safe_cbrt`, `pow_1_3`) may not be inlined by the GPU compiler, causing function-call overhead on every grid point for every temporary variable.

**Prevention:** Mark all `#[cube]` helper functions with `#[inline(always)]` (though verify CubeCL respects this hint in its codegen). Profile after initial implementation.

**Phase relevance:** Phase 5 (optimization).

---

### Pitfall 13: Verify Harness Linking Against Wrong libxc Version

**What goes wrong:** The verify crate builds libxc from `libxc-master/` vendored source. If the vendored source gets out of sync with the target (7.0.0), or if a system-installed libxc is accidentally linked instead, all oracle comparisons are meaningless.

**Prevention:**
- Pin the vendored libxc version. Add a build-time assertion that checks `xc_version()` returns `(7, 0, 0)`.
- The build.rs should use `cargo:rustc-link-lib=static=xc` (already does) to avoid picking up system libxc.
- Never update the vendored libxc without re-running the full test suite.

**Detection:** Oracle tests passing "too easily" or failing in patterns that do not match code changes.

**Phase relevance:** Phase 1 (infrastructure).

---

### Pitfall 14: Forgetting to Handle Removed Functionals

**What goes wrong:** libxc 7.0.0 removed 52 functional IDs that existed in previous versions. If the registry silently ignores these IDs, downstream codes that use old functional IDs get no output instead of a helpful error.

**Prevention:** Maintain a static list of removed IDs with their replacement functional IDs. Return a typed error: `Error::FunctionalRemoved { old_id, replacement_id, since_version }`.

**Phase relevance:** Phase 1 (registry).

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Phase 1: Infrastructure/Registry | Verify harness linking wrong libxc (#13), removed functionals (#14) | Pin version, static removed-ID table |
| Phase 2: I/O Bundles, CubeCL Substrate | cbrt negative numbers (#4), WGPU f64 (#5), piecewise macro translation (#2), threshold semantics (#6), branch-free piecewise (#10) | Implement and test all math helpers before kernels |
| Phase 3: Kernel Translation | Operation order (#1), accumulation semantics (#7), translator fragility (#8), constant precision (#11), kernel size limits (#3) | Automated translator with oracle verification per-functional |
| Phase 4: API and Lifecycle | External parameter mutation (#9) | Lazy recomputation or invalidation pattern |
| Phase 5: GPU Optimization | Large kernel register pressure (#3), thread divergence (#10), inlining (#12) | Profile early, tiered GPU support (LDA/GGA first) |

## Sources

- [Subtle floating-point differences between C library and its Rust re-write](https://users.rust-lang.org/t/subtle-floating-point-differences-between-c-library-and-its-rust-re-write/82355) - MEDIUM confidence
- [Rust RFC 3514: Float Semantics](https://rust-lang.github.io/rfcs/3514-float-semantics.html) - HIGH confidence
- [WebGPU f64 support discussion](https://github.com/gpuweb/gpuweb/issues/2805) - HIGH confidence
- [WGPU f64 type restriction](https://github.com/gfx-rs/wgpu/issues/7017) - HIGH confidence
- [CubeCL error solution guide](docs/manual/Cubecl/cubecl_error_solution_guide/) - HIGH confidence (local project docs)
- [libxc GPU issues](https://gitlab.com/libxc/libxc/-/issues/135) - MEDIUM confidence
- Direct analysis of vendored libxc-master source (maple2c kernels, util.h, xc.h) - HIGH confidence
=======
# Pitfalls Research

**Domain:** Rust re-architecture of libxc with unified CubeCL CPU/GPU compute paths  
**Researched:** March 22, 2026  
**Confidence:** MEDIUM

## Critical Pitfalls

### Pitfall 1: Generated API coverage drift

**What goes wrong:** Generated registries fall out of sync with the upstream libxc headers, so some functional IDs, constants, or lifecycle hooks simply never appear in the Rust surface and downstream lookups break.  
**Why it happens:** The generator depends on parsing macros and structs; using an older bindgen/libclang combo or failing to re-run the pipeline after a header tweak can silently drop definitions, which is exactly what happens when bindgen cannot translate a new header update and the build fails with “cannot find struct…in crate” errors when the missing symbol is referenced. citeturn2search0  
**How to avoid:** Automate the parser/code-gen in Phase 0, lock the toolchain versions, and treat the generated count of public APIs/IDs as a regression check so any deviation fails the build before release.  
**Warning signs:** The API catalog test flags missing names, CI pulls in header changes without a matching generated diff, or `docs/libxc_rs_detailed_design.md` counts no longer match the generated registry table.  
**Phase to address:** Phase 0 (codegen) + Phase 1 (metadata/registry completeness)

---

### Pitfall 2: CPU/GPU semantic divergence

**What goes wrong:** Without tight coordination, the CPU CubeCL kernel can drift from the GPU path, leading to different numerical outputs or resource starvation when one backend accelerates more aggressively than the other. citeturn0search3  
**Why it happens:** GPUs have complex pipelines that expose subtle differences in scheduling, precision, and memory traffic; industry parity efforts already add parity checks to catch pipeline discrepancies because the hardware itself can behave differently from the CPU pipeline. citeturn0search1  
**How to avoid:** Share as much kernel logic as possible (Phase 3/4), run parity tests on every derivative order across CPU, CUDA, HIP, and WGPU backends, and make divergence detection part of the resident/verification flows.  
**Warning signs:** GPU utilization curves stay flat while CPU runs faster, libxc comparison tolerances wander, or CubeCL logs show different internal branches executed per backend.  
**Phase to address:** Phase 3 (shared CubeCL substrate) + Phase 4 (family kernels) with follow-up verifications in Phase 5.

---

### Pitfall 3: Verification gaps

**What goes wrong:** Heterogeneous systems lack enough regression/acceptance tests, so changes slip through without the rigorous HPC-style verification demanded by cross-device compute. citeturn13view0  
**Why it happens:** Building verification infrastructure is harder than adding features, and teams cut corners until a real mismatch surfaces; the AI infrastructure testing community stresses the same point—without dedicated clustered validation frameworks, production issues (and costly downtime) emerge unexpectedly. citeturn0search4  
**How to avoid:** Use Phase 7’s verification harness to compare every API path against libxc, run nightly CPU vs GPU parity suites, and log abs/rel/ULP discrepancies so regressions and backend-specific failures are immediately visible.  
**Warning signs:** Verification harness was skipped for a release, oracle comparison metrics are stale, or local tests only cover a single backend.  
**Phase to address:** Phase 6/7 (safe API completion and verification harness)

---

### Pitfall 4: Runtime capability mismatches

**What goes wrong:** CubeCL’s CPU backend still throws “not yet implemented” for common operations, so enabling that backend causes panics instead of graceful fallbacks. citeturn10view0  
**Why it happens:** The runtime exposes broad APIs, but the CPU compiler visitor has unimplemented operations, meaning new kernels trigger these gaps before they surface in production.  
**How to avoid:** Treat capability probing as mandatory—before allowing a CPU runtime, query CubeCL for supported operations, and if the new kernel uses an unsupported operation, revert to a well-defined error rather than exposing a panic.  
**Warning signs:** CI logs show “not yet implemented: This operation (…)” even though the kernel compiles for CUDA, or CubeCL CPU tests are skipped.  
**Phase to address:** Phase 3/4 (shared kernel compile + per-family launches)

---

### Pitfall 5: Transfer costs and small-batch overhead

**What goes wrong:** Data movement dominates low-batch execution, so transferring small slices per launch (and not overlapping transfers) leaves GPUs idle for 56 % of their cycles and wastes 27.9 % of runtime on data operations. citeturn1search1turn1search12  
**Why it happens:** Small batches force frequent launches with little amortization, so bandwidth stalls and synchronous checkpoints (data operations) swamp compute unless transfers are overlapped and resident buffers reused.  
**How to avoid:** Emphasize Phase 5/6 resident execution, make the workspace planner aware of dirty ranges, and use buffered transfer pipelines and cache reuse before exposing the high-level API.  
**Warning signs:** Benchmark transfer tests spike, small-batch profiling shows >25 % transfer time, or resident APIs default to synchronous readbacks.  
**Phase to address:** Phase 5/6/8 (resident flow, safe API, and benchmark stabilization)

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Skip GPU/CPU parity runs to keep CI fast | CI builds are quicker | Silent semantic divergence that only surfaces in production | Never; heterogeneity requires regression coverage citeturn13view0 |
| Skip transfer instrumentation or profiling dashboards | Save time on metrics work | Unable to detect data-starvation drags, which already cost 56 % of GPU cycles waiting for data citeturn1search1 | Only for quick prototypes |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| CubeCL CPU runtime | Enabling the backend without capability probing causes “not yet implemented” panics citeturn10view0 | Probe CubeCL’s unsupported operations map, gate CPU runs behind capability checks, and keep a fallback error path. |
| Storage/data layer | Assuming storage keeps up; the GPU/storage imbalance already makes compute sit idle | Prioritize transfer overlap, caching, and multi-tier storage planning so hosts feed the kernels without starvation citeturn6view0 |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Small batches without overlapped transfers | Transfer time ≳25 % of runtime, GPU idle | Resident execution + asynchronous staging for small launches citeturn1search12turn1search1 | When batch size drops below the threshold that amortizes kernel launch + transfer |
| Treating storage as “fast enough” | GPUs stall waiting for data, reports of 56 % idle cycles citeturn1search1 | Instrument I/O, add caching layers, or throttle storage demand to match the GPU rhythm citeturn6view0 | At petabyte-scale or multi-tenant clusters |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Leaving GPU/resident buffers populated | Sensitive data survives in device memory, creating a data-leak risk in shared environments citeturn1search6 | Zero-out/stage GPU buffers after use and limit device access to the runtime’s trusted scope. |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Surface raw CubeCL or libxc panics through the ergonomic API | Users lose confidence if the API explodes in production-only runtimes | Wrap low-level failures with explicit diagnostics, and document which devices/backends are safe to call; treat environment parity like a testing practice citeturn0search4 |

## "Looks Done But Isn't" Checklist

- [ ] **API coverage:** Generated inventory numbers still match libxc’s 85 functions/649 IDs; a single missing ID means the registry wasn’t regenerated. citeturn2search0  
- [ ] **Parity testing:** GPU-only results were compared against CPU and libxc oracle before release; heterogeneity testing requires this to avoid divergence. citeturn13view0  
- [ ] **Transfer profiling:** Small-batch runs still spend ≳25 % of time moving data; the transfer instrumentation was reviewed. citeturn1search12  
- [ ] **CubeCL CPU runtime:** Every code path that claims CPU support has passed the capability probe; unimplemented operations are fatal to parity. citeturn10view0

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Semantic divergence revealed after release | HIGH | Pause new feature work, rerun cross-backend parity harness, and rebuild the shared kernel with the corrected specialization. citeturn0search3 |
| Missing IDs drop from the generated registry | MEDIUM | Re-run the parser/codegen, refresh the generated tables, and extend the API catalog test to count regression coverage. citeturn2search0 |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Generated API coverage drift | Phase 0 (codegen) + Phase 1 (metadata/registry) | API catalog test that counts IDs and functions. citeturn2search0 |
| CPU/GPU semantic divergence | Phase 3/4 (shared kernel + family kernels) | Cross-backend parity suite per derivative order, resident mode comparisons. citeturn0search1 |
| Verification gaps | Phase 6/7 (safe API + verification harness) | libxc oracle runs + HPCTESTS-style regression tests. citeturn13view0 |
| Runtime capability mismatches | Phase 3/4 (CubeCL substrate + kernels) | Capability probe log, failure injection for unsupported operations. citeturn10view0 |
| Transfer & small-batch trap | Phase 5/6/8 (resident flow, safe API, benchmarks) | Transfer-time profiling dashboards, transfer-to-compute ratio targets. citeturn1search12 |

## Sources

- Bindgen/version mismatch breaking generated APIs (Rust issue report). citeturn2search0  
- GPU acceleration requires orchestration, and GPU pipelines already need parity checks. citeturn0search3turn0search1  
- AI infrastructure validation frameworks and HPC testing demands. citeturn0search4turn13view0  
- CubeCL CPU backend still throws “not yet implemented” for common ops. citeturn10view0  
- GPU cycles idle waiting for data; storage bottlenecks starve compute. citeturn1search1turn6view0  
- Data operations already consume ~27.9 % of runtime in small batches. citeturn1search12  
- GPU memory leaks can expose sensitive data in shared runtimes. citeturn1search6

---
>>>>>>> origin/main
