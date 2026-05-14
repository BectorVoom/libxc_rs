# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Pattern Map

**Mapped:** 2026-05-14
**Files analyzed:** 14 file classes (this is a tooling/codegen phase — "files" are tool modules + generated-artifact templates + config edits)
**Analogs found:** 13 / 14 (1 net-new: the CSE pass module)

> **REFRESH NOTE.** This PATTERNS.md was fully regenerated against the 2026-05-14 CONTEXT revision. The unification target is now **per-functional subcrates** (`crates/kernels/{family}/<func>/`, ~264 crates; family dir is a plain directory), and the within-subcrate layout is **nested-by-output** (`src/kxc_pol/part01.rs`), per q02 commit `504d8560`. The prior per-family-crate / flat-`_partNN` patterns are superseded.

> **This is a code-generation phase.** Most "files to create" are *emitted artifacts* — the planner's plans modify the **Python emitters** in `tools/`, and the analogs are (a) the current emitter code being re-engineered and (b) the q02 `mgga_c_b94` hand-built layout that the emitter must learn to reproduce. Treat the q02 layout as the **golden output spec**.

---

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `tools/translate_v2/cse.py` (or in-place CSE in `translate_*.py`) | utility (codegen pass) | transform | `tools/translate_lda_v2.py` §`build_dependency_graph`/`transitive_deps`/`split_by_output_array` (L370–447) | role-match |
| `tools/translate_lda_v2.py` (modified — chunk ABI + emit target) | utility (codegen) | transform | itself L480–852 (chunked-scratch path being replaced) | exact (self) |
| `tools/translate_gga.py` (modified — add CSE + emit target) | utility (codegen) | transform | `tools/translate_lda_v2.py` (the only translator with a chunked path) | role-match |
| `tools/translate_mgga.py` (modified — add CSE + emit target) | utility (codegen) | transform | `tools/translate_lda_v2.py` chunked path + `tools/translate_mgga.py` `split_by_output_array` L591 | role-match |
| `tools/maple_to_kernels.py` (modified — re-tune thresholds, per-subcrate emit) | config/driver | batch | itself (L120 stale `DEFAULT_SPLIT_THRESHOLD`) | exact (self) |
| Emitted: `crates/kernels/{family}/<func>/Cargo.toml` (~264 generated) | config | — | `crates/kernels/mgga-2/Cargo.toml` | exact |
| Emitted: `crates/kernels/{family}/<func>/src/lib.rs` (~264 generated) | config (crate root) | — | `crates/kernels/mgga-2/src/mgga_c_b94/mod.rs` (q02 nested) | exact (golden spec) |
| Emitted: `crates/kernels/{family}/<func>/src/<output>/mod.rs` (nested-by-output wrapper) | model (cube entry) | request-response | `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/mod.rs` (q02) | exact (golden spec) |
| Emitted: `crates/kernels/{family}/<func>/src/<output>/partNN.rs` (CSE chunk / output part) | model (cube helper) | transform | `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/part1.rs` (q02) | exact (golden spec) |
| Emitted: `crates/kernels/{family}/<func>/src/<output>.rs` (single-file output, ≤5K) | model (cube entry) | request-response | `crates/kernels/mgga-2/src/mgga_c_rppscan/exc_pol.rs` | exact |
| `Cargo.toml` root — `[dependencies]` + `[workspace] default-members` rewrite | config | — | itself L6–55 (current numbered-subcrate lists) | exact (self) |
| `src/eval/{gga,mgga}_dispatch/*.rs` (regenerated — drop `batchN::`) | route (dispatch) | request-response | `src/eval/mgga_dispatch/batch17.rs` + `tools/generate_gga_dispatch.py` | exact |
| `tools/generate_gga_dispatch.py` (+ MGGA equiv) (modified — per-func paths) | utility (codegen) | transform | `tools/generate_gga_dispatch.py` (self, L1–728) | exact (self) |
| `tools/audit_subcrate_collapse.sh` (modified — add per-family-crate-absence invariant) | test (audit) | — | `tools/audit_subcrate_collapse.sh` (self) | exact (self) |
| `CLAUDE.md` (modified — D-03a precision policy) | config (docs) | — | `CLAUDE.md` § Constraints | exact (self) |

**No analog for:** `tools/translate_v2/cse.py` as a *standalone CSE/min-cut module* — no existing tool does dep-DAG min-cut partitioning. It is role-matched (dep-graph utility) by the existing `build_dependency_graph`/`transitive_deps` in `translate_lda_v2.py` L370–402, which the planner should extend rather than rebuild. See "No Analog Found" below.

---

## Pattern Assignments

### Emitted: `crates/kernels/{family}/<func>/Cargo.toml` (config, ~264 generated)

**Analog:** `crates/kernels/mgga-2/Cargo.toml` — a current numbered-subcrate manifest. The per-functional subcrate `Cargo.toml` is structurally identical; only `name` changes.

**Full template** (`crates/kernels/mgga-2/Cargo.toml` lines 1-8):
```toml
[package]
name = "libxc-kernel-mgga-2"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../math" }
```

**What changes for the per-functional emit (D-10, Claude's Discretion on naming):**
- `name = "libxc-kernel-<func>"` — e.g. `libxc-kernel-gga_c_acgga`. **Open question for planner:** cargo package names conventionally use hyphens, but functional ids use underscores. The current convention is `libxc-kernel-mgga-2` (hyphenated). Recommendation per CONTEXT Claude's Discretion: keep `name = "libxc-kernel-<func>"` with the underscore-bearing func id verbatim (cargo permits underscores in package names) and lib name `libxc_kernel_<func>`. Planner confirms.
- `libxc-kernel-math = { path = "../../math" }` — note the depth changes: numbered subcrates are at `crates/kernels/mgga-2/` (`../math`); per-functional subcrates are at `crates/kernels/mgga/<func>/` (`../../math`).
- Keep `cubecl` dep verbatim — `version = "0.10.0", default-features = false, features = ["cpu"]`. Do NOT add `cuda`/`wgpu`/`hip` features (feature-gated at workspace root, not per-kernel).
- Some functionals also need `libm` — inspect the per-functional `detect_imports` output; the numbered subcrates that use `libm` list it. Emit conditionally.
- **No `[dev-dependencies]`** on kernel subcrates (the `crates/kernels/lda/Cargo.toml` `libxc_rs` dev-dep is a façade-crate-only artifact and must NOT propagate to per-functional subcrates — it would re-create the verify/ OOM cycle).

---

### Emitted: `crates/kernels/{family}/<func>/src/lib.rs` (crate root, ~264 generated)

**Analog (GOLDEN SPEC):** `crates/kernels/mgga-2/src/mgga_c_b94/mod.rs` — q02's nested-by-output module enumeration. The per-functional `src/lib.rs` is exactly this `mod.rs` content, promoted from a `mod.rs` inside a fat crate to the `lib.rs` of its own crate.

**Pattern — output-module enumeration** (`mgga_c_b94/mod.rs` lines 10-17):
```rust
pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
```

**Emission rules for splitter v2:**
- One `pub mod <output>;` per `(level, spin)` the functional actually routes (`exc_unpol`, `vxc_pol`, `kxc_pol`, …). Order: unpol levels then pol levels, lowest derivative first (matches q02).
- Each `<output>` is EITHER a single file `src/<output>.rs` (functional ≤5K at that output) OR a directory `src/<output>/` with its own `mod.rs` + `partNN.rs` (output exceeded 5K → CSE-chunked). The `pub mod <output>;` line is identical in both cases — Rust resolves `.rs` file vs `<output>/mod.rs` directory transparently.
- Prepend the crate-level `#![allow(...)]` block — see q02's `kxc_pol/mod.rs` L12 for the canonical allow-list: `#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]`. The current numbered-subcrate `lib.rs` (`crates/kernels/mgga-2/src/lib.rs` L1-3) uses a narrower set; **adopt the q02 wider allow-list** for per-functional `lib.rs`.
- Module doc-comment naming the functional + a one-line provenance note (q02 `mod.rs` L1-8 style).

---

### Emitted: `crates/kernels/{family}/<func>/src/<output>/mod.rs` (cube entry wrapper, nested-by-output)

**Analog (GOLDEN SPEC):** `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/mod.rs` — the q02 commit `504d8560` precedent that D-04 standardizes on. This is the single most load-bearing analog in the phase.

**Pattern — private partN modules + single `#[cube]` entry wrapper** (`mgga_c_b94/kxc_pol/mod.rs` lines 12-52, 98-166):
```rust
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
// ... mod part15;

use cubecl::prelude::*;

use part0::mgga_c_b94_kxc_pol_part0;
use part1::mgga_c_b94_kxc_pol_part1;
// ... use part15::...;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_kxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    /* ... all output &mut Array<f64> for this level ... */
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_b94_kxc_pol_part0(
        rho, sigma, lapl, tau,
        zk, vrho, /* ... outputs this part writes ... */
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part1(
        rho, sigma, lapl, tau, v3rho3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    // ... one call per partN, in order ...
}
```

**Emission rules for splitter v2 (the wrapper emitter):**
- `mod partNN;` declarations are **private** (`mod`, not `pub mod`) — partN modules are NOT re-exported (q02 `mod.rs` L6-7 doc-comment makes this explicit).
- The wrapper `pub fn <func>_<output>` is the **single** `#[cube]` (or `#[cube(launch_unchecked)]` if it's a routed entry kernel — see Shared Pattern: routing) entry point. It takes the full output-buffer set + params + thresholds.
- Wrapper body = ordered sequence of `partNN(...)` calls. **Each call passes only the subset of buffers that part writes** — q02 part0 takes all the low-order outputs, part1 takes only `v3rho3`, part2 only `v3rho2sigma`, etc. This subset-passing is the existing `split_by_output_array` per-output-component cut, preserved.
- **CRITICAL — this is the D-02 transition point.** q02's `mgga_c_b94` parts are still the OLD ABI (`&Array<f64>` / `&mut Array<f64>` buffer-passing, hardcoded `f64`). For functionals where a *single output* still exceeds 5K after the per-output cut (r4scan, br89_explicit, kcisk lxc_pol, etc.), splitter v2 must additionally CSE-subdivide that single output into D-02 tuple-return `<F: Float>` chunks. The wrapper for those cases destructures tuples: `let (t89, t142) = chunk_3::<F>(t12, t34);` then feeds the next chunk. q02's `mod.rs` shows the *outer* shape (per-output buffer-passing); the *inner* shape for over-5K single outputs is the D-02 spike `crates/kernels/math/tests/spike_tuple_return_cube.rs`.
- Output ordering: q02 calls `part0..part15` strictly in emission order; preserve operation order (CLAUDE.md constraint, relaxed by D-05 to 1e-12 but order-preservation is still the emit default).

---

### Emitted: `crates/kernels/{family}/<func>/src/<output>/partNN.rs` (cube helper / CSE chunk)

**Analog (GOLDEN SPEC for outer/per-output-component shape):** `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/part1.rs`.
**Analog (GOLDEN SPEC for inner/CSE-chunk D-02 shape):** `crates/kernels/math/tests/spike_tuple_return_cube.rs` (the proven tuple-return `<F: Float>` ABI).

**Pattern — per-output-component part (current ABI, q02 part1.rs lines 1-27):**
```rust
//! MGGA_C_B94 kxc pol kernel — split part 1/16 (v3rho3).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b94.c`.
//! Split sub-kernel: outputs [v3rho3] (2927 lines).

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_kxc_pol_part1(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    v3rho3: &mut Array<f64>,
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v3rho3.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // ... let tN = ...; chains ...
        // v3rho3[ip * dim] += tN;   (output write)
    }
}
```

**Import-block pattern (per-part `use libxc_kernel_math::...`):** q02 part1.rs L8-12 — `use cubecl::prelude::*;` always, then exactly the math primitives this part references (`detect_imports` output). **Note the path is `libxc_kernel_math::` — the underscore lib name, NOT the hyphenated package name.** Per-functional subcrates depend on `libxc-kernel-math` (package) → `libxc_kernel_math` (crate path) — unchanged from numbered subcrates.

**Pattern — CSE chunk (D-02 tuple-return, the NEW shape for over-5K single outputs):** See `crates/kernels/math/tests/spike_tuple_return_cube.rs` for the verified shape:
```rust
#[cube]
fn chunk_NN<F: Float>(a: F, b: F, /* explicit value args */) -> (F, F /* tuple return */) {
    // ... CSE-stage body ...
    (out1, out2)
}
```
- D-02 chunks are plain `#[cube]`, **never** `#[cube(launch_unchecked)]` (CONTEXT D-02; manual §4, §19).
- Generic `<F: Float>` — args are `F`, returns are tuples of `F`. Bool intermediates can't pass through `F` tuples — handle inline (the existing LDA chunked-scratch path at `translate_lda_v2.py` L532-548 already understands this; carry the logic forward).
- Tuple arity cap recommended ≤16 in / ≤16 out (RESEARCH "CSE Detection Heuristic" — may need to drop to 8).

**Header doc-comment pattern:** q02 part1.rs L1-4 — name the functional, the output level, the part index `N/total`, the provenance C path, and the output list + line count. The splitter emits this; keep it.

---

### Emitted: `crates/kernels/{family}/<func>/src/<output>.rs` (single-file output, ≤5K)

**Analog:** `crates/kernels/mgga-2/src/mgga_c_rppscan/exc_pol.rs` — a per-output file that fit under the cap with no chunking. Same `#[cube]` fn shape as a partN minus the part-index doc-comment; this is what `generate_function` (`translate_lda_v2.py` L501-586) already emits. The emitter for this case is **unchanged** structurally — only the *destination path* changes (into `crates/kernels/{family}/<func>/src/` instead of `crates/kernels/{family}-N/src/<func>/`).

---

### `Cargo.toml` root — `[dependencies]` + `[workspace] default-members` rewrite (D-10a)

**Analog:** `Cargo.toml` itself (self-analog) — L6-38 `[dependencies]`, L44-77 `[workspace]`.

**Current `[dependencies]` shape (to be rewritten, L6-38):**
```toml
[dependencies]
bitflags = "2.10.0"
bytemuck = { version = "1.25.0", features = ["derive"] }
cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }
thiserror = "2.0.18"
libxc-kernel-math = { path = "crates/kernels/math" }
libxc-kernel-lda = { path = "crates/kernels/lda" }
libxc-kernel-lda-1 = { path = "crates/kernels/lda-1" }   # ← DELETE all numbered
# ... libxc-kernel-mgga-1 .. mgga-14 ...                  # ← DELETE all numbered
```

**Rewrite rules (D-10a):**
- DELETE every `libxc-kernel-{lda,gga,mgga}-N` path-dep AND the three façade deps `libxc-kernel-{lda,gga,mgga}` (the family crates cease to exist).
- ADD `libxc-kernel-<func> = { path = "crates/kernels/{family}/<func>" }` entries for each routed functional. The 7 deferred subcrates (D-11) still get a dep entry so `cargo build -p` works on demand, but are EXCLUDED from `default-members`. **Decision point for planner:** does root `libxc_rs` actually need every kernel as a direct dep, or only those the dispatch tree imports? Inspect `src/lib.rs` `pub use` and `src/eval/*_dispatch/` imports — RESEARCH "What references the numbered subcrates" table shows dispatch reaches kernels via `crate::kernel::{family}::...` re-export paths, so the root dep set is whatever those re-exports need.
- `[workspace] default-members` (L44-77): replace the 31-entry numbered list with the ~257-entry per-functional list (264 minus 7 deferred). Keep `crates/kernels/math`. The 7 deferred subcrates exist as workspace members but are omitted from `default-members` (D-11) — `cargo build` skips them, `cargo build -p libxc-kernel-<func>` still builds them.
- `[workspace] members` (L45-48: `xtask`, `verify`, `libxc-sys`) — unchanged.
- **Generation:** this array is large and mechanical — emit programmatically from the routed-functional roster (CONTEXT Claude's Discretion). Recommend the splitter writes it as a sorted list, deterministically, so D-LOCK-D idempotency holds (`tools/test_idempotency.sh` will diff it).

---

### `src/eval/{gga,mgga}_dispatch/*.rs` — regenerated (D-10b, drop `batchN::`)

**Analog:** `src/eval/mgga_dispatch/batch17.rs` (the per-batch dispatch helper being restructured) + `src/eval/mgga_dispatch/mod.rs` (the top-level match + `MggaLaunchCtx` + launch macro).

**Current dispatch path pattern (`batch17.rs` lines 23-28) — the thing being rewritten:**
```rust
mgga_zero_scalar_unpol_dispatch!(
    ctx, order, spin,
    [crate::kernel::mgga::batch17::mgga_k_gea2::exc_unpol::mgga_k_gea2_exc_unpol],
    [crate::kernel::mgga::batch17::mgga_k_gea2::vxc_unpol::mgga_k_gea2_vxc_unpol],
    "mgga_k_gea2"
);
```

**Rewrite target (D-10b):** drop the `batchN::` segment. Path becomes `crate::kernel::mgga::mgga_k_gea2::exc_unpol::mgga_k_gea2_exc_unpol` (façade re-export per functional) — OR direct `libxc_kernel_mgga_k_gea2::exc_unpol::...`. **Planner's call on the exact façade shape** (CONTEXT D-10b). RESEARCH "Strategy 1 (façade preserves dispatch paths)" is REJECTED by the revised CONTEXT — there is no `batchN` to preserve.

**Recommended shape (CONTEXT D-10b):** drop the per-batch submodule layer entirely; emit one `dispatch_<func>` helper per functional under `src/eval/{family}_dispatch/funcs/<func>.rs`. The `mod.rs` top-level match (`mgga_dispatch/mod.rs` L352-378) stays — it just delegates to `funcs::<func>::dispatch_<func>` instead of `batchN::dispatch_<func>`.

**Patterns to preserve verbatim from `mgga_dispatch/mod.rs`:**
- The `MggaLaunchCtx<'a>` struct (L64-88) — handle + scalar bag.
- The `mgga_zero_scalar_unpol_dispatch!` macro (L102-185) — `ArrayArg::from_raw_parts`, `launch_unchecked::<CpuRuntime>`, the typed-error guards. **Note `from_raw_parts::<f64>(handle, len, 1)`** is the 3-arg form used in this dispatch macro; the Wave-0 spike (deviation D1) noted the 2-arg form `from_raw_parts(handle, len)` for the standalone spike — confirm which arity each call site uses against cubecl 0.10.
- The output-zeroing `zero_field!` macro (L239-262) and `readback!` macro (L390-400).
- The top-level `dispatch_mgga` entry-point match (L352-378) — keep the `match functional { ... }` shape, just retarget the delegate path.
- The header doc-comment "Auto-generated for plan ... from `mgga_roster.tsv`" — regenerate, don't hand-edit (`batch17.rs` L4).

**`#![allow(...)]` for dispatch files:** `batch17.rs` L8: `#![allow(unused_imports, unused_variables, clippy::too_many_arguments)]`.

---

### `tools/generate_gga_dispatch.py` (+ MGGA equivalent) — modified (D-10b)

**Analog:** `tools/generate_gga_dispatch.py` itself (728 lines, self-analog).

**Modification scope:**
- Input rosters `.planning/phases/04-bulk-kernel-translation/{gga,mgga}_roster.tsv` — the `batch` column is dropped or made degenerate (CONTEXT D-10b). The generator stops emitting `batchN.rs` files and `pub mod batchN;` in `mod.rs`.
- Emit per-functional dispatch helpers (recommended `funcs/<func>.rs`) with `crate::kernel::{family}::<func>::...` paths — no `batchN::` segment.
- The `audit_dispatch_tree.sh` Blocker B1 (stale `batch15..22` GGA / `batch17..35` MGGA references — see `11-DISPATCH-AUDIT.md`) is resolved by this regeneration: the dispatch tree currently references batch IDs that no longer match the kernel layout. Regen against the new per-functional layout fixes it.

---

### `tools/audit_subcrate_collapse.sh` — modified (add per-family-crate-absence invariant)

**Analog:** `tools/audit_subcrate_collapse.sh` itself (self-analog).

**Current invariant (the whole script):** fails if any `crates/kernels/{lda,gga,mgga}-N` numbered subcrate dir exists. `find ... -maxdepth 1 ... | grep -E '^(lda|gga|mgga)-[0-9]'`.

**Add (per CONTEXT canonical_refs note + D-LOCK-A):** also fail if `crates/kernels/{lda,gga,mgga}/Cargo.toml` OR `crates/kernels/{lda,gga,mgga}/src/lib.rs` exists — the family level must be a **plain directory**, not a crate. New check shape (mirror the existing one):
```bash
FAMILY_CRATES=$(for f in lda gga mgga; do
  [[ -f "$REPO_ROOT/crates/kernels/$f/Cargo.toml" ]] && echo "$f/Cargo.toml"
  [[ -f "$REPO_ROOT/crates/kernels/$f/src/lib.rs" ]] && echo "$f/src/lib.rs"
done)
```
Keep the existing `set -euo pipefail`, `REPO_ROOT` derivation, and the `Build env source of truth: .cargo/config.toml` header comment verbatim.

---

### `tools/translate_lda_v2.py` / `translate_gga.py` / `translate_mgga.py` — modified (CSE pass + emit target)

**Analog:** `tools/translate_lda_v2.py` — it is the *only* translator with a single-output chunked-scratch fallback (L480-852, the path D-02 replaces). GGA and MGGA bottom out at `split_by_output_array` (`translate_mgga.py` L591) with no chunked path.

**Existing dep-graph foundation to extend (`translate_lda_v2.py` L370-447):**
```python
def build_dependency_graph(compute_lines):   # L370 — var -> set of referenced vars
def transitive_deps(variables, var_deps):    # L389 — transitive closure
def split_by_output_array(compute_lines, output_writes, is_pol):  # L404 — per-output cut
def merge_small_splits(splits, threshold):   # L448 — cap-suffix merge
```

**Existing split decision tree (`translate_lda_v2.py` L778-811, `translate_mgga.py` L798-836) — the integration point for CSE:**
```python
if est <= SPLIT_THRESHOLD:           # single file
    ...
else:
    splits = split_by_output_array(...)
    splits = merge_small_splits(splits, SPLIT_THRESHOLD)
    if sub_est > SPLIT_THRESHOLD and len(sub_outputs) > 1:
        # split per output component
    # >>> CSE-AWARE SUBDIVISION HOOKS IN HERE <<<
    # currently (LDA only): chunk_single_output_split — WRONG ABI per D-02
```

**Existing chunk-helper ABI being replaced (`translate_lda_v2.py` ~L571-702) — the WRONG shape per D-02:** shared `&mut Array<f64>` scratch, `s[idx] = expr` writes, hardcoded `f64`. RESEARCH §"Current chunked-scratch ABI" documents this in full. D-02 replaces all 4 functions (`chunk_single_output_split`, `_generate_chunk_helper`, `_generate_chunked_wrapper`, `_build_scratch_replacer`) with the tuple-return `<F: Float>` emitter.

**`generate_function` emitter (`translate_lda_v2.py` L501-586) — the per-output `#[cube]` fn emitter, mostly reusable:**
- L516-518: `is_split_helper = fn_suffix.startswith('_part')` → emits plain `#[cube]`; routed entry kernels → `#[cube(launch_unchecked)]`. **This routing logic is the Shared Pattern below; keep it.**
- L554-555: `let ip = ABSOLUTE_POS; if ip < {bounds_arr}.len() {` bounds-guard prologue — keep.
- L562-582: per-line `let {var} = {translate_expr(expr)};` + output-write insertion — keep, this is the operation-order-preserving emit.
- **Change:** `fn_suffix` convention. Currently `_partNN` flat suffix on the filename. Under D-04 nested-by-output, the part is `src/<output>/partNN.rs` with fn name `<func>_<output>_partNN` — see q02 `part1.rs` `mgga_c_b94_kxc_pol_part1`. The emitter must write into the nested dir and emit the `<output>/mod.rs` wrapper (golden spec above).

**`SPLIT_THRESHOLD` (`translate_lda_v2.py` L362, `translate_gga.py` L483, `translate_mgga.py` L553):** currently `6000`. Phase 11 hard cap is 5000 (`audit_kernel_size.py` `KERNEL_LINE_CAP = 5000`). Lower `SPLIT_THRESHOLD` toward ~4500 (leave headroom for boilerplate vs the 5K hard cap — RESEARCH "CSE Detection Heuristic"). **Memory `project_split_threshold_history.md` warns: don't go below 4500 without recalibrating.** All three translators accept `--split-threshold` argv override (`translate_lda_v2.py` L1198, `translate_mgga.py` L1302).

**Whether to fork `tools/translate_v2/`:** CONTEXT Claude's Discretion. RESEARCH recommends a new `tools/translate_v2/cse.py` (~600 lines) sharing only `kernel_routing.py`. `tools/translators/` dir exists but is empty — available namespace.

---

### `tools/maple_to_kernels.py` — modified (threshold re-tune + per-subcrate emit)

**Analog:** `tools/maple_to_kernels.py` itself (246 lines, self-analog; the unified driver, commit `37820e2d`).

**Modification:** `DEFAULT_SPLIT_THRESHOLD = 100_000` / `DEFAULT_TARGET_MAX = 500_000` (L~120 region) are stale — RESEARCH notes they aren't even honored (translator module constants win). Re-tune for the 5K hard cap, OR make the driver pass `--split-threshold 4500` through to each translator. The driver also orchestrates `regen_phase09.py` + the now-obsolete `split_*.py` post-processors — under D-10a the `split_oversized_*.py` / `rebatch_mgga.py` / `split_mgga_7_kcis.py` helpers are obsolete (they assume the numbered-subcrate layout). Planner decides keep-as-scaffolding vs delete (CONTEXT Claude's Discretion).

---

### `CLAUDE.md` — modified (D-03a precision policy)

**Analog:** `CLAUDE.md` § "Constraints" (self-analog).

**Edit:** "Precision: f64 only; energy relative error <= 10^-12 vs libxc oracle" → "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate" (CONTEXT D-03a). Also soften "Maple2c formula translations must preserve floating-point operation order for bit-level equivalence" — D-05 relaxes the gate to 1e-12 relative (CSE introduces named temporaries that legitimately reorder accumulation). Land this in an executor commit within Phase 11.

---

## Shared Patterns

### `#[cube]` vs `#[cube(launch_unchecked)]` routing decision
**Source:** `tools/translate_lda_v2.py` L516-518 + `tools/kernel_routing.py`
**Apply to:** Every emitted kernel `.rs` file (`partNN.rs`, `<output>/mod.rs`, `<output>.rs`)
```python
is_split_helper = fn_suffix.startswith('_part')
is_unrouted = func_name not in cached_routed_funcnames(KERNEL_FAMILY)
cube_attr = '#[cube]' if (is_split_helper or is_unrouted) else '#[cube(launch_unchecked)]'
```
- All CSE chunks and partN helpers: plain `#[cube]` (D-02; manual §4, §19).
- The per-`(level,spin)` entry wrapper for a **routed** functional: `#[cube(launch_unchecked)]`.
- The entry wrapper for an **unrouted/deferred** functional (the 7 per D-11 + `mgga_x_br89_explicit`): plain `#[cube]` even at entry level (the `260512-q01` change).
- `tools/kernel_routing.py` (~220 lines) is the single source of truth for "is routed?" — translators import it. KEEP unchanged.
- **Audit gate:** `tools/audit_cube_launch.sh` baseline = 23 `#[cube(launch_unchecked)]` (Wave 0 post-spike). Phase 11 MUST keep this at 23 throughout — adding a new `launch_unchecked` requires an explicit deviation + baseline bump (Wave 0 deviation D2).

### Math-primitive import block
**Source:** `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/part1.rs` L8-12
**Apply to:** Every emitted kernel `.rs` file
```rust
use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};
```
- `use cubecl::prelude::*;` always first.
- Then exactly the `libxc_kernel_math::` primitives referenced — the splitter's `detect_imports` pass computes this set per file. Underscore crate path (`libxc_kernel_math`), hyphen package name (`libxc-kernel-math`).
- For CSE chunks: the import set is per-*chunk* (each chunk imports only what its stage uses) — smaller than the per-output set.

### `#![allow(...)]` lint block
**Source:** q02 `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/mod.rs` L12 (file-level), `part1.rs` L6 (file-level), L14 (fn-level)
**Apply to:** Every emitted kernel `.rs` file
```rust
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
// ... and per-fn:
#[allow(unused_variables, non_snake_case)]
```
Adopt the **q02 wider allow-list** (it covers `needless_return`, which the narrower numbered-subcrate `lib.rs` L1-3 list omits).

### Bounds-guarded kernel prologue
**Source:** `tools/translate_lda_v2.py` L554-555; `crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol/part1.rs` L28-37
**Apply to:** Every `#[cube]` kernel fn (entry wrappers and partN helpers that touch `Array`s)
```rust
let ip = ABSOLUTE_POS;
if ip < {first_output_buffer}.len() {
    let rho0 = rho[ip * 2];      // pol: stride-2 loads
    let rho1 = rho[ip * 2 + 1];
    // ...
}
```
D-02 tuple-return CSE chunks are the exception — they take scalar `F` args, not `Array`s, so no `ABSOLUTE_POS` guard (the *wrapper* does the array load + guard, then passes scalars into chunks). See the spike file for the chunk shape.

### Idempotency contract
**Source:** `tools/test_idempotency.sh` (Wave 0) + CONTEXT D-LOCK-D
**Apply to:** All splitter emitters + the `Cargo.toml` / `default-members` generator
Running the pipeline twice must produce zero diff. Concretely: sorted deterministic ordering of `default-members`, deterministic part numbering, deterministic import ordering. `test_idempotency.sh` is FAIL-by-design until the splitter v2 lands.

---

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `tools/translate_v2/cse.py` (the CSE min-cut partitioner itself) | utility | transform | No existing tool does dep-DAG min-cut chunk partitioning. `translate_lda_v2.py` `build_dependency_graph` (L370) + `transitive_deps` (L389) provide the **dep-graph primitives** — extend them — but the "find min-cut breakpoints, partition into ≤4500-line chunks, plan tuple in/out signatures" logic (RESEARCH "CSE Detection Heuristic") is genuinely new. Planner should use RESEARCH Option C (operate on the C `compute_lines` list) as the design, with `build_dependency_graph` as the reused foundation. |
| D-02 tuple-return chunk *body* emission | utility | transform | The existing `_generate_chunk_helper` (`translate_lda_v2.py` ~L571-702) emits the WRONG ABI (shared `&mut Array<f64>` scratch). The *only* proven analog for the correct shape is the test spike `crates/kernels/math/tests/spike_tuple_return_cube.rs` — not production emitter code. The emitter for tuple-return `<F: Float>` chunks must be written fresh, using the spike as the target shape. |

---

## Metadata

**Analog search scope:** `crates/kernels/` (q02 `mgga_c_b94` nested layout, numbered-subcrate `Cargo.toml`/`lib.rs`, family façades), `tools/` (translators, dispatch generators, audit scripts), `src/eval/{gga,mgga}_dispatch/`, `Cargo.toml` root, `crates/kernels/math/tests/`.
**Files scanned:** ~22 (4 q02 nested-layout files, 3 translators, `maple_to_kernels.py`, `generate_gga_dispatch.py`, 2 dispatch files, 3 family façade `lib.rs` + 2 `Cargo.toml`, root `Cargo.toml`, `audit_subcrate_collapse.sh`, `audit_kernel_size.py`, directory listings).
**Golden-spec analogs (commit `504d8560`, q02):** `crates/kernels/mgga-2/src/mgga_c_b94/{mod.rs, kxc_pol/mod.rs, kxc_pol/part1.rs}` — the nested-by-output layout D-04 standardizes the splitter on.
**Pattern extraction date:** 2026-05-14
