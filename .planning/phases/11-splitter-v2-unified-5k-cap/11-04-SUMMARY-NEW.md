---
phase: 11-splitter-v2-unified-5k-cap
plan: 04
type: documentation
status: PARTIAL — Task 1A committed, replan triggered
completed-partial: 2026-05-15
---

# Plan 11-04: Retroactive SUMMARY (Partial — D-02 Architectural Blocker)

**Status: PAUSED at Task 1A after commit `39eb75f93` landed cleanly.**

This plan was designed as the first per-`-p` verification gate (per D-12). Task 1A (narrowing `verify/Cargo.toml` dev-dependencies per D-05) succeeded and committed. Task 1B (first per-`-p` canary compile) triggered an architectural blocker that escalated to a replan (D-14..D-17), pausing the phase.

## What Landed

**Commit `39eb75f93`** — `verify/Cargo.toml` rewrite (D-05 OOM structural fix):
- Removed umbrella `libxc-kernel-{lda,mgga}` dev-dependencies.
- Added individual per-functional subcrate dev-dependencies for the initial canary set:
  - LDA: `lda_x`, `lda_c_pw`, `lda_xc_teter93`
  - GGA: `gga_x_pbe`, `gga_c_pbe`, `gga_x_lb`
  - MGGA: `mgga_c_b94`, `mgga_x_tpss`, `mgga_x_r4scan`
- This is the D-05 structural fix for verify-crate OOM: no longer pulling all 266 kernel subcrates into one linker invocation.

**Impact:** The narrowed dev-deps pattern is correct and is preserved across the replan. Future per-`-p` work (11-05..08) builds on this structure.

## Why the Pause: D-02 Chunk ABI × Helper-Layer Incompatibility

**Root cause:** The 11-01 D-02 spike tested tuple-return genericity in **isolation** against synthesized expressions. It never called the `crates/kernels/math/src/` helper functions. Task 1B's first `cargo build -p libxc-kernel-lda_xc_teter93` revealed:

- Generated chunks are `<F: Float>` generic per D-02.
- 38 math/src/ helpers are concrete `f64` (zero genericity).
- CubeCL 0.10 does not auto-coerce `F` → `f64` at call sites.
- Result: type mismatch at every helper call in the generated chunks.

**Empirical evidence:** Quick task `260515-q01-cse-chunk-arity-cap-12` (commit `5c379dc25`) discovered this via a spike at `crates/kernels/math/tests/spike_cse_emit_q01.rs` — the first real generated chunk calling real helpers. Full analysis in `.planning/quick/260515-q01-cse-chunk-arity-cap-12/SPIKE-FINDINGS.md`.

## Four-Layer Bug Structure (from q01 analysis)

| Layer | Bug | CubeCL 0.10 | Fix Status | Notes |
|-------|-----|-----------|-----------|-------|
| **1** | Tuple arity cap: `CubeType` max 12; CSE emitted 16 | ❌ E0277 | ✅ Fixed `5c379dc25` | `tools/translate_v2/cse.py MAX_TUPLE_ARITY` |
| **2A** | Raw `0.123e1` literal; no `F::new` wrap | ❌ E0277 | ✅ Partial `5c379dc25` | `_wrap_f64_literals` regex; misses integer-mantissa + named constants |
| **2B** | Integer-mantissa (`2e-21`) + named constants (`M_PI`, `M_CBRT3`, …) | ❌ E0277 | ⏳ D-16 work | ~4136 chunks affected; ready for implementation |
| **3** | `let` inside `-> (F,)` infers tuple-typed binding | ❌ E0308 | ✅ Fixed `5c379dc25` | Single-output chunks now emit `-> F` scalar |
| **4** | **`<F: Float>` chunks call `pow_1_3(x: f64) -> f64` etc. — 38 concrete-f64 helpers cannot accept F-typed args** | ❌ **ARCHITECTURAL** | ⏳ **D-14 spike** | **This layer blocks forward progress** |

Layer 4 is the architectural blocker: it requires a fundamental ABI choice for the entire phase, not a quick regex fix.

## D-14 Spike Scope (Locked via /gsd-discuss-phase 2026-05-18)

Phase 11 planning locked two ABI candidates to test on `mgga_c_b94` (worst case: 16,703-line `kxc_pol`):

### **Option A (LOCKED 2026-05-18):** Refactor all 38 helpers to generic `<F: Float>`
- **Lines touched:** ~3K–5K (f64 literal wraps in helper bodies; `F::new`, named-constant wraps)
- **Complexity:** LOW (mechanical; `cubecl_macro_fanout_manual.md` §6 supports it)
- **Call sites:** Direct, no wrapping boilerplate — `let r = pow_1_3(x);`
- **Risk:** None identified; natural reading of D-02 extended to helper-layer

### **Option C (fallback):** Keep helpers f64; wrap at call site in translator
- **Lines touched math/src/:** 0 (helpers unchanged)
- **Complexity:** HIGH (~581K call-site wraps across generated tree, 2 days of translator work)
- **Call sites:** Every helper call becomes `F::new(helper(F::cast_into(...)))`
- **Risk:** Translation bloat, correctness surface area

**Both options must pass three gates:**
1. `cargo build -p libxc-kernel-mgga_c_b94` (kernel chunk + dispatch macros)
2. `cargo build -p libxc_rs` (full registry + evaluation)
3. Oracle parity ≥1e-12 relative error on energy + routed derivatives (f64, one-shot `is_deferred` bypass allowed)

**Outcome (per D-14):** Option A was user-locked. 11-05..08 replan validates and implements it.

## Carry-Forward Artifacts (All Preserved)

| Component | Commit | Status |
|-----------|--------|--------|
| 11-01 SUMMARY | `c181b469` + `a5790c26` + `d17e2968` | ✓ Wave 0: audit tools, D-02 isolated spike, baseline |
| 11-02 SUMMARY | `61c9f620` | ✓ Routing-aware emit.py + MAX_TUPLE_ARITY=12 work |
| 11-03 SUMMARY | `eea58fed7` + `f820fae90` + `b7c0bd7e9` | ✓ 266-subcrate clean-slate + D-13 audit + path-resolution gate |
| 11-04 Task 1A | `39eb75f93` | ✓ Verify dev-dep narrowing (D-05 OOM fix) |
| Q01 emit fixes | `5c379dc25` | ✓ Layers 1–3 fixes (arity, literal, 1-tuple); Layer 4 blocker identified |
| Wave 0 spike | `spike_tuple_return_cube.rs` + `spike_cse_emit_q01.rs` | ✓ D-02 isolated (WIP), Layer 4 blocker empirical evidence |

## Metrics Snapshot

| Metric | Value | Status |
|--------|-------|--------|
| Per-functional subcrates committed | 266 (`97d6347be`) | ✓ 11-03 clean-slate |
| Oversized files (>5K) | ~235 → ~1118 (post-regen) | ⏳ Blocked pending D-02 lock |
| Dispatch staleness (batchN refs) | 0 (11-03 verified) | ✓ Blocker B1 closed |
| Helper-layer genericity | 0 of 38 generic | ⏳ D-14 scope |
| Routed subcrates | 259 | ✓ From 11-03 routing table |

## Critical Anti-Patterns Flagged (AP-1..6)

| Anti-pattern | Severity | Why |
|---|---|---|
| Re-execute without replanning | **BLOCKING** | Per-`-p` compile will loop on Layer 4 type errors. Replan first. |
| Modify `.cargo/config.toml` | **BLOCKING** | D-07/D-08/D-09 (jobs=1, RUST_MIN_STACK, sccache). Committed is source of truth. |
| Hand-edit kernel files | **BLOCKING** | D-LOCK-D idempotency. Fixes must go through translator. |
| Revert `5c379dc25` | **WARNING** | Layers 1–3 are correct and spike-validated. Replan builds on them. |
| Redo 11-01/02/03 | **WARNING** | Their deliverables (tooling, baseline, dispatch audit) are intact. Replan reframes 11-04..08. |
| Declare structural completion without per-`-p` gates | **BLOCKING** (AP-6) | Defining failure of Phase 11 2026-05-13..05-15. Every plan must have compile-first entry criterion. |

## Next Phase: 11-05..08 Replan

**Structure (per D-17):**
- **11-05 (Wave 1):** D-14 spike on mgga_c_b94 — Option A (lock D-02 via 3-leg gate: compile + dispatch + parity 1e-12)
- **11-06 (Wave 2):** Translator update per chosen ABI + math/src/ test drift fix (CubeCL 0.10 API)
- **11-07 (Wave 3):** Full 266-subcrate regen + D-15 compile-first entry gate (mgga_c_b94 three-leg + dispatch gate)
- **11-08 (Wave 4):** Per-`-p` sweep (259 routed subcrates), audits, phase close

**Entry gate for 11-05:** Spike is live; no blocking condition.

---

**Retroactive SUMMARY Status:** ✓ COMPLETE

This plan was paused mid-execution when an architectural issue surfaced. Task 1A (verify dev-dep narrowing) is preserved and correct. Task 1B (first per-`-p` compile) triggered a replan that now defines the forward path. No further work in 11-04; 11-05 begins the spike.

---
*Phase: 11-splitter-v2-unified-5k-cap*
*Plan: 04*
*Paused: 2026-05-15 (architectural blocker, D-02 ABI incompatibility with math/src/ helpers)*
*Replan: 2026-05-18 (D-14..D-17 locked via discuss-phase; Option A chosen)*
