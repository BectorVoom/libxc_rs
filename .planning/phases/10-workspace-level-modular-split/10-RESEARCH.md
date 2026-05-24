# Phase 10: Workspace-Level Modular Split — Research

**Researched:** 2026-05-25
**Domain:** Cargo workspace topology / crate-boundary refactor / feature-forwarding mechanics
**Confidence:** HIGH (all claims verified against the live tree at HEAD `31eb1dc6cb`)

> **SUPERSEDES the 2026-05-07 version.** That research described a kernel layer of
> `crates/kernel-{lda,gga,mgga}*` umbrella crates + 4 façade path-deps. **Those crates
> no longer exist** — Phase 11's D-10a clean-slate restructure (now COMPLETE, 2026-05-25)
> replaced them with **306 per-functional kernel crates** under `crates/kernels/{lda,gga,mgga,math}/`,
> and Phase 11-12 made every kernel dep `optional = true` behind `[features] oracle-{lda,gga,mgga}`.
> Do NOT trust the old research's kernel-crate paths or its "4 umbrella deps" claim.
> The current-topology facts below were re-derived live (`grep`, file reads, `git log`).

## Summary

Phase 10 is a pure mechanical refactor: lift `src/` into three layered crates (`libxc-core` ← `libxc-eval` ← `libxc-compat`) plus a thin root `libxc_rs` facade, enforced by compiler crate boundaries. **No new logic, no kernel restructure** (one surgical exception: the `deferred` registry moves from `libxc-kernel-math` to `libxc-core`, D-11).

The single highest-leverage finding is the **feature-forwarding chain**. Since Phase 11-12, the root `Cargo.toml` carries all 306 kernel path-deps as `optional = true`, gated behind a `[features]` block (`default = ["oracle-lda","oracle-gga","oracle-mgga"]`, each `oracle-*` activating `dep:libxc-kernel-*`). `verify/` consumes the root via `libxc_rs = { path = "..", default-features = false }` and re-forwards `oracle-lda = ["libxc_rs/oracle-lda"]`. **D-10 moves all 306 kernel deps + the entire `[features]` block from root into `crates/libxc-eval/Cargo.toml`, and the root facade must re-forward each `oracle-*` feature to `libxc-eval` so verify's `default-features = false` + `oracle-*` forward chain stays intact.** Get this wrong (a non-optional kernel left in eval, or a missing root→eval feature forward) and either verify's chunked builds OOM or `--no-default-features` stops resolving.

Visibility breakage is minimal: the only confirmed `pub(crate)`-crosses-crate item is `meta::generated_propagation::PROPAGATION_RULES` (consumed by `functional/lifecycle.rs`, which moves to libxc-eval). Everything else `api/`, `compat/`, and eval-side already reach is `pub`. The refactor is bisectable per-crate; **workspace-wide `cargo check` OOMs on this box, so per-`-p` checks are the only viable gate** (D-13's `--workspace` precondition must be reframed as per-crate).

**Primary recommendation:** Sequence as core→eval→compat→root-facade. Move the `[features]` machinery as a unit with the kernel deps into libxc-eval, add a verbatim re-forwarding `[features]` block to the root facade, and verify SC 2/3/4 with `cargo tree -p <crate>` (zero compile, zero OOM risk).

<user_constraints>
## User Constraints (from CONTEXT.md)

> Decisions D-01..D-09a are LOCKED. The "Restructure Update (2026-05-21)" + D-10..D-14
> capture the post-Phase-11 topology delta and SUPERSEDE older lines. Where an older line
> conflicts with a higher-numbered decision, the higher number wins.

### Locked Decisions (verbatim summary — see 10-CONTEXT.md for full text)

- **D-01 / D-01a / D-01b** — `src/error/` (mod.rs 382 lines + 3 stubs `ffi.rs`/`internal.rs`/`public.rs`) moves into `crates/libxc-core/src/error/`. The FFI errno layer (thread-local `RefCell<Option<CString>>`, `xc_rs_last_error_*` accessors, `extern_c_wrapper!`) stays in libxc-compat. Only the typed `LibxcRsError` enum crosses into core. Empty stubs may be deleted in the same plan.
- **D-02 / D-02a** — DELETE `src/math/mod.rs` (12-line dead `pub use libxc_kernel_math::{...}` shim; zero `crate::math` callsites in `src/`, zero `libxc_rs::math` in verify/ — both verified). Check verify/benches/root re-exports for `libxc_rs::math::*` first; none found.
- **D-03 / D-04** — xtask writes generated output directly into `crates/libxc-core/src/...`. 7 hard-coded path strings update (4 in `xtask/src/main.rs:291,329,355,387`; 3 in `xtask/src/generate_metadata.rs:445,595,643`) — prefix `crates/libxc-core/`. No generated file crosses the core/eval/compat boundary.
- **D-05** — No pre-emptive multi-target xtask abstraction. `src/compat/ids.rs` stays a stub.
- **D-06 / D-06a** — xtask stays a string emitter, NO path-dep on libxc-core. `find_workspace_root()` walking-up logic unchanged.
- **D-07** — `crates/libxc-compat/Cargo.toml` declares `[lib] crate-type = ["rlib","cdylib","staticlib"]`. No `[bin]`.
- **D-08** — cdylib name = `libxc_rs` (Rust default, no override) → `libxc_rs.so`/`.a`/`.rlib`. Drop-in is source-level, not binary-level.
- **D-09 / D-09a** — C header hand-written at `crates/libxc-compat/include/xc_rs.h`, committed; mirrors `libxc-master/src/xc.h` 1:1 minus the `void → int` Phase-6 signature changes. Ships only if Phase 6 produced it before Phase 10 starts; Phase 10 does not block on writing one.
- **D-10** — All 306 `libxc-kernel-*` path-deps migrate root `Cargo.toml [dependencies]` → `crates/libxc-eval/Cargo.toml`. Root depends on NO kernel crate directly; reaches them transitively via `libxc_rs → libxc-eval → libxc-kernel-*`. Derive count via `grep -cE "^libxc-kernel-" Cargo.toml` (never hardcode). **Live count = 306.**
- **D-10a** — `[workspace] default-members` = per-functional kernel enumeration MINUS the 7 D-11-deferred kernels (`mgga_c_b94, mgga_x_br89, mgga_x_mbr, mgga_x_mbrxc_bg, mgga_x_mbrxh_bg, mgga_x_mggac, mgga_x_br89_explicit`) + `crates/libxc-core` + `crates/libxc-eval` + root `libxc_rs`. **`crates/libxc-compat` EXCLUDED** (its cdylib links all kernels → OOM at jobs=1; build on demand `cargo build -p libxc-compat`). Preserve Phase-11's exclusion list AND explanatory comment block VERBATIM.
- **D-10b** — Current `[workspace] members` = `xtask, verify, verify-canary, libxc-sys`; kernels are implicit members via path-deps. Phase 10 makes libxc-core/eval/compat members. `default-members ⊆ members` must hold.
- **D-11** — Relocate `crates/kernels/math/src/deferred.rs` (id-table + `is_deferred`) OUT of libxc-kernel-math INTO libxc-core. Update 4 consumers. kernel-math deletes its `deferred` module. **Required because** model/ → libxc-core and kernel-math deps cubecl; keeping the call would pull CubeCL into core and break SC 2.
- **D-12 / D-12a** — `libxc-sys` (bindgen+cmake oracle FFI) is standalone OUTSIDE the layering; untouched by Phase 10. verify/ keeps its curated ~17-kernel dev-dep subset (do NOT expand to 306 — OOM). verify routes through root facade.
- **D-13** — EXECUTION was hard-blocked on Phase 11.1 reaching workspace-green. **Phase 11 is now COMPLETE (2026-05-25) — gate LIFTED.** The per-commit GREEN invariant is retained, but reframed: `cargo check --workspace` OOMs here, so the realistic precondition is a **per-`-p` / per-crate check**, not `--workspace` (see Validation Architecture).
- **D-14** — New crates use current pins: `cubecl 0.10.0` (default-features=false, features=["cpu"]), `bitflags 2.10.0`, `bytemuck 1.25.0` (features=["derive"]), `thiserror 2.0.18`. libxc-core: NO cubecl, needs bitflags+bytemuck+thiserror. libxc-eval: cubecl+bytemuck+all kernel deps+libxc-core. libxc-compat: thiserror+libxc-core+libxc-eval.

### Claude's Discretion

- Plan decomposition across 3 plans (10-01 core, 10-02 eval, 10-03 compat+facade). Sequence core→eval→compat→root.
- Root-facade re-export curation: (a) explicit line-for-line matching `src/lib.rs:23-38`; (b) blanket `pub use ...::*`; (c) split-by-module. **CONTEXT recommends (a).**
- Whether verify/integration tests re-point to libxc-core directly or stay through root (recommend stay-through-root).
- Workspace `[default-members]` post-split mechanics.
- `src/main.rs` "Hello, world!" disposition (delete or leave).
- `git mv` (recommended, preserves blame) vs copy-then-delete.
- Where `src/error/{ffi,internal,public}.rs` stubs go (libxc-core if non-empty, delete if empty).
- Exact per-crate Cargo.toml dep partitioning.
- Whether root adds any `[lib] crate-type` override (recommend NO — root stays rlib-only).

### Deferred Ideas (OUT OF SCOPE)

- Pre-emptive multi-target xtask abstraction (D-05); `libxc-codegen` crate (D-06); xtask path-dep on libxc-core.
- Phase directory rename; `libxc.so` binary-drop-in name (D-08 alt); cbindgen-generated header (D-09 alt); `libxc-error` micro-crate (D-01 alt).
- Splitting any kernel sub-crate further; adding new functionals/kernels; performance benchmarks; binary drop-in with system libxc.
- `[default-members]` glob (Cargo doesn't support it); libxc-core staticlib / libxc-eval cdylib targets.
</user_constraints>

## Architectural Responsibility Map

| Capability | Primary Crate | Secondary | Rationale |
|------------|--------------|-----------|-----------|
| Functional metadata, registry, IDs | `libxc-core` | — | Pure data; no compute. `model/meta/registry/input/output/layout/dims/error`. |
| Deferred-id predicate (`is_deferred`) | `libxc-core` | — | Pure metadata (which ids are not-yet-translated). Relocated from kernel-math (D-11) to keep core CubeCL-free. |
| Typed error (`LibxcRsError`) | `libxc-core` | — | Depends only on `model::{DerivativeOrder,Family,FunctionalId,Spin}` — all core types. |
| Eval orchestration, dispatch, workspace buffers | `libxc-eval` | `libxc-core` | `eval/functional/kernel/workspace`; one-way dep on core; owns the 306 kernel deps + CubeCL. |
| Kernel launch glue (`launch_unchecked`, dispatch arms) | `libxc-eval` | kernel crates | `src/kernel/{mod,lda,gga,mgga,launch}.rs` references 269 distinct kernel crates directly. |
| extern "C" FFI shim, errno, opaque handles | `libxc-compat` | `libxc-eval`+`libxc-core` | The ~85 entry points, `FunctionalSlot`, thread-local errno, `extern_c_wrapper!`, cdylib output. Nothing depends on it (SC 4). |
| High-level Rust API (`BatchEvaluator`, `FunctionalBuilder`) | root `libxc_rs` facade | `libxc-eval`+`libxc-core` | `api/` stays in root; reaches BOTH core (`model/input/output/error/registry`) AND eval (`eval::workspace`, `functional`). |
| Oracle FFI (libxc-master C source) | `libxc-sys` | — | Standalone, OUTSIDE layering, verify-only (D-12). Untouched. |

## Standard Stack

### Core (production crates, verified pins)
| Crate | Version | Used By | Verified Source |
|-------|---------|---------|-----------------|
| cubecl | 0.10.0 (default-features=false, features=["cpu"]) | libxc-eval, libxc-compat(transitive) | `[VERIFIED: root Cargo.toml:9]` |
| bitflags | 2.10.0 | libxc-core (OutputMask, FunctionalFlags) | `[VERIFIED: root Cargo.toml:7]` |
| bytemuck | 1.25.0 (features=["derive"]) | libxc-core, libxc-eval | `[VERIFIED: root Cargo.toml:8]` |
| thiserror | 2.0.18 | libxc-core (LibxcRsError), libxc-compat | `[VERIFIED: root Cargo.toml:10]` |

> Note: root `Cargo.toml` already pins these at the D-14 versions — the partition is a *move*, not a version bump. `[VERIFIED: live read 2026-05-25]`

### Per-crate dependency partition (concrete)

```toml
# crates/libxc-core/Cargo.toml  — ZERO cubecl, ZERO kernel deps (SC 2)
[dependencies]
bitflags  = "2.10.0"
bytemuck  = { version = "1.25.0", features = ["derive"] }
thiserror = "2.0.18"

# crates/libxc-eval/Cargo.toml  — cubecl + 306 kernel deps + the [features] block + core
[dependencies]
libxc-core      = { path = "../libxc-core" }
cubecl          = { version = "0.10.0", default-features = false, features = ["cpu"] }
bytemuck        = { version = "1.25.0", features = ["derive"] }
libxc-kernel-math = { path = "../kernels/math" }        # NON-optional (always needed)
# … all 305 optional per-functional kernel deps (see Feature-Forwarding Chain) …
[features]
default     = ["oracle-lda","oracle-gga","oracle-mgga"]
oracle-lda  = [ "dep:libxc-kernel-…", … ]   # moved verbatim from root
oracle-gga  = [ … ]
oracle-mgga = [ … ]

# crates/libxc-compat/Cargo.toml
[lib]
crate-type = ["rlib","cdylib","staticlib"]   # default name → libxc_rs.so / .a / .rlib
[dependencies]
libxc-core = { path = "../libxc-core" }
libxc-eval = { path = "../libxc-eval" }
thiserror  = "2.0.18"

# root libxc_rs/Cargo.toml  — facade; NO kernel deps; re-forwards oracle-* to eval
[dependencies]
libxc-core   = { path = "crates/libxc-core" }
libxc-eval   = { path = "crates/libxc-eval", default-features = false }
libxc-compat = { path = "crates/libxc-compat" }   # only if api/ or re-exports need it
[features]
default     = ["oracle-lda","oracle-gga","oracle-mgga"]
oracle-lda  = ["libxc-eval/oracle-lda"]
oracle-gga  = ["libxc-eval/oracle-gga"]
oracle-mgga = ["libxc-eval/oracle-mgga"]
```

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `dep:libxc-kernel-*` syntax in features | Implicit optional-dep feature (no `dep:`) | `dep:` prevents an implicit feature of the same name leaking; the live tree already uses `dep:` — keep it. |
| Re-forward features through root facade | Make verify dep `libxc-eval` directly | Would bypass the facade's whole point (SC 5 path preservation) and require verify test-source edits. Reject. |
| Root depends on libxc-compat | Root only core+eval | Only add a libxc-compat *build* dep to root if a re-export needs a compat type. `api/` does NOT use `crate::compat` (verified). A `pub use libxc_compat as compat;` surface re-export still requires the dep, but it's surface-only. |

## Feature-Forwarding Chain (HIGHEST-RISK MECHANIC)

This is the single most error-prone part of the refactor. The chain has FOUR links, all verified live:

```
verify/Cargo.toml                 libxc_rs (root facade)        libxc-eval                 libxc-kernel-*
─────────────────                 ──────────────────────        ──────────                 ──────────────
libxc_rs = { path="..",           [features]                    [features]                 optional = true
  default-features = false }      default = [oracle-*]          default = [oracle-*]       deps, activated
[features]                        oracle-lda =                  oracle-lda = [             only via dep:
oracle-lda =                  ──> ["libxc-eval/oracle-lda"] ──> "dep:libxc-kernel-lda_x", ──> resolved
  ["libxc_rs/oracle-lda"]                                       "dep:…", … ]
```

### Current state (root, pre-move) — `[VERIFIED: Cargo.toml:323-645, verify/Cargo.toml:13-19]`

- **Root `Cargo.toml`:**
  - `libxc-kernel-math` is **NON-optional** (line 11) — always pulled.
  - All 305 per-functional kernel deps are `{ path = "...", optional = true }`.
  - `[features]` block: `default = ["oracle-lda","oracle-gga","oracle-mgga"]`; each `oracle-<fam>` is a list of `"dep:libxc-kernel-<name>"` entries (43 LDA, ~131 GGA, ~131 MGGA incl. all `_pK` shards).
- **verify/Cargo.toml:** `libxc_rs = { path = "..", default-features = false }`; `[features] default = [oracle-*]; oracle-lda = ["libxc_rs/oracle-lda"]; …`. Plus a curated ~17 individual kernel **dev-deps** + `libxc-kernel-math` dev-dep (NOT family crates — OOM avoidance).

### The migration (D-10) — what MUST move and HOW

1. **Move all 306 `libxc-kernel-*` lines** from root `[dependencies]` → `libxc-eval/[dependencies]`, preserving the `optional = true` markers on the 305 and keeping `libxc-kernel-math` NON-optional. Fix path prefixes: root has `path = "crates/kernels/..."`; from `crates/libxc-eval/` it becomes `path = "../kernels/..."`.
2. **Move the entire `[features]` block** (`default` + `oracle-lda` + `oracle-gga` + `oracle-mgga`, lines 323-645) verbatim into `libxc-eval/Cargo.toml`. The `dep:libxc-kernel-*` references resolve against libxc-eval's now-local deps.
3. **Add a re-forwarding `[features]` block to the root facade** so the root's public feature surface is unchanged:
   ```toml
   [features]
   default     = ["oracle-lda","oracle-gga","oracle-mgga"]
   oracle-lda  = ["libxc-eval/oracle-lda"]
   oracle-gga  = ["libxc-eval/oracle-gga"]
   oracle-mgga = ["libxc-eval/oracle-mgga"]
   ```
   And declare `libxc-eval = { path = "crates/libxc-eval", default-features = false }` so the root controls feature activation (otherwise eval's own `default` re-activates all three regardless of the verify `--no-default-features` request).
4. **verify/Cargo.toml needs ZERO changes to its feature forwards** — it still says `libxc_rs/oracle-lda`, and the root now forwards that to `libxc-eval/oracle-lda`. The curated kernel dev-deps also need no change (they path to `../crates/kernels/...` directly, independent of the layering). One verify dev-dep DOES need attention — `libxc-kernel-math` — but for the D-11 deferred relocation, not the feature chain (see Open Questions).

### Failure modes (verification checklist for the planner)

| Failure | Symptom | Detection (cheap, no full build) |
|---------|---------|----------------------------------|
| A kernel dep left **non-optional** in libxc-eval | `cargo tree -p libxc-eval --no-default-features` shows kernels it shouldn't; chunked verify builds OOM | `cargo tree -p libxc-eval -e no-dev --no-default-features` — should show only math + core |
| Missing root→eval feature forward | `cargo build -p libxc_rs --features oracle-lda` resolves zero LDA kernels | `cargo tree -p libxc_rs --no-default-features --features oracle-lda` — should show LDA kernels only |
| Root re-declares eval **with** default-features | `--no-default-features` on libxc_rs still pulls all 306 → OOM | inspect `libxc-eval = { …, default-features = false }` present |
| `dep:` prefix dropped during move | Cargo emits implicit-feature collision or fails to gate | `cargo metadata --no-deps -p libxc-eval` features map |
| `libxc-kernel-math` accidentally made optional | math missing from a `--no-default-features` core+math check | `cargo tree -p libxc-eval --no-default-features` must still show math |

**Recommended proof at each step (all `cargo tree`, zero compile, zero OOM):**
```bash
cargo tree -p libxc-core  -e no-dev                                  # SC 2: no cubecl, no kernel-*
cargo tree -p libxc-eval  -e no-dev                                  # SC 3: core present
cargo tree -p libxc-eval  -e no-dev | grep -c libxc-compat           # SC 3: must be 0
cargo tree -p libxc-compat -e no-dev | grep -E 'libxc-(core|eval)'   # SC 4: both present
cargo tree -p libxc_rs --no-default-features --features oracle-lda | grep -c libxc-kernel  # 43 LDA
cargo tree -p libxc_rs --no-default-features --features oracle-mgga | grep -c libxc-kernel # MGGA incl _pK
```

## Module Move Recipe

### Module → crate map (verified against live `src/`)

| Current `src/` module | Target crate | Notes |
|-----------------------|-------------|-------|
| `model/` | libxc-core | Imports `is_deferred` from kernel-math today → repoint to local `crate::deferred` after D-11 |
| `meta/` (+ generated*.rs) | libxc-core | xtask write target (D-03). `generated_propagation` visibility fix needed (below) |
| `registry/` (+ by_id.rs, by_name.rs, removed.rs) | libxc-core | xtask write target. `all_functional_ids` already `pub` |
| `input/` | libxc-core | 0 `pub(crate)`, clean |
| `output/` | libxc-core | 0 `pub(crate)`, clean |
| `layout/` | libxc-core | 0 `pub(crate)`, clean |
| `dims/` | libxc-core | 0 `pub(crate)`, clean |
| `error/` | libxc-core | mod.rs + 3 stubs (D-01b). `LibxcRsError` imports only core types — zero import edits |
| `eval/` (+ `eval/workspace.rs`, gga_dispatch, mgga_dispatch) | libxc-eval | 487 `crate::kernel` refs stay intra-crate; 5 `crate::meta::` refs become cross-crate |
| `functional/` | libxc-eval | Imports `crate::meta::{FunctionalMeta,HybridTerm,generated_propagation::PROPAGATION_RULES}` |
| `kernel/` (mod, lda, gga, mgga, launch, mix/, shared/) | libxc-eval | References 269 distinct `libxc_kernel_*` crates directly |
| `workspace/` (host, planner, resident, scratch_map) | libxc-eval | **Placeholder module** — zero `crate::workspace` consumers anywhere; moves as dead weight (distinct from `eval/workspace.rs`!) |
| `compat/` (c_layout, errno, ids, legacy_eval, macros, mod, raw_handle, removed) | libxc-compat | Moves as a unit (Phase 6 already partitioned it correctly) |
| `api/` (batch, builder, evaluate) | **stays in root** | References BOTH core AND eval (see below) |
| `math/mod.rs` | DELETE | D-02; 12-line dead shim, zero consumers |
| `main.rs` | delete or leave | Planner discretion (3-line vestige) |

> **NAMING TRAP:** there are TWO "workspace" things. `src/workspace/` (top-level placeholder, 5 files, zero consumers) AND `src/eval/workspace.rs` (the real `EvaluationWorkspace`, used by `api/`). Both go to libxc-eval, but they are different modules — `eval::workspace` is `pub mod` and live; top-level `workspace` is dead. Do not conflate.

### Cross-crate visibility changes (the ONLY confirmed break)

`[VERIFIED: grep across src/, 2026-05-25]` — the refactor is almost visibility-clean. The new crate walls turn intra-crate `pub(crate)` into cross-crate-invisible. Exhaustive audit results:

| Item | Current visibility | Crosses boundary? | Action |
|------|-------------------|-------------------|--------|
| `meta::generated_propagation` (module) | `pub(crate) mod` `[src/meta/mod.rs:3]` | YES — `functional/lifecycle.rs:13` (→eval) imports it | **Change to `pub mod`** |
| `meta::generated_propagation::PROPAGATION_RULES` | `pub(crate) const` `[src/meta/generated_propagation.rs:8]` | YES — same | **Change to `pub const`** (or add a `pub` re-export in core's lib) |
| `meta::FunctionalMeta` (struct) | `pub` `[src/meta/mod.rs:51]` | YES (functional/, compat/) | None — already pub |
| `meta::{Reference,ExtParamSpec,HybridTerm}` | pub (re-exported lib:28) | YES | None |
| `meta::generated`, `meta::generated_hybrid` | `pub(crate) mod` | NO — consumed only within core (registry by_id) | None |
| `registry::all_functional_ids` | `pub` `[src/registry/mod.rs:72]` | YES (compat/errno.rs:142) | None — already pub |
| `eval::workspace` (module) | `pub mod` `[src/eval/mod.rs:13]` | YES (api/batch, api/evaluate) | None — already pub |
| `eval::workspace::EvaluationWorkspace` | `pub struct` `[src/eval/workspace.rs:158]` | YES (api/) | None |
| `functional::Functional` | `pub struct` `[src/functional/mod.rs:30]` | YES (api/) | None |
| `functional::Functional::meta()` | `pub fn` `[src/functional/mod.rs:53]` | (returns `&'static FunctionalMeta`) | None |
| `compat` pub(crate) (×2) | `pub(crate)` | NO — intra-compat | None |

**Net visibility work: ~2 edits** (the `generated_propagation` module + `PROPAGATION_RULES` const). Because `generated_propagation.rs` IS an xtask output (`generate_metadata.rs:643`), **the planner must update the xtask emitter's visibility token for this file, not just the file** — otherwise the next `cargo xtask` reverts the fix and breaks the eval build. Alternative that survives regen: add a hand-written `pub use generated_propagation::PROPAGATION_RULES;` in core's `meta/mod.rs` and keep the const `pub(crate)` (the re-export makes it reachable; eval imports the re-exported path).

> The 654 `pub(crate)` in `meta/` and 136 in `eval/` are overwhelmingly intra-module — they do NOT cross the new crate walls. Only `generated_propagation` was found to cross. Confidence HIGH: grep covered all `crate::(eval|functional|kernel|workspace)` from core modules (none) and all `crate::meta::`/`crate::registry::` from eval+compat (5+2, all resolved above).

### The `api/` dual-dependency fact (root facade)

`src/api/` stays in the root crate and references BOTH layers `[VERIFIED: grep src/api]`:
- core: `crate::error::LibxcRsError`, `crate::model::{...}`, `crate::input::{...}`, `crate::output::{...}`, `crate::registry`
- eval: `crate::eval::workspace::EvaluationWorkspace`, `crate::functional::Functional`

Post-split these `crate::...` paths break (api is now in root, not the same crate as model/eval). The planner must rewrite them to `libxc_core::...` and `libxc_eval::...` (or rely on the root facade's own re-exports if `api/` uses the re-exported names). **Both libxc-core and libxc-eval must be direct deps of the root crate** for `api/` to compile. `api/` does NOT reference `crate::compat` — so root does not strictly need a libxc-compat *build* dep for `api/` (SC 4 friendly; a surface `pub use libxc_compat as compat;` is a separate decision).

### Move tactic
- `git mv crates-path` per directory (preserves blame — CONTEXT recommendation).
- Each new crate's `src/lib.rs` sets `#![deny(warnings)]` to match root `src/lib.rs:1`, plus the three `#![allow(clippy::...)]` lines (excessive_precision, needless_late_init, too_many_arguments) for libxc-eval (CubeCL macro expansion triggers them); libxc-core likely needs none but copy to be safe.

## Public Surface Preservation (SC 5)

The root facade must re-export today's exact surface. Current `src/lib.rs:23-38` `[VERIFIED: live read]`:

```rust
pub use model::{
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
    LdaFunctional, GgaFunctional, MggaFunctional,
};
pub use meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use error::LibxcRsError;
pub use dims::Dimensions;
pub use registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
pub use input::{LdaInput, GgaInput, MggaInput};
pub use output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};
pub use eval::{dispatch_lda, dispatch_gga, dispatch_mgga};
pub use functional::{
    classify_hybrid, CamCoefficients, Functional, FunctionalParams, NlcCoefficients, NoParams,
};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};
```

Plus the `pub mod` declarations (lines 9-21): `model, meta, error, dims, registry, math, kernel, input, output, eval, functional, api, compat`.

**Post-split root facade (recommended strategy (a) — line-for-line):**
```rust
#![deny(warnings)]
#![allow(clippy::excessive_precision, clippy::needless_late_init, clippy::too_many_arguments)]

pub mod api;   // stays local

// re-export module namespaces so `libxc_rs::model::X` style paths resolve
pub use libxc_core::{model, meta, error, dims, registry, input, output};
pub use libxc_eval::{eval, functional, kernel, workspace};
// math: the deleted src/math shim — IF any downstream used libxc_rs::math, add:
//   pub use libxc_kernel_math as math;  (D-02a — none found, so likely omit)
pub use libxc_compat as compat;   // optional — preserves `libxc_rs::compat::*`

// then the exact item re-exports above, repointed to libxc_core::/libxc_eval::
pub use libxc_core::model::{Family, Kind, Spin, /* … */};
pub use libxc_eval::eval::{dispatch_lda, dispatch_gga, dispatch_mgga};
pub use libxc_eval::functional::{classify_hybrid, CamCoefficients, Functional, /* … */};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};
```

**Caveat — module paths are part of the surface too** (lines 14, 15, 21):
- `libxc_rs::math` — `src/math/mod.rs` deleted (D-02); zero consumers found. Omit unless D-02a audit surfaces one.
- `libxc_rs::kernel` — verify/ routes through `libxc_rs::kernel::...` per D-12a comment. **MUST re-export** `pub use libxc_eval::kernel;`.
- `libxc_rs::eval` — verify uses `libxc_rs::eval::dispatch_*`. MUST re-export.
- `libxc_rs::compat` — re-export to be safe (FFI symbols may be referenced through the path).
- `libxc_rs::deferred` — NEW after D-11. If verify's `*_oracle.rs` route the deferred import through the facade, add `pub use libxc_core::deferred;`.

> **SC 5 verification (cheap):** prefer per-crate `cargo check -p libxc_rs --lib -j1` (umbrella, ~536 MB peak per 11-14) over `--workspace`. Optionally `cargo doc -p libxc_rs --no-deps` and diff the exported-symbol list against a pre-refactor snapshot.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Feature gating kernels per family | A custom build.rs that toggles deps | Cargo `[features]` + `dep:` syntax (already built by 11-12) | Cargo resolves the dep graph; build.rs can't remove deps |
| Verifying dep boundaries (SC 2/3/4) | Parsing Cargo.lock by hand | `cargo tree -p <crate> -e no-dev` | Native, exact, no compile, no OOM |
| Moving modules | Manual file copy + blame loss | `git mv` | Preserves history; bisect-friendly |
| C header generation | cbindgen (D-09 rejected) | Hand-written `xc_rs.h` | Small stable surface; cbindgen noisy + build-dep |
| Preserving generated-file output across move | Re-running full codegen blind | xtask path-string edits + byte-diff against snapshot | 7 isolated string edits; deterministic output |

**Key insight:** The hard part is NOT writing code — it's the Cargo dependency-graph surgery. Every boundary claim is provable with `cargo tree` (cheap), so the planner should make `cargo tree` assertions first-class verification steps, not afterthoughts.

## Runtime State Inventory

> This is a refactor/migration phase. All five categories audited explicitly.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — no database/datastore keyed on crate or module names. `[VERIFIED: no SQLite/Chroma/Mem0 in tree]` | None |
| Live service config | None — no external service references crate names. | None |
| OS-registered state | None — no Task Scheduler/systemd/pm2 referencing this project's crate names. | None |
| Secrets/env vars | `LIBXC_RS_BYPASS_DEFERRED` was REMOVED by Phase 11-13 (D-11 restored). No env var references the crate layering. `[VERIFIED: STATE.md "LIBXC_RS_BYPASS_DEFERRED removed"]` | None |
| Build artifacts | **Cargo.lock** changes (3 new crates added, 306 kernel deps re-parented to libxc-eval). Stale `target/` dirs are fine (CARGO_TARGET_DIR shared at repo root). The cdylib output (`libxc_rs.so`/`.a`) is now produced ONLY by `libxc-compat`. No `.egg-info`-style stale artifacts (Rust). | Commit regenerated Cargo.lock; `cargo build -p libxc-compat` produces the `.so`/`.a` under `target/<profile>/`. |

**Most important runtime fact:** after every `src/` file moves, the **xtask generator's hard-coded output paths still point at root `src/`** until D-03 edits them. If a `cargo xtask` runs post-move but pre-D-03-edit, it regenerates files into the now-empty root `src/meta/` and `src/registry/`, NOT into libxc-core — silently producing stale/orphaned output. **D-03 must land in the same plan/commit as the meta+registry move** (the 10-01 libxc-core plan).

## Common Pitfalls

### Pitfall 1: Forgetting eval's own `default` re-activates all kernels
**What goes wrong:** verify runs `--no-default-features --features oracle-lda` expecting 43 LDA kernels; instead all 306 compile (OOM).
**Why:** if root declares `libxc-eval = { path = "..." }` WITHOUT `default-features = false`, libxc-eval's `default = [oracle-lda,oracle-gga,oracle-mgga]` fires regardless of what the root/verify request.
**How to avoid:** root MUST pin `libxc-eval = { path = "crates/libxc-eval", default-features = false }`.
**Warning sign:** `cargo tree -p libxc_rs --no-default-features --features oracle-lda` shows GGA/MGGA kernels.

### Pitfall 2: xtask regen reverts the `generated_propagation` visibility fix
**What goes wrong:** planner changes `pub(crate) mod generated_propagation` → `pub`, but the next `cargo xtask` regenerates the file with `pub(crate)` and the eval build breaks again.
**Why:** `generated_propagation.rs` is xtask-emitted (`generate_metadata.rs:643`).
**How to avoid:** patch the xtask emitter's visibility token, OR re-export via a hand-written `pub use generated_propagation::PROPAGATION_RULES;` in core's `meta/mod.rs` (survives regen).
**Warning sign:** `git diff` after `cargo xtask` shows the visibility reverting.

### Pitfall 3: Kernel dep path prefixes wrong after move
**What goes wrong:** kernel deps moved to `libxc-eval/Cargo.toml` keep `path = "crates/kernels/..."` (root-relative) — Cargo resolves relative to libxc-eval's dir → `crates/libxc-eval/crates/kernels/...` (nonexistent).
**Why:** path-deps are relative to the manifest's directory.
**How to avoid:** rewrite all 306 paths `crates/kernels/...` → `../kernels/...`. A uniform bulk edit is safe.
**Warning sign:** `cargo metadata -p libxc-eval` errors "failed to load source for dependency".

### Pitfall 4: `--workspace` check OOMs and masks per-crate green
**What goes wrong:** D-13's literal "refuse to start unless `cargo check --workspace` is green" can never be satisfied — it OOMs at jobs=1 (306 kernels via libxc-compat / default-members).
**Why:** RAM-constrained box; workspace build pulls all kernels.
**How to avoid:** reframe the precondition as **per-`-p` green**: `cargo check -p libxc-core`, `-p libxc-eval` (no-default or single-family), `-p libxc_rs --lib`. NEVER `cargo build -p libxc-compat` or `--workspace` as a gate (cdylib links all kernels).
**Warning sign:** SIGKILL / RSS > available RAM. `[VERIFIED: STATE.md — kcisk SIGKILL @ 30.29 GB; per-`-p` is the standing rule]`

### Pitfall 5: `verify-canary` member overlooked
**What goes wrong:** CONTEXT.md D-10b lists members as `xtask, verify, libxc-sys` — but the LIVE tree has a 4th: `verify-canary` (`libxc_rs-verify-canary`), added 2026-05-22 after CONTEXT was written.
**Why:** CONTEXT predates the canary crate.
**How to avoid:** preserve `verify-canary` in `[workspace] members`. It deps `libxc-kernel-mgga_c_b94` + `libxc-sys` directly (NOT the umbrella), is NOT a default-member, and is layering-independent — Phase 10 leaves it untouched, same as libxc-sys.
**Warning sign:** dropping it from members → `cargo test -p libxc_rs-verify-canary` fails to resolve.

### Pitfall 6: `deferred` is a two-submodule structure, not a flat fn
**What goes wrong:** D-11 relocation flattens `deferred` and breaks the 4 consumers' `deferred::lda::is_deferred` / `deferred::mgga::is_deferred` paths.
**Why:** `deferred.rs` contains `pub mod lda { pub fn is_deferred(...) }` and `pub mod mgga { ... }` `[VERIFIED: head of deferred.rs]`.
**How to avoid:** preserve the `lda::`/`mgga::` submodule shape in libxc-core. Consumers then call `libxc_core::deferred::lda::is_deferred` (or via root facade). The `//!` provenance header references the Phase-11-deleted façade crates — update it to note the Phase-10 relocation.
**Warning sign:** `error[E0425]: cannot find function is_deferred`.

## Code Examples

### D-11 deferred relocation — consumer repoint
```rust
// BEFORE (src/model/lda_functional.rs:80, while model/ still in root):
if libxc_kernel_math::deferred::lda::is_deferred(id.raw()) { … }

// AFTER (model/ now in libxc-core, deferred also in libxc-core):
if crate::deferred::lda::is_deferred(id.raw()) { … }
// (or crate::model::deferred::... if placed as a model submodule — planner picks)
```
```rust
// verify/tests/lda_oracle.rs:36 (routes through facade or core direct — planner picks):
use libxc_kernel_math::deferred::lda::is_deferred;   // BEFORE
use libxc_rs::deferred::lda::is_deferred;             // AFTER (via facade re-export)
// requires root facade: pub use libxc_core::deferred;
```
> `[VERIFIED: 4 consumers — src/model/{lda,mgga}_functional.rs:80/43, verify/tests/{lda,mgga}_oracle.rs:36/44]`
> verify/Cargo.toml ALSO keeps `libxc-kernel-math` as a dev-dep (line 41) explicitly "for the deferred registry". After D-11 removes `deferred` from kernel-math, that comment is stale — repoint verify's deferred import to libxc-core, then check whether verify uses any OTHER `libxc_kernel_math::` symbol before dropping the dev-dep.

### Boundary proof (SC 2) — the canonical assertion
```bash
# SC 2: libxc-core has zero cubecl/kernel deps
cargo tree -p libxc-core -e no-dev 2>&1 | tee log/10-final-cargo-tree-core.log
! cargo tree -p libxc-core -e no-dev | grep -qE 'cubecl|libxc-kernel'   # must succeed (grep finds nothing)
```

## State of the Art

| Old (2026-05-07 RESEARCH) | Current (2026-05-25) | When Changed | Impact |
|---------------------------|----------------------|--------------|--------|
| `crates/kernel-{lda,gga,mgga}*` ~170 umbrella crates | 306 per-functional crates `crates/kernels/{lda,gga,mgga,math}/` | Phase 11 D-10a (clean-slate restructure) | All kernel paths in old research are dead; D-10 moves 306 not 4 |
| libxc-eval inherits 4 umbrella deps | libxc-eval inherits all 306 kernel deps | Phase 11 | Feature-forwarding chain is the new central mechanic |
| Kernel deps unconditional | Kernel deps `optional=true` + `[features] oracle-*` | Phase 11-12 (2026-05-23) | Must migrate the WHOLE feature machinery, not just deps |
| 281 kernel count | **306** (25 `_pK` shards added: rmggac/tpss/kcisk + tpssloc) | Phase 11-10 (2026-05-25) | Sharded for the 30 GB OOM ceiling |
| `cubecl 0.9.0` | `cubecl 0.10.0` | Phase 11-14 | Launch ABI migrated; new crates pin 0.10 |
| D-13 gate active (Phase 11 red) | D-13 gate LIFTED (Phase 11 COMPLETE) | 2026-05-25 (11-13/11-14) | Phase 10 execution unblocked |
| 3 workspace members | 4 members (+`verify-canary`) | Phase 11.1 (2026-05-22) | Preserve the canary |

**Deprecated/outdated in the old research:** every `crates/kernel-*` path; the "4 umbrella deps" Architectural Responsibility row; the `cargo check --workspace`-as-gate recipe (OOMs); the 281 count. The old research's topology-INDEPENDENT mechanics (git mv, rlib/cdylib/staticlib, cross-crate visibility theory, xtask byte-diff recipe) remain valid and were folded above.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The 305 optional kernel deps split exactly 43 LDA / ~131 GGA / ~131 MGGA per the `[features]` lists | Feature chain | Low — the exact split is in the verbatim `[features]` block being moved; counts are informational, not load-bearing |
| A2 | `cargo check -p libxc_rs --lib` (umbrella) peaks ~536 MB (per 11-14) and is a safe gate | Validation | Medium — if a later change inflated it, a single-family `--no-default-features --features oracle-lda` is the fallback |
| A3 | No downstream consumer uses `libxc_rs::math::*` (D-02a) | Public surface | Low — verified zero in src/ and verify/; external consumers unknown but project is pre-1.0 single-repo |
| A4 | `generated_propagation.rs` is the ONLY xtask-emitted file needing a visibility bump | Visibility | Medium — only 5 `crate::meta::` refs from eval today, all audited; re-grep after any meta change |
| A5 | Root does NOT need a `libxc-compat` *build* dep for `api/` to compile | Module map | Low — verified `api/` has zero `crate::compat` refs; a surface `pub use libxc_compat as compat;` is a separate choice that does add the dep |

## Open Questions

1. **Does any external/downstream consumer rely on `libxc_rs::compat::*` or `libxc_rs::kernel::*` module paths?**
   - Known: verify/ uses `libxc_rs::kernel::...` and `libxc_rs::eval::dispatch_*` (D-12a). compat symbols are FFI (extern "C"), consumed via the cdylib, not `use libxc_rs::compat`.
   - Unclear: whether re-exporting `pub use libxc_compat as compat;` from root is needed or dead surface.
   - Recommendation: re-export it (low cost, preserves surface); drop later if SC 5 verification shows it unused.

2. **After D-11 removes `deferred` from kernel-math, does verify still need its `libxc-kernel-math` dev-dep?**
   - Known: verify/Cargo.toml:41 keeps `libxc-kernel-math` "for the deferred registry". The `*_oracle.rs` tests import `libxc_kernel_math::deferred::{lda,mgga}::is_deferred`.
   - Unclear: whether verify uses any OTHER kernel-math symbol.
   - Recommendation: repoint deferred import to libxc-core; grep verify for other `libxc_kernel_math::` uses — drop the dev-dep only if none.

3. **Is the cdylib's `xc_rs.h` already committed by Phase 6?** (D-09a)
   - Known: Phase 6 is the header's owner; Phase 10 only relocates it.
   - Recommendation: planner checks `crates/libxc-compat/include/xc_rs.h` (or Phase-6 location) at plan time; if absent, header is Phase-6's deliverable and Phase 10 does not block.

4. **SC 6 "matches pre-refactor pass/fail set" — what is the pre-refactor set?**
   - Known: 6 MGGA functionals fail the f64 oracle today (routed to Phase 12: mgga_x_th, mgga_x_2d_js17, mgga_c_cs, mgga_x_pkzb, mgga_x_pbe_gx, mgga_x_tm). 3 pre-existing math-precision test failures (bessel::ref_i0_small_arg, dft_quantities alpha/tf_kinetic ~4e-8) also exist.
   - Recommendation: capture the per-family pass/fail baseline (incl. these expected failures) BEFORE 10-01 so SC 6 compares apples-to-apples.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| cargo / rustc (Edition 2024, MSRV 1.85+) | all | ✓ (project builds) | edition 2024 | — |
| `cargo tree` | SC 2/3/4 verification | ✓ (built into cargo) | — | `cargo metadata` JSON parse |
| `git mv` | module moves | ✓ (git repo) | — | copy+delete (blame loss) |
| RAM for full workspace build | SC 6/8 (`cargo build/test --workspace`) | ✗ (OOMs at jobs=1) | — | per-`-p` builds + curated verify subset |
| libxc-master C source + cmake + bindgen | verify/ oracle (SC 7) | ✓ (libxc-sys already builds) | bindgen 0.72.1, cmake 0.1.58 | — |

**Missing dependencies with no fallback:** none that block planning.
**Missing dependencies with fallback:** full-workspace RAM → use per-crate verification (the standing project constraint, not a Phase-10-specific gap).

## Validation Architecture

> nyquist_validation is enabled (no `.planning/config.json` override found → treat as enabled).

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + `approx` (relative_eq) for oracle parity; verify/ harness over libxc-sys oracle |
| Config file | none (Cargo-native); `verify/Cargo.toml` features gate the family-chunked builds |
| Quick run command | `cargo tree -p <crate> -e no-dev` (boundary SCs, zero compile) |
| Full suite command | per-family: `cargo test -p libxc_rs-verify --no-default-features --features oracle-<fam> --test <fam>_oracle -j1` (USER-RUN, heavy) |

### Success Criteria → Test Map (the 8 SCs are the anchors; no REQ-IDs)
| SC | Behavior | Test Type | Automated Command | Cost |
|----|----------|-----------|-------------------|------|
| SC 1 | Four crates exist | structural | `test -d crates/libxc-core -a -d crates/libxc-eval -a -d crates/libxc-compat` + `cargo metadata --no-deps` grep `libxc-` | trivial |
| SC 2 | libxc-core zero cubecl/kernel deps | dep-graph | `! cargo tree -p libxc-core -e no-dev \| grep -qE 'cubecl\|libxc-kernel'` | cheap, no compile |
| SC 3 | libxc-eval deps core, NOT compat | dep-graph | `cargo tree -p libxc-eval -e no-dev \| grep libxc-core` AND `! cargo tree -p libxc-eval -e no-dev \| grep -q libxc-compat` | cheap |
| SC 4 | libxc-compat deps both; nothing deps it | dep-graph | `cargo tree -p libxc-compat -e no-dev \| grep -E 'libxc-(core\|eval)'` AND reverse-dep check (no crate lists libxc-compat) | cheap |
| SC 5 | root public surface unchanged | compile/doc | `cargo check -p libxc_rs --lib -j1` (umbrella, ~536 MB) OR `cargo doc -p libxc_rs --no-deps` symbol diff vs snapshot | medium (per-`-p`) |
| SC 6 | `cargo test --workspace` parity | test | per-family verify runs (workspace OOMs) — diff pass/fail set vs pre-refactor snapshot | heavy, USER-RUN |
| SC 7 | oracle parity 1e-12 LDA/GGA/MGGA | oracle | `cargo test -p libxc_rs-verify --no-default-features --features oracle-lda --test lda_oracle -j1` (repeat gga/mgga) | heavy, USER-RUN |
| SC 8 | `cargo build --workspace` zero new warnings | build | NOT feasible whole-workspace (OOM); per-`-p` `cargo build -p libxc-core` + `-p libxc-eval --no-default-features -j1` + spot-check; `#![deny(warnings)]` makes any warning a hard error | medium per-`-p` |

### Sampling Rate
- **Per task commit:** the relevant `cargo tree -p <crate>` boundary assertion (SC 2/3/4) — milliseconds, no OOM.
- **Per crate-extraction merge:** `cargo check -p <new-crate> --no-default-features -j1` (light, single-family).
- **Phase gate (SC 6/7):** USER-RUN per-family oracle sweeps (LDA, GGA, MGGA separately) — diff against pre-refactor pass/fail. The 6 known MGGA f64-parity failures (Phase 12) + 3 math-precision failures must be in the "expected fail" baseline so SC 6 holds.

### Representative kernels available to verify/ at 1e-12 (SC 7) WITHOUT OOM
From verify/'s curated dev-dep subset `[VERIFIED: verify/Cargo.toml:43-63]`:
- **LDA:** `lda_x`, `lda_c_pw`, `lda_xc_teter93`
- **GGA:** `gga_x_pbe`, `gga_c_pbe`, `gga_x_b88`
- **MGGA:** `mgga_x_lta`, `mgga_x_tpss`, `mgga_x_pkzb`, `mgga_x_th` (+ worst-case `mgga_c_revtpss`, `mgga_c_kcisk`, `mgga_c_b94`, `mgga_x_r4scan`, `mgga_x_br89_explicit`, `mgga_xc_b97mv`)
This subset compiles without pulling all 306 — it is the SC-7 witness. Do NOT expand it.

### Wave 0 Gaps
- [ ] None for test *infrastructure* — verify/ harness, oracle (libxc-sys), and the curated subset already exist and exercise all three families.
- [ ] **Pre-refactor baseline snapshot** — before 10-01, capture the current per-family pass/fail set (incl. the 6 expected MGGA Phase-12 failures + 3 math-precision failures) and the generated-files byte-snapshot, so SC 6 / D-03 byte-diff has a reference. (A plan step, not missing infra.)

## Sources

### Primary (HIGH confidence — verified live this session)
- `Cargo.toml` (root) — kernel deps (306, all `optional=true` except math), `[features] oracle-*` block (323-645), `[workspace] members` (652-656), `default-members` (260 entries), profiles. `[VERIFIED]`
- `verify/Cargo.toml` lines 1-63 — `libxc_rs default-features=false`, feature forwards (15-19), curated kernel dev-deps (43-63), kernel-math dev-dep (41). `[VERIFIED]`
- `verify-canary/Cargo.toml` — 4th workspace member, b94 direct, NOT in CONTEXT. `[VERIFIED]`
- `src/lib.rs` lines 1-38 — full public surface, `#![deny(warnings)]`. `[VERIFIED]`
- `src/math/mod.rs` — 12-line shim, zero `crate::math` consumers, zero `libxc_rs::math` in verify. `[VERIFIED]`
- `crates/kernels/math/src/deferred.rs` head + `lib.rs:22 pub mod deferred` — submodule structure, provenance. `[VERIFIED]`
- deferred consumers: `src/model/{lda,mgga}_functional.rs`, `verify/tests/{lda,mgga}_oracle.rs`. `[VERIFIED via grep]`
- Visibility: `meta/generated_propagation.rs:8 pub(crate) const`, `meta/mod.rs:3 pub(crate) mod`, `registry/mod.rs:72 pub fn all_functional_ids`, `eval/mod.rs:13 pub mod workspace`, `functional/mod.rs:30,53`. `[VERIFIED via grep]`
- xtask paths: `main.rs:291,329,355,387`, `generate_metadata.rs:445,595,643`; `xtask/Cargo.toml` NOTE (no libxc_rs dep). `[VERIFIED]`
- `git log` — 11-14 landed (`6434274b5e`, umbrella `cargo check` exit 0, 3031→0). `[VERIFIED]`
- `.planning/STATE.md` — Phase 11 COMPLETE 2026-05-25, D-13 gate context, per-`-p` rule, 306-pkg sweep, 6 MGGA→Phase 12, 3 math-precision failures. `[VERIFIED]`
- `.planning/ROADMAP.md` §Phase 10 — 8 SCs. `[VERIFIED]`
- `10-CONTEXT.md` — full decisions D-01..D-14. `[VERIFIED — read in full]`

### Secondary (folded from prior research)
- Old `10-RESEARCH.md` (2026-05-07) — topology-INDEPENDENT mechanics only (git mv, crate-type, visibility theory, xtask byte-diff). Kernel-path claims DISCARDED.

## Metadata

**Confidence breakdown:**
- Feature-forwarding chain: HIGH — read root + verify Cargo.toml in full, traced all 4 links.
- Module map & visibility: HIGH — exhaustive grep of cross-crate references; only 1 break found.
- Kernel count (306) & `_pK` shards: HIGH — `grep -cE` derivation, matches STATE.md.
- Phase 11 complete / D-13 lifted: HIGH — git log + STATE.md.
- Validation strategy: HIGH — per-`-p` constraint is the documented standing rule.

**Research date:** 2026-05-25
**Valid until:** ~2026-06-08 (stable refactor domain; re-verify kernel count and `[features]` block if any Phase 11/12 follow-up touches Cargo.toml before execution)
