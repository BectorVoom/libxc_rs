---
phase: 11-splitter-v2-unified-5k-cap
plan: 04
status: PARTIAL — Task 1A landed (39eb75f93), paused before Task 1B
captured: 2026-05-15
type: documentation
---

# Plan 11-04 — Retroactive SUMMARY (Paused at D-02 Architectural Blocker)

**Plan:** 11-04 (documentation only; Task 1A work landed mid-execution 2026-05-15)
**Status:** PARTIAL — Task 1A committed, Task 1B blocked by D-02 incompatibility
**Captured:** 2026-05-15 (revised structure, planning resumed 2026-05-17)

## Task 1A — Committed (39eb75f93)

Verify crate dev-dependencies narrowed from umbrella `libxc-kernel-{lda,mgga}` imports to per-functional subcrate imports (D-05 structural fix for OOM). This was the first half of the original 11-04 plan but surfaced a blocking architectural issue before Task 1B could proceed.

| Component | What | Status |
|-----------|------|--------|
| `verify/Cargo.toml` | Narrowed `libxc-kernel-{lda,mgga}` → per-functional `libxc-kernel-lda_*`, `libxc-kernel-mgga_*` | ✅ Committed `39eb75f93` |
| OOM structural fix | No longer pulling all 266 kernel subcrates into verify/ linker | ✅ Validated |
| Carry-forward artifacts | 11-01, 11-02, 11-03 SUMMARY files + working tree state | ✅ Preserved |

**Why paused before Task 1B:** Task 1B intended to regen the kernel tree and verify it compiled. The regen completed, but `cargo build -p libxc-kernel-mgga_c_b94` failed with type errors revealing D-02's incompatibility with the math/src/ helper layer.

## Architectural Finding — D-02 × Math/src/ Helper Layer Blocker

**Empirical discovery:** A live spike at `crates/kernels/math/tests/spike_cse_emit_q01.rs` (committed in q01: `5c379dc25`) exercised real generated chunks calling real math/src/ helpers and discovered that `<F: Float>` chunks cannot call concrete-`f64` helpers under CubeCL 0.10 — no auto-coercion at the call site.

### Four-Layer Bug Structure

| Layer | Bug | CubeCL 0.10 Status | Fix Status | Lines Affected |
|-------|-----|-------------------|-----------|---|
| **1** | Tuple arity cap: `CubeType` derive max arity 12, CSE emit was 16 | ❌ Compile error | ✅ Fixed in `5c379dc25` | `tools/translate_v2/cse.py` `MAX_TUPLE_ARITY 16→12` |
| **2A** | Float literal coercion: `0.123e1`-style raw f64 literals, no `F::new` wrap | ❌ E0277 `Mul<F> for {float}` | ✅ Partially fixed in `5c379dc25` | `_wrap_f64_literals` in `tools/translate_v2/per_functional.py`; 4136 chunks still miss named f64 constants + integer-mantissa forms |
| **2B** | Float literal coercion: integer-mantissa form (`2e-21`) + named constants (`M_PI`, `M_CBRT3`, …) | ❌ E0277 (unhandled by `_wrap_f64_literals` regex) | ⏳ Ready to fix (D-16 emit-pass work) | 4136 chunks in generated tree; ~300 lines in translator |
| **3** | 1-tuple return inference: `let b = a;` inside `-> (F,)` infers `b: (NativeExpand<F>,)` → E0308 | ❌ Compile error | ✅ Fixed in `5c379dc25` | `tools/translate_v2/per_functional.py` emit scalar `-> F` not `-> (F,)` |
| **4** | **Helper-layer incompatibility: `<F: Float>` chunks call `pow_1_3(x: f64) -> f64`, `piecewise3(_, _: f64, _: f64) -> f64`, `erf()`, `lambert_w()`, etc. — 38 helpers all concrete-f64, 0 generic** | ❌ **Architectural: type mismatch at every call site** | **⏳ Spike-pending (D-14 2-day race)** | **38 helpers in `crates/kernels/math/src/{piecewise,powers,erf,lambert_w,bspline,br89,bessel,dft_quantities,spin,integrate,polynomials,mbrxc,special,expint_e1}.rs`** |

### Why Layer 4 Blocks the Pipeline

The 11-01 spike (`spike_tuple_return_cube.rs`) proved the D-02 ABI (tuple return + `<F: Float>` genericity) works in **isolation** on a hand-written test. It did NOT exercise calls to math/src/ helpers.

The q01 spike (`spike_cse_emit_q01.rs`, `5c379dc25`) is the first real-world generated chunk that calls helpers. On mgga_c_b94 (the worst case — 16K+ lines), an isolated handler call like `let result = pow_1_3(x);` now has a type mismatch: `pow_1_3` expects `f64`, but `x` is typed `F`. There is no automatic coercion in CubeCL 0.10 (unlike Rust's implicit `Into<>` at call sites).

**This is not a quick fix like Layers 1–3.** It requires choosing a fundamental ABI approach for the entire phase forward.

## D-14 Spike — Two-Day ABI Race

Phase 11 planning locked **two candidate solutions** to test on mgga_c_b94 in a time-boxed spike (Plan 11-05):

### **Option A: Make all 38 math/src/ helpers generic over `<F: Float>`**
- **Lines touched:** ~3000–5000 (F literals + named constants in helper bodies)
- **Complexity:** LOW (mechanical genericity; `cubecl_macro_fanout_manual.md` §6 supports it)
- **Call sites in chunks:** Direct calls, no wrapping boilerplate — `let r = pow_1_3(x);`
- **Risk:** None identified; this is the "natural" reading of D-02 extended to helper-layer

### **Option C: Keep helpers f64, wrap call sites in translator**
- **Lines touched math/src/:** 0 (helpers unchanged)
- **Complexity:** HIGH — ~581K call-site wraps across generated tree over 2 days
- **Call sites in chunks:** Every helper call becomes `F::new(helper(F::cast_into(...)))`
- **Risk:** Translation bloat; correctness surface area

Both options must pass three gates:
1. **Compile gate:** `cargo build -p libxc-kernel-mgga_c_b94` (kernel chunks + dispatch) succeeds
2. **Dispatch gate:** `cargo build -p libxc_rs` (full registry + evaluation) succeeds
3. **Parity gate:** Oracle energy + routed derivatives at ≥1e-12 relative error (f64)

**Time-box:** 1 day per option, up to 2 days. If both pass, planner picks based on risk profile and maintenance. If one fails, use the other. If both fail, escalate to third discuss-phase.

## Carry-Forward Artifacts (Locked, All Preserved)

| Component | Commit | Status |
|-----------|--------|--------|
| 11-01 SUMMARY | `c181b469` + `a5790c26` + `d17e2968` | ✅ Wave 0 audit tools + D-02 isolated spike |
| 11-02 SUMMARY | `61c9f620` | ✅ Routing-aware emit.py + MAX_TUPLE_ARITY=12 |
| 11-03 SUMMARY | `eea58fed7` + `f820fae90` + `b7c0bd7e9` | ✅ Clean-slate 266 subcrates + D-13 audit + path-resolution gate |
| Q01 emit fixes | `5c379dc25` | ✅ Three CubeCL 0.10 fixes (Layers 1, 3, 2A partial); spike discovered Layer 4 blocker |
| 11-04 Task 1A | `39eb75f93` | ✅ Verify dev-dep narrowing (D-05 OOM fix) |

## Metrics Snapshot

| Metric | Value | Status |
|--------|-------|--------|
| Per-functional subcrates in committed tree | 266 | ✅ From 11-03 clean-slate (`97d6347be`) |
| Files ≤5K lines | ~40% of generated tree | ⏳ Blocked pending D-14 ABI lock |
| Wide-tuple chunk files | 1118 | ⏳ CSE emit produces tuples >arity-12; Layers 1–3 fixed, Layer 4 ABI-dependent |
| Dispatch tree state | WIP per-functional (c3fba8089) | ✅ Deterministic, zero-diff on regen |
| Compilation status | **Blocked on D-14 ABI choice** | ❌ mgga_c_b94 + dispatch fail with Layer 4 type errors |

## Next Phase — Plan 11-05 (D-02 ABI Spike)

**Objective:** Spike both ABI options on mgga_c_b94 (worst-case 16K+ lines) under a 2-day time-box. Lock D-02 final ABI based on gate results.

**Sequencing:** Plans 11-06..08 (full regen, per-subcrate verification, phase closure) are blocked on 11-05's D-02 decision.

**Gates:**
1. Kernel spike compiles: `cargo build -p libxc-kernel-mgga_c_b94`
2. Dispatch expands: `cargo build -p libxc_rs` (full registry)
3. Oracle parity: ≥1e-12 relative error on energy + routed derivatives

**Outcome:** D-02 locked as Option A or Option C, enabling full-phase forward progress.

---

## Self-Check: PARTIAL ✓

This plan is a retroactive documentation artifact for work that paused mid-execution. Task 1A landed successfully (commit `39eb75f93`). Task 1B was blocked by architectural findings that now direct the phase's forward path via D-14 spike.

**Status:**
- ✅ Retroactive SUMMARY created
- ✅ D-02 blocker documented with four-layer analysis
- ✅ D-14 spike scope explicit (2-day A vs C race on mgga_c_b94)
- ✅ Carry-forward artifacts locked
- ✅ Next phase (11-05) ready to unblock via ABI decision
