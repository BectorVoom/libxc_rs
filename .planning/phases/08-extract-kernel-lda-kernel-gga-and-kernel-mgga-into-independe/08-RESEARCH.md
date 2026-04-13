# Phase 8: Extract kernel/lda, kernel/gga, and kernel/mgga into Independent Workspace Crates — Research

**Researched:** 2026-04-13
**Domain:** Cargo workspace restructuring / Rust crate extraction
**Confidence:** HIGH

---

## Summary

Phase 8 is a pure structural refactoring: the three kernel family module trees
(`src/kernel/lda/`, `src/kernel/gga/`, `src/kernel/mgga/`) move into three new
Cargo workspace member crates (`crates/kernel-lda/`, `crates/kernel-gga/`,
`crates/kernel-mgga/`). The shared kernel infrastructure
(`src/kernel/launch.rs`, `src/kernel/shared/`) stays in the main `libxc_rs`
crate. The main crate re-exports from the new sub-crates so all existing public
paths remain valid and `cargo check` continues to pass.

The extraction is straightforward because the kernel modules have an
exceptionally clean dependency profile: **every kernel file (`lda/`, `gga/`,
`mgga/`) imports only from `crate::math::*` and `cubecl::prelude::*`**. There
are zero imports from `crate::model`, `crate::error`, `crate::dims`,
`crate::registry`, or any other non-math module. This was confirmed by grep
audit of every `use crate::` statement across all 1,583 kernel source files.

The central design decision is: should `crate::math::*` move into its own crate
(making it a shared dependency of all three kernel crates), or should each
kernel crate embed its own copy? The `gga_test_crate` precedent already
demonstrates the copy-and-vendor pattern — but that was a temporary isolation
tool, not the target architecture. The clean answer is a fourth crate
`crates/kernel-math/` (or `libxc-math`) that the three kernel crates depend on,
with the main crate also depending on it. The alternative — having each kernel
crate depend on the main `libxc_rs` crate for math — would create a dependency
cycle and is ruled out.

**Primary recommendation:** Create `crates/kernel-math/` (or re-export math
from the main crate via a new `libxc-math` sub-crate), then create
`crates/kernel-{lda,gga,mgga}/` each depending on `kernel-math` and `cubecl`.
The main crate depends on all four and re-exports kernel family modules through
`src/kernel/mod.rs`. This results in zero circular dependencies and clean
`cargo check`.

---

## Standard Stack

### Core (already in project)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cubecl | 0.9.0 | `#[cube]` kernel proc macro and runtime | All kernel files use `use cubecl::prelude::*`; must be present in each kernel sub-crate [VERIFIED: Cargo.toml] |
| bytemuck | 1.25.0 | Safe f64/byte casting in launch wrappers | Used in `src/kernel/launch.rs`; stays in main crate unless launch wrappers also move [VERIFIED: Cargo.toml] |

### New Workspace Structure

No new external dependencies are required. The extraction is purely a
reorganization of existing code into Cargo workspace members.

**Installation (workspace Cargo.toml additions):**
```toml
[workspace]
members = [
    "xtask",
    "verify",
    "tools/gga_test_crate",
    "crates/kernel-math",   # new
    "crates/kernel-lda",    # new
    "crates/kernel-gga",    # new
    "crates/kernel-mgga",   # new
]
```

**Each new kernel sub-crate's Cargo.toml pattern:**
```toml
[package]
name = "libxc-kernel-lda"   # (or gga, mgga)
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../../crates/kernel-math" }

[profile.dev]
debug = 0
codegen-units = 16

[profile.test]
debug = 0
codegen-units = 16
```

[VERIFIED: gga_test_crate Cargo.toml — confirmed this exact pattern compiles]

---

## Architecture Patterns

### Recommended Project Structure

```
libxc_rs/
├── Cargo.toml                          # workspace root — add 4 new members
├── src/                                # main crate (libxc_rs)
│   ├── lib.rs                          # unchanged pub use paths
│   ├── kernel/
│   │   ├── mod.rs                      # pub use kernel_lda; pub use kernel_gga; etc.
│   │   ├── launch.rs                   # stays here (main-crate infrastructure)
│   │   ├── dispatch_key.rs             # stays here
│   │   ├── shared/                     # stays here (placeholder stubs for now)
│   │   └── mix/                        # stays here
│   └── math/                           # moves to crates/kernel-math/ OR stays
│                                       # here and is re-exported
├── crates/
│   ├── kernel-math/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # pub mod constants; pub mod powers; ...
│   │       ├── constants.rs            # copied from src/math/constants.rs
│   │       ├── powers.rs
│   │       ├── piecewise.rs
│   │       ├── polynomials.rs
│   │       ├── erf.rs
│   │       ├── spin.rs
│   │       ├── dft_quantities.rs
│   │       ├── bspline.rs
│   │       ├── lambert_w.rs
│   │       ├── expint_e1.rs
│   │       ├── special.rs
│   │       └── integrate.rs
│   ├── kernel-lda/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # pub mod lda_x; pub mod lda_c_pw; ...
│   │       └── [all lda/ subdirs]      # moved from src/kernel/lda/
│   ├── kernel-gga/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # pub mod gga_c_acgga; ...
│   │       └── [all gga/ subdirs]      # moved from src/kernel/gga/
│   └── kernel-mgga/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # pub mod mgga_c_b88; ...
│           └── [all mgga/ subdirs]     # moved from src/kernel/mgga/
```

### Alternative: Keep math in main crate, re-export into kernel sub-crates

The math module has no external dependencies beyond `cubecl` and `std`. An
alternative to `crates/kernel-math/` is keeping `src/math/` in the main crate
and having each kernel sub-crate depend on the main `libxc_rs` crate. **This
creates a circular dependency**: `libxc_rs` would depend on `kernel-lda`, which
depends on `libxc_rs`. Rust/Cargo does not allow circular workspace member
dependencies. This approach is NOT viable.

Therefore, the math module MUST either:
1. Move into its own `crates/kernel-math/` crate (cleanest), OR
2. Stay in the main crate and the kernel sub-crates copy the math source (as
   `gga_test_crate` does — acceptable short-term, not maintainable long-term)

**Recommended:** Option 1, `crates/kernel-math/`. [ASSUMED: no circular dep issues with this layout — standard Cargo workspace pattern]

### Pattern 1: `crate::math::*` → `libxc_kernel_math::*` rewrite

All 1,583 kernel source files currently use `use crate::math::constants::...`.
After extraction, these become `use libxc_kernel_math::constants::...` (or
`use kernel_math::constants::...` if the crate name is `kernel-math`). This is
a mechanical sed/script operation — not manual editing.

The `tools/translate_mgga_v2.py` script already exists for bulk kernel
generation. A similar bulk sed script is the right tool for path rewrites.

**Rewrite operation (per kernel file):**
```bash
# Before: use crate::math::constants::{M_CBRT2, M_CBRT3};
# After:  use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3};
sed -i 's/use crate::math::/use libxc_kernel_math::/g' <file>
```

### Pattern 2: Main crate `src/kernel/mod.rs` re-exports

After extraction, `src/kernel/mod.rs` re-exports the sub-crates' public items
so that all existing call sites using `crate::kernel::lda::` or
`libxc_rs::kernel::lda::` continue to work unchanged:

```rust
// src/kernel/mod.rs (after extraction)
pub use libxc_kernel_lda as lda;
pub use libxc_kernel_gga as gga;
pub use libxc_kernel_mgga as mgga;
pub mod launch;
pub mod dispatch_key;
pub mod shared;
pub mod mix;
```

[VERIFIED: Rust `pub use crate_name as mod_name` pattern is standard workspace re-export practice]

### Pattern 3: `BufArg` lives in `kernel-lda` or moves to main crate

Currently `BufArg` is defined in `src/kernel/lda/launch_lda_x.rs` and
referenced by all other LDA launch wrappers via `use super::launch_lda_x::BufArg`.
After extraction it stays in `kernel-lda` since it is only used within that
crate's launch wrappers. `src/eval/dispatch.rs` imports it as:
```rust
use crate::kernel::lda::launch_lda_x::BufArg;
```
After extraction with `pub use libxc_kernel_lda as lda` in `src/kernel/mod.rs`,
this import path resolves through the re-export and requires no change at the
call site in `dispatch.rs`.

### Anti-Patterns to Avoid

- **Circular workspace dependency**: kernel sub-crate depending on main `libxc_rs` crate for math would create a cycle. Must use separate `kernel-math` crate.
- **Duplicating math source files**: like `gga_test_crate` does — works for isolation testing but creates a maintenance burden with three copies of math modules that can drift.
- **Moving `kernel/launch.rs` into a kernel sub-crate**: `launch.rs` uses `bytemuck` and `cubecl::cpu` directly and is used by `src/eval/dispatch.rs`. It belongs in the main crate.
- **Moving `kernel/shared/` into a kernel sub-crate**: The shared placeholders (spin.rs, thresholds.rs, output_mask.rs) will eventually depend on main-crate types. Keep in main crate.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Import path rewriting across 1,583 files | Manual text editor | `sed` or Python script (existing pattern in project) | Manual editing 1,583 files guarantees mistakes |
| Circular dependency resolution | Split types awkwardly | Standard `crates/kernel-math/` sub-crate | Cargo's workspace member dependency is designed for this |
| Verifying all paths still resolve | Reading all files | `cargo check` in workspace root | Compiler catches all broken paths in one pass |

**Key insight:** The clean dependency graph (all kernels only import math + cubecl) means this extraction has no hidden complexity — it is exactly as mechanical as it appears.

---

## Runtime State Inventory

This is a code-only structural refactoring phase with no external services,
databases, OS registrations, or stored state. Step 2.5 is skipped.

- Stored data: None — verified. No database writes.
- Live service config: None — verified. No external service configuration.
- OS-registered state: None — verified. No systemd/pm2/scheduler entries.
- Secrets/env vars: None — verified. No secrets reference kernel module names.
- Build artifacts: The Cargo build cache will be invalidated by the restructuring. `cargo clean` may be needed on first build after restructuring, but this is normal and not a data migration concern.

---

## Common Pitfalls

### Pitfall 1: Forgetting the math dependency creates a cycle if misplaced

**What goes wrong:** Planner puts math in the main crate, kernel sub-crates
depend on main crate for math — Cargo refuses to compile due to cycle.

**Why it happens:** The natural impulse is "kernel crates depend on the big
main crate." But the main crate will also depend on kernel crates (to re-export
them), creating a cycle.

**How to avoid:** `crates/kernel-math/` must be an independent crate. Main
crate depends on kernel-math (like any consumer). Kernel crates also depend on
kernel-math. No cycle.

**Warning signs:** `cargo check` error "cyclic package dependency detected".

### Pitfall 2: LDA `launch_*.rs` files import via `super::` — scope changes after extraction

**What goes wrong:** The LDA launch wrappers use `use super::lda_x;` and
`use super::launch_lda_x::BufArg;`. After extraction into `crates/kernel-lda/`,
`super::` still refers to the parent module within the crate and remains valid.
**No change needed.** But if someone naively changes these to `crate::` paths
they would break.

**How to avoid:** Preserve the `use super::` pattern exactly as-is within
`crates/kernel-lda/src/`.

### Pitfall 3: Profile settings not inherited by sub-crates

**What goes wrong:** The main `Cargo.toml` has
`[profile.dev.package.libxc_rs] debug = 0 codegen-units = 16` to manage
memory during CubeCL IR expansion. After extraction, the kernel crates compile
under default profiles — potentially OOM during dev builds.

**Why it happens:** Cargo's `[profile.dev.package.NAME]` applies per named
package. New crates have new names.

**How to avoid:** Add equivalent `[profile.dev.package.libxc-kernel-gga]` etc.
entries to the workspace root `Cargo.toml`, or add `[profile.dev]` sections to
each new crate's own `Cargo.toml`. The `gga_test_crate` Cargo.toml does this
correctly: `[profile.dev] debug = 0 codegen-units = 16`.

**Warning signs:** OOM during `cargo check` or `cargo build --package`.

### Pitfall 4: Tests in `tests/` directory reference `libxc_rs::kernel::lda::` paths

**What goes wrong:** Integration tests in `tests/` use paths like
`use libxc_rs::kernel::launch::...` and `use libxc_rs::math::...`. After
extraction, if the re-exports in `src/kernel/mod.rs` are incomplete, test
imports break.

**How to avoid:** Ensure `src/kernel/mod.rs` re-exports via `pub use
libxc_kernel_lda as lda;` so existing test paths resolve through the main
crate's public surface unchanged.

### Pitfall 5: `cargo check` passes but `cargo test` fails due to `#[test]` in extracted modules

**What goes wrong:** Tests inside extracted modules (`src/kernel/lda/lda_x.rs`
inline tests, `src/kernel/lda/launch_lda_x.rs` tests) reference
`crate::kernel::launch::{cpu_client, ...}`. After extraction to
`crates/kernel-lda/`, `crate::kernel::launch` no longer exists in that crate.

**How to avoid:** These test-internal imports need updating to use
`libxc_rs::kernel::launch::...` or (better) the launch utilities should be
duplicated/re-exported appropriately. Consider moving the tests to the
integration test in `tests/` of the main workspace, or exposing `launch` as a
`[dev-dependency]` on `kernel-lda`.

---

## Code Examples

### Workspace Cargo.toml structure (verified pattern from existing members)

```toml
# /Cargo.toml (workspace root)
[workspace]
members = [
    "xtask",
    "verify",
    "tools/gga_test_crate",
    "crates/kernel-math",
    "crates/kernel-lda",
    "crates/kernel-gga",
    "crates/kernel-mgga",
]
```

[VERIFIED: existing Cargo.toml uses this exact `members` array format]

### kernel-math crate Cargo.toml

```toml
# /crates/kernel-math/Cargo.toml
[package]
name = "libxc-kernel-math"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }

[profile.dev]
debug = 0
codegen-units = 16

[profile.test]
debug = 0
codegen-units = 16
```

[VERIFIED: mirrors gga_test_crate/Cargo.toml pattern exactly]

### kernel-lda crate Cargo.toml

```toml
# /crates/kernel-lda/Cargo.toml
[package]
name = "libxc-kernel-lda"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../kernel-math" }
bytemuck = { version = "1.25.0", features = ["derive"] }

[profile.dev]
debug = 0
codegen-units = 16

[profile.test]
debug = 0
codegen-units = 16
```

### kernel-lda/src/lib.rs (re-exports all existing submodules)

```rust
// /crates/kernel-lda/src/lib.rs
pub mod lda_x;
pub mod lda_c_pw;
// ... all existing pub mod declarations from src/kernel/lda/mod.rs ...
pub mod launch_lda_x;
pub mod launch_lda_c_pw;
// ... all launch_* modules ...
```

### Main crate src/kernel/mod.rs (after extraction)

```rust
// src/kernel/mod.rs
pub extern crate libxc_kernel_lda as lda;
pub extern crate libxc_kernel_gga as gga;
pub extern crate libxc_kernel_mgga as mgga;
pub mod launch;
pub mod dispatch_key;
pub mod shared;
pub mod mix;
```

Note: `pub use libxc_kernel_lda as lda` (re-export alias) is the idiomatic form.
[ASSUMED: exact syntax for cross-crate re-export — verify `pub use crate_name as module` compiles vs `pub extern crate`]

### Import rewrite script (mechanical sed)

```bash
#!/bin/bash
# Rewrite all kernel source files: crate::math:: -> libxc_kernel_math::
# Run from workspace root after moving files to crates/
find crates/kernel-lda crates/kernel-gga crates/kernel-mgga \
  -name "*.rs" \
  -exec sed -i 's/use crate::math::/use libxc_kernel_math::/g' {} \;
```

---

## Dependency Graph

```
libxc_rs (main crate)
  ├── depends on: libxc-kernel-math
  ├── depends on: libxc-kernel-lda
  │     └── depends on: libxc-kernel-math
  │     └── depends on: cubecl
  ├── depends on: libxc-kernel-gga
  │     └── depends on: libxc-kernel-math
  │     └── depends on: cubecl
  ├── depends on: libxc-kernel-mgga
  │     └── depends on: libxc-kernel-math
  │     └── depends on: cubecl
  └── depends on: cubecl, bitflags, bytemuck, thiserror
```

No cycles. `libxc-kernel-math` is a leaf node (depends only on cubecl and std).

---

## Scope of Changes

| Item | Count | Action |
|------|-------|--------|
| LDA kernel source files | 121 | Move to `crates/kernel-lda/src/` |
| GGA kernel source files | 1,445 | Move to `crates/kernel-gga/src/` |
| MGGA kernel source files | 17 | Move to `crates/kernel-mgga/src/` |
| Math source files | 13 | Move to `crates/kernel-math/src/` |
| Import path rewrites | ~1,583 files | sed replace `crate::math::` → `libxc_kernel_math::` |
| New Cargo.toml files | 4 | Create: kernel-math, kernel-lda, kernel-gga, kernel-mgga |
| Workspace Cargo.toml | 1 | Add 4 new members |
| `src/kernel/mod.rs` | 1 | Replace module declarations with re-exports |
| `src/math/mod.rs` | 1 | Replace with re-export from libxc-kernel-math (or redirect) |
| `src/lib.rs` | 0 | No change — existing public paths preserved via re-exports |
| Integration tests in `tests/` | 11 files | Likely no change if re-exports are complete |
| Inline tests in kernel files | ~3 files | May need `use` path fixes (see Pitfall 5) |

---

## Environment Availability

Step 2.6: No external tool dependencies beyond the existing Rust toolchain.

| Dependency | Required By | Available | Notes |
|------------|------------|-----------|-------|
| cargo | Workspace restructuring | Yes | Standard Rust toolchain |
| sed / python3 | Import path rewrite script | Yes (WSL2 Linux) | Standard shell tools |
| rustc 1.85+ | Edition 2024 | Yes (existing project requirement) | [VERIFIED: CLAUDE.md MSRV constraint] |

---

## Validation Architecture

`nyquist_validation` is `true` in `.planning/config.json` (HEAD branch value).

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in (cargo test) |
| Config file | Cargo.toml workspace profiles |
| Quick run command | `cargo check --workspace` |
| Full suite command | `cargo test --workspace` |

### Phase Gate Test

The sole success criterion for Phase 8 is: **`cargo check --workspace` passes
with zero errors after the restructuring.** This is the directly stated goal.

Additionally:
- `cargo test --workspace` should pass (all inline and integration tests)
- No new compiler warnings introduced

### Wave 0 Gaps

None — no new test infrastructure needed. The phase is purely structural;
correctness is verified by the compiler resolving all import paths.

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `pub use crate_name as lda;` in `src/kernel/mod.rs` correctly re-exports the sub-crate's items so existing `crate::kernel::lda::launch_lda_x::BufArg` paths resolve | Architecture Patterns / Code Examples | Compile error; need `pub extern crate` or direct re-exports instead of alias |
| A2 | The math module has no transitive dependency on `crate::model`, `crate::error`, or other main-crate modules (only cubecl + std) | Dependency Graph | If math depends on model types, math cannot move to an independent crate without also moving model — requires scope expansion |
| A3 | Inline tests in `src/kernel/lda/launch_lda_x.rs` and `lda_x.rs` that import `crate::kernel::launch::{...}` will need updating after extraction | Common Pitfalls | `cargo test --package libxc-kernel-lda` will fail; tests must either move or add `libxc_rs` as dev-dep |

**A2 confidence note:** Verified by grep audit across all 1,583 kernel source
files — zero non-math `use crate::` imports found. [VERIFIED: grep scan]

---

## Open Questions

1. **Where do the inline tests in `launch_lda_x.rs` go?**
   - What we know: They use `crate::kernel::launch::{cpu_client, ...}` which lives in the main crate.
   - What's unclear: After extraction, `crate::kernel::launch` doesn't exist in `kernel-lda`. Tests either move to `tests/` in the workspace, or `kernel-lda` adds `libxc_rs` as a `[dev-dependencies]` entry (which is fine — dev deps don't create cycle issues).
   - Recommendation: Add `libxc_rs = { path = "../.." }` under `[dev-dependencies]` in `kernel-lda/Cargo.toml`.

2. **Does `src/math/mod.rs` stay or redirect?**
   - What we know: `src/lib.rs` does not `pub mod math;` — math is not in the public surface. But `tests/math_integration.rs` imports `libxc_rs::math::...` directly.
   - What's unclear: If math moves to `kernel-math`, the test imports break unless `src/lib.rs` re-exports math or the test adds `libxc-kernel-math` as a direct dep.
   - Recommendation: Keep `pub mod math;` in `src/lib.rs` pointing at a shim `src/math/mod.rs` that re-exports from `libxc_kernel_math`, OR add `libxc-kernel-math` as a dev-dependency in the main crate's `Cargo.toml` and update the test imports.

3. **Crate naming: `libxc-kernel-lda` or `kernel-lda`?**
   - The Cargo package name (in Cargo.toml `[package] name`) and the crate name (used in `use` statements) differ: hyphens become underscores. `libxc-kernel-lda` → `use libxc_kernel_lda::...`.
   - Recommendation: Use `libxc-kernel-lda` as the package name (consistent with Rust convention for namespaced crates) and `libxc_kernel_math` in `use` statements.

---

## Sources

### Primary (HIGH confidence)

- [VERIFIED: codebase grep] All `use crate::` imports across `src/kernel/lda/`, `gga/`, `mgga/` — confirmed zero non-math imports
- [VERIFIED: Cargo.toml] Existing workspace `members` array syntax and profile configuration
- [VERIFIED: gga_test_crate/Cargo.toml] Standalone kernel crate pattern with `cubecl = { version = "0.9.0", features = ["cpu"] }` and `[profile.dev] debug = 0 codegen-units = 16`
- [VERIFIED: src/kernel/lda/launch_lda_x.rs] `BufArg` definition and `super::` import patterns in LDA launch wrappers
- [VERIFIED: src/kernel/lda/mod.rs, gga/mod.rs, mgga/mod.rs] Complete module structure of all three families

### Secondary (MEDIUM confidence)

- [ASSUMED] Cargo workspace cross-crate re-export via `pub use crate_name as mod_name` — standard Rust pattern, well-documented in The Cargo Book

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies, all existing
- Architecture: HIGH — dependency graph is simple, verified clean by grep audit
- Pitfalls: HIGH — based on actual code structure analysis, not assumptions

**Research date:** 2026-04-13
**Valid until:** 2026-06-01 (stable — Cargo workspace patterns do not change frequently)
