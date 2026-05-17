---
quick_id: 260514-q02
slug: evaluate-mgga2-memory-peaks
captured: 2026-05-14
status: memo
predecessor: 260514-q01-split-mgga-2-large-kernels
related_phase: 11-splitter-v2-unified-5k-cap
---

# Re-evaluating the mgga-2 memory-peak design

Quick task **260514-q01** brought every `mgga-2/src/*.rs` file under the 5K-line
cap. `cargo check -p libxc-kernel-mgga-2` now succeeds at **22m 11s** on the
24 GB workstation. The win is real, but the cost is still high and three
distinct peak surfaces remain:

1. **Build-time RSS** — rustc + the `#[cube]` proc-macro expanding ~250
   functions per crate.
2. **Runtime RSS** — `Array<f64>` scratch buffers when a kernel is launched.
3. **Test-run RSS** — `cargo test -p libxc_rs-verify` transitively compiles
   `libxc-kernel-mgga`, which forces every sub-crate (mgga-2 included) through
   macro expansion before any verify-level test binary links. Documented
   blocker — see `[[feedback_verify_crate_oom]]`.

This memo re-evaluates the design behind each peak, then proposes one
bounded refactor for the current session.

---

## 1. The three peak surfaces, factored

### 1.1 Build-time peak (rustc + cubecl macro)

**Driver**: number and size of `#[cube] fn` definitions inside the crate.
Each `#[cube]` is a macro expansion target. The macro emits an expansion
function that builds CubeCL IR; both grow super-linearly with `#[cube]` body
size (see `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` §10).

**Today, in mgga-2:**

| Metric | Value |
|---|---|
| Functional directories | 17 |
| `.rs` files (each = one `#[cube] fn`) | ~290 |
| Total lines | 445,862 |
| Largest single `#[cube] fn` | 4,892 lines (`mgga_k_csk_loc/lxc_pol_part0.rs`) |
| `cargo check -p libxc-kernel-mgga-2` wall-clock | 22m 11s |

**Splitting algorithm bottoms out at one Maple2c output.** Per
`[[project_splitter_algorithm_floor]]`, the translator splits along output
boundaries (`vrho`, `v2rho2`, …). A single output with 4 < N < 17 cross-coupled
dependent variables can produce an 8K–16K-line monolithic file that the
algorithm cannot subdivide. mgga-2 dodged this because its remaining
worst-case is a 4,892-line file that **fits under 5K with margin**. mgga-3,
mgga-4, mgga-7, mgga-8b, lda-2, gga-1/3/4/5 still own the 8K–16K floors that
Phase 11's chunked-emission (plans 11-02..05) is designed to break.

**So for mgga-2 specifically, the floor is no longer a per-file problem; it is
now a per-crate problem.** With ~290 small `#[cube] fn`s, the proc-macro
runs ~290 times per `cargo check`. Each expansion is bounded, but the
aggregate macro work dominates wall-clock.

### 1.2 Runtime peak (kernel execution)

**Driver**: number and size of `&mut Array<f64>` scratch buffers passed to
each kernel. Each polarized 4th-order kernel writes ~70 output arrays of
length `N_points`. For `N = 4` (the test point count) and f64 each, that's
~2.2 KB per kernel — trivial.

For DFT-scale grids (`N ~ 10^5`–`10^6`), 70 output arrays at f64 is
56 MB – 560 MB per kernel invocation. The host-side allocation happens in
`MggaOutput::new`-style sites and is controlled by `OutputMask` gating (we
only allocate the buffers the caller asks for).

**This peak is bounded by the user's grid size, not by the crate's design.**
Per-kernel buffer allocation already follows the canonical
`client.create(bytemuck::cast_slice(...))` pattern and is gated by
`OutputMask`. There is no low-hanging fix here without breaking the public
API contract.

### 1.3 Test-run peak (verify/ OOM)

**Driver**: `verify/Cargo.toml` dev-dependencies pull in
`libxc-kernel-lda` + `libxc-kernel-mgga`, both of which transitively re-export
**every** numbered sub-crate (lda-1/2, mgga-1..14 family). `cargo test
-p libxc_rs-verify --test <anything>` must compile all of them before
linking the test binary. That's how the D-02 spike got OOM-killed at Wave 0
(`11-01-SUMMARY.md` D1, 11-BASELINE.md). Documented as `[[feedback_verify_crate_oom]]`.

The test that does need the full kernel tree (`mgga_oracle.rs`,
`parity_phase11.rs`) treats deferred kernels as **runtime SKIPs** via
`libxc_kernel_mgga::deferred::is_deferred(id)` — the SKIP check runs *before*
`MggaFunctional::from_id`, so the deferred kernel modules are never accessed
even though they were compiled. **The compile cost is paid for code that
is never instantiated at runtime.**

---

## 2. The hidden lever: deferred kernels are compile-cost-only

`crates/kernels/mgga/src/deferred.rs` lists 6 MGGA functionals that
`MggaFunctional::from_id` rejects with `UnsupportedFunctional`:

| Functional | libxc id | Crate | Lines | Files | Why deferred |
|---|---|---|---|---|---|
| `mgga_c_b94` | 397 | mgga-2 | 72,477 | 26 | Brent's root-finder for BR89 |
| `mgga_x_br89` | 206 | mgga-13 | ? | ? | same |
| `mgga_x_mbr` | 716 | mgga-14 | ? | ? | same |
| `mgga_x_mbrxc_bg` | 696 | mgga-5 | ? | ? | MBRXC variant |
| `mgga_x_mbrxh_bg` | 697 | mgga-3 | ? | ? | BR89 variant |
| `mgga_x_mggac` | 711 | mgga-7 | ? | ? | MBRXC variant |
| **Total** | | | **569,323** | | |

Plus `mgga_x_br89_explicit` (mgga-14, **91,286 lines, ~8 files**) which is
NOT in the deferred list but is also unrouted by `MggaFunctional::from_id`
— it's referenced only by `verify/tests/parity_phase11.rs` as a future
target.

**Each deferred kernel is `pub mod`-d unconditionally** in its host crate's
`lib.rs`. There is no `cfg`-gate. `cargo check` builds them every time.

**Routing already blocks them at runtime.** `mgga_oracle.rs:496` and
`mgga_functional.rs:from_id` both check `is_deferred(id)` before any kernel
module is touched. The deferred-list gate is **load-bearing at runtime but
free at compile time** — the situation is upside-down.

**Quantified blast radius**: feature-gating the 6 deferred modules excludes
~570K lines from default `cargo check`. In mgga-2 specifically, removing
`mgga_c_b94` from compilation drops the crate from 445,862 lines to
373,385 lines — **16.3% reduction** in proc-macro fan-out work and
proportional drop in cargo-check RSS + wall-clock.

This is **independent** of Phase 11's chunked-emission program and lands
faster than any plan in `11-02..05`.

---

## 3. Options on the table

| # | Option | mgga-2 build-RSS impact | risk | scope | Phase 11 collision |
|---|---|---|---|---|---|
| **A** | **Feature-gate the 6 deferred kernel modules** (this memo's pick) | **-16% lines, -X% RSS, -X min wall-clock** | low — deferred-list is already authoritative for routing; tests already SKIP via `is_deferred` | small — 6 cfg-attrs + 1 cargo feature per affected crate + verify/ tests guarded by the same cfg | none — orthogonal to chunked emission |
| B | Raise `SPLIT_THRESHOLD` from 4500/5000 → 4900 and merge small adjacent splits more aggressively | unclear, mgga-2 already compiles; possibly **negative** if larger files trigger macro fanout | medium — `[[project_split_threshold_history]]` shows 100K OOM'd lda-2 | small | low — but treads in q01's territory |
| C | Move `verify/`'s `libxc-kernel-mgga` dev-dep to a feature-gated dev-dep so `cargo test -p libxc_rs-verify` does not transitively force the full kernel build | large drop for test-run peak | medium — every existing oracle test would need `--features oracle-kernels` | medium | medium — also affects Phase 11's parity_phase11 wiring |
| D | Wait for Phase 11 chunked-emission (plans 11-02..05) | will land 0 oversized files but doesn't change the **count** of `#[cube] fn`s | low | very large (whole phase) | this IS Phase 11 |
| E | Introduce a `comptime!`-gated all-orders kernel that subsumes exc/vxc/fxc/kxc/lxc and OutputMask-selects at IR time, halving the `#[cube] fn` count | very large | very high — re-architects the whole translator output | extra-large | conflicts directly with Phase 11's D-02 chunking ABI |

**Why A is the right next step**:

- Pure dead-code removal at the cfg gate. Routing is unchanged (the
  deferred-list still rejects at runtime).
- No translator change — purely cargo-feature + module-attribute work.
- Test infrastructure already skips deferred kernels via `is_deferred()`,
  so the existing oracle tests remain green with or without the feature.
- The benefit applies **immediately** and **stacks with** Phase 11's
  chunked emission later.
- B sits inside q01's tested envelope — risk-of-regression unfavourable
  given `[[project_split_threshold_history]]`.
- C is a verify/ refactor; it doesn't reduce mgga-2's own
  `cargo check -p libxc-kernel-mgga-2` cost.
- D is a real plan, not a quick task.
- E re-architects too much for a quick task.

---

## 4. Proposed bounded refactor (Option A)

### 4.1 What changes

**Per affected crate** (mgga-2, mgga-3, mgga-5, mgga-7, mgga-13, mgga-14):

1. Add to `Cargo.toml`:
   ```toml
   [features]
   deferred-kernels = []   # opt-in: include kernels whose libxc IDs are deferred at runtime
   ```

2. In `src/lib.rs`, wrap the `pub mod` line for the deferred module:
   ```rust
   #[cfg(feature = "deferred-kernels")]
   pub mod mgga_c_b94;
   ```

**In the umbrella `libxc-kernel-mgga` crate** (`crates/kernels/mgga/Cargo.toml`):

3. Add the same `deferred-kernels` feature and forward it to each sub-crate:
   ```toml
   [features]
   deferred-kernels = [
       "libxc-kernel-mgga-2/deferred-kernels",
       "libxc-kernel-mgga-3/deferred-kernels",
       "libxc-kernel-mgga-5/deferred-kernels",
       "libxc-kernel-mgga-7/deferred-kernels",
       "libxc-kernel-mgga-13/deferred-kernels",
       "libxc-kernel-mgga-14/deferred-kernels",
   ]
   ```

**`mgga_x_br89_explicit` is NOT in this refactor.** It is unrouted but not
in the formal deferred list. Per
`verify/tests/parity_phase11.rs:107`, Phase 11 wants this functional to come
online. Touching it here would prejudge a Phase 11 decision.

### 4.2 What does NOT change

- `crates/kernels/mgga/src/deferred.rs` — the data table stays; routing
  contracts are unchanged.
- `verify/tests/mgga_oracle.rs` — already skips deferred IDs via
  `is_deferred()`. With the feature off, the routing still rejects;
  with the feature on, the routing still rejects. The test passes both ways.
- `verify/tests/parity_phase11.rs` — entry for `mgga_c_b94` (id 568 per
  the parity table — note id divergence is intentional in that table)
  remains; if the entry attempts to dispatch, `is_deferred` blocks it.
- Translator scripts — no change. Re-translating `mgga_c_b94` later still
  emits to `crates/kernels/mgga-2/src/mgga_c_b94/`; only the `pub mod` line
  is gated.

### 4.3 Verification plan

1. **Without the feature** (default):
   - `cargo check -p libxc-kernel-mgga-2` — must succeed. Measure wall-clock
     and compare to the q01 baseline of 22m 11s.
   - `cargo check -p libxc-kernel-mgga` — must succeed.
2. **With the feature**:
   - `cargo check -p libxc-kernel-mgga-2 --features deferred-kernels` —
     must succeed (this is the pre-refactor build).
3. **Oracle parity unaffected**: `verify/tests/mgga_oracle.rs` test count
   unchanged (deferred SKIPs count both with and without the feature; the
   underlying mechanism is `is_deferred()`, not module presence).

The cargo check is RAM-expensive — per `[[feedback_ram_constraints]]` we run
inline with `jobs ≤ 2`. The user will run the cargo check after the edit
lands; this quick task commits the cfg-gating in one atomic commit and the
measurement in a second commit once the user pastes the wall-clock.

### 4.4 Why this is the right size for /gsd-quick

- Only 12 small file edits (6 lib.rs lines + 6 Cargo.toml stanzas + 1
  umbrella forward).
- No translator changes.
- No oracle changes.
- Test contract preserved.
- Phase 11 unaffected; lands an immediate win for build-time peak without
  treading on chunked-emission work.

---

## 5. Pending follow-ups (NOT this task)

- Re-evaluate `mgga_x_br89_explicit` (91K lines, mgga-14, unrouted but
  outside deferred-list). Likely candidate for the same feature gate
  once Phase 11 chunked emission decides whether to route it.
- Phase 11 plans 11-02..05 will turn the 8K–16K single-output floor into
  ≤5K chunks. After that, the `cargo check` cost converges on
  `(num_cube_fns × per_fn_macro_work)`. At that point option E (collapsing
  exc/vxc/fxc/kxc/lxc into one comptime-gated kernel) becomes a real
  option for the next order-of-magnitude drop.
- Option C (gating verify's kernel dev-deps) is a separate quick task
  candidate; gives the biggest single win on test-run peak but doesn't
  help the per-crate `cargo check` baseline.

---

## 6. Decision

Recommend executing Option A in this `/gsd-quick` session.

If approved, this memo's commit and the cfg-gate commit are both made
under `quick-260514-q02`. The cargo-check measurement post-edit is the
verification gate.
