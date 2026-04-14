# Phase 9: Reduce Kernel Build Time — Research

**Researched:** 2026-04-14
**Domain:** Rust workspace build optimization, Cargo feature gating, CubeCL proc macro scaling
**Confidence:** HIGH (all findings verified against live codebase)

---

## Summary

Plans 09-01 and 09-02 added structural annotations to translators and re-generated
all kernel files, but produced zero actual code volume reduction — the kernel files
still contain monolithic functions. The total codebase remains ~3.7M Rust lines.
Plans 09-01 and 09-02 are done. Plan 09-03 as written is also **already done**
(sccache configured, incremental=false set, no sub-crate profile sections remain).

What actually remains from the phase success criteria:
- Success criterion 2: Default `cargo build` compiles only LDA (~2m) — **NOT done**
- Success criterion 3: `--features gga` and `--features all-kernels` gates — **NOT done**
- Enabling 25 deferred GGA functionals (requested by user as "GGA workspace splitting") — **NOT done**

The user's request to "split the GGA kernel workspace" translates to two complementary
actions: (A) add `[features]` to workspace Cargo.toml to gate GGA/MGGA compilation,
and (B) create `crates/kernel-gga-4` to house the 25 deferred GGA functionals with
derivative-order feature gates (`#[cfg(feature = "order-kxc")]` etc.) on their large
kxc/lxc files, which currently OOM during proc macro expansion.

**Primary recommendation:** Implement feature gating by kernel family (lda/gga/mgga)
at workspace level AND create one new GGA sub-crate for the 25 deferred functionals
with derivative-order cfg gates to eliminate OOM.

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| BUILD-OPT-01 | sccache working, incremental compilation not blocked | DONE: sccache 0.14.0 installed, `rustc-wrapper = "sccache"` in .cargo/config.toml, `incremental = false` in workspace profiles |
| BUILD-OPT-02 | Default `cargo build` compiles only LDA kernels | NOT done: no `[features]` section in Cargo.toml; all kernels compile unconditionally |
| BUILD-OPT-03 | Feature gates for gga/all-kernels; no redundant profile sections | Profile cleanup done. Feature gates NOT done. |
</phase_requirements>

---

## Current State (Verified)

### Build Configuration (DONE — no action needed)

| Setting | File | Value | Correct? |
|---------|------|-------|----------|
| sccache | `.cargo/config.toml` | `rustc-wrapper = "sccache"` | YES |
| jobs | `.cargo/config.toml` | `jobs = 6` | YES (comment says 3, but 6 is set) |
| incremental | `Cargo.toml` `[profile.dev]` | `incremental = false` | YES |
| incremental | `Cargo.toml` `[profile.test]` | `incremental = false` | YES |
| sub-crate profiles | All crates in `crates/` | None found | YES — all clean |

[VERIFIED: grep across workspace]

### Feature Gating (NOT done)

The workspace root `Cargo.toml` has **no `[features]` section**. All kernel crates
are unconditional `[dependencies]`:

```toml
libxc-kernel-lda = { path = "crates/kernel-lda" }
libxc-kernel-gga = { path = "crates/kernel-gga" }
libxc-kernel-mgga = { path = "crates/kernel-mgga" }
```

Every `cargo build` compiles all 44 workspace crates.

[VERIFIED: grep in Cargo.toml]

### GGA Kernel Crate Structure (VERIFIED)

| Crate | Functionals | Lines | Build time (est.) |
|-------|-------------|-------|-------------------|
| kernel-gga-1 | 35 | 175K | ~4-6m |
| kernel-gga-2 | 35 | 144K | ~3-5m |
| kernel-gga-3 | 36 | 156K | ~4-6m |
| kernel-gga (facade) | 0 compiled, 25 deferred | 492K (source only) | ~0 (empty re-export) |
| **Total compiled GGA** | 106 | 475K | **~12m serial** |
| **Deferred 25** | 25 | 635K | **OOM — cannot compile** |

[VERIFIED: find + wc -l across crate directories]

### Why the 25 GGA Functionals Are Deferred

The CubeCL `#[cube(launch_unchecked)]` proc macro expands source lines at O(n^1.56).
A ~5K-line function expands to ~87K lines (17x). A 37K-line function would expand
to ~970K lines per proc macro invocation, exhausting available RAM.

The largest deferred GGA lxc_pol.rs files:

| Functional | lxc_pol.rs lines | Status |
|------------|-----------------|--------|
| gga_c_ft97 | 37,787 | Deferred (OOM) |
| gga_x_wpbeh | 25,973 | Deferred (OOM) |
| gga_c_pbe_erf_gws | 23,663 | Deferred (OOM) |
| gga_c_optc | 19,357 | Deferred (OOM) |
| gga_c_q2d | 17,770 | Deferred (OOM) |
| gga_c_revtca | 5,518 | Deferred (smallest) |

The OOM threshold is approximately **5,000–5,500 lines** per `#[cube(launch_unchecked)]`
function. All 25 deferred functionals have lxc_pol.rs above this threshold.

Crucially: **the fxc_pol.rs files for all 25 deferred functionals are under 3,900 lines**
(max: gga_c_ft97 fxc_pol.rs = 3,739 lines). So orders 0-2 (exc/vxc/fxc) compile fine.
Only kxc and lxc (orders 3-4) cause OOM.

[VERIFIED: find + wc -l on kernel-gga/src/*/lxc_pol.rs and fxc_pol.rs]

### Code Volume Distribution (VERIFIED)

From the debug investigation (kernel-build-time.md):

| Order | Level | Lines (total) | % of Total |
|-------|-------|---------------|------------|
| 0 | exc | 87,445 | 2.3% |
| 1 | vxc | 176,314 | 4.7% |
| 2 | fxc | 412,331 | 11.0% |
| 3 | kxc | 1,031,356 | 27.6% |
| 4 | lxc | 2,017,612 | 54.2% |

**kxc + lxc = 81.9% of all kernel code.** Feature-gating orders 3-4 eliminates
the vast majority of compilation work.

For the 25 deferred GGA functionals specifically:
- exc: 9,456 lines
- vxc: 22,590 lines
- fxc: 92,345 lines (pol + unpol combined)
- kxc: 172,911 lines
- lxc: 368,580 lines
- **exc+vxc+fxc only: 124,391 lines** — easily fits in one sub-crate

[VERIFIED: per-level totals computed from find + wc on crates/kernel-gga/src/]

---

## Standard Stack

### Rust Cargo Feature Gating

The standard Rust pattern for optional compilation units:

```toml
# Cargo.toml (workspace root package)
[features]
default = []          # default: only LDA (unconditional dep)
gga = ["dep:libxc-kernel-gga"]
mgga = ["dep:libxc-kernel-mgga"]
all-kernels = ["gga", "mgga"]

[dependencies]
libxc-kernel-lda = { path = "crates/kernel-lda" }              # always
libxc-kernel-gga = { path = "crates/kernel-gga", optional = true }
libxc-kernel-mgga = { path = "crates/kernel-mgga", optional = true }
```

Usage pattern:
```bash
cargo build                          # LDA only (~2m)
cargo build --features gga           # LDA + GGA (~14m)
cargo build --features all-kernels   # Everything (~30-70m)
```

[ASSUMED: Cargo optional dependency semantics — standard Cargo behavior, high confidence]

### Per-File Derivative Order Feature Gating

The standard pattern for conditional module inclusion in mod.rs:

```rust
// In kernel-gga-4/src/gga_c_ft97/mod.rs
pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;

#[cfg(feature = "order-kxc")]
pub mod kxc_unpol;
#[cfg(feature = "order-kxc")]
pub mod kxc_pol;

#[cfg(feature = "order-lxc")]
pub mod lxc_unpol;
#[cfg(feature = "order-lxc")]
pub mod lxc_pol;
```

In the sub-crate Cargo.toml:
```toml
[features]
order-kxc = []
order-lxc = ["order-kxc"]
all-orders = ["order-lxc"]
```

The parent workspace would forward these features if needed:
```toml
# In libxc-kernel-gga facade Cargo.toml
[features]
order-kxc = ["libxc-kernel-gga-4/order-kxc"]
order-lxc = ["libxc-kernel-gga-4/order-lxc"]
all-orders = ["libxc-kernel-gga-4/all-orders"]
```

[ASSUMED: Feature forwarding in Cargo workspace — standard Cargo behavior]

### Workspace Crate Addition Pattern

Adding a new sub-crate to the workspace follows the existing kernel-gga-{1,2,3} pattern:

1. Create `crates/kernel-gga-4/Cargo.toml` mirroring kernel-gga-1
2. Copy functional source directories from `crates/kernel-gga/src/` to `crates/kernel-gga-4/src/`
3. Create `crates/kernel-gga-4/src/lib.rs` with `pub mod <functional>;` for each
4. Add `libxc-kernel-gga-4 = { path = "../kernel-gga-4" }` to `kernel-gga/Cargo.toml`
5. Add `pub use libxc_kernel_gga_4 as batch4;` to `kernel-gga/src/lib.rs`
6. Add `"crates/kernel-gga-4"` to workspace members in root `Cargo.toml`
7. Remove the commented-out `// pub mod` entries from `kernel-gga/src/lib.rs`

[VERIFIED: Inferred from existing kernel-gga/Cargo.toml, lib.rs, and crate structure]

---

## Architecture Patterns

### Recommended Project Structure After Change

```
Cargo.toml                          # Add [features]: gga, mgga, all-kernels
crates/
├── kernel-math/                    # Always compiled (no change)
├── kernel-lda/                     # Always compiled (no change)
├── kernel-gga/                     # Facade — optional dep, forwarded features
│   ├── Cargo.toml                  # Add optional deps on gga-4; [features]
│   └── src/lib.rs                  # Add: pub use libxc_kernel_gga_4 as batch4
├── kernel-gga-1/ through -3/       # Existing (no change needed for phase goal)
├── kernel-gga-4/ (NEW)             # 25 previously-deferred GGA functionals
│   ├── Cargo.toml                  # [features]: order-kxc, order-lxc, all-orders
│   └── src/
│       ├── lib.rs                  # pub mod for each functional
│       └── gga_c_ft97/
│           ├── mod.rs              # kxc/lxc modules gated by #[cfg(feature)]
│           ├── exc_pol.rs          # Always compiled
│           ├── lxc_pol.rs          # Only with #[cfg(feature = "order-lxc")]
│           └── ...
├── kernel-mgga/                    # Facade — optional dep (no change to contents)
└── kernel-mgga-{1..37}/            # Existing sub-crates (no change needed)
src/kernel/mod.rs                   # Add #[cfg(feature = "gga")] on gga re-export
```

### Feature Flag Hierarchy

```
all-kernels (workspace)
├── gga
│   └── dep:libxc-kernel-gga
│       ├── dep:libxc-kernel-gga-1 (existing, no order gating needed)
│       ├── dep:libxc-kernel-gga-2 (existing, no order gating needed)
│       ├── dep:libxc-kernel-gga-3 (existing, no order gating needed)
│       └── dep:libxc-kernel-gga-4 (new, has order-kxc/lxc gates)
└── mgga
    └── dep:libxc-kernel-mgga
        └── dep:libxc-kernel-mgga-{1..37} (existing, no change needed)
```

Note: The existing kernel-gga-{1,2,3} and kernel-mgga-{1..37} do NOT need derivative
order gating — their largest lxc_pol.rs files are 4,889 lines (below the ~5K OOM
threshold). Only the 25 deferred functionals (moving to kernel-gga-4) have the
oversized files requiring cfg gates.

### Anti-Patterns to Avoid

- **Gating existing kernel-gga-{1,2,3}**: These compile fine. No change needed.
  Adding unnecessary cfg complexity to working crates risks breakage.

- **One crate per deferred functional**: Would create 25 new workspace members.
  All 25 fit in one crate when kxc/lxc are gated.

- **Removing kxc/lxc source files**: `#[cfg(feature)]` is cleaner than deleting files;
  deletion would make adding kxc/lxc support in future harder.

- **Using `compiler = "sccache"` in .cargo/config.toml**: Wrong key. The current
  `rustc-wrapper = "sccache"` is correct. The 09-03-PLAN.md contains a bug here —
  do NOT change the existing working config to use `compiler`.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Conditional compilation | Manual #[cfg] chains across files | Cargo `[features]` + `optional = true` | Standard Cargo mechanism, understood by all tooling |
| Sub-crate creation | Script from scratch | Copy kernel-gga-1/Cargo.toml as template | Identical structure, just new name and feature declarations |
| Deferred functional source | Re-translate from C | Move from kernel-gga/src/ to kernel-gga-4/src/ | Files already exist, fully translated |

---

## Common Pitfalls

### Pitfall 1: `compiler = "sccache"` is not a valid Cargo key

**What goes wrong:** `compiler` is not a recognized `[build]` key. It silently does nothing.
**Why it happens:** The 09-03-PLAN.md mistakenly wrote `compiler = "sccache"` — the correct key is `rustc-wrapper`.
**How to avoid:** Do NOT modify `.cargo/config.toml` at all — it already has the correct `rustc-wrapper = "sccache"`.
**Warning sign:** sccache stats show 0 requests despite builds running.

### Pitfall 2: Feature gating in the wrong place

**What goes wrong:** Adding `[features]` to a sub-crate (kernel-gga-1) instead of
the facade (kernel-gga) and root package (libxc_rs). Users would need to specify
sub-crate features directly instead of top-level `--features gga`.
**Why it happens:** Feature declarations feel "local" to the crate that uses them.
**How to avoid:** `[features]` in root `Cargo.toml` with `optional = true` deps on
kernel-gga and kernel-mgga. Feature forwarding in kernel-gga/Cargo.toml for order-gates.

### Pitfall 3: Forgetting to update `src/kernel/mod.rs`

**What goes wrong:** Even if kernel-gga is optional in Cargo.toml, the `pub use
libxc_kernel_gga as gga;` in `src/kernel/mod.rs` will cause a compile error when
the feature is absent.
**Why it happens:** Cargo makes the dep optional but does not automatically cfg-gate
the use statements.
**How to avoid:** Wrap the use statement with `#[cfg(feature = "gga")]`.

### Pitfall 4: 25 deferred sources are in kernel-gga/src/, not kernel-gga-4/src/

**What goes wrong:** Creating kernel-gga-4/src/ without moving the functional
directories from kernel-gga/src/. The kernel-gga facade still has the commented-out
entries; kernel-gga-4 is empty.
**Why it happens:** The source files physically exist in kernel-gga/src/ already.
**How to avoid:** Move (not copy) the 25 functional directories from
`crates/kernel-gga/src/` to `crates/kernel-gga-4/src/` and remove the commented
`// pub mod` entries from `kernel-gga/src/lib.rs`.

### Pitfall 5: Order-feature default conflicts with currently compiled functionals

**What goes wrong:** If order-kxc/lxc features are added to kernel-gga-4 but NOT
defaulted, then `--features gga` compiles the 25 functionals only through fxc.
This is intentional and correct. However, the existing kernel-gga-{1,2,3} always
compile kxc+lxc (no gating). This asymmetry is acceptable — those crates compile fine.
**Why it happens:** Different compilation history for pre-existing vs new sub-crate.
**How to avoid:** Document the asymmetry explicitly in the Cargo.toml and lib.rs comments.

---

## Code Examples

### Adding optional dependency (root Cargo.toml)

```toml
# Source: Cargo reference - optional dependencies
[features]
default = []
gga = ["dep:libxc-kernel-gga"]
mgga = ["dep:libxc-kernel-mgga"]
all-kernels = ["gga", "mgga"]

[dependencies]
libxc-kernel-math = { path = "crates/kernel-math" }
libxc-kernel-lda = { path = "crates/kernel-lda" }                              # unconditional
libxc-kernel-gga = { path = "crates/kernel-gga", optional = true }             # --features gga
libxc-kernel-mgga = { path = "crates/kernel-mgga", optional = true }           # --features mgga
```

### Gating use in src/kernel/mod.rs

```rust
// Source: Cargo book - cfg-gating optional dependency usage
pub use libxc_kernel_lda as lda;

#[cfg(feature = "gga")]
pub use libxc_kernel_gga as gga;

#[cfg(feature = "mgga")]
pub use libxc_kernel_mgga as mgga;

pub mod launch;
pub mod dispatch_key;
pub mod shared;
pub mod mix;
```

### kernel-gga-4/Cargo.toml template

```toml
[package]
name = "libxc-kernel-gga-4"
version = "0.1.0"
edition = "2024"

[features]
order-kxc = []
order-lxc = ["order-kxc"]
all-orders = ["order-lxc"]

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../kernel-math" }
```

### kernel-gga-4/src/gga_c_ft97/mod.rs template

```rust
//! GGA_C_FT97 kernel — derivative-order gated to prevent OOM.
//! lxc_pol.rs is 37,787 lines; compile only when order-lxc feature is enabled.

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;

#[cfg(feature = "order-kxc")]
pub mod kxc_unpol;
#[cfg(feature = "order-kxc")]
pub mod kxc_pol;

#[cfg(feature = "order-lxc")]
pub mod lxc_unpol;
#[cfg(feature = "order-lxc")]
pub mod lxc_pol;
```

### kernel-gga facade update (kernel-gga/Cargo.toml)

```toml
[package]
name = "libxc-kernel-gga"
version = "0.1.0"
edition = "2024"

[features]
order-kxc = ["libxc-kernel-gga-4/order-kxc"]
order-lxc = ["libxc-kernel-gga-4/order-lxc"]
all-orders = ["libxc-kernel-gga-4/all-orders"]

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../kernel-math" }
libxc-kernel-gga-1 = { path = "../kernel-gga-1" }
libxc-kernel-gga-2 = { path = "../kernel-gga-2" }
libxc-kernel-gga-3 = { path = "../kernel-gga-3" }
libxc-kernel-gga-4 = { path = "../kernel-gga-4" }
```

### kernel-gga/src/lib.rs update

```rust
// existing
pub use libxc_kernel_gga_1 as batch1;
pub use libxc_kernel_gga_2 as batch2;
pub use libxc_kernel_gga_3 as batch3;

// new — batch4 always compiled (but large files are feature-gated within)
pub use libxc_kernel_gga_4 as batch4;

// Remove the 25 commented-out // pub mod entries (they are now in batch4)
```

---

## 09-03 Plan Scope Clarification

The existing 09-03-PLAN.md describes tasks that are **already complete**:

| 09-03-PLAN Task | Actual Status |
|-----------------|---------------|
| Task 1: Configure sccache, set incremental=false | DONE (`rustc-wrapper = "sccache"`, `incremental = false` in profiles) |
| Task 2: Remove redundant profile sections | DONE (0 sub-crate profile sections found) |
| Task 3: Verify sccache caching | Partially done (infrastructure correct, just needs a build run) |

The **new 09-03 plan** should instead focus on what actually remains to satisfy
phase success criteria 2 and 3:

1. Add `[features]` to root `Cargo.toml` for lda/gga/mgga/all-kernels family gating
2. Make kernel-gga and kernel-mgga optional deps in root Cargo.toml
3. Add `#[cfg(feature = "gga")]` and `#[cfg(feature = "mgga")]` to src/kernel/mod.rs
4. Create `crates/kernel-gga-4/` as a new workspace crate
5. Move 25 deferred GGA functionals from `crates/kernel-gga/src/` to `crates/kernel-gga-4/src/`
6. Add derivative-order feature gates to kernel-gga-4 mod.rs files for each functional
7. Add kernel-gga-4 to workspace members and kernel-gga facade deps
8. Verify: `cargo check` passes (no features); `cargo check --features gga` also passes
9. Verify: `cargo check --features gga,order-kxc` compiles kxc files
10. Confirm sccache is active by checking `sccache --show-stats` post-build

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| All kernels compiled unconditionally | Feature-gated by family (proposed) | 09-03 plan | ~98% reduction in default build time |
| 25 GGA functionals in facade src/ (commented out) | In kernel-gga-4 with order gating | 09-03 plan | 25 more functionals enabled |
| sccache non-functional (incremental=true) | sccache working (incremental=false) | Post 09-02 | Cache hits on unchanged kernel rebuilds |

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| sccache | Build caching | YES | 0.14.0 | Remove rustc-wrapper |
| Rust/Cargo | All | YES | (workspace edition 2024) | N/A |

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | OOM threshold is ~5,000-5,500 lines per `#[cube(launch_unchecked)]` function | GGA Kernel Structure | If threshold is lower, some currently-compiled functionals might also OOM on some machines; if higher, the order gating might not be needed for some files |
| A2 | exc/vxc/fxc for all 25 deferred functionals compile without OOM (fxc_pol.rs max 3,854 lines) | Architecture | If wrong, order-fxc gating would also be needed (adds minor complexity) |
| A3 | Feature forwarding from kernel-gga facade to kernel-gga-4 works for order-kxc/lxc | Code Examples | Standard Cargo dep feature forwarding — should work per Cargo spec |

---

## Open Questions

1. **Should kernel-gga-{1,2,3} also get derivative-order feature gates?**
   - What we know: Their largest lxc_pol.rs is 4,889 lines (below OOM threshold), so they compile fine
   - What's unclear: Future additions might push them over 5K
   - Recommendation: No for now. Don't add unnecessary complexity to working crates.

2. **Should `order-kxc` and `order-lxc` be separate from the family gating?**
   - What we know: Higher-order derivatives are rarely needed (most DFT uses exc+vxc)
   - What's unclear: Whether existing callers depend on kxc/lxc from GGA
   - Recommendation: Add them as separate optional features in kernel-gga-4 Cargo.toml
     but do NOT add them to the workspace root's `[features]` yet (scope creep risk).
     The primary goal is enabling the 25 functionals at exc+vxc+fxc level.

3. **Does sccache actually produce cache hits in practice?**
   - What we know: Configuration is correct (rustc-wrapper + incremental=false)
   - What's unclear: Whether CubeCL proc macro output is deterministic enough for sccache
   - Recommendation: Run `sccache --zero-stats && cargo check && sccache --show-stats`
     to verify after implementing feature gating.

---

## Validation Architecture

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| BUILD-OPT-02 | Default build compiles only LDA | build | `cargo check 2>&1 \| grep -v kernel-gga \| grep -v kernel-mgga` |
| BUILD-OPT-03 | `--features gga` enables GGA | build | `cargo check --features gga` |
| BUILD-OPT-03 | `--features all-kernels` enables all | build | `cargo check --features all-kernels` |
| GGA unblock | All 25 deferred GGA functionals compile | compile | `cargo check --features gga -p libxc-kernel-gga-4` |

---

## Sources

### Primary (HIGH confidence)
- Live codebase inspection: `/workspace/crates/kernel-gga*/`, `/workspace/Cargo.toml`, `/workspace/.cargo/config.toml`
- Debug analysis: `/workspace/.planning/debug/kernel-build-time.md`
- Plan summaries: `09-01-SUMMARY.md`, `09-02-SUMMARY.md`

### Secondary (MEDIUM confidence)
- Cargo reference on optional dependencies and feature forwarding [ASSUMED — standard Cargo behavior]
- Cargo book on `[build]` section keys (`rustc-wrapper` vs incorrect `compiler`) [ASSUMED — standard toolchain]

---

## Metadata

**Confidence breakdown:**
- Current state (what's done vs undone): HIGH — verified against live files
- OOM threshold (~5K lines): MEDIUM — inferred from compiled max (4,889) vs deferred min (5,518)
- Feature gating pattern: HIGH — standard Rust/Cargo
- Build time estimates: MEDIUM — extrapolated from measured LDA baseline

**Research date:** 2026-04-14
**Valid until:** This research reflects a specific codebase snapshot; valid until the next kernel re-translation or Cargo.toml restructuring.
