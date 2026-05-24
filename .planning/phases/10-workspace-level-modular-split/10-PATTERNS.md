# Phase 10: Workspace-Level Modular Split - Pattern Map

**Mapped:** 2026-05-25 (FORCE-REFRESH — overwrites stale 2026-05-07 map that referenced deleted `crates/kernel-{lda,gga,mgga}*` umbrella crates)
**Files analyzed:** 11 created / 5 modified / 3 deleted (per CONTEXT "What Phase 10 Creates/Modifies/Deletes")
**Analogs found:** 14 / 14 created-or-modified files have an in-repo analog (this is a pure refactor — every target is modeled on something that exists at HEAD `31eb1dc6cb`)
**HEAD verification:** every file:line cited below was read live this session.

> This phase is a PURE REFACTOR. Almost nothing is net-new logic. The "patterns to copy"
> are: (1) the closest existing `Cargo.toml` + `lib.rs` skeletons to model the 3 new crates on,
> (2) mechanical Cargo-graph + path-string edit recipes, and (3) the one cross-crate visibility
> fix. Source-code bodies (`model/`, `eval/`, `compat/`, …) MOVE verbatim via `git mv` — they
> are not re-authored, so there is no "core pattern / error handling" body to copy for them.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/libxc-core/Cargo.toml` (new) | config (leaf rlib) | n/a | `crates/kernels/math/Cargo.toml` + root `Cargo.toml:6-11` | role-match (dep partition) |
| `crates/libxc-core/src/lib.rs` (new) | config (module decls) | n/a | `crates/kernels/math/src/lib.rs:1-23` + root `src/lib.rs:1-21` | exact (module-decl skeleton) |
| `crates/libxc-eval/Cargo.toml` (new) | config (rlib + 306 deps + `[features]`) | n/a | root `Cargo.toml:6-377` (deps + `[features]` block) | exact (whole machinery moves) |
| `crates/libxc-eval/src/lib.rs` (new) | config (module decls) | n/a | root `src/lib.rs:1-21` | exact |
| `crates/libxc-compat/Cargo.toml` (new) | config (cdylib/staticlib) | n/a | `libxc-sys/Cargo.toml` (member shape) + CONTEXT D-07 sample | role-match (no existing cdylib in repo) |
| `crates/libxc-compat/src/lib.rs` (new) | config (module decls) | n/a | root `src/lib.rs:1-21` | exact |
| `crates/libxc-compat/include/xc_rs.h` (new, conditional) | header | n/a | `libxc-master/src/xc.h` (1:1 minus `void→int`) — Phase-6 deliverable | external-mirror (D-09a: only if Phase 6 shipped it) |
| `crates/libxc-core/src/deferred*` (relocated) | model (pure metadata) | lookup | `crates/kernels/math/src/deferred.rs` (move verbatim) | exact (`git mv`, preserve `lda::`/`mgga::` shape) |
| `src/lib.rs` (root, reduced to facade) | config (re-export facade) | n/a | current `src/lib.rs:23-38` (preserve surface, repoint paths) + RESEARCH "Public Surface" | exact (line-for-line strategy (a)) |
| root `Cargo.toml` (deps shrink + members + `[features]` re-forward) | config (workspace) | n/a | `verify/Cargo.toml:13-19` (re-forward pattern) + current `Cargo.toml:652-662` | exact |
| `xtask/src/main.rs` (4 path strings) | utility (codegen emitter) | file-I/O | `xtask/src/main.rs:291,329,355,387` (self — prefix edit) | exact (mechanical) |
| `xtask/src/generate_metadata.rs` (3 path strings) | utility (codegen emitter) | file-I/O | `xtask/src/generate_metadata.rs:445,595,643` (self — prefix edit) | exact (mechanical) |
| `meta/generated_propagation` visibility (in xtask emitter) | model (generated const) | n/a | RESEARCH Pitfall 2 (emitter token OR re-export) | role-match |
| `verify/tests/{lda,mgga}_oracle.rs` (deferred import repoint) | test | lookup | `verify/tests/lda_oracle.rs:36`, `mgga_oracle.rs:44` (self — `use` path swap) | exact |

## Pattern Assignments

### `crates/libxc-core/Cargo.toml` (config, leaf rlib — ZERO cubecl, ZERO kernel deps; SC 2)

**Analog:** `crates/kernels/math/Cargo.toml` (package shape) + root `Cargo.toml:6-11` (the deps to partition IN).

**Package header to copy** (`crates/kernels/math/Cargo.toml:1-4`):
```toml
[package]
name = "libxc-core"      # (analog uses libxc-kernel-math)
version = "0.1.0"
edition = "2024"
```

**Dependency partition** — pull EXACTLY these three from root `Cargo.toml:7,8,10` (NOT line 9's cubecl, NOT line 11's kernel-math — D-11/D-14 forbid cubecl in core):
```toml
[dependencies]
bitflags  = "2.10.0"
bytemuck  = { version = "1.25.0", features = ["derive"] }
thiserror = "2.0.18"
```
> Why these three: `LibxcRsError` needs `thiserror`; `OutputMask`/`FunctionalFlags` need `bitflags`; GPU-byte casts in `output/`/`layout/` need `bytemuck`. cubecl is excluded BY DESIGN (SC 2) — the D-11 `deferred` relocation is the enabling move (it was the only core-bound caller of `libxc_kernel_math::`).

**Verification (SC 2, cheap, no compile):**
```bash
! cargo tree -p libxc-core -e no-dev | grep -qE 'cubecl|libxc-kernel'
```

---

### `crates/libxc-core/src/lib.rs` (config, module-decl skeleton)

**Analog:** `crates/kernels/math/src/lib.rs:1-23` (the `pub mod X;` list shape) + root `src/lib.rs:1` (`#![deny(warnings)]`).

**Skeleton to write** (module names from CONTEXT "What Phase 10 Creates" + the D-11 `deferred` addition):
```rust
#![deny(warnings)]
// (copy the 3 clippy allows from root src/lib.rs:5-7 only if a moved module triggers them;
//  core is cubecl-free so likely needs none — add if a cargo check surfaces a lint.)

pub mod model;
pub mod meta;
pub mod error;
pub mod dims;
pub mod registry;
pub mod input;
pub mod output;
pub mod layout;     // present in src/ today; CONTEXT lists it in the core set
pub mod deferred;   // NEW — relocated from libxc-kernel-math (D-11)
```
> The `pub mod` list mirrors how `crates/kernels/math/src/lib.rs:7-22` declares its 17 submodules. Do NOT add `pub use` item re-exports here — those live in the ROOT facade (`src/lib.rs:23-38`), not in core (the facade's whole job is surface preservation, SC 5).

---

### `crates/libxc-eval/Cargo.toml` (config — cubecl + 306 kernel deps + the WHOLE `[features]` machinery)

**Analog:** root `Cargo.toml` — the deps block (`6-321`) and the `[features]` block (`323-377…645`) MOVE here near-verbatim. This is the highest-risk edit in the phase.

**Package header** (model on `crates/kernels/math/Cargo.toml:1-4`, name `libxc-eval`).

**Dependency partition** — pull from root `Cargo.toml`:
```toml
[dependencies]
libxc-core = { path = "../libxc-core" }                                  # NEW one-way dep
cubecl     = { version = "0.10.0", default-features = false, features = ["cpu"] }  # root:9
bytemuck   = { version = "1.25.0", features = ["derive"] }               # root:8
libxc-kernel-math = { path = "../kernels/math" }                         # root:11 — NON-optional, KEEP non-optional
# … all 305 optional per-functional kernel lines from root:16-321 …
```

**CRITICAL path-prefix rewrite** (RESEARCH Pitfall 3). Root deps are root-relative; from `crates/libxc-eval/` they need `../`:
```
root  Cargo.toml:16:  path = "crates/kernels/gga/gga_c_acgga"
                                ↓ (uniform bulk edit, all 306 lines)
eval  Cargo.toml:     path = "../kernels/gga/gga_c_acgga"
```
(i.e. `crates/kernels/...` → `../kernels/...`; and `crates/kernels/math` → `../kernels/math` for the non-optional line 11.)

**`[features]` block — MOVE VERBATIM** from root `Cargo.toml:323-645` (block starts line 323, `default` line 331, `oracle-lda` 333-377, `oracle-gga` 379-…, `oracle-mgga` 513-…). The `dep:libxc-kernel-*` entries (e.g. `Cargo.toml:334` `"dep:libxc-kernel-hyb_lda_xc_bn05"`) resolve against eval's now-local deps. Keep the explanatory comment block (`Cargo.toml:324-330`) verbatim.
```toml
[features]
default     = ["oracle-lda", "oracle-gga", "oracle-mgga"]   # root:331
oracle-lda  = [ "dep:libxc-kernel-…", … ]                   # root:333-377 verbatim
oracle-gga  = [ … ]                                          # root:379-…
oracle-mgga = [ … ]                                          # root:513-…
```

**Failure-mode checklist** (RESEARCH "Feature-Forwarding Chain" — all `cargo tree`, no compile, no OOM):
```bash
cargo tree -p libxc-eval -e no-dev --no-default-features        # should show ONLY core + kernel-math
cargo tree -p libxc-eval -e no-dev | grep -c libxc-compat       # MUST be 0 (SC 3)
cargo tree -p libxc-eval -e no-dev | grep libxc-core            # MUST be present (SC 3)
```
- `libxc-kernel-math` must stay NON-optional (it has no `optional = true` today, root:11).
- The 305 per-functional lines keep `optional = true`.
- Keep `dep:` prefix on every `[features]` entry (prevents implicit-feature collision).

---

### `crates/libxc-eval/src/lib.rs` (config, module-decl skeleton)

**Analog:** root `src/lib.rs:1-7` (attrs) + the `eval/functional/kernel/workspace` mod set.

**Skeleton:**
```rust
#![deny(warnings)]
#![allow(clippy::excessive_precision)]    // root src/lib.rs:5 — CubeCL macro expansion triggers these
#![allow(clippy::needless_late_init)]     // root:6
#![allow(clippy::too_many_arguments)]     // root:7

pub mod eval;
pub mod functional;
pub mod kernel;
pub mod workspace;   // top-level placeholder (dead, zero consumers) — distinct from eval::workspace
```
> Copy ALL THREE clippy allows here (unlike core) — eval owns the `kernel/` dispatch glue whose CubeCL `#[cube]` macro expansion is exactly what root `src/lib.rs:2-4` documents these allows for.
> NAMING TRAP (RESEARCH): `pub mod workspace;` here = the dead top-level `src/workspace/` placeholder. The LIVE `EvaluationWorkspace` is `eval::workspace` (declared inside `eval/mod.rs:13`, moves with `eval/`). Two different modules — do not conflate or drop the live one.

---

### `crates/libxc-compat/Cargo.toml` (config — cdylib + staticlib; D-07/D-08)

**Analog:** No existing cdylib crate in the repo (this is the one genuinely-new artifact shape). Closest member-shape analog is `libxc-sys/Cargo.toml` (a small standalone member). The `[lib] crate-type` triple is from CONTEXT D-07 (`<specifics>` lines 254-269).

**Full Cargo.toml to write:**
```toml
[package]
name = "libxc-compat"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["rlib", "cdylib", "staticlib"]
# NO `name =` override → default crate name "libxc_rs" → libxc_rs.so / libxc_rs.a / libxc_rs.rlib (D-08)
# NO [bin] target (D-07)

[dependencies]
libxc-core = { path = "../libxc-core" }
libxc-eval = { path = "../libxc-eval" }
thiserror  = "2.0.18"   # root Cargo.toml:10 pin
```
> SC 4: nothing depends on libxc-compat. It is EXCLUDED from `[workspace] default-members` (D-10a — its cdylib links all 306 kernels → OOM at jobs=1). Build on demand: `cargo build -p libxc-compat`.

---

### `crates/libxc-compat/src/lib.rs` (config, module-decl skeleton)

**Analog:** root `src/lib.rs:1-7` attrs. The `compat/` directory moves as a unit (`c_layout, errno, ids, legacy_eval, macros, mod, raw_handle, removed` — already correctly partitioned by Phase 6).

**Skeleton:** declare the moved `compat` submodules at the crate root (the planner picks whether to keep a `compat` module wrapper or flatten — moving `src/compat/mod.rs` → `crates/libxc-compat/src/lib.rs` and re-rooting its `pub mod`s is the lowest-churn option). Copy `#![deny(warnings)]` + the 3 clippy allows (compat re-exports eval types whose CubeCL-derived signatures can trip `too_many_arguments`).

---

### `crates/libxc-core/src/deferred*` (relocated module — D-11; the ONE intentional kernel-crate touch)

**Analog:** `crates/kernels/math/src/deferred.rs` — `git mv` it verbatim into libxc-core; delete the `pub mod deferred;` line from `crates/kernels/math/src/lib.rs:22`.

**PRESERVE the two-submodule shape** (RESEARCH Pitfall 6 — `deferred.rs:15` `pub mod lda {`, `:109` `pub mod mgga {`, each with `pub fn is_deferred(id: u16) -> bool` at `:77` / `:186`):
```rust
// crates/kernels/math/src/deferred.rs (head — moves verbatim, EXCEPT the //! provenance):
pub mod lda  { /* … pub struct DeferredLda; pub fn is_deferred(id: u16) -> bool { … } */ }
pub mod mgga { /* … pub fn is_deferred(id: u16) -> bool { … } */ }
```
> Update the `//!` header (`deferred.rs:1-13`) — it currently says "Relocated here from the per-family façade crates … the root crate's model layer depends on it too, so this is the natural home." Phase 10 REVERSES that premise (model → core, core must NOT dep kernel-math), so note the Phase-10 relocation in the header.

**Consumer repoint** (4 sites, all verified at HEAD):
```rust
// src/model/lda_functional.rs:80  (model/ now in libxc-core → crate-local):
//   BEFORE: if libxc_kernel_math::deferred::lda::is_deferred(id.raw()) {
//   AFTER:  if crate::deferred::lda::is_deferred(id.raw()) {
// src/model/mgga_functional.rs:43  (use alias):
//   BEFORE: use libxc_kernel_math::deferred::mgga::is_deferred as is_deferred_mgga;
//   AFTER:  use crate::deferred::mgga::is_deferred as is_deferred_mgga;
// verify/tests/lda_oracle.rs:36:
//   BEFORE: use libxc_kernel_math::deferred::lda::is_deferred;
//   AFTER:  use libxc_rs::deferred::lda::is_deferred;   (via facade re-export — see root lib.rs)
// verify/tests/mgga_oracle.rs:44:
//   BEFORE: use libxc_kernel_math::deferred::mgga::is_deferred as is_deferred_mgga;
//   AFTER:  use libxc_rs::deferred::mgga::is_deferred as is_deferred_mgga;
```
> Also: doc-comment lines reference the old path — `src/model/lda_functional.rs:13`, `src/model/mgga_functional.rs:30`. Update for accuracy (non-load-bearing).
> Open Q (RESEARCH): after this move, `verify/Cargo.toml:41`'s `libxc-kernel-math` dev-dep ("for the deferred registry") may be droppable — grep verify for any OTHER `libxc_kernel_math::` symbol before removing.

---

### `src/lib.rs` (root, reduced to thin facade — SC 5 surface preservation)

**Analog:** the CURRENT `src/lib.rs:23-38` IS the spec. Strategy (a) (CONTEXT-recommended): preserve the item re-export list line-for-line, repointing `model::`→`libxc_core::model::`, `eval::`→`libxc_eval::eval::`, etc. `api/` stays local.

**Attrs to keep** (`src/lib.rs:1-7`): `#![deny(warnings)]` + the 3 clippy allows (verbatim).

**Module-path re-exports** (preserve the `pub mod` namespace surface from `src/lib.rs:9-21` — these paths are PART of the API):
```rust
pub mod api;   // stays LOCAL (references BOTH core and eval — see below)

pub use libxc_core::{model, meta, error, dims, registry, input, output};  // + layout if it was pub
pub use libxc_eval::{eval, functional, kernel, workspace};
pub use libxc_core::deferred;     // NEW (D-11) — verify *_oracle.rs route through this
pub use libxc_compat as compat;   // preserves libxc_rs::compat::* (RESEARCH Open Q 1 — re-export to be safe)
// pub use libxc_kernel_math as math;  // OMIT — D-02 deletes src/math, zero consumers found (D-02a)
```

**Item re-exports** — repoint each line of current `src/lib.rs:23-38`:
```rust
pub use libxc_core::model::{                       // was: pub use model::{   (src/lib.rs:23)
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
    LdaFunctional, GgaFunctional, MggaFunctional,
};
pub use libxc_core::meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};   // src/lib.rs:28
pub use libxc_core::error::LibxcRsError;                                           // :29
pub use libxc_core::dims::Dimensions;                                             // :30
pub use libxc_core::registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string}; // :31
pub use libxc_core::input::{LdaInput, GgaInput, MggaInput};                       // :32
pub use libxc_core::output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};       // :33
pub use libxc_eval::eval::{dispatch_lda, dispatch_gga, dispatch_mgga};            // :34
pub use libxc_eval::functional::{                                                 // :35
    classify_hybrid, CamCoefficients, Functional, FunctionalParams, NlcCoefficients, NoParams,
};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};                  // :38 — local
```

**`api/` path repoint** (RESEARCH "api/ dual-dependency fact"): `src/api/{batch,builder,evaluate}.rs` reference `crate::error/model/input/output/registry` (→ now `libxc_core::`) AND `crate::eval::workspace::EvaluationWorkspace` + `crate::functional::Functional` (→ now `libxc_eval::`). Both libxc-core AND libxc-eval must be root direct deps. `api/` has ZERO `crate::compat` refs — so root needs a libxc-compat dep ONLY for the surface `pub use libxc_compat as compat;` re-export, not for `api/` to compile (SC-4-friendly).

**SC 5 verification (cheap, per-`-p`):**
```bash
cargo check -p libxc_rs --lib -j1     # umbrella, ~536 MB peak per 11-14 — NOT --workspace (OOMs)
```

---

### root `Cargo.toml` (deps shrink + members + `[features]` re-forward)

**Analog:** `verify/Cargo.toml:13-19` is the EXACT re-forward pattern to copy; current `Cargo.toml:652-662` is the members/default-members block to amend.

**`[dependencies]` shrinks** to the facade set (cubecl/bitflags/kernel deps move OUT to leaf crates):
```toml
[dependencies]
libxc-core   = { path = "crates/libxc-core" }
libxc-eval   = { path = "crates/libxc-eval", default-features = false }  # ← default-features=false is LOAD-BEARING
libxc-compat = { path = "crates/libxc-compat" }   # only for `pub use libxc_compat as compat;` surface
```

**`[features]` re-forward block** — copy the SHAPE from `verify/Cargo.toml:15-19` (which forwards to `libxc_rs/oracle-*`); root forwards one level down to `libxc-eval/oracle-*`:
```toml
[features]
default     = ["oracle-lda", "oracle-gga", "oracle-mgga"]   # mirror verify/Cargo.toml:16
oracle-lda  = ["libxc-eval/oracle-lda"]                     # mirror verify/Cargo.toml:17 (s/libxc_rs/libxc-eval/)
oracle-gga  = ["libxc-eval/oracle-gga"]
oracle-mgga = ["libxc-eval/oracle-mgga"]
```
> RESEARCH Pitfall 1 — the `default-features = false` on the `libxc-eval` dep line is MANDATORY. Without it, eval's own `default = [oracle-lda,oracle-gga,oracle-mgga]` re-fires regardless of verify's `--no-default-features` request → all 306 compile → OOM.
> verify/Cargo.toml needs ZERO feature-forward changes: it still says `libxc_rs/oracle-lda` (`verify/Cargo.toml:17`); root now relays that to `libxc-eval/oracle-lda`.

**`[workspace] members`** (`Cargo.toml:652-656`) — ADD the 3 new crates; PRESERVE `verify-canary` (RESEARCH Pitfall 5 — it postdates CONTEXT, not in CONTEXT's D-10b list):
```toml
[workspace]
members = [ "xtask", "verify", "verify-canary", "libxc-sys",
            "crates/libxc-core", "crates/libxc-eval", "crates/libxc-compat" ]
```

**`[workspace] default-members`** (`Cargo.toml:662-…`) — append `crates/libxc-core`, `crates/libxc-eval`, root `.` to the existing kernel enumeration; EXCLUDE `crates/libxc-compat` (D-10a, OOM). PRESERVE the explanatory comment block (`Cargo.toml:657-661`) and the 7-deferred-kernel exclusion VERBATIM. `default-members ⊆ members` must hold.

**Boundary proofs (SC 2/3/4, cheap):**
```bash
cargo tree -p libxc_rs --no-default-features --features oracle-lda  | grep -c libxc-kernel   # 43 LDA only
cargo tree -p libxc_rs --no-default-features --features oracle-mgga | grep -c libxc-kernel   # MGGA incl _pK shards
cargo tree -p libxc-compat -e no-dev | grep -E 'libxc-(core|eval)'                           # both present (SC 4)
```

---

### `xtask/src/main.rs` + `xtask/src/generate_metadata.rs` (utility, file-I/O — D-03 mechanical edits)

**Analog:** the files themselves (self-modeled). 7 hard-coded `root.join("src/...")` strings get a `crates/libxc-core/` prefix. The `root` arg is the workspace/output root (`find_output_root()`, `main.rs:264-274`, walks up to the dir containing top-level `Cargo.toml`) — D-06a leaves that walking-up logic UNCHANGED.

**`xtask/src/main.rs`** (4 edits — verified `:291,329,355,387`):
```rust
// :291  let path = root.join("src/meta/generated.rs");      → root.join("crates/libxc-core/src/meta/generated.rs");
// :329  let path = root.join("src/registry/by_id.rs");      → root.join("crates/libxc-core/src/registry/by_id.rs");
// :355  let path = root.join("src/registry/by_name.rs");    → root.join("crates/libxc-core/src/registry/by_name.rs");
// :387  let path = root.join("src/registry/removed.rs");    → root.join("crates/libxc-core/src/registry/removed.rs");
```

**`xtask/src/generate_metadata.rs`** (3 edits — verified `:445,595,643`):
```rust
// :445  let path = root.join("src/meta/generated.rs");             → crates/libxc-core/src/meta/generated.rs
// :595  let path = root.join("src/meta/generated_hybrid.rs");      → crates/libxc-core/src/meta/generated_hybrid.rs
// :643  let path = root.join("src/meta/generated_propagation.rs"); → crates/libxc-core/src/meta/generated_propagation.rs
```
> No xtask LOGIC change (D-03/D-06a). xtask/Cargo.toml gets NO path-dep on libxc-core (D-06) — preserve the NOTE comment (`xtask/Cargo.toml:14-18`) verbatim.
> RUNTIME-ORDER TRAP (RESEARCH "Runtime State Inventory"): the D-03 path edits MUST land in the SAME plan/commit as the `meta/` + `registry/` move (the 10-01 core plan). If `cargo xtask` runs post-move but pre-edit, it regenerates into the now-empty root `src/meta/` — silent stale orphans.

---

### `meta::generated_propagation` visibility (the ONE confirmed cross-crate break)

**Analog:** RESEARCH Pitfall 2 — two survivable options. This is an xtask-EMITTED file (`generate_metadata.rs:643`), so a plain in-file edit gets reverted on next regen.

**The break** (verified):
- `src/meta/mod.rs:3` — `pub(crate) mod generated_propagation;`
- `src/meta/generated_propagation.rs:8` — `pub(crate) const PROPAGATION_RULES: &[PropagationRule] = …;`
- Consumed cross-crate by `src/functional/lifecycle.rs:13` (`use crate::meta::generated_propagation::PROPAGATION_RULES;`) which moves to **libxc-eval**.

**Two fix options** (planner picks; option B is regen-proof and preferred):
- **A — patch the emitter token:** change the `pub(crate)` the xtask writes for this file (in `generate_metadata.rs`) to `pub`. Risk: must touch the emitter, not just the file (Pitfall 2).
- **B — hand-written re-export (survives regen):** keep the generated const `pub(crate)`, and add to `src/meta/mod.rs` (which becomes `crates/libxc-core/src/meta/mod.rs`):
  ```rust
  pub use generated_propagation::PROPAGATION_RULES;   // re-export makes it cross-crate reachable
  ```
  then eval imports `libxc_core::meta::PROPAGATION_RULES` (or `libxc_eval`'s `crate::meta` if routed via facade). Also bump `pub(crate) mod` → `pub mod` for the module path if eval imports the module path rather than the re-exported symbol.
> Everything else audited is already `pub`: `registry::all_functional_ids` (`registry/mod.rs:72`), `eval::workspace` (`eval/mod.rs:13`), `Functional`/`meta()` (`functional/mod.rs:30,53`), `FunctionalMeta` (`meta/mod.rs:51`). `meta::generated`/`generated_hybrid` stay `pub(crate)` — consumed only within core. Net visibility work = ~2 edits.

## Shared Patterns

### Edition + lint header (all 3 new `lib.rs` + moved bodies)
**Source:** root `src/lib.rs:1-7`; `crates/kernels/math/src/lib.rs:1-5`
**Apply to:** every new crate `lib.rs`.
```rust
#![deny(warnings)]
#![allow(clippy::excessive_precision)]   // CubeCL #[cube] macro expansion (eval/compat); core may omit
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]
```
- Edition `2024` (MSRV 1.85+) in every `[package]` — copy from any analog `Cargo.toml:4`.
- libxc-core MAY drop the 3 clippy allows (cubecl-free); add back if a `cargo check -p libxc-core` lint appears.

### Dependency-version pins (D-14 — already at target in root; MOVE not bump)
**Source:** root `Cargo.toml:7-10`
**Apply to:** the per-crate partition.
```toml
bitflags  = "2.10.0"                                                     # core
bytemuck  = { version = "1.25.0", features = ["derive"] }                # core + eval
thiserror = "2.0.18"                                                     # core + compat
cubecl    = { version = "0.10.0", default-features = false, features = ["cpu"] }  # eval only
```

### Feature re-forward chain (the 4-link mechanic)
**Source:** `verify/Cargo.toml:13-19` (links 1-2) + root `Cargo.toml:323-377…` (links 3-4)
**Apply to:** root facade `[features]` (forwards `oracle-*` → `libxc-eval/oracle-*`) + the `default-features = false` pin on the root's `libxc-eval` dep.
```
verify ──libxc_rs/oracle-lda──▶ root ──libxc-eval/oracle-lda──▶ eval ──dep:libxc-kernel-lda_x──▶ kernel
```
Get any link wrong → either `--no-default-features` stops resolving (missing forward) or all 306 compile (missing `default-features=false`).

### `git mv` per-directory move (preserve blame; CONTEXT-recommended)
**Apply to:** every moved subtree (`model/ meta/ registry/ input/ output/ layout/ dims/ error/` → core; `eval/ functional/ kernel/ workspace/` → eval; `compat/` → compat; `crates/kernels/math/src/deferred.rs` → core). Each commit must leave a per-`-p` `cargo check` green (RESEARCH Pitfall 4 — `--workspace` OOMs; NEVER use it as a gate).

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `crates/libxc-compat/Cargo.toml` `[lib] crate-type = ["rlib","cdylib","staticlib"]` | config | n/a | No existing cdylib/staticlib crate in the repo. The triple is from CONTEXT D-07 (`<specifics>:254-269`), not an in-repo analog. Member-shape modeled on `libxc-sys/Cargo.toml`; the crate-type stanza is novel. |
| `crates/libxc-compat/include/xc_rs.h` | header | n/a | Hand-written, mirrors `libxc-master/src/xc.h` 1:1 minus `void→int` (D-09). It is a Phase-6 deliverable; Phase 10 only RELOCATES it (D-09a) — write only if Phase 6 has not committed it by execution time. No Rust-side analog. |

## Metadata

**Analog search scope:** `crates/kernels/math/`, `libxc-sys/`, `verify/`, `verify-canary/`, `xtask/`, root `Cargo.toml`, root `src/lib.rs`, `src/meta/`, `src/functional/lifecycle.rs`, `src/model/{lda,mgga}_functional.rs`, `verify/tests/{lda,mgga}_oracle.rs`, `crates/kernels/math/src/deferred.rs`.
**Files scanned (read or grepped at HEAD):** 16.
**HEAD:** `31eb1dc6cb`; kernel-dep count `grep -cE "^libxc-kernel-" Cargo.toml` = **306** (do not hardcode in plans — re-derive).
**Pattern extraction date:** 2026-05-25.
**Note on a CONTEXT/RESEARCH drift:** both cite `find_workspace_root()` at `xtask/src/main.rs:265-278`; the LIVE function is `find_output_root()` (`main.rs:264-274`) walking up to the top-level `Cargo.toml`. Semantics are identical (workspace root). D-06a "unchanged" still holds — just under the live function name.
