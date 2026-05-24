# Phase 12: MGGA f64 Parity - Pattern Map

**Mapped:** 2026-05-25
**Files analyzed:** 11 (5 modified existing + 6 new canary tests; +1 modified Cargo.toml)
**Analogs found:** 11 / 11 (every file has a concrete in-repo analog)

> This is a **numerical-correctness debug phase**, not a feature phase. Most "modified"
> files are EXISTING files whose analog is their own current implementation + the libxc
> C source they must match. The genuinely NEW artifacts are the **6 permanent canary tests**
> in `verify-canary/tests/`, whose proven analog is `g3_mgga_c_b94_parity.rs`.

---

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `verify-canary/tests/mgga_x_th_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact (simpler: 0 ext_params) |
| `verify-canary/tests/mgga_x_2d_js17_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact |
| `verify-canary/tests/mgga_c_cs_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact |
| `verify-canary/tests/mgga_x_pkzb_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact |
| `verify-canary/tests/mgga_x_pbe_gx_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact |
| `verify-canary/tests/mgga_x_tm_parity.rs` *(NEW)* | test harness (canary) | transform → oracle-compare | `verify-canary/tests/g3_mgga_c_b94_parity.rs` | exact |
| `verify-canary/Cargo.toml` *(MODIFIED)* | config | dep-declaration | its own current single-kernel-dep block | exact (self) |
| `src/eval/mgga_dispatch/prepare.rs` *(MODIFIED — core fix D-01)* | dispatch driver (input regularization) | transform | self `tau_von_weizsacker` ⟷ `work_mgga_inc.c:57-92` | role-match (C reference) |
| `src/eval/mgga_dispatch/mod.rs` *(MODIFIED — call site D-01/D-08)* | dispatch driver (chokepoint) | request-response | self `:270-283` + `:260-268` | exact (self) |
| `verify-canary/tests/g1_tau_clamp_dispatch_parity.rs` *(MODIFIED — D-02)* | test harness (clamp canary) | transform → oracle-compare | self (host clamp `:93-103`) | exact (self) |
| `verify-canary/tests/g3_mgga_c_b94_parity.rs` *(MODIFIED — D-02)* | test harness (canary) | transform → oracle-compare | self (host clamp `:96-100`) | exact (self) |
| `crates/kernels/mgga/mgga_x_th/src/exc_unpol.rs` *(REGEN ONLY, D-03/D-04)* | generated kernel | transform | `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c::func_exc_unpol` | exact (C translation oracle) |
| `tools/translate_mgga.py` / `tools/translate_v2/*` *(MODIFIED IF mgga_x_th is a translation bug)* | translator | transform | self + 11-PATTERN.md Rule 3/9/10 | n/a (only if structural bug) |

> AP-3: the generated kernel files (`crates/kernels/mgga/<func>/`) are NEVER hand-edited.
> A per-functional kernel fix routes through the translator (`tools/translate_*`) + regen.

---

## Shared Patterns (apply across all Phase-12 work)

### A. libxc work_mgga input regularization — the D-01 reference sequence

**Source (the translation oracle):** `libxc-master/src/work_mgga_inc.c:57-92`
**Apply to:** `src/eval/mgga_dispatch/prepare.rs` (the single edit that propagates to all 6 targets)

The exact pre-functional sequence libxc applies per grid point, UNPOLARIZED branch
(`work_mgga_inc.c`, with `XC_ENFORCE_FERMI_HOLE_CURVATURE` ACTIVE — see note below):

```c
/* :54 */  if(dens < p->dens_threshold) continue;            /* low-density screen */
/* :58 */  my_rho[0]   = m_max(p->dens_threshold, VAR(rho, ip, 0));
/* :59 */  my_sigma[0] = m_max(p->sigma_threshold * p->sigma_threshold, VAR(sigma, ip, 0));  /* σ FLOOR */
/* :63 */  if(p->info->flags & XC_FLAGS_NEEDS_TAU){
/* :64 */    my_tau[0] = m_max(p->tau_threshold, VAR(tau, ip, 0));                            /* τ FLOOR */
/* :65 */  #ifdef XC_ENFORCE_FERMI_HOLE_CURVATURE
/* :67 */    my_sigma[0] = m_min(my_sigma[0], 8.0*my_rho[0]*my_tau[0]);                       /* σ-DOWN clamp */
/* :68 */  #endif
/* :69 */  }
/* :94 */  FUNC(...)(p, ip, my_rho, my_sigma, &VAR(lapl, ip, 0), my_tau, out);
```

**Order matters (FP-order lock, CLAUDE.md §Constraints):** ρ-floor → σ-floor → τ-floor →
σ-DOWN clamp `σ ← min(σ, 8ρτ)`. The σ-down clamp consumes the ALREADY-floored `my_rho`
and `my_tau`. Note it is `8·my_rho[0]·my_tau[0]`, NOT `8·raw_rho·raw_tau`.

**Threshold constants** (`libxc-master/src/functionals.c:303-311`, applied at `xc_func_init`):
```c
func->dens_threshold  = func->info->dens_threshold;        /* 1e-15 for all 6 (info struct) */
func->sigma_threshold = pow(func->info->dens_threshold, 4.0/3.0);  /* σ floor uses THIS squared */
func->tau_threshold   = 1e-20;                             /* hardcoded constant */
```
So the σ-floor magnitude is `pow(dens_threshold, 4/3)²` = `pow(1e-15, 8/3)` (effectively 0
at these inputs), `τ`-floor = `1e-20`. The σ-down clamp at `:67` is the term that actually
moves on the test grid.

**`XC_ENFORCE_FERMI_HOLE_CURVATURE` is ON by default** — `libxc-master/CMakeLists.txt:91-93`:
```cmake
if(NOT DISABLE_FHC)
  add_definitions (-DXC_ENFORCE_FERMI_HOLE_CURVATURE)
endif(NOT DISABLE_FHC)
```
The vendored oracle build does NOT set `DISABLE_FHC`, so `:67` IS compiled in. The Rust
regularization MUST replicate it.

**What is being REPLACED (current Rust, the wrong-variable clamp):**
`src/eval/mgga_dispatch/prepare.rs:32-47` — `tau_von_weizsacker` raises τ instead of lowering σ:
```rust
pub(crate) fn tau_von_weizsacker(rho: &[f64], sigma: &[f64], tau: &[f64], dens_threshold: f64) -> Vec<f64> {
    (0..tau.len())
        .map(|i| {
            if rho[i] < dens_threshold { tau[i] }
            else { tau[i].max(sigma[i] / (8.0 * rho[i])) }   // τ-UP — WRONG variable per D-01
        })
        .collect()
}
```
D-01 replaces this with a function that returns the regularized (σ, τ) PAIR (it now must
mutate σ as well as τ), mirroring the C sequence above. Both enforce the same boundary
`σ ≤ 8ρτ ⟺ τ ≥ σ/(8ρ)` but feed DIFFERENT (ρ,σ,τ) triples to functionals that read σ and τ
independently — the prime suspect for the 5 small-error functionals.

### B. The direct-kernel-launch canary harness (the cubecl 0.10 launch ABI)

**Source:** `verify-canary/tests/g3_mgga_c_b94_parity.rs:51-148`
**Apply to:** all 6 new canaries.

Key ABI facts the planner/executor must respect (from the proven g3 canary):
- Thin `#[cube(launch_unchecked)]` wrapper that delegates verbatim to the kernel (lines 55-72).
- `ArrayArg::from_raw_parts(handle, len)` MOVES the handle — NO turbofish, NO vectorization arg.
- Scalar params pass as BARE values (not `ScalarArg`).
- Clone the output handle before launch (`let zk_read = zk_h.clone();`) and read via
  `client.read_one(zk_read)` → `bytemuck::cast_slice`.
- The kernel is concrete-f64 → no `::<f64>` turbofish at the call site (11-PATTERN.md Rule 9/10).

### C. Oracle-compare scaffolding (rel_err floor + libxc-sys call)

**Source:** `g3_mgga_c_b94_parity.rs:74-83` (rel_err) and `:151-172` (oracle call); same logic
in `verify/tests/mgga_oracle.rs:241-250`.
**Apply to:** all 6 new canaries.
```rust
fn rel_err_with_floor(rust_val: f64, c_val: f64) -> f64 {
    if rust_val.abs() < REL_FLOOR && c_val.abs() < REL_FLOOR { return 0.0; }
    if c_val.abs() < 1e-300 { rust_val.abs() } else { ((rust_val - c_val) / c_val).abs() }
}
```
The oracle call: `xc_func_alloc` → `xc_func_init(func, ID, XC_UNPOLARIZED)` → `xc_mgga_exc(func, np, rho, sigma, lapl, tau, zk)` → `xc_func_end` → `xc_func_free`. Pass RAW (unclamped) inputs
to the oracle — libxc applies its own regularization internally; the Rust side passes the
pre-regularized inputs. They must agree at 1e-12.

### D. Path-scoped commits

**Source:** memory `feedback_path_scoped_commits`.
**Apply to:** every commit in this phase. Sessions open with thousands of pre-staged files;
use `git commit --only -- <path>` so unrelated index entries are not swept in.

---

## Pattern Assignments

### 6× NEW `verify-canary/tests/<func>_parity.rs` (test harness, transform→oracle-compare)

**Analog:** `verify-canary/tests/g3_mgga_c_b94_parity.rs` — copy-adapt 6×.

**KEY SIMPLIFICATION vs the b94 template:** all 6 targets carry ZERO ext_params
(`{0, NULL, NULL, NULL, NULL}` in their func-level `.c` — verified for all 6). So:
- DROP the `PARAM_GAMMA/PARAM_CSS/PARAM_CAB` constants (b94 lines 30-32) entirely.
- The wrapper + kernel call take only `(rho, sigma, lapl, tau, zk, dens_threshold, zeta_threshold)`
  — confirmed identical signature across all 6 (see "Per-target kernel facts" below).

This makes each canary a SIMPLER version of g3, not a more complex one.

**Per-target kernel facts (all verified):**

| Functional | libxc id | kernel crate | exc_unpol fn | ext_params |
|------------|----------|--------------|--------------|------------|
| `mgga_x_th`      | 225 | `libxc-kernel-mgga_x_th` | `exc_unpol::mgga_x_th_exc_unpol` | 0 |
| `mgga_x_2d_js17` | 609 | `libxc-kernel-mgga_x_2d_js17` | `exc_unpol::mgga_x_2d_js17_exc_unpol` | 0 |
| `mgga_c_cs`      | 72  | `libxc-kernel-mgga_c_cs` | `exc_unpol::mgga_c_cs_exc_unpol` | 0 |
| `mgga_x_pkzb`    | 213 | `libxc-kernel-mgga_x_pkzb` | `exc_unpol::mgga_x_pkzb_exc_unpol` | 0 |
| `mgga_x_pbe_gx`  | 576 | `libxc-kernel-mgga_x_pbe_gx` | `exc_unpol::mgga_x_pbe_gx_exc_unpol` | 0 |
| `mgga_x_tm`      | 540 | `libxc-kernel-mgga_x_tm` | `exc_unpol::mgga_x_tm_exc_unpol` | 0 |

The uniform kernel signature for all 6 (verified — `mgga_x_th/src/exc_unpol.rs:15-23`):
```rust
pub fn mgga_x_th_exc_unpol(
    rho: &Array<f64>, sigma: &Array<f64>, lapl: &Array<f64>, tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64, zeta_threshold: f64,
) { /* ip = ABSOLUTE_POS; if ip < zk.len() { ... } */ }
```

**Wrapper to copy-adapt** (g3 `:55-72`, drop the 3 param args):
```rust
#[cube(launch_unchecked)]
fn th_exc_unpol_kernel(
    rho: &Array<f64>, sigma: &Array<f64>, lapl: &Array<f64>, tau: &Array<f64>,
    zk: &mut Array<f64>, dens_threshold: f64, zeta_threshold: f64,
) {
    mgga_x_th_exc_unpol(rho, sigma, lapl, tau, zk, dens_threshold, zeta_threshold);
}
```

**Thresholds** (g3 `:34-36`, `Thresholds::default()`): `DENS_THRESHOLD = 1e-15`,
`ZETA_THRESHOLD = 1e-10`. NOTE for D-01 alignment: libxc's `tau_threshold` is `1e-20` and the
σ-floor uses `pow(dens_threshold, 4/3)²` — the canary's clamp helper (next section) must use
the SAME regularization the production `prepare.rs` ends up with.

**D-01 σ-down clamp in the canary** (REPLACES the b94 `tau_von_weizsacker` host helper at
g3 `:96-100`): the canary must apply the NEW σ-floor → τ-floor → σ-down sequence to inputs
before launch (and the launched σ must be the clamped σ, not raw σ — the b94 canary only
clamped τ). Mirror exactly what `prepare.rs` does post-D-01.

**Test-point grid (D-10):** baseline = `mgga_oracle` 4-point unpol grid
(`verify/tests/mgga_oracle.rs:215-218`):
```rust
const RHO:   &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA: &[f64] = &[0.01, 0.1, 0.5, 2.0];
const LAPL:  &[f64] = &[0.001, 0.01, 0.05, 0.2];
const TAU:   &[f64] = &[0.01, 0.05, 0.2, 1.0];
```
ADD at least one sub-Fermi-hole point where `σ > 8ρτ` so the D-01 σ-down clamp ACTIVATES
(otherwise the fix is untested). The g1 canary (`g1_tau_clamp_dispatch_parity.rs:39-45`)
shows how to construct + ASSERT active clamp points; reuse that assertion idiom adapted to
σ-down (assert `σ_clamped[i] < σ_raw[i]` on the active points, no-op on the others).

**Test body + summary logging** (g3 `:174-213`): per-point `rel_err`, `max_e`, `failures`
vector, `STRICT_TOL = 1e-12`, `REL_FLOOR = 1e-30`, eprintln tuple lines, final `assert!`.

---

### `verify-canary/Cargo.toml` (config — MODIFIED)

**Analog:** its own current single-kernel-dep block (`verify-canary/Cargo.toml:21-25`).
```toml
[dependencies]
cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }
bytemuck = "1.25"
libxc-kernel-mgga_c_b94 = { path = "../crates/kernels/mgga/mgga_c_b94" }
libxc-sys = { path = "../libxc-sys" }
```
**Add 6 path deps** (one per target — memory-safe under `jobs=1`; each canary builds ONE kernel):
```toml
libxc-kernel-mgga_x_th       = { path = "../crates/kernels/mgga/mgga_x_th" }
libxc-kernel-mgga_x_2d_js17  = { path = "../crates/kernels/mgga/mgga_x_2d_js17" }
libxc-kernel-mgga_c_cs       = { path = "../crates/kernels/mgga/mgga_c_cs" }
libxc-kernel-mgga_x_pkzb     = { path = "../crates/kernels/mgga/mgga_x_pkzb" }
libxc-kernel-mgga_x_pbe_gx   = { path = "../crates/kernels/mgga/mgga_x_pbe_gx" }
libxc-kernel-mgga_x_tm       = { path = "../crates/kernels/mgga/mgga_x_tm" }
```
> `cubecl` is `0.10.0` here (NOT 0.9.0 as CLAUDE.md's stack table states — the kernel crates
> and canary already migrated; see memory `project_umbrella_cubecl010_launch_abi_drift`). Match
> the kernel crate's `Cargo.toml` (`crates/kernels/mgga/mgga_x_th/Cargo.toml`: `cubecl 0.10.0`).
> Building all 6 canaries at once still adds 6 kernel-compiles; per `feedback_ram_constraints`
> run the canaries one `-p`/`--test` at a time during the fix loop.

---

### `src/eval/mgga_dispatch/prepare.rs` (dispatch driver — MODIFIED, core fix D-01)

**Analog:** its own current `tau_von_weizsacker` (`:32-47`) for the function shape +
`work_mgga_inc.c:57-92` for the new arithmetic (see Shared Pattern A).

**Change:** the function currently returns `Vec<f64>` (clamped τ only). Post-D-01 it must
return BOTH regularized σ and τ (e.g. `(Vec<f64>, Vec<f64>)` or mutate in place), because the
σ-down clamp changes σ that is then fed to the kernel. The module doc (`:1-23`) and the
function doc (`:25-31`) both currently describe the τ-up semantics and MUST be rewritten to
the σ-floor → τ-floor → σ-down sequence. Preserve FP operation order (CLAUDE.md).

**Guard to preserve:** the `ρ < dens_threshold` continue/skip guard (current `:40`,
mirrors `work_mgga_inc.c:54`).

---

### `src/eval/mgga_dispatch/mod.rs` (dispatch chokepoint — MODIFIED call site)

**Analog:** self, `:270-283` (the τ-clamp call site) + `:250-268` (order/pol rejection).

**Current call site (`:278-283`) — what D-01 rewires:**
```rust
// G-1: regularize τ to its von Weizsäcker lower bound ... See prepare.rs.
let tau_clamped =
    prepare::tau_von_weizsacker(input.rho(), input.sigma(), input.tau(), thresholds.density);
let tau_handle = create_input_buffer(&client, &tau_clamped);
let tau_len = tau_clamped.len();
```
Post-D-01 this must also build a `sigma_regularized` buffer (currently `:274-275` builds the
sigma handle from RAW `input.sigma()`). The regularized σ must flow into the kernel launch,
not the raw σ. One edit here covers all routed MGGA functionals.

**Scope boundary to PRESERVE (D-08):** the polarized rejection at `:260-268`:
```rust
if spin != Spin::Unpolarized {
    return Err(LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "MGGA polarized dispatch deferred pending Phase 4 follow-up ...",
    });
}
```
Phase 12 is exc-UNPOLARIZED only — do NOT touch pol. The Fxc+ rejection at `:252-258` also stays.
(The `mgga_zero_scalar_unpol_dispatch!` macro at `:87-102` carries its own pol guard too.)

---

### `verify-canary/tests/g1_tau_clamp_dispatch_parity.rs` (clamp canary — MODIFIED, D-02)

**Analog:** self. Its host clamp helper (`:93-103`) is BYTE-IDENTICAL to the production
`tau_von_weizsacker` by design (the test docstring at `:86-92` asserts this invariant). When
D-01 changes production to σ-down, this canary's host clamp MUST be updated in lockstep to the
σ-down sequence, AND it must still PASS at 1e-12. The sub-vW grid (`:42-45`) and the
"clamp was active" assertions (`:179-189`) must be re-expressed for σ-down (assert σ lowered,
not τ raised). This is the implicit pre-flight that the regularization change didn't break the
proven canary.

---

### `verify-canary/tests/g3_mgga_c_b94_parity.rs` (canary — MODIFIED, D-02)

**Analog:** self. Its host helper `tau_von_weizsacker` (`:96-100`) clamps τ-up only. Update to
the σ-down sequence (matching production) and confirm it still passes at the b94 oracle (~5e-13
historically). b94 carries 3 ext_params (`:30-32`) — keep those; only the regularization helper
changes.

---

### `crates/kernels/mgga/mgga_x_th/src/exc_unpol.rs` (generated kernel — REGEN ONLY, D-03/D-09)

**Analog (the translation oracle):** `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c` →
`func_exc_unpol` (starts `:19`). The Rust `mgga_x_th_exc_unpol` body is a near-verbatim
translation of this C function; a line-by-line diff is the way to find the 20% structural bug.

**Why mgga_x_th is special (D-09):** 20% error is structural, NOT the regularization issue.
Suspect: constant mapping, piecewise logic, or exponentiation. Compare the Rust temporaries
(`t3, t4, t5, ...` in `exc_unpol.rs:24+`) against the C temporaries 1:1. Example alignment
already matches at the top:
```
C  (mgga_x_th.c:25-27):  t4 = M_CBRTPI; t5 = t4 * t4; ...
Rust (exc_unpol.rs:27-28): t4 = M_CBRTPI; t5 = t4 * t4; ...
```
The named constants are defined in `crates/kernels/math/src/constants.rs`
(`M_CBRTPI = 1.4645918875615232` `:10`, `M_CBRT2 :16`, `M_CBRT3 :19`, `M_PI :4`) and are
self-tested to 1e-15 — so a constant VALUE bug is unlikely; a constant-WRAP / piecewise /
operation-order bug is more likely (cf. 11-PATTERN.md Rule 3).

**AP-3 — NO hand-edit of `crates/kernels/mgga/mgga_x_th/`.** If the bug is in the translation,
fix `tools/translate_mgga.py` / `tools/translate_v2/{cse,per_functional,emit}.py` and regen:
- Selective loop (fast): single-functional regen while iterating.
- **Closing regen MUST be full-tree** (`python3 tools/maple_to_kernels.py translate --all-families`)
  and byte-idempotent (D-LOCK-D) to catch perturbation of other families (D-04).

---

### `tools/translate_mgga.py` / `tools/translate_v2/*` (translator — MODIFIED only if mgga_x_th is a translation bug)

**Analog:** the existing emit pipeline + 11-PATTERN.md conventions (Rule 3 named-const wrap,
Rule 9/10 turbofish). Only touched if D-09's investigation proves a translator-level fault.
Present scripts confirmed: `tools/translate_mgga.py`, `tools/maple_to_kernels.py`,
`tools/translate_v2/{cse.py, emit.py, per_functional.py, helpers_allowlist.py}`.

---

## Authoritative Gate (D-05/D-06) — not a file to create, the final check

**`verify/tests/mgga_oracle.rs`** (`#![cfg(feature = "oracle-mgga")]`) is the AUTHORITATIVE gate.
The 6 canaries are the fast inner loop; the family oracle is the real gate (avoids the
Phase-11.1 b94 "hollow gate" trap — memory `project_g3_b94_hollow_gate`).
- Tolerances: `TOL_EXC = 1e-12` (`:205`), `TOL_VXC = 1e-10` (`:206`).
- Build path: `cargo test -p libxc_rs --features oracle-mgga` resolves only MGGA kernels + math
  (proven memory-safe in 11-12; feature lists the 6 at workspace `Cargo.toml:525,591,619,620,635,636`).
- D-06 regression confirm: also re-run `oracle-lda` + `oracle-gga` (they don't route through the
  MGGA driver, so should be untouched — confirm, don't assume).

---

## No Analog Found

None. Every Phase-12 file maps to a concrete in-repo analog (the 6 canaries → g3 template;
the dispatch edits → self + work_mgga_inc.c; the kernel regen → maple2c C source). The
`mgga_x_2d_js17` D-13 escape hatch (de-route if the residual is inherently 2D-dimensional) is a
DECISION branch, not a missing analog — its canary still copies the g3 template.

---

## Metadata

**Analog search scope:** `verify-canary/tests/`, `src/eval/mgga_dispatch/{,funcs/}`,
`crates/kernels/mgga/<6 targets>/`, `crates/kernels/math/src/`, `libxc-master/src/`
(`work_mgga_inc.c`, `functionals.c`, `xc_funcs.h`, `CMakeLists.txt`, `maple2c/mgga_exc/`,
per-functional `.c`), `tools/`, workspace + `verify-canary` `Cargo.toml`.
**Files scanned:** ~25
**Pattern extraction date:** 2026-05-25
