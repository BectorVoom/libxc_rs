# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Pattern Map

**Mapped:** 2026-05-13
**Files analyzed:** 14 modify + 9 create + bulk-regen target tree (kernel files)
**Analogs found:** 22 / 23 (one new pattern — generic `<F: Float>` tuple-return `#[cube]` — has NO existing analog in libxc_rs and requires the Wave 0 spike)

## File Classification

### MODIFY (existing files to be edited or restructured)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `tools/translate_lda_v2.py` | translator | Maple AST → Rust source | self (extend in place) | self — extend chunked-scratch path |
| `tools/translate_gga.py` | translator | Maple AST → Rust source | `tools/translate_lda_v2.py` lines 480-852 (chunked-scratch path) | role-match, MUST DEPART from analog (no `&mut Array<f64>`) |
| `tools/translate_mgga.py` | translator | Maple AST → Rust source | `tools/translate_lda_v2.py` lines 480-852 | role-match, MUST DEPART from analog |
| `tools/maple_to_kernels.py` | orchestrator/driver | configuration | self | self — re-tune defaults from 100K/500K → 5K |
| `tools/split_oversized_kernel.py` | splitter | post-emit bin-pack | n/a — DELETE candidate | n/a (subcrate collapse obviates) |
| `tools/split_oversized_mgga.py` | splitter | post-emit bin-pack | n/a — DELETE candidate | n/a (also has known `rmtree` bug) |
| `tools/split_mgga_7_kcis.py` | splitter | one-off | n/a — DELETE candidate | n/a |
| `tools/rebatch_mgga.py` | splitter | first-fit-decreasing | n/a — DELETE candidate | n/a |
| `Cargo.toml` (workspace root) | workspace-config | configuration | self | self — drop 22 numbered deps + default-members |
| `crates/kernels/{lda,gga,mgga}/Cargo.toml` | re-export-shim config | configuration | `crates/kernels/gga/Cargo.toml` (already owns `cubecl + math` deps) | exact for GGA/MGGA; LDA needs to add `cubecl + math` direct deps |
| `crates/kernels/lda/src/lib.rs` | re-export-shim | re-export | `crates/kernels/lda-1/src/lib.rs` lines 7-21 (`pub mod <func>;` form) | exact pattern to expand to |
| `crates/kernels/gga/src/lib.rs` | re-export-shim | re-export | self lines 11-19 (current `pub use ... as batchN;`) → become `pub mod batchN;` | self, single-token rewrite per Strategy 1 |
| `crates/kernels/mgga/src/lib.rs` | re-export-shim | re-export | self lines 13-30 → become `pub mod batchN;` | self, single-token rewrite per Strategy 1 |
| `CLAUDE.md` | docs | configuration | self lines 13, 17 (Constraints section) | self — D-03a edit |
| `.cargo/config.toml` | workspace-config | configuration | n/a — READ-ONLY (D-08, D-09 invariants) | INVARIANT |

### CREATE (new files)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `tools/audit_kernel_size.py` | audit-tool | filesystem scan → JSON/exit-code | `tools/audit_deferred_gga.py` lines 1-77 | exact (same pattern: walk crates/kernels, emit JSON+md, --strict exit) |
| `tools/audit_subcrate_collapse.sh` | audit-tool | filesystem scan → exit-code | none (inline shell) | n/a — write minimal `find ... | grep -E '^...' && exit 1` |
| `tools/audit_cube_launch.sh` | audit-tool | filesystem scan → exit-code | RESEARCH.md §"audit script" lines 478-484 (~10 line snippet) | spec-driven |
| `tools/test_idempotency.sh` | audit-tool | git-diff → exit-code | RESEARCH.md §"Idempotency test" lines 510-519 | spec-driven |
| `verify/tests/parity_phase11.rs` | verify-test | request-response (test) | `verify/tests/parity_phase09.rs` lines 1-120, 450-565 | exact (extend canonical-list pattern to Phase 11 functionals) |
| `verify/tests/spike_tuple_return_cube.rs` | spike | smoke test | `verify/tests/metadata_oracle.rs` (smallest verify test, 75 lines) | structural (smallest test as scaffold; CONTENT is novel and is the whole point of the spike) |
| Re-emitted kernel files under `crates/kernels/{lda,gga,mgga}/` | kernel-emission | Rust source output | existing `crates/kernels/lda-1/src/lda_x/exc_unpol.rs` lines 1-25 (small) AND `crates/kernels/lda-2/src/lda_xc_ksdt/lxc_pol_part5_v4rho4_1.rs` lines 1-35 (chunked-scratch wrapper) | role-match, chunked wrapper is **anti-pattern** (uses `&mut Array<f64>`); see "No Analog Found" below for D-02 ABI |
| Per-functional `<chunk>_chunk{0,1,...}.rs` files (NEW D-02 ABI) | kernel-emission | Rust source output | existing `crates/kernels/lda-2/src/lda_xc_ksdt/lxc_pol_part5_v4rho4_1_chunk0.rs` lines 1-25 (closest existing) | **partial — STRUCTURAL only**: the file naming, header, imports, `#[cube]` placement carry over; the ABI itself (signature + body) MUST DEPART (D-02/D-03) |
| `tools/translate_v2/cse.py` (recommended per RESEARCH.md §"Recommendation: Option C") | translator helper | Python AST → Rust source | `tools/translate_lda_v2.py` lines 370-401 (`build_dependency_graph` + `transitive_deps`) | role-match — extend the existing dep-graph to compute min-cut chunk boundaries |

## Pattern Assignments

### `tools/translate_{lda_v2,gga,mgga}.py` — chunked-scratch path replacement

**Closest analog:** `tools/translate_lda_v2.py` lines 480-852 (the entire chunked-scratch path: `_parse_var_defs`, `_build_scratch_replacer`, `_generate_chunk_helper`, `_generate_chunked_wrapper`, `chunk_single_output_split`).

**Imports / module-level constants pattern** (`tools/translate_lda_v2.py` lines 362, 502-507):
```python
SPLIT_THRESHOLD = 6000   # → Phase 11: lower to 5000 (D-LOCK-B hard cap)
UNSPLITTABLE = 999999

# Safety margin for header/blank lines/closing braces above the precisely
# counted boilerplate (signature + imports). 16 covers: 1 doc, 1 blank, 1
# `#![allow]`, 1 blank, 1 blank after imports, 1 `#[allow]`, 1 `#[cube]`,
# 1 `pub fn ... (`, 1 `) {`, 1 `let ip = ABSOLUTE_POS;`, 2 `rho0`/`rho1`,
# 1 closing `}`, plus 3 spare lines.
CHUNK_FIXED_OVERHEAD_LINES = 16
```

**Dep-graph pattern to EXTEND for CSE breakpoints** (`tools/translate_lda_v2.py` lines 370-401):
```python
def build_dependency_graph(compute_lines):
    """Build a dependency graph from C compute lines."""
    var_order = []
    var_deps = {}
    for cline in compute_lines:
        stripped = cline.rstrip(';').strip()
        m = re.match(r'(\w+)\s*=\s*(.*)', stripped)
        if not m:
            continue
        var = m.group(1)
        expr = m.group(2)
        refs = set(re.findall(r'\b(t\w+)\b', expr))
        refs.discard(var)
        refs -= {'true', 'tanh', 'tan', 'trunc'}
        var_order.append(var)
        var_deps[var] = refs
    return var_order, var_deps

def transitive_deps(variables, var_deps):
    result = set()
    stack = list(variables)
    while stack:
        v = stack.pop()
        if v in result: continue
        result.add(v)
        for dep in var_deps.get(v, set()):
            if dep not in result: stack.append(dep)
    return result
```
Phase 11 CSE pass adds: reverse-dep counts, walk-with-budget chunker, tuple-arg/return inference. Per RESEARCH.md §"CSE Detection Heuristic" lines 287-300, target a `tools/translate_v2/cse.py` of ~600 lines.

**Outermost dispatch pattern (where chunked path is invoked)** (`tools/translate_lda_v2.py` lines 1191-1210):
```python
for idx, (suffix, sub_compute, sub_outputs, sub_bufs) in enumerate(final_splits):
    fn_suffix = f'_part{idx}_{suffix}'
    sub_lines = estimate_function_lines(sub_compute, sub_outputs)
    if sub_lines > SPLIT_THRESHOLD:
        chunked = chunk_single_output_split(
            func_name, level, spin, fn_suffix,
            sub_compute, sub_outputs, sub_bufs,
            all_params, is_vxc_only,
            chunk_threshold=SPLIT_THRESHOLD,
        )
        for sub_name, text in chunked:
            path = os.path.join(subdir, f'{sub_name}.rs')
            with open(path, 'w') as f:
                f.write(text)
            written.append(path)
            mod_entries.append(f'pub mod {sub_name};')
        continue
```
This dispatch wedge already exists in LDA. **GGA/MGGA must add an equivalent wedge** at their per-component splitter exit (`tools/translate_gga.py` ~line 1100s; `tools/translate_mgga.py` ~line 900s — verify exact line during planning).

**Anti-pattern in analog (MUST DEPART per D-02/D-03):** the existing `_generate_chunk_helper` (lines 571-633) emits:
```python
lines.append(f'pub fn {fn_name}(')
lines.append('    rho: &Array<f64>,')
lines.append('    s: &mut Array<f64>,')           # ← VIOLATES D-02 (shared mutable state)
for pa in used_params:
    lines.append(f'    {pa.rust_name}: f64,')      # ← VIOLATES D-03 (hardcoded f64)
...
lines.append(f'    s[{idx}usize] = {translated};') # ← VIOLATES D-02 (scratch indexing)
```
Phase 11 replaces this with: free function, generic `<F: Float>`, explicit-args + tuple-return signature. See "No Analog Found" → "D-02 ABI" below for the target shape.

---

### `tools/audit_kernel_size.py` (NEW)

**Closest analog:** `tools/audit_deferred_gga.py` lines 1-77.

**Module docstring + CLI signature pattern** (`tools/audit_deferred_gga.py` lines 1-50):
```python
#!/usr/bin/env python3
"""Audit per-functional derivative-order coverage for the 25 historically-deferred GGA functionals.

...

Usage:
    python3 tools/audit_deferred_gga.py [--strict] \\
        [--json-out PATH] [--md-out PATH]

Exits:
    0 — all canonical functionals pass; no gaps.
    1 — at least one functional has status != "OK" (only with --strict).

Public API (used by tests):
    load_canonical_list(maple2c_root=...) -> list[str]
    audit_functional(name, repo_root=...) -> dict
    main() -> int
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
```

**Imports + REPO_ROOT discovery** — copy verbatim from `audit_deferred_gga.py` lines 43-52.

**Threshold constant pattern** (`audit_deferred_gga.py` lines 68-77):
```python
LXC_POL_BODY_LINE_THRESHOLD = 6000  # → Phase 11 audit equivalent: KERNEL_LINE_CAP = 5000
```

**Action for Phase 11:** the audit walks `find crates/kernels -name '*.rs' -exec wc -l {} +` (per RESEARCH.md §"P11-INV-2"), aggregates files >5K, emits JSON + markdown, exits non-zero under `--strict` if any oversized file remains. Public API mirrors the analog: `audit_kernel_size(repo_root) -> dict`, `main() -> int`.

---

### `verify/tests/parity_phase11.rs` (NEW)

**Closest analog:** `verify/tests/parity_phase09.rs` (733 lines — entire file is the pattern source).

**Imports + tolerance constants** (`verify/tests/parity_phase09.rs` lines 25-40):
```rust
use libxc_rs::LibxcRsError;
use libxc_rs::eval::{dispatch_gga, dispatch_mgga};
use libxc_rs::input::{GgaInput, MggaInput};
use libxc_rs::model::{
    DerivativeOrder, FunctionalId, GgaFunctional, MggaFunctional, Spin, Thresholds,
};
use libxc_rs::output::{GgaOutput, MggaOutput};
use libxc_rs_verify::{
    FLAGS_HAVE_EXC, FLAGS_HAVE_FXC, FLAGS_HAVE_KXC, FLAGS_HAVE_LXC, FLAGS_HAVE_VXC,
    GgaOracleOutput, MggaOracleOutput, oracle_func_flags, oracle_gga_all, oracle_mgga_all,
};

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;
```

**Canonical-list table pattern** (`parity_phase09.rs` lines 47-119):
```rust
const DEFERRED_GGA_NAMES: &[&str] = &[
    "gga_c_acgga",
    "gga_c_acggap",
    ...
];

struct DeferredEntry {
    canonical: &'static str,
    ids: &'static [i32],
}

const DEFERRED_GGA_ENTRIES: &[DeferredEntry] = &[
    DeferredEntry { canonical: "gga_c_acgga", ids: &[39] },
    ...
];
```
Phase 11's smoke list (per RESEARCH.md §"Failure Modes" line 549 + §"Validation Architecture" line 466) is: `lda_x`, `gga_x_pbe`, `mgga_x_scan`, `mgga_c_revtpss`, `mgga_c_kcisk`, `mgga_c_b94`, `mgga_x_r4scan` — extend DEFERRED_GGA_ENTRIES analog with these.

**Per-tuple compare pattern** (`parity_phase09.rs` lines 467-540):
```rust
let mut zk = vec![0.0f64; np];
let mut vrho = vec![0.0f64; np * d_vrho];
...
let mut output = MggaOutput {
    zk: if functional.has_exc() { Some(&mut zk) } else { None },
    vrho:  if order >= DerivativeOrder::Vxc { Some(&mut vrho) } else { None },
    ...
    ..Default::default()
};

if let Err(e) = dispatch_mgga(
    functional, &input, order, &mut output,
    &libxc_rs::NoParams, &Thresholds::default(),
) {
    return classify_dispatch_err(e);
}

let pairs: Vec<(&str, &[f64], &[f64])> = match order {
    DerivativeOrder::Exc => { if functional.has_exc() { vec![("zk", &zk[..], &oracle.zk[..])] } else { Vec::new() } }
    DerivativeOrder::Vxc => vec![
        ("vrho",   &vrho[..],   &oracle.vrho[..]),
        ("vsigma", &vsigma[..], &oracle.vsigma[..]),
        ("vlapl",  &vlapl[..],  &oracle.vlapl[..]),
        ("vtau",   &vtau[..],   &oracle.vtau[..]),
    ],
    _ => Vec::new(),
};

for (label, rust_slice, c_slice) in pairs {
    for i in 0..rust_slice.len().min(c_slice.len()) {
        let e = rel_err_with_floor(rust_slice[i], c_slice[i]);
        if e > max_e { max_e = e; }
        if e > STRICT_TOL {
            return TupleResult::Fail { ... };
        }
    }
}
```

**Skip-classification pattern** (`parity_phase09.rs` lines 18-23, also 493):
```rust
//! **No-skip invariant (plan Test 4):** A tuple is allowed to skip ONLY if
//! `dispatch_gga`/`dispatch_mgga` returns `UnsupportedFunctional` or
//! `UnsupportedDerivativeOrder` — exactly mirroring the skip semantics of
//! `gga_oracle.rs`/`mgga_oracle.rs`. Every skipped tuple is printed with a
//! `PARITY_TUPLE: ... SKIP <reason>` line so the post-run audit harness can
//! assemble the full report.
```
Phase 11 inherits the no-skip invariant verbatim — D-05's strict 1e-12 gate has no relaxation per CONTEXT.

---

### `verify/tests/spike_tuple_return_cube.rs` (NEW — Wave 0 spike)

**Closest analog (structural only):** `verify/tests/metadata_oracle.rs` (75 lines, smallest existing verify test).

**Module docstring pattern** (`metadata_oracle.rs` lines 1-7):
```rust
//! D-04 metadata round-trip: every FunctionalMeta field compared to a fresh
//! `xc_func_init` FFI snapshot for all 649 IDs.

use libxc_rs::meta::{ExtParamSpec, FunctionalMeta, HybridTerm, Reference};
use libxc_rs::model::{FunctionalFlags, FunctionalId, HybridTermKind, HybridType};
use libxc_rs::registry::{all_functional_ids, lookup_by_id};
use libxc_sys::{xc_func_end, xc_func_init, xc_func_type, xc_hyb_type, XC_UNPOLARIZED};
```

**Test signature pattern** (`metadata_oracle.rs` lines 42-65):
```rust
#[test]
fn metadata_round_trip_all_649() {
    ...
}
```

**ANALOG ENDS HERE.** The body of the spike is novel — it is the FIRST `#[cube] fn f<F: Float>(...) -> (F, F)` in the codebase per RESEARCH.md A1 (line 704). The spike body shape (specified in RESEARCH.md §"Code Examples" lines 638-680 and §"Wave 0 Gaps" line 598):
```rust
#[cube]
fn add_sub<F: Float>(x: F, y: F) -> (F, F) { (x + y, x - y) }
// + a launch test that asserts both outputs match expected f64 values to 1e-15
```
Per A1 (HIGH risk if wrong) this MUST run and pass before bulk rollout.

---

### `crates/kernels/{lda,gga,mgga}/src/lib.rs` — re-export-shim restructure (Strategy 1) — **OBSOLETE per D-10b (2026-05-13 user revision)**

> **NOTE — DO NOT USE FOR NEW PLANS.** Strategy 1 envisioned numbered subcrates surviving as `pub mod batchN;` re-exports inside the family façade. The user revision (D-10/D-10a/D-10b in 11-CONTEXT.md) DELETES the numbered subcrates upfront and the splitter emits directly into the unified per-family layout. The new pattern is "Splitter emission target redirect" (added below). The LDA per-functional `pub mod <func>;` form (the LDA analog block in this section) is STILL CORRECT and applies to all three families post-D-10. The GGA/MGGA `pub mod batchN;` form described below is the part that becomes obsolete.


**Closest analog (LDA):** `crates/kernels/lda-1/src/lib.rs` lines 7-21:
```rust
pub mod hyb_lda_xc_bn05;
pub mod lda_c_1d_csc;
pub mod lda_c_1d_loos;
pub mod lda_c_2d_amgb;
pub mod lda_c_2d_prm;
pub mod lda_c_chachiyo;
...
```
Post-collapse `crates/kernels/lda/src/lib.rs` becomes this exact shape (every functional as `pub mod`, no `pub use libxc_kernel_lda_N::...`).

**Closest analog (GGA/MGGA Strategy 1):** existing `crates/kernels/gga/src/lib.rs` lines 11-19:
```rust
pub use libxc_kernel_gga_1 as batch1;
pub use libxc_kernel_gga_2 as batch2;
...
pub use libxc_kernel_gga_8 as batch8;
```
After Strategy 1 collapse, REPLACE `pub use ... as batchN;` with `pub mod batchN;` (real submodules holding the moved subcrate content). Per RESEARCH.md §"Sub-crate Collapse Mechanics" line 313, this preserves dispatch import path `crate::kernel::gga::batchN::<func>::...` unchanged (verified at `src/eval/mgga_dispatch/batch17.rs` line 25).

**`Cargo.toml` workspace member pattern to MIRROR for cleanup** — root `Cargo.toml` lines 11-34 (current numbered deps) shrink to just the 3 family façades + `math`. Default-members (lines 44-76) shrinks to the same set.

**Family façade Cargo.toml deps** — existing `crates/kernels/gga/Cargo.toml` already owns the right deps (`cubecl + libxc-kernel-math`). LDA façade (`crates/kernels/lda/Cargo.toml`) currently lacks `cubecl + math` direct deps (only re-exports lda-1, lda-2) — must ADD them as part of collapse:
```toml
# crates/kernels/lda/Cargo.toml after collapse — mirror gga/Cargo.toml shape
[dependencies]
cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../math" }
```

---

### Re-emitted kernel files — D-02 chunked tuple-return (NEW per-functional pattern)

**Closest existing analog (file structure, mod placement, imports):** `crates/kernels/lda-2/src/lda_xc_ksdt/lxc_pol_part5_v4rho4_1.rs` (wrapper) + `_chunk0.rs`, `_chunk1.rs` (helpers).

**Wrapper imports + header pattern (file-level — KEEP)** (`lxc_pol_part5_v4rho4_1.rs` lines 1-13):
```rust
//! LDA_XC_KSDT chunked-scratch entry — wraps 2 `_chunkN` helpers via a shared `Array<f64>` slot file.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};
use super::lxc_pol_part5_v4rho4_1_chunk0::lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk0;
use super::lxc_pol_part5_v4rho4_1_chunk1::lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk1;

#[allow(unused_variables, non_snake_case)]
#[cube]
```
Phase 11 **keeps the file structure, header, attribute placement, `use super::` chunk-import pattern verbatim**. Update the docstring to "wraps N `_chunkN` helpers via tuple-return ABI" (no shared scratch language).

**Chunk helper imports + header pattern (file-level — KEEP)** (`lxc_pol_part5_v4rho4_1_chunk0.rs` lines 1-11):
```rust
//! LDA_XC_KSDT chunk helper #0 — do not call directly; invoked by `lda_xc_ksdt_lxc_pol_part5_v4rho4_1`.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
#[cube]
```
Same — keep file shape; the **signature and body** are what change (see "No Analog Found" below).

**Routing-aware `#[cube]` vs `#[cube(launch_unchecked)]` decision** (`tools/translate_lda_v2.py` lines 651-652 + RESEARCH.md §"CubeCL Macro Fan-out Manual §4"):
```python
is_unrouted = func_name not in cached_routed_funcnames(KERNEL_FAMILY)
cube_attr = '#[cube]' if (is_split_helper or is_unrouted) else '#[cube(launch_unchecked)]'
```
Phase 11 chunks ALWAYS get `#[cube]` (per D-02 + cubecl manual §4, also RESEARCH.md line 207-209). The wrapper retains its existing routing-aware decision (already correct per 260512-q01).

---

### Splitter emission target redirect (NEW per D-10 — replaces "what to copy" in legacy chunked-scratch path)

**Pattern source:** the existing per-functional emit pattern in `tools/translate_lda_v2.py` `translate_file_split` (lines 1116-1240) and `translate_file_incremental` (lines 1412-1583) which take a `write_dir` argument and emit `write_dir/<func_name>/...`. PRE-D-10 the orchestrator (`tools/regen_phase09.py:143-181`) passes `write_dir = crates/kernels/<existing-numbered-subcrate-name>` (discovered by scanning the filesystem, line 78-100). POST-D-10 the orchestrator passes `write_dir = crates/kernels/<family>` for ALL functionals of that family.

**Mechanical redirect:** the splitter itself does NOT change its emission shape — `translate_file_split(c_path, func_name, write_dir, ...)` still produces `write_dir/<func_name>/mod.rs` plus `write_dir/<func_name>/<level>_<spin>*.rs`. Only the `write_dir` argument changes:

| Family | Pre-D-10 `write_dir` (per regen_phase09.py) | Post-D-10 `write_dir` |
|--------|---------------------------------------------|------------------------|
| LDA    | `crates/kernels/lda-1/src` or `crates/kernels/lda-2/src` (whichever subcrate currently owns the functional, discovered by `scan_functional_distribution`) | `crates/kernels/lda/src` (always) |
| GGA    | `crates/kernels/gga-N/src` (one of 8) | `crates/kernels/gga/src` (always) |
| MGGA   | `crates/kernels/mgga-{1..14 with letter suffixes}/src` (one of 17) | `crates/kernels/mgga/src` (always) |

**The discovery mechanism MUST change too.** `regen_phase09.py:scan_functional_distribution` (line 78-100) walks `crates/kernels/kernel-{lda,gga,mgga}*/src/<func>/` to discover which functionals exist and which subcrate they belong to. POST-D-10 there are no subcrates — the source-of-truth for "which functionals exist" becomes `libxc-master/maple/{lda,gga,mgga}_exc/*.c` (the Maple input directories, already referenced as `MAPLE2C` constant at line 38). New plan 11-02 task 2 introduces `tools/regen_unified.py` (or a `--unified` mode of `regen_phase09.py`) that:
1. Walks `libxc-master/src/maple2c/{family}_exc/*.c` and `_vxc/*.c` to enumerate functionals.
2. For each functional, calls the family translator with `write_dir = crates/kernels/{family}/src`.
3. Maintains the per-functional dir replacement pattern (delete pre-existing files in `crates/kernels/{family}/src/<func>/`, then copy from staging).

**Per-family `lib.rs` regeneration:** after all functionals emit, the unified family `lib.rs` is rewritten with `pub mod <func>;` lines (one per emitted top-level subdir under `crates/kernels/{family}/src/`), sorted alphabetically for splitter-emit determinism (per RESEARCH.md "Idempotency Contract"). The existing `crates/kernels/lda/src/lib.rs` (lines 14-54, the per-functional `pub use libxc_kernel_lda_N::<func>;` block) is structurally what the new lib.rs becomes — just with `pub mod <func>;` instead of `pub use ...`. The `pub mod deferred;` line on line 10 stays; `deferred.rs` is in-aggregator metadata and is NOT regenerated.

**Anti-pattern in the LEGACY analog (CARRY OVER from the now-obsolete Strategy 1 section):** the chunked-scratch helpers in `tools/translate_lda_v2.py` lines 480-852 (`_parse_var_defs`, `_build_scratch_replacer`, `_generate_chunk_helper`, `_generate_chunked_wrapper`, `chunk_single_output_split`) emit `&mut Array<f64>` shared scratch + hardcoded `f64`. These remain anti-patterns under D-10 — the D-02 chunked-scratch wrapper analog (LDA's existing `_chunkN` pattern in `crates/kernels/lda-2/src/lda_xc_ksdt/lxc_pol_part5_v4rho4_1.rs` and its `_chunk0/_chunk1` siblings) STILL applies for the chunking work in waves 2-4, but the file is now emitted under `crates/kernels/lda/src/lda_xc_ksdt/...` instead of `crates/kernels/lda-2/src/lda_xc_ksdt/...`. The chunk file shape, header, imports, `#[cube]` placement, `use super::` chunk-import lines all carry over verbatim; only the parent path differs.

**Dispatch generator path consequences:** `tools/generate_gga_dispatch.py` line 701 currently emits `kpath = f"crate::kernel::gga::batch{batch}::{name}"`. POST-D-10 this becomes `kpath = f"crate::kernel::gga::{name}"` (drop the `batch{batch}::` segment). The `_roster.tsv` files at `.planning/phases/04-bulk-kernel-translation/{gga,mgga}_roster.tsv` retain the `batch` column for now (truncating the column would invalidate the existing parser at line 39-46) but the column value becomes a no-op for path emission. The per-batch submodule emit (`pub mod batch{N};` in `mod.rs`, plus per-batch files at `src/eval/{family}_dispatch/batch{N}.rs`) is REPLACED with a per-functional emit (`pub mod funcs;` plus `src/eval/{family}_dispatch/funcs/<func>.rs`, OR a single flat `src/eval/{family}_dispatch.rs` containing all dispatch helpers). Planner's call which form to use; the unified flat form is recommended for symmetry with LDA dispatch (which uses no batch layer today).

---

## Shared Patterns

### Imports for any newly emitted kernel chunk file
**Source:** `crates/kernels/lda-1/src/lda_x/exc_unpol.rs` lines 7-13 (smallest, cleanest example).
**Apply to:** Every emitted kernel `.rs` file.
```rust
use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};
```
The exact set of constants/powers imports varies per functional — `tools/translate_*.py` already has `detect_imports` + `generate_import_lines` that produce the deterministic sorted set. Reuse unchanged.

### File-level allow-attributes
**Source:** Every emitted kernel file (consistent across `lda-1`, `lda-2`, `gga-*`, `mgga-*`).
**Apply to:** Every emitted kernel `.rs` file.
```rust
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
```

### `#[cube]` attribute placement on functions
**Source:** `crates/kernels/math/src/powers.rs` lines 12-15 (minimal example).
**Apply to:** Every chunk helper AND wrapper that is not a routed entry kernel.
```rust
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn <name>(...) -> ... { ... }
```

### Functional `mod.rs` pattern (per-functional dir)
**Source:** `crates/kernels/lda-1/src/lda_x/mod.rs` lines 17-26 (after the docstring comment block).
**Apply to:** Every per-functional `mod.rs` post-regen.
```rust
pub mod exc_unpol;
pub mod vxc_unpol;
...
pub mod lxc_pol_part0_v4rho4;
pub mod lxc_pol_part0_v4rho4_chunk0;  // ← new under D-02 chunking
pub mod lxc_pol_part0_v4rho4_chunk1;
```

### Python translator deterministic-emit conventions (idempotency)
**Source:** `tools/translate_lda_v2.py` and RESEARCH.md §"Idempotency Contract" lines 489-505.
**Apply to:** Every translator emit and CSE chunk-id assignment.
- Sets must be sorted before iteration.
- Dict iteration relies on Python 3.7+ insertion order.
- Tuple member naming: sort by first-use line index (NOT by hash).
- `_chunkN` indices reset per `_partN`, not global.
- Filename basename ≤ 200 chars (Linux 255-byte limit minus headroom).

### Build env citation (D-08, D-09 invariants)
**Source:** `.cargo/config.toml` lines 1-10 (entire file).
**Apply to:** Every plan, every executor prompt, every audit.
- Do NOT inline `RUST_MIN_STACK` value in plans — cite `.cargo/config.toml`.
- Do NOT inline `jobs = 1` in plans — cite `.cargo/config.toml`.
- Do NOT inline `target-dir` — cite `.cargo/config.toml`.
- D-09 mandates that subagent prompts citing build commands MUST reference the config file, not duplicate values.

### Verify command pattern (D-05 oracle gate)
**Source:** RESEARCH.md §"Verify command" line 467; verified against `verify/tests/parity_phase09.rs` invocation pattern.
**Apply to:** Every verify-gate task in Phase 11.
```bash
cargo test -p libxc_rs-verify --test parity_phase11 -- --test-threads=1 --nocapture
```
`--test-threads=1` is mandatory under RAM constraint (D-07). NEVER add `--jobs N` or override `CARGO_BUILD_JOBS`.

---

## No Analog Found

| File / Pattern | Role | Data Flow | Reason |
|----------------|------|-----------|--------|
| **D-02 ABI: `#[cube] fn chunk_N<F: Float>(args: F, ...) -> (F, F, ...)`** | kernel-emission ABI | Rust source output | RESEARCH.md A1 (line 704): no oracle-validated tuple-returning `#[cube]` kernel exists in libxc_rs today. The cubecl-macros 0.10 PARSER supports tuples (RESEARCH.md line 833) but **no kernel exercises it**. `crates/kernels/math/src/br89.rs:38` and `mbrxc.rs:9` source comments still claim "CubeCL doesn't support tuples" — those comments are STALE per cubecl 0.10 but not yet rebutted by a passing test. Wave 0 spike (`verify/tests/spike_tuple_return_cube.rs`) IS the first analog and MUST land first. |
| **`<F: Float>` generic `#[cube]` kernels** | kernel-emission | Rust source output | RESEARCH.md §19 line 231: "**Currently NOT met anywhere in libxc_rs** (every existing `#[cube]` is hardcoded `f64`)." `crates/kernels/math/src/powers.rs` lines 13-19 (`pub fn safe_cbrt(x: f64) -> f64`) — every math primitive is `f64` today. Phase 11 introduces the FIRST generic kernels. The spike result determines whether the math primitives need to be re-emitted as `<F: Float>` too, or whether wrapping `param: f64 → F::new(param)` at use sites is sufficient (RESEARCH.md §"Code Examples" lines 651-657 picks the latter). |
| **CSE pass over `compute_lines` (Strategy C)** | translator helper | Python AST → chunked partition | The CSE algorithm itself is novel. The CLOSEST existing primitive is `tools/translate_lda_v2.py` lines 370-401 (`build_dependency_graph` + `transitive_deps`) — Phase 11 extends with reverse-dep counts + walk-with-budget chunker + tuple-arg/return inference. RESEARCH.md §"CSE Detection Heuristic" lines 287-300 specifies the algorithm; `tools/translate_v2/cse.py` is the recommended new file (~600 lines). |
| **Subcrate collapse helper (`tools/collapse_subcrates.py`)** — **OBSOLETE per D-10/D-10a (2026-05-13)** | n/a | n/a | The new D-10 architecture deletes numbered subcrates upfront via a small one-shot `rm -rf` + `Cargo.toml` edit (no migrator needed because the splitter populates the unified layout from scratch on next run). Pattern superseded by the bash one-liner in plan 11-02 Task 1. |
| **Workspace `cargo build --workspace` peak-RSS measurement** | audit-tool | runtime measurement | No existing in-repo tool. Closest external analog: `260510-q01-SUMMARY.md` measurement methodology (`/usr/bin/time -v cargo build -p libxc-kernel-mgga-1`). Phase 11 plans should reproduce that pattern, not invent a new one. |

---

## Metadata

**Analog search scope:**
- `tools/*.py` (full inventory: 27 Python files, 4 already in CONTEXT-listed deletion candidates)
- `tools/*.sh` (none exist — Phase 11 INTRODUCES `.sh` audit scripts)
- `crates/kernels/{lda,gga,mgga,math}/src/**/*.rs` (sampled: lda-1 small + lda-2 chunked-scratch wrappers/helpers + math primitives)
- `crates/kernels/{lda,gga,mgga}-*/Cargo.toml` (3 family façades + 22 numbered subcrates)
- `verify/tests/*.rs` (10 tests; smallest=metadata_oracle 75L, largest=gga_oracle 750L; parity_phase09=733L is direct analog)
- `src/eval/{gga,mgga}_dispatch/*.rs` (auto-generated; sampled batch17.rs)
- `src/kernel/mod.rs` (root family re-export pattern)
- `Cargo.toml` (root + family façades)
- `.cargo/config.toml` (full read — D-08, D-09 invariants)
- `CLAUDE.md` (Constraints section — D-03a target)

**Files scanned:** ~50 files read or grep'd in full or in targeted ranges.

**Pattern extraction date:** 2026-05-13

**Key cross-cutting observations for the planner:**

1. **The chunked-scratch pattern in LDA is the structural skeleton for the new D-02 ABI**, but its three load-bearing details (`&mut Array<f64>`, hardcoded `f64`, LDA-only) are EXACTLY what D-02/D-03 reject. Plans should treat the LDA path as both "what to copy" (file naming, mod-placement, header, import discovery, dispatch wedge in `translate_file_split`) AND "what to delete" (lines 480-852 of `translate_lda_v2.py` are the exact deletion target once the new ABI lands; see RESEARCH.md line 561).

2. **Strategy 1 collapse is mechanically simpler than Strategy 2** — verified by `src/eval/mgga_dispatch/batch17.rs:25` using `crate::kernel::mgga::batch17::...` paths that work UNCHANGED if `batch17` becomes a real `pub mod batch17;` instead of a `pub use libxc_kernel_mgga_17 as batch17;`. RESEARCH.md §"Two collapse strategies" line 322 explicitly recommends Strategy 1 for D-LOCK-A.

3. **The Wave 0 spike is non-negotiable.** A1 (RESEARCH.md line 704) flags the tuple-return `<F: Float>` ABI as HIGH risk if wrong — and the existing `crates/kernels/math/src/br89.rs:38` and `mbrxc.rs:9` comments saying "CubeCL doesn't support tuples" are stale-but-uncontested evidence in the codebase. The spike must land FIRST and PASS before any chunked translator extension begins.

4. **Identity submodule preservation pattern:** When collapsing GGA/MGGA, write a one-line `pub mod batchN;` inside the family façade `lib.rs` for each numbered child to keep dispatch paths working. The LDA façade already uses per-functional `pub use` (lines 14-54 of `crates/kernels/lda/src/lib.rs`) — collapse those into `pub mod <func>;` directly (no `batchN` for LDA — already flat per-functional).

5. **`tools/maple_to_kernels.py` defaults are stale and not enforced** — `DEFAULT_SPLIT_THRESHOLD = 100_000` and `DEFAULT_TARGET_MAX = 500_000` (lines 89-90) are warnings-only; the translator-level constants win (`SPLIT_THRESHOLD = 6000` in each `translate_*.py`). Phase 11 should either align the driver defaults to 5000 AND make the driver mutate translator constants, or remove the unused CLI knobs (RESEARCH.md Open Question #5, line 738).

## PATTERN MAPPING COMPLETE
