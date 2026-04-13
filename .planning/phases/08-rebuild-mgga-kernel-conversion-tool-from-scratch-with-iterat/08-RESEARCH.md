# Phase 8: Rebuild MGGA Kernel Conversion Tool — Research

**Researched:** 2026-04-13
**Domain:** Python code generation tool / Rust CubeCL kernel translation / MGGA functional math
**Confidence:** HIGH

---

## Summary

Phase 8 rebuilds the MGGA maple2c-to-Rust conversion tool from scratch using an iterative
verify-as-you-go methodology. The existing GGA translator (`tools/translate_gga.py`) provides
the correct foundation pattern but cannot be used verbatim for MGGA: MGGA C functions accept
two additional input arrays (`lapl`, `tau`), produce ~70 output fields (vs ~15 for GGA), have
more complex conditional output guards involving `NEEDS_LAPLACIAN` and `NEEDS_TAU` flags, and
span C files 7K–100K lines (vs 1K–37K for GGA), with 29 out of 90 MGGA files exceeding 30K
lines. The OOM problem that forced GGA into 3 sub-crates will be more severe for MGGA, and the
correct fix is to generate kernels in a form that keeps individual `#[cube(launch_unchecked)]`
functions below ~5K lines of generated Rust.

The iterative approach dictated by the phase objective is correct: translate one representative
functional at a time, compile it, run the oracle comparison, fix the tool, repeat. The GGA
translator demonstrated that pattern-based regex translation produces correct output for ~80% of
functionals in a single pass; the remaining ~20% require targeted fixes (special math functions,
unusual parameter forms, unusual guard patterns). MGGA will have similar ratio but with higher
absolute complexity.

The verification infrastructure (`verify/` crate) already provides `oracle_mgga_all()` and
`MggaOracleOutput` with all 70 output fields correctly sized for both spin modes.
[VERIFIED: verify/src/lib.rs, lines 326-491]

**Primary recommendation:** Build `tools/translate_mgga.py` modeled on `translate_gga.py`,
add MGGA-specific input arrays (lapl, tau), add MGGA-specific output field tables, handle
all 4 output guard patterns, and test iteratively starting with `mgga_xc_lp90.c` (simplest:
7,108 lines), verifying each translated functional compiles and matches the oracle before
proceeding to the next pattern category.

---

## Standard Stack

### Core (all already in project)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Python 3 | system | Translation tool language | All existing translators (translate_lda.py, translate_gga.py) are Python 3 [VERIFIED: codebase] |
| cubecl | 0.9.0 | `#[cube(launch_unchecked)]` kernel authoring | All kernel crates depend on this [VERIFIED: Cargo.toml] |
| libxc-kernel-math | 0.1.0 (path dep) | Math functions used inside MGGA kernels | Same as LDA/GGA kernels [VERIFIED: crates/kernel-math/] |
| libxc (verify/) | 7.0.0 | Oracle for equivalence testing | `oracle_mgga_all()` already implemented [VERIFIED: verify/src/lib.rs] |
| re, os, pathlib (stdlib) | stdlib | Regex translation engine | Used in translate_gga.py [VERIFIED: tools/translate_gga.py] |

### New: MGGA Sub-Crate Structure

No new external Cargo dependencies needed. The MGGA kernels will be placed in sub-crates of
`crates/kernel-mgga/` using the same sub-crate split pattern as GGA.

**Expected MGGA sub-crate layout:**
```
crates/
  kernel-mgga/          # facade crate (already exists, currently stubs)
  kernel-mgga-1/        # ~15-20 functionals
  kernel-mgga-2/        # ~15-20 functionals
  kernel-mgga-3/        # ~15-20 functionals
  kernel-mgga-4/        # remainder (if needed)
```

The exact number of sub-crates depends on how large the generated Rust files are per functional.
GGA experience: ~35 modules per sub-crate. MGGA files are 2-5x larger, so expect ~10-15 modules
per sub-crate to stay within the ~16GB compilation budget.

---

## MGGA vs GGA: Key Differences Requiring New Tool

### Difference 1: Additional Input Arrays

**GGA C signature:**
```c
func_exc_unpol(const xc_func_type *p, size_t ip,
               const double *rho, const double *sigma,
               xc_gga_out_params *out)
```

**MGGA C signature:**
```c
func_exc_unpol(const xc_func_type *p, size_t ip,
               const double *rho, const double *sigma,
               const double *lapl, const double *tau,
               xc_mgga_out_params *out)
```

In the Rust kernel, this requires two additional `&Array<f64>` parameters: `lapl` and `tau`.

**Unpol input array indexing:**
- `rho[0]` → `rho[ip]`
- `sigma[0]` → `sigma[ip]`
- `lapl[0]` → `lapl[ip]`
- `tau[0]` → `tau[ip]`

**Pol input array indexing:**
- `rho[0]` → `rho[ip * 2]`, `rho[1]` → `rho[ip * 2 + 1]`
- `sigma[0]` → `sigma[ip * 3]`, `sigma[1]` → `sigma[ip * 3 + 1]`, `sigma[2]` → `sigma[ip * 3 + 2]`
- `lapl[0]` → `lapl[ip * 2]`, `lapl[1]` → `lapl[ip * 2 + 1]`
- `tau[0]` → `tau[ip * 2]`, `tau[1]` → `tau[ip * 2 + 1]`

[VERIFIED: libxc-master/src/work_mgga_inc.c — confirms FUNC(ORDER,SPIN)(p, ip, my_rho, my_sigma, &VAR(lapl,ip,0), my_tau, out) call pattern, and the dimension tables in verify/src/lib.rs confirm d_vlapl=2, d_vtau=2 for polarized spin]

### Difference 2: MGGA Output Fields (70 total vs 15 for GGA)

GGA has 15 output fields. MGGA has 70, grouped by derivative order:

| Order | MGGA Fields | Notes |
|-------|-------------|-------|
| 0 (exc) | zk | Same as GGA |
| 1 (vxc) | vrho, vsigma, **vlapl**, **vtau** | +2 vs GGA |
| 2 (fxc) | v2rho2, v2rhosigma, **v2rholapl**, **v2rhotau**, v2sigma2, **v2sigmalapl**, **v2sigmatau**, **v2lapl2**, **v2lapltau**, **v2tau2** | +6 vs GGA |
| 3 (kxc) | 20 fields (10 GGA + 10 MGGA-specific) | |
| 4 (lxc) | 35 fields (15 GGA + 20 MGGA-specific) | |

Fields in **bold** are MGGA-only (involve lapl and/or tau derivatives).

[VERIFIED: verify/src/lib.rs MggaOracleOutput struct lines 329-405]

### Difference 3: Four Output Guard Patterns

GGA has one output guard pattern: `if(out->zk != NULL)`. MGGA has four:

```c
// Pattern A: Standard (same as GGA)
if(out->zk != NULL && (flags & XC_FLAGS_HAVE_EXC))
  out->zk[ip*dim.zk + N] += var;

// Pattern B: Laplacian-conditional (vlapl, v2rholapl, v2sigmalapl, v2lapl2, etc.)
if(out->vrho != NULL && (flags & XC_FLAGS_NEEDS_LAPLACIAN) && (flags & XC_FLAGS_HAVE_VXC))
  out->vlapl[ip*dim.vlapl + N] += var;

// Pattern C: Tau-conditional (vtau, v2rhotau, v2sigmatau, v2tau2, etc.)
if(out->vrho != NULL && (flags & XC_FLAGS_NEEDS_TAU) && (flags & XC_FLAGS_HAVE_VXC))
  out->vtau[ip*dim.vtau + N] += var;

// Pattern D: Both-conditional (v2lapltau, mixed lapl+tau cross-derivatives)
if(out->v2rho2 != NULL && (flags & XC_FLAGS_NEEDS_LAPLACIAN) && (flags & XC_FLAGS_NEEDS_TAU) && (flags & XC_FLAGS_HAVE_FXC))
  out->v2lapltau[ip*dim.v2lapltau + N] += var;
```

[VERIFIED: libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c — all 4 patterns confirmed]

In the Rust kernel translation, these guards become unconditional writes. Each per-functional
`#[cube(launch_unchecked)]` function is compiled per (derivative-order, spin-mode), so the
flag semantics are encoded in which kernel gets called at dispatch time, not inside the kernel.

### Difference 4: Two Source Directories (mgga_exc and mgga_vxc)

Like GGA which has gga_exc and gga_vxc, MGGA has:
- `libxc-master/src/maple2c/mgga_exc/` — 90 functionals (have EXC + VXC + higher)
- `libxc-master/src/maple2c/mgga_vxc/` — 2 functionals (VXC-only: mgga_x_2d_prp10.c, mgga_x_tb09.c)

[VERIFIED: ls output]

### Difference 5: OOM Risk Is Higher for MGGA

GGA experience: `#[cube(launch_unchecked)]` functions over ~5K generated Rust lines cause OOM
in rustc even in isolation. GGA lxc_pol.rs files ranged from 5K–37K lines, causing 25/131
functionals to be deferred.

MGGA total C source: 2,437,745 lines across 90 functionals (avg ~27K lines per file). 29 out of
90 functionals exceed 30K lines in C. The generated Rust per functional will typically be
proportional to the C source.

**Mitigation strategy** (two options, to be decided during iterative testing):

Option A: Split each functional's largest derivative-order files (lxc_pol.rs, kxc_pol.rs)
into sub-functions (e.g., `lxc_pol_part1.rs`, `lxc_pol_part2.rs`), each with a separate
`#[cube(launch_unchecked)]` function that computes half the temporaries.

Option B: Skip lxc (4th-order) and kxc (3rd-order) polarized kernels for large functionals.
Only generate exc/vxc/fxc unpol and exc/vxc pol — the most practically needed derivative orders.

The planner should plan for Option A by default and fall back to Option B if needed.
[ASSUMED: Option A is implementable without CubeCL API changes — this needs verification during execution]

---

## Architecture Patterns

### Recommended Project Structure After Phase 8

```
tools/
  translate_mgga.py       # NEW: MGGA-specific translation tool
  batch_translate_mgga.py # NEW: Batch runner for all 90 MGGA functionals
  translate_gga.py        # Existing (reference implementation)
  translate_lda.py        # Existing (reference implementation)

crates/
  kernel-mgga/            # Facade (already exists, replace stubs)
    Cargo.toml            # Re-exports sub-crates as batch1/batch2/batch3
    src/
      lib.rs              # pub use libxc_kernel_mgga_1 as batch1; etc.
  kernel-mgga-1/          # NEW: first batch of MGGA functionals
  kernel-mgga-2/          # NEW: second batch
  kernel-mgga-3/          # NEW: third batch (and more as needed)

tests/
  oracle_mgga.rs          # UPGRADE: real oracle comparison tests (currently placeholder)
```

### Pattern 1: translate_mgga.py Tool Structure

The tool follows the same structure as `translate_gga.py` with these additions:

```python
# Additional MGGA-specific output field tables
LEVEL_OUTPUTS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau'],
    'fxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
            'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
            'v2sigma2', 'v2sigmalapl', 'v2sigmatau',
            'v2lapl2', 'v2lapltau', 'v2tau2'],
    'kxc': [...],  # 20 fields total
    'lxc': [...],  # 35 fields total
}

# MGGA polarized dimension table (from verify/src/lib.rs)
POL_DIMS = {
    'zk': 1,
    'vrho': 2, 'vsigma': 3, 'vlapl': 2, 'vtau': 2,
    'v2rho2': 3, 'v2rhosigma': 6, 'v2rholapl': 4, 'v2rhotau': 4,
    'v2sigma2': 6, 'v2sigmalapl': 6, 'v2sigmatau': 6,
    'v2lapl2': 3, 'v2lapltau': 4, 'v2tau2': 3,
    # ... all 70 fields
}
```

The function signature generator adds `lapl: &Array<f64>` and `tau: &Array<f64>` parameters.

### Pattern 2: Iterative Verification Workflow

```
For each representative MGGA pattern:
  1. Run translate_mgga.py on one functional
  2. cargo check -p libxc-kernel-mgga  (catches syntax/type errors)
  3. Write a test in tests/oracle_mgga.rs calling oracle_mgga_all()
  4. cargo test --test oracle_mgga (runs libxc oracle comparison)
  5. If errors: fix translate_mgga.py, regenerate, goto 2
  6. If pass: document pattern, proceed to next representative
```

### Pattern 3: Representative Functional Selection

Choose representatives that cover the full pattern space:

| Functional | C Lines | Why Representative |
|------------|---------|-------------------|
| mgga_xc_lp90.c | 7,108 | Simplest: uses lapl+tau, all 4 guard patterns, small enough to debug |
| mgga_k_gea2.c | 7,542 | Kinetic energy (K family, not X or C) |
| mgga_x_lta.c | 7,616 | Exchange-only, tau-dependent, no lapl |
| mgga_c_b88.c | ~15K | Correlation with lapl threshold, polarized has rho[0]+rho[1] pattern |
| hyb_mgga_x_dldf.c | ~51K | Hybrid with lapl, large polarized functions |
| mgga_c_rmggac.c | 99,938 | Largest: stress test for OOM mitigation |

### Pattern 4: MGGA Rust Kernel Template

For `mgga_xc_lp90_exc_unpol`:

```rust
// Source: libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_lp90_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,   // NEW vs GGA
    tau: &Array<f64>,    // NEW vs GGA
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // Unpol: rho[ip], sigma[ip], lapl[ip], tau[ip]
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569e0 + 0.37655e-3 * sigma[ip] * t6
                - 0.37655e-3 * lapl[ip] * t10;  // lapl access
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.40743e-2;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
    }
}
```

For polarized variant (`mgga_xc_lp90_exc_pol`):

```rust
// Load polarized inputs
let rho0 = rho[ip * 2];
let rho1 = rho[ip * 2 + 1];
let sigma0 = sigma[ip * 3];
let sigma1 = sigma[ip * 3 + 1];
let sigma2 = sigma[ip * 3 + 2];
let lapl0 = lapl[ip * 2];    // NEW vs GGA
let lapl1 = lapl[ip * 2 + 1]; // NEW vs GGA
let tau0 = tau[ip * 2];      // NEW vs GGA
let tau1 = tau[ip * 2 + 1]; // NEW vs GGA
```

[VERIFIED: work_mgga_inc.c confirms VAR(lapl, ip, 0) = lapl[ip * 2 + 0] for polarized]
[VERIFIED: verify/src/lib.rs confirms d_vlapl=2, d_vtau=2 for polarized spin]

### Anti-Patterns to Avoid

- **Generating one giant #[cube] function per functional**: GGA experience shows functions over ~5K lines cause rustc OOM during CubeCL macro expansion. MGGA lxc_pol functions will be far larger — must split.
- **Skipping the iterative test loop**: The whole point of this phase is verify-as-you-go. Do not generate all 90 functionals at once — this replicates the original failure mode.
- **Assuming GGA guard pattern works for MGGA**: MGGA has 4 guard patterns (A/B/C/D above). A guard parser that only handles pattern A will silently drop vlapl/vtau outputs.
- **Using `crate::math::` imports**: Kernel crates use `libxc_kernel_math::` since they live in separate workspace crates [VERIFIED: existing kernel-gga files].

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Oracle comparison | Custom floating-point comparator | `verify::oracle_mgga_all()` + `approx::assert_relative_eq!` | Oracle already wired to libxc 7.0.0 FFI [VERIFIED: verify/src/lib.rs] |
| MGGA dimension tables | Custom calculation | Copy from `verify/src/lib.rs` lines 440-491 | Already verified against libxc util.c |
| C body extraction | Writing new parser | Adapt `extract_function_bodies()` from translate_gga.py | The regex parser already handles brace-depth tracking correctly |
| Import detection | Writing new scanner | Adapt `detect_imports()` from translate_gga.py | Only needs additions for lapl/tau-specific math (likely none — same math functions) |

**Key insight:** The GGA translator is 90% of what is needed. The MGGA translator is a targeted extension, not a rewrite.

---

## Common Pitfalls

### Pitfall 1: lapl/tau arrays in the C source are passed as pointers to a *single* element

**What goes wrong:** In the C source, `lapl[0]` and `tau[0]` look like array index 0, but the
actual memory they point to is `&VAR(lapl, ip, 0)` from the outer loop — i.e., they already point
to the ip-th element. So `lapl[0]` in the C function body means "the lapl value for this grid point."

**In the Rust kernel:** For unpol mode, `lapl[0]` translates to `lapl[ip]` (not `lapl[0]`).
For pol mode, `lapl[0]` → `lapl[ip * 2]` and `lapl[1]` → `lapl[ip * 2 + 1]`.

**Why it happens:** The same pattern exists for rho and sigma in GGA (already handled correctly in translate_gga.py). Forgetting to add the same treatment for lapl and tau is the MGGA-specific trap.

**How to avoid:** In `translate_line()`, add the same substitution rules for lapl and tau as already exist for rho and sigma:
```python
if is_pol:
    s = s.replace('lapl[0]', 'lapl0')
    s = s.replace('lapl[1]', 'lapl1')
    s = s.replace('tau[0]', 'tau0')
    s = s.replace('tau[1]', 'tau1')
else:
    s = s.replace('lapl[0]', 'lapl[ip]')
    s = s.replace('tau[0]', 'tau[ip]')
```

**Warning signs:** Generated code has `lapl[0]` or `tau[0]` as literals (not variables), or oracle comparison returns incorrect values for functionals that use laplacian.

### Pitfall 2: Output guard parsing misses NEEDS_LAPLACIAN / NEEDS_TAU conditions

**What goes wrong:** The GGA body parser looks for `if(out->field != NULL)` to identify output
writes. In MGGA, the guard pattern for lapl/tau outputs is:
```c
if(out->vrho != NULL && (flags & XC_FLAGS_NEEDS_LAPLACIAN) && ...)
  out->vlapl[...] += var;
```

The existing GGA parser (`parse_body()`) uses a simple `if(out->...)` regex to find output writes.
In MGGA this still works because the guard always starts with `if(out->` — but the parser must
correctly identify the FIELD being written (vlapl, vtau, etc.) and include it in the output map.

**How to avoid:** Extend the output write regex to handle the multi-condition guard:
```python
# Current GGA pattern (match field from out->field[...] on the NEXT line after if):
m = re.match(r'out->(\w+)\[ip\s*\*\s*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)\s*;', nxt)
```
This regex already matches `out->vlapl[...]` correctly — the guard line itself is skipped by
the `if(out->` detector, and the following write line is matched by the existing pattern.
Verify this works for all 4 guard patterns before proceeding.

**Warning signs:** Generated Rust code missing `vlapl[ip] += tvlapl0;` lines even though the
C source contains `out->vlapl[ip*dim.vlapl + 0] += tvlapl0;`.

### Pitfall 3: CubeCL OOM for large MGGA functions (kxc_pol, lxc_pol)

**What goes wrong:** Same as GGA but worse. MGGA lxc_pol functions for large functionals
(mgga_c_rmggac, mgga_c_revtpss, etc.) will generate 10K-50K lines of Rust. CubeCL
`#[cube(launch_unchecked)]` expands each function into multiple macro artifacts; at ~5K lines
per function the expansion exhausts available RAM (23GB RAM + swap on this machine).

**How to avoid:** Before attempting to compile any large MGGA functional, check generated
line counts:
```bash
wc -l crates/kernel-mgga-1/src/mgga_c_rmggac/lxc_pol.rs
```
If over 4000 lines, split the file into two `#[cube]` functions that each compute half the
temporaries and write half the output fields.

**Warning signs:** SIGKILL from OOM during `cargo check -p libxc-kernel-mgga-N`. rustc process
memory grows unbounded before kernel compilation completes.

### Pitfall 4: Two MGGA functionals are vxc-only (no EXC)

**What goes wrong:** `mgga_x_2d_prp10.c` and `mgga_x_tb09.c` live in `mgga_vxc/` and only
contain `func_vxc_*` functions (no `func_exc_*`). The batch translator must handle these with
`--vxc-only` mode (same as `gga_vxc/` handling in translate_gga.py).

**How to avoid:** Mirror the GGA batch translator's handling of vxc files:
```python
# mgga_vxc files use vxc-only mode
for fname in sorted(os.listdir(mgga_vxc_dir)):
    ...
    translate_functional(c_path, func_name, out_dir, is_vxc_only=True)
```

### Pitfall 5: MGGA pol functions access sigma as sigma[0], sigma[1], sigma[2] (not just sigma[0]/sigma[2])

**What goes wrong:** GGA pol functions access sigma[0], sigma[1], sigma[2]. MGGA pol functions
also access sigma[0]+sigma[1]*2+sigma[2] aggregates (as seen in mgga_xc_lp90.c pol function:
`t3 = sigma[0] + 0.2e1 * sigma[1] + sigma[2]`). The translator must handle all three sigma
components the same way as GGA already does.

**How to avoid:** The GGA translator already handles sigma[0], sigma[1], sigma[2] → sigma0,
sigma1, sigma2 in pol mode. Verify this carries over correctly to MGGA.

---

## Runtime State Inventory

This phase creates a new Python tool and Rust kernel files. No external services, databases,
or OS registrations are involved.

- Stored data: None — verified. The MGGA kernel files are new files, no existing data migration.
- Live service config: None — verified.
- OS-registered state: None — verified.
- Secrets/env vars: None — verified.
- Build artifacts: `crates/kernel-mgga/src/order*.rs` placeholder stubs must be removed and replaced with real kernel module declarations. The stub Cargo.toml for kernel-mgga must be updated.

---

## Code Examples

### Tool Structure: translate_mgga.py (skeleton showing key differences from translate_gga.py)

```python
# Source: modeled on tools/translate_gga.py
# Key additions for MGGA

MGGA_LEVEL_OUTPUTS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau'],
    'fxc': ['zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
            'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
            'v2sigma2', 'v2sigmalapl', 'v2sigmatau',
            'v2lapl2', 'v2lapltau', 'v2tau2'],
    'kxc': [  # 20 fields
        'zk', 'vrho', 'vsigma', 'vlapl', 'vtau',
        'v2rho2', 'v2rhosigma', 'v2rholapl', 'v2rhotau',
        'v2sigma2', 'v2sigmalapl', 'v2sigmatau', 'v2lapl2', 'v2lapltau', 'v2tau2',
        'v3rho3', 'v3rho2sigma', 'v3rho2lapl', 'v3rho2tau',
        'v3rhosigma2', 'v3rhosigmalapl', 'v3rhosigmatau',
        'v3rholapl2', 'v3rholapltau', 'v3rhotau2',
        'v3sigma3', 'v3sigma2lapl', 'v3sigma2tau',
        'v3sigmalapl2', 'v3sigmalapltau', 'v3sigmatau2',
        'v3lapl3', 'v3lapl2tau', 'v3lapltau2', 'v3tau3',
    ],
    'lxc': [  # 35 fields (all above + 4th order)
        # ... 35 fields from MggaOracleOutput struct
    ],
}

# Polarized dimensions from verify/src/lib.rs
MGGA_POL_DIMS = {
    'zk': 1,
    'vrho': 2, 'vsigma': 3, 'vlapl': 2, 'vtau': 2,
    'v2rho2': 3, 'v2rhosigma': 6, 'v2rholapl': 4, 'v2rhotau': 4,
    'v2sigma2': 6, 'v2sigmalapl': 6, 'v2sigmatau': 6,
    'v2lapl2': 3, 'v2lapltau': 4, 'v2tau2': 3,
    'v3rho3': 4, 'v3rho2sigma': 9, 'v3rho2lapl': 6, 'v3rho2tau': 6,
    'v3rhosigma2': 12, 'v3rhosigmalapl': 12, 'v3rhosigmatau': 12,
    'v3rholapl2': 6, 'v3rholapltau': 8, 'v3rhotau2': 6,
    'v3sigma3': 10, 'v3sigma2lapl': 12, 'v3sigma2tau': 12,
    'v3sigmalapl2': 9, 'v3sigmalapltau': 12, 'v3sigmatau2': 9,
    'v3lapl3': 4, 'v3lapl2tau': 6, 'v3lapltau2': 6, 'v3tau3': 4,
    'v4rho4': 5, 'v4rho3sigma': 12, 'v4rho3lapl': 8, 'v4rho3tau': 8,
    'v4rho2sigma2': 18, 'v4rho2sigmalapl': 18, 'v4rho2sigmatau': 18,
    'v4rho2lapl2': 9, 'v4rho2lapltau': 12, 'v4rho2tau2': 9,
    'v4rhosigma3': 20, 'v4rhosigma2lapl': 36, 'v4rhosigma2tau': 36,
    'v4rhosigmalapl2': 18, 'v4rhosigmalapltau': 24, 'v4rhosigmatau2': 36,
    'v4rholapl3': 8, 'v4rholapl2tau': 12, 'v4rholapltau2': 12, 'v4rhotau3': 8,
    'v4sigma4': 15, 'v4sigma3lapl': 20, 'v4sigma3tau': 30,
    'v4sigma2lapl2': 18, 'v4sigma2lapltau': 24, 'v4sigma2tau2': 18,
    'v4sigmalapl3': 12, 'v4sigmalapl2tau': 18, 'v4sigmalapltau2': 18, 'v4sigmatau3': 12,
    'v4lapl4': 5, 'v4lapl3tau': 8, 'v4lapl2tau2': 9, 'v4lapltau3': 8, 'v4tau4': 5,
}

def generate_function_signature(func_name, level, spin, out_bufs, used_params):
    """Generate #[cube(launch_unchecked)] function signature for MGGA."""
    is_pol = (spin == 'pol')
    fn_name = f'{func_name}_{level}_{spin}'
    lines = [
        f'#[allow(unused_variables, non_snake_case)]',
        f'#[cube(launch_unchecked)]',
        f'pub fn {fn_name}(',
        f'    rho: &Array<f64>,',
        f'    sigma: &Array<f64>,',
        f'    lapl: &Array<f64>,',   # MGGA addition
        f'    tau: &Array<f64>,',    # MGGA addition
    ]
    for buf in out_bufs:
        lines.append(f'    {buf}: &mut Array<f64>,')
    for field, indices in used_params:
        lines.append(f'    {param_rust_name(field, indices)}: f64,')
    lines.append(f'    dens_threshold: f64,')
    lines.append(f'    zeta_threshold: f64,')
    lines.append(f') {{')
    return lines
```

### Oracle Test Pattern for MGGA

```rust
// tests/oracle_mgga.rs (expanded from placeholder)
use verify::{oracle_mgga_all, FLAGS_HAVE_EXC, FLAGS_HAVE_VXC};

#[test]
fn test_mgga_xc_lp90_exc_unpol() {
    // XC_MGGA_XC_ZLP = 42, XC_UNPOLARIZED = 1
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.0; np];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = oracle_mgga_all(42, 1, &rho, &sigma, &lapl, &tau).unwrap();

    // TODO: Call Rust kernel, compare with oracle
    // Relative error threshold: 1e-12 for exc
}
```

---

## Environment Availability

| Dependency | Required By | Available | Version | Notes |
|------------|------------|-----------|---------|-------|
| Python 3 | translate_mgga.py | Yes | 3.x | Standard WSL2 environment |
| cargo / rustc | Kernel compilation | Yes | 1.85+ | [VERIFIED: project uses edition 2024] |
| libxc (system) | verify/ oracle calls | Yes | 7.0.0 | [VERIFIED: verify/ build script links against vendored libxc-master] |
| RUST_MIN_STACK | kernel-lda compilation | Yes (env var) | N/A | Required: `RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-lda` |
| ~23GB RAM | CubeCL proc macro expansion | Yes | N/A | GGA sub-crate split kept each batch under ~16GB; MGGA will need similar or smaller batches |

---

## Validation Architecture

`nyquist_validation` is `true` on the HEAD branch (conflict shows true in HEAD, false in origin/main).

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in (cargo test) |
| Config file | Cargo.toml workspace profiles |
| Quick run command | `cargo check -p libxc-kernel-mgga-1` |
| Full suite command | `cargo test --test oracle_mgga` |

### Phase Requirements to Test Map

| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| KERN-05 | All MGGA kernel files translated to #[cube] | unit | `cargo check -p libxc-kernel-mgga` |
| KERN-06 | Translation preserves FP operation order | oracle | `cargo test --test oracle_mgga` |
| VERIFY-03 | Energy relative error <= 1e-12 | oracle | Assert in oracle_mgga.rs |

### Wave 0 Gaps

- `tests/oracle_mgga.rs` — upgrade from placeholder to real oracle comparison tests
- `tools/translate_mgga.py` — does not exist yet (Wave 0 task)
- `crates/kernel-mgga-1/`, `kernel-mgga-2/`, etc. — do not exist yet (Wave 0 task)

---

## Security Domain

This phase involves code generation (Python tool) and numerical library implementation (Rust
kernels). No network access, authentication, or user-facing API surfaces are involved.

| ASVS Category | Applies | Control |
|---------------|---------|---------|
| V5 Input Validation | Partial | translate_mgga.py inputs are trusted C source files from the vendored libxc repository |
| V6 Cryptography | No | N/A |

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `lapl[0]` in MGGA C unpol functions maps to `lapl[ip]` (pointer to ip-th element), same pattern as rho[0] in GGA | Pitfall 1 / Code Examples | Generated kernels produce wrong numerical values; oracle comparison catches this |
| A2 | Splitting `#[cube]` functions into sub-functions (Option A for OOM) is implementable without CubeCL API changes | Common Pitfalls / OOM | May require downgrading to Option B (skip large derivative orders) |
| A3 | The existing `parse_body()` guard detection in translate_gga.py correctly handles MGGA's 4 guard patterns without modification | Pitfall 2 | vlapl/vtau outputs silently dropped; oracle comparison catches this |
| A4 | MGGA POL_DIMS values in this document are correct (copied from verify/src/lib.rs) | Code Examples | Buffer sizing errors in generated kernels; would manifest as out-of-bounds access or wrong results |

---

## Open Questions

1. **How large will generated MGGA Rust files be per functional?**
   - What we know: GGA generated ~10x expansion from C to Rust (1K C lines → ~500-5K Rust lines per derivative file)
   - What's unclear: MGGA's extra fields (70 vs 15) may produce proportionally larger files
   - Recommendation: Translate `mgga_xc_lp90.c` first and measure generated file sizes before deciding on sub-crate batch sizes

2. **Should lxc (4th-order) and kxc (3rd-order) kernels be generated at all for large MGGA functionals?**
   - What we know: GGA deferred 25/131 functionals due to OOM; these were all lxc/kxc polarized
   - What's unclear: How many MGGA functionals will hit the OOM limit; whether any workaround besides sub-function splitting exists
   - Recommendation: Generate exc+vxc+fxc first for all 90 functionals; add kxc/lxc iteratively where memory permits

3. **Does the MGGA dispatch infrastructure (dispatch_mgga) need to be built as part of this phase?**
   - What we know: `src/eval/dispatch.rs` only has LDA dispatch; GGA and MGGA dispatch is deferred
   - What's unclear: Whether the phase goal ("rebuild conversion tool") implies just the tool or also the dispatch wiring
   - Recommendation: Phase 8 = tool + compiled kernels. Dispatch wiring is Phase 4-04 (MGGA batch).

---

## Sources

### Primary (HIGH confidence)

- [VERIFIED: tools/translate_gga.py] Complete GGA translator — reference implementation for MGGA tool
- [VERIFIED: verify/src/lib.rs lines 326-491] oracle_mgga_all() and MggaOracleOutput with all 70 fields and polarized dimension multipliers
- [VERIFIED: libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c] Smallest MGGA file — all 4 guard patterns confirmed
- [VERIFIED: libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c] polarized tau[0]/tau[1], lapl handling patterns
- [VERIFIED: libxc-master/src/work_mgga_inc.c] Confirms FUNC(ORDER,SPIN)(p, ip, my_rho, my_sigma, &VAR(lapl,ip,0), my_tau, out) calling convention
- [VERIFIED: crates/kernel-gga/src/lib.rs] GGA sub-crate split pattern (3 sub-crates, ~35 modules each, deferred list)
- [VERIFIED: .planning/phases/08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe/08-04-SUMMARY.md] GGA OOM experience: single crate OOM, 35-module sub-crates fit in ~16GB
- [VERIFIED: crates/kernel-gga/src/gga_c_acgga/exc_unpol.rs] Example translated GGA kernel showing final Rust structure

### Secondary (MEDIUM confidence)

- [VERIFIED: file size count] 90 MGGA maple2c files, 29 over 30K lines, largest mgga_c_rmggac.c at 99K lines

---

## Metadata

**Confidence breakdown:**
- MGGA C source structure: HIGH — verified by reading C files
- Translation differences from GGA: HIGH — verified by comparing C signatures and output field lists
- OOM risk: HIGH — documented in 08-04-SUMMARY.md (GGA experience)
- Sub-crate split strategy: MEDIUM — MGGA splits may need smaller batches than GGA (not yet tested)
- Polarized dimension values: HIGH — copied from verified verify/src/lib.rs

**Research date:** 2026-04-13
**Valid until:** 2026-06-01 (stable — libxc 7.0.0 source files do not change)
