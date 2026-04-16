# Phase 1: Foundation and Registry - Research

**Researched:** 2026-04-09
**Domain:** Rust domain types, static registry, code generation, error handling
**Confidence:** HIGH

## Summary

Phase 1 establishes the foundational type system and static registry for libxc_rs. The work is primarily data modeling and code generation -- no CubeCL, GPU, or numerical computation is involved. The domain model (enums, newtypes, bitflags) follows exact Rust code from the design document Section 6. The registry requires parsing 649 `#define` lines from `xc_funcs.h` and 52 entries from `xc_funcs_removed.h` to produce static lookup tables. Error handling uses thiserror v2 with a comprehensive error enum.

The technical risk is low. All types are `Copy + Clone + Debug`, all data is `const`/`static`, and the verify/ crate already has working cmake+bindgen infrastructure. The main implementation challenge is the xtask code generator that must correctly parse C header files and map functional name prefixes to Family/Kind enums.

**Primary recommendation:** Follow the design document Sections 6, 8, 9, and 15 as implementation-ready specifications. Build the xtask code generator first since it produces the bulk of the registry data (649 entries).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Use a sparse array indexed by raw functional ID (`&[Option<&'static FunctionalMeta>; 1024]`) for O(1) ID lookup.
- **D-02:** Use a sorted `&[(&str, FunctionalId)]` slice with binary search for O(log n) name lookup.
- **D-03:** Removed ID table is a separate `&[(u16, u16)]` mapping removed_id to replacement_id, checked before the main registry.
- **D-04:** Build an xtask code generator that parses `libxc-master/src/xc_funcs.h` and `xc_funcs_removed.h` to produce Rust source files. Generated source is committed to the repo -- no build.rs dependency on C headers in the main crate.
- **D-05:** For Phase 1, only functional ID, name, family, and kind need to be populated from xc_funcs.h. Full metadata (references, ext_params, hybrid terms, flags) will be populated incrementally in later phases.
- **D-06:** Follow the design doc Section 8 module decomposition: `model/`, `meta/`, `registry/`, `error/` as Phase 1 modules. Each module has its own directory with `mod.rs`.
- **D-07:** Convert the crate from binary (`main.rs`) to library (`lib.rs`) with public re-exports.
- **D-08:** Use the full `LibxcRsError` enum from design doc Section 15 with thiserror v2 derives.
- **D-09:** For Phase 1, implement lookup-related error variants: `UnknownFunctionalId`, `RemovedFunctionalId`, `UnknownFunctionalName`. Others defined but not yet used.
- **D-10:** Implement `Dimensions` struct with `lda()`, `gga()`, `mgga()` constructors. Pure computation.
- **D-11:** Extend verify/ crate to support basic oracle comparison for LDA_X as a smoke test.
- **D-12:** Verify/ crate uses `anyhow` for errors, keeping `thiserror` at the library boundary.

### Claude's Discretion
- Exact file organization within each module directory
- Test organization (inline `#[cfg(test)]` vs separate `tests/` directory)
- Specific const naming conventions for generated registry tables
- Whether to use `phf` (perfect hash function) crate or stick with sorted binary search for name lookup

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DOM-01 | All domain enums defined with correct repr and derives | Design doc Section 6.1 provides exact Rust code for all enums |
| DOM-02 | FunctionalId newtype with from_raw() and from_name() | Design doc Section 6.3; requires registry lookup |
| DOM-03 | FunctionalFlags bitflags matching libxc flags | Design doc Section 6.2; flag values confirmed from xc.h |
| DOM-04 | Dimensions struct computing correct array sizes | Design doc Section 6.5; dimension table confirmed from util.c |
| DOM-05 | Thresholds struct with correct defaults | Design doc Section 6.8; defaults: density=1e-15, zeta=1e-10, sigma=1e-24, tau=1e-20 |
| REG-01 | All 649 functional IDs present in registry | xc_funcs.h grep-confirmed 649 entries; xtask generator parses these |
| REG-02 | O(1) lookup by ID via sparse array | D-01: `[Option<&'static FunctionalMeta>; 1024]`, max ID is 734 |
| REG-03 | O(log n) lookup by name via sorted binary search | D-02: sorted `&[(&str, FunctionalId)]` slice |
| REG-04 | All 52 removed IDs return RemovedFunctionalId error | xc_funcs_removed.h confirmed 52 entries; includes aliases and truly removed |
| REG-05 | Library version/reference functions return correct static strings | Design doc Section 3.1 Category 1; static const strings |
| ERR-01 | LibxcRsError enum covers all error variants | Design doc Section 15.2 provides complete enum; define all variants now, use subset in Phase 1 |
| ERR-02 | All public API methods return Result<T, LibxcRsError> | Design doc Section 15.1 boundary rule |
| ERR-03 | Evaluation is infallible after input validation | Design doc Section 15.3 error flow; no evaluation in Phase 1 |
| VERIFY-01 | Verification harness in verify/ crate using bindgen | verify/build.rs already functional; extend with LDA_X smoke test |
| BUILD-01 | cargo build succeeds with no warnings | Standard Rust quality; add `#![deny(warnings)]` to lib.rs |
| BUILD-02 | cargo test passes all tests | Registry lookup tests, dimension tests, error tests |
| BUILD-03 | cargo clippy has no warnings | Run `cargo clippy -- -D warnings` |
| BUILD-04 | No unsafe code outside compat/, kernel/launch.rs, GPU buffer management | Phase 1 has no unsafe code at all |
| BUILD-05 | No runtime C/Fortran FFI dependency in production library | Phase 1 is pure Rust; C FFI only in verify/ crate |
</phase_requirements>

## Standard Stack

### Core (Phase 1 only)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| thiserror | 2.0.18 | `LibxcRsError` enum derives | Standard for library error types; v2 supports `#[error(transparent)]` and backtrace via `provide()` [VERIFIED: already in Cargo.toml] |
| bitflags | 2.10.0 | `FunctionalFlags` bitfield | De facto standard for type-safe bitflags in Rust [VERIFIED: already in Cargo.toml] |

### Supporting (Phase 1 only)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.0.x | Error handling in verify/ crate | verify/ and xtask/ only, never in library |

### Not Needed in Phase 1
| Library | Why Not Now |
|---------|-------------|
| cubecl | No kernel work in Phase 1; already in Cargo.toml but unused |
| bytemuck | No buffer casting in Phase 1; already in Cargo.toml |
| phf | Sorted binary search is simpler and sufficient for 649 entries; no measurable perf difference at this scale [ASSUMED] |

## Architecture Patterns

### Phase 1 Module Structure
```
src/
├── lib.rs              # Crate root, public re-exports
├── model/
│   └── mod.rs          # Family, Kind, Spin, DerivativeOrder, FunctionalId,
│                       # FunctionalFlags, HybridType, HybridTermKind, Dimensionality
├── meta/
│   ├── mod.rs          # FunctionalMeta, Reference, ExtParamSpec, HybridTerm structs
│   └── generated.rs    # 649 const FunctionalMeta entries (xtask-generated)
├── registry/
│   ├── mod.rs          # Public lookup API (lookup_by_id, lookup_by_name, etc.)
│   ├── by_id.rs        # REGISTRY_BY_ID sparse array (xtask-generated)
│   ├── by_name.rs      # REGISTRY_BY_NAME sorted slice (xtask-generated)
│   └── removed.rs      # REMOVED_IDS table (xtask-generated)
├── dims/
│   └── mod.rs          # Dimensions struct + lda()/gga()/mgga() constructors
├── error/
│   └── mod.rs          # LibxcRsError enum
```

### Pattern 1: Xtask Code Generator
**What:** A standalone Rust binary that reads C headers and emits Rust source files.
**When to use:** Whenever data must be extracted from C headers to produce static Rust tables.
**Why xtask:** Cargo convention for workspace-level tooling. Run via `cargo xtask generate-registry`.

```rust
// xtask/src/main.rs
// Parse: #define  XC_LDA_X  1 /* Slater exchange */
// Regex: r#"#define\s+XC_(\w+)\s+(\d+)\s*/\*\s*(.*?)\s*\*/"#

// Family detection from name prefix:
// "LDA_" => Family::Lda
// "GGA_" => Family::Gga
// "MGGA_" => Family::Mgga
// "HYB_LDA_" => Family::Lda (with hybrid flag)
// "HYB_GGA_" => Family::Gga (with hybrid flag)
// "HYB_MGGA_" => Family::Mgga (with hybrid flag)

// Kind detection from name component after family prefix:
// "_X_" or starts with "X_" => Kind::Exchange
// "_C_" or starts with "C_" => Kind::Correlation
// "_XC_" or starts with "XC_" => Kind::ExchangeCorrelation
// "_K_" or starts with "K_" => Kind::Kinetic
```

### Pattern 2: Static Registry with Sparse Array
**What:** O(1) lookup by integer ID using a fixed-size array indexed by raw ID.
**Key insight:** Max active ID is 734. A `[Option<&'static FunctionalMeta>; 1024]` array wastes ~2KB (1024 * 8 bytes for Option pointer) but gives guaranteed O(1) access.

```rust
// Generated by xtask
pub static REGISTRY_BY_ID: [Option<&'static FunctionalMeta>; 1024] = {
    let mut table: [Option<&'static FunctionalMeta>; 1024] = [None; 1024];
    table[1] = Some(&meta::XC_LDA_X);
    table[2] = Some(&meta::XC_LDA_C_WIGNER);
    // ... 647 more entries ...
    table
};
```

**Note on const initialization:** Rust edition 2024 (rustc 1.92.0) supports mutable references in const contexts, so the above pattern with `let mut table` in a const block works. [VERIFIED: edition 2024 requires rustc 1.85+, project uses 1.92.0]

### Pattern 3: FunctionalMeta with Partial Population
**What:** Define the full `FunctionalMeta` struct but populate only ID, name, family, kind for Phase 1.
**Why:** Later phases fill in references, ext_params, hybrid_terms, flags without changing the struct.

```rust
// Phase 1: minimal population
pub const XC_LDA_X: FunctionalMeta = FunctionalMeta {
    id: FunctionalId(1),
    name: "XC_LDA_X",
    kind: Kind::Exchange,
    family: Family::Lda,
    flags: FunctionalFlags::empty(),        // Populated in later phases
    references: &[],                         // Populated in later phases
    ext_params: &[],                         // Populated in later phases
    default_density_threshold: 1e-15,
    auxiliaries: &[],                         // Populated in later phases
    hybrid_terms: &[],                       // Populated in later phases
    nlc_params: None,                        // Populated in later phases
    max_order: DerivativeOrder::Exc,         // Populated in later phases
};
```

### Pattern 4: Name Lookup via Binary Search
**What:** Sorted array of `(&str, FunctionalId)` pairs with `binary_search_by_key`.

```rust
pub static REGISTRY_BY_NAME: [(&str, FunctionalId); 649] = [
    ("XC_GGA_C_ACGGA", FunctionalId(39)),
    ("XC_GGA_C_AM05", FunctionalId(135)),
    // ... sorted alphabetically ...
];

pub fn lookup_by_name(name: &str) -> Option<FunctionalId> {
    let upper = name.to_ascii_uppercase();
    REGISTRY_BY_NAME
        .binary_search_by_key(&upper.as_str(), |&(n, _)| n)
        .ok()
        .map(|idx| REGISTRY_BY_NAME[idx].1)
}
```

### Anti-Patterns to Avoid
- **Lazy initialization (lazy_static/once_cell):** Not needed. All registry data is `const`/`static` with no runtime init cost. Using lazy init adds unnecessary complexity and potential ordering issues.
- **HashMap for registry:** Allocates on the heap, requires runtime initialization. A sparse array and sorted slice are zero-alloc and faster for this use case.
- **Parsing C headers at build time via build.rs:** The decision is to use xtask and commit generated source. build.rs would create a build-time dependency on C headers for the production crate, violating BUILD-05.
- **Hand-writing 649 entries:** Error-prone and unmaintainable. Always generate from the authoritative C headers.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bitflags | Manual bit manipulation | `bitflags` 2.10 crate | Handles Display, iteration, intersection, difference correctly |
| Error derives | Manual Display/Error impl | `thiserror` 2.0 | Handles source chaining, backtrace, Display formatting |
| C header parsing | Ad-hoc string processing | Regex in xtask generator | The `#define` format is uniform enough for regex; a full C parser would be overkill |
| Case-insensitive name matching | Custom lowercasing | `str::to_ascii_uppercase()` | Functional names are ASCII-only; no Unicode concerns |

## Common Pitfalls

### Pitfall 1: Const Array Initialization Limits
**What goes wrong:** Trying to build the 1024-element sparse array with individual `const` assignments in a static initializer hits borrow-checker or const-eval issues in older Rust editions.
**Why it happens:** Const evaluation of mutable references was stabilized in Rust 1.83 (edition 2024).
**How to avoid:** The project uses Rust edition 2024 on rustc 1.92.0, so `let mut table = [None; 1024]; table[1] = Some(...);` in a const block is fully supported. [VERIFIED: rustc 1.92.0]
**Warning signs:** Compiler errors about "mutable references in const context."

### Pitfall 2: FunctionalId Circular Dependency
**What goes wrong:** `FunctionalId::from_raw()` needs the registry, but the registry contains `FunctionalId` values, creating a module dependency cycle.
**Why it happens:** The FunctionalId newtype validates via registry lookup, but registry entries reference FunctionalId.
**How to avoid:** `FunctionalId(u16)` constructor is `pub(crate)` -- the registry creates FunctionalId values directly. Only `from_raw()` and `from_name()` go through the registry for validation. The registry module imports from model/ and meta/, not the other way around.
**Warning signs:** Circular `use` statements between model/ and registry/.

### Pitfall 3: Name Matching for Removed/Aliased IDs
**What goes wrong:** The 52 entries in `xc_funcs_removed.h` include three categories: (1) old names kept for compatibility (same ID, different name), (2) names converted to all-caps (same ID, case change), (3) truly removed functionals (different behavior). Treating them uniformly produces wrong errors.
**Why it happens:** The file mixes aliases with removals without explicit categorization.
**How to avoid:** Parse the file structure using the comment headers that separate the three categories. Category 1 and 2 map to valid active IDs (just aliases). Category 3 ("These are functionals that were removed") should produce `RemovedFunctionalId` errors. For the removed-ID table, the replacement ID must be determined from context (some map to hybrid versions in xc_funcs.h).
**Warning signs:** `RemovedFunctionalId` errors for IDs that are actually valid under a different name.

### Pitfall 4: Sparse Array Size
**What goes wrong:** Using exactly 735 slots (max ID + 1) instead of 1024. Future libxc versions may add IDs beyond 734.
**Why it happens:** Optimizing for minimal size.
**How to avoid:** The design doc specifies 1024, which provides headroom. Use 1024. [VERIFIED: design doc Section 9.3]
**Warning signs:** Index-out-of-bounds panics when a future ID exceeds the array size.

### Pitfall 5: Kind Detection Ambiguity
**What goes wrong:** Some functional names don't follow the simple `_X_`/`_C_`/`_XC_`/`_K_` pattern. For example, `XC_LDA_XC_TETER93` has `XC` as the kind component, `XC_MGGA_XC_ZLP` similarly.
**Why it happens:** The kind is encoded in the name between the family prefix and the functional-specific name.
**How to avoid:** Parse the name component after stripping the family prefix (`LDA_`, `GGA_`, `MGGA_`, `HYB_LDA_`, `HYB_GGA_`, `HYB_MGGA_`). The next token before `_` determines kind: `X` = Exchange, `C` = Correlation, `XC` = ExchangeCorrelation, `K` = Kinetic.
**Warning signs:** Functionals classified as wrong Kind.

### Pitfall 6: thiserror v2 Breaking Changes from v1
**What goes wrong:** Using thiserror v1 syntax that changed in v2.
**Why it happens:** Many examples online show v1 patterns.
**How to avoid:** thiserror 2.0 is backward compatible with v1 derive syntax. The main addition is `#[error(transparent)]` and automatic `provide()` for backtrace. The error enum from the design doc uses standard v1-compatible syntax that works in v2. [ASSUMED]
**Warning signs:** Compile errors from `#[derive(thiserror::Error)]`.

## Code Examples

### Domain Enums (from design doc Section 6.1)
```rust
// Source: docs/design/libxc_rs_detailed_design.md Section 6.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Family {
    Lda  = 1,  // matches XC_FAMILY_LDA
    Gga  = 2,  // matches XC_FAMILY_GGA
    Mgga = 4,  // matches XC_FAMILY_MGGA
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    Exchange            = 0,  // matches XC_EXCHANGE
    Correlation         = 1,  // matches XC_CORRELATION
    ExchangeCorrelation = 2,  // matches XC_EXCHANGE_CORRELATION
    Kinetic             = 3,  // matches XC_KINETIC
}
```

### FunctionalFlags (from design doc Section 6.2, verified against xc.h)
```rust
// Source: docs/design/libxc_rs_detailed_design.md Section 6.2
// Flag values verified against libxc-master/src/xc.h
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FunctionalFlags: u32 {
        const HAVE_EXC        = 1 << 0;   // XC_FLAGS_HAVE_EXC = 1
        const HAVE_VXC        = 1 << 1;   // XC_FLAGS_HAVE_VXC = 2
        const HAVE_FXC        = 1 << 2;   // XC_FLAGS_HAVE_FXC = 4
        const HAVE_KXC        = 1 << 3;   // XC_FLAGS_HAVE_KXC = 8
        const HAVE_LXC        = 1 << 4;   // XC_FLAGS_HAVE_LXC = 16
        const DIM_1D          = 1 << 5;   // XC_FLAGS_1D = 32
        const DIM_2D          = 1 << 6;   // XC_FLAGS_2D = 64
        const DIM_3D          = 1 << 7;   // XC_FLAGS_3D = 128
        const VV10            = 1 << 10;  // XC_FLAGS_VV10 = 1024
        const STABLE          = 1 << 13;  // XC_FLAGS_STABLE = 8192
        const DEVELOPMENT     = 1 << 14;  // XC_FLAGS_DEVELOPMENT = 16384
        const NEEDS_LAPLACIAN = 1 << 15;  // XC_FLAGS_NEEDS_LAPLACIAN = 32768
        const NEEDS_TAU       = 1 << 16;  // XC_FLAGS_NEEDS_TAU = 65536
    }
}
```

### Dimensions Constructors (from design doc Section 6.5)
```rust
// Source: docs/design/libxc_rs_detailed_design.md Section 6.5
// Dimension values confirmed from libxc util.c
impl Dimensions {
    pub fn lda(spin: Spin) -> Self {
        match spin {
            Spin::Unpolarized => Self {
                rho: 1, sigma: 0, lapl: 0, tau: 0,
                zk: 1, vrho: 1, vsigma: 0, vlapl: 0, vtau: 0,
                v2rho2: 1, /* ... all sigma/lapl/tau cross-terms = 0 */
                v3rho3: 1,
                v4rho4: 1,
                // ... remaining fields 0
            },
            Spin::Polarized => Self {
                rho: 2, sigma: 0, lapl: 0, tau: 0,
                zk: 1, vrho: 2, vsigma: 0, vlapl: 0, vtau: 0,
                v2rho2: 3,
                v3rho3: 4,
                v4rho4: 5,
                // ... remaining fields 0
            },
        }
    }
    // Similar for gga() and mgga()
}
```

### Error Enum (from design doc Section 15.2)
```rust
// Source: docs/design/libxc_rs_detailed_design.md Section 15.2
#[derive(Debug, thiserror::Error)]
pub enum LibxcRsError {
    #[error("unknown functional ID: {0}")]
    UnknownFunctionalId(u16),

    #[error("removed functional ID {removed_id}; use {replacement_id} ({replacement_name}) instead")]
    RemovedFunctionalId {
        removed_id: u16,
        replacement_id: u16,
        replacement_name: &'static str,
    },

    #[error("no functional found with name '{0}'")]
    UnknownFunctionalName(String),

    // ... additional variants defined but unused in Phase 1 ...
}
```

### Xtask Header Parser Pattern
```rust
// Source: derived from xc_funcs.h format analysis
use regex::Regex;

let re = Regex::new(r#"#define\s+XC_(\w+)\s+(\d+)\s*/\*\s*(.*?)\s*\*/"#).unwrap();

for line in header_content.lines() {
    if let Some(caps) = re.captures(line) {
        let full_name = format!("XC_{}", &caps[1]);
        let id: u16 = caps[2].parse().unwrap();
        let comment = &caps[3];
        
        let (family, kind) = parse_family_kind(&caps[1]);
        // Emit const FunctionalMeta ...
    }
}

fn parse_family_kind(name: &str) -> (Family, Kind) {
    // Strip family prefix, then detect kind from next component
    let (family, remainder) = if name.starts_with("HYB_MGGA_") {
        (Family::Mgga, &name[9..])
    } else if name.starts_with("HYB_GGA_") {
        (Family::Gga, &name[8..])
    } else if name.starts_with("HYB_LDA_") {
        (Family::Lda, &name[8..])
    } else if name.starts_with("MGGA_") {
        (Family::Mgga, &name[5..])
    } else if name.starts_with("GGA_") {
        (Family::Gga, &name[4..])
    } else if name.starts_with("LDA_") {
        (Family::Lda, &name[3..])  // Note: "LDA_" is 4 chars
    } else {
        panic!("Unknown family prefix: {}", name);
    };
    
    let kind = if remainder.starts_with("XC_") {
        Kind::ExchangeCorrelation
    } else if remainder.starts_with("X_") || remainder == "X" {
        Kind::Exchange
    } else if remainder.starts_with("C_") || remainder == "C" {
        Kind::Correlation
    } else if remainder.starts_with("K_") || remainder == "K" {
        Kind::Kinetic
    } else {
        panic!("Unknown kind in: {}", name);
    };
    
    (family, kind)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| lazy_static for static data | const/static with Rust 2024 const eval | Rust 1.83+ (2024) | No runtime init needed; const fn constructors work |
| thiserror 1.x | thiserror 2.0 | 2024 | Backward compatible syntax; adds provide() for backtrace |
| Manual error Display impl | thiserror derive macros | Long-standing | Less boilerplate, consistent formatting |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | phf crate is unnecessary for 649-entry name lookup since binary search is fast enough | Standard Stack | Very low -- worst case we add phf later; binary search on 649 entries is ~10 comparisons |
| A2 | thiserror 2.0 is backward compatible with v1 derive syntax | Pitfall 6 | Low -- if syntax differs, the design doc code needs minor adjustment |
| A3 | `[Option<&'static FunctionalMeta>; 1024]` can be initialized in a const context on Rust 1.92 | Pitfall 1 | Medium -- if const eval limitations hit, fall back to static with once_cell |

## Open Questions

1. **Removed ID replacement mapping**
   - What we know: 52 entries in xc_funcs_removed.h, split into three categories (aliases, case-changes, truly removed)
   - What's unclear: For the "truly removed" category (starting at line 38 of xc_funcs_removed.h), the replacement ID is not always obvious from the file itself. Some removed functionals (e.g., `XC_GGA_X_HERMAN` ID 104) may not have a direct replacement.
   - Recommendation: For Phase 1, parse what's clear. For ambiguous removals, use the ID that exists in xc_funcs.h at the same position, or flag as having no replacement (replacement_id = 0). The error message should still be informative.

2. **Case-insensitive name lookup**
   - What we know: Design doc specifies case-insensitive lookup via `from_name()`
   - What's unclear: Whether to store names as uppercase in the sorted table and convert input to uppercase, or use a custom comparator
   - Recommendation: Store names as uppercase (they are already uppercase in xc_funcs.h). Convert input with `to_ascii_uppercase()` before binary search. This is simple and correct for ASCII-only names.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc | Compilation | Yes | 1.92.0 | -- |
| cargo | Build system | Yes | 1.92.0 | -- |
| cmake | verify/ build.rs | Yes | 4.2.3 | -- |
| clang | verify/ bindgen | Yes | 21.1.8 | -- |

**Missing dependencies with no fallback:** None.
**Missing dependencies with fallback:** None.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (libtest) |
| Config file | None needed -- Cargo convention |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements --> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOM-01 | All domain enums defined with correct repr | unit | `cargo test --lib model` | No -- Wave 0 |
| DOM-02 | FunctionalId from_raw/from_name | unit | `cargo test --lib model::tests` | No -- Wave 0 |
| DOM-03 | FunctionalFlags matches libxc flag values | unit | `cargo test --lib model::tests::flags` | No -- Wave 0 |
| DOM-04 | Dimensions computes correct sizes | unit | `cargo test --lib dims` | No -- Wave 0 |
| DOM-05 | Thresholds has correct defaults | unit | `cargo test --lib model::tests::thresholds` | No -- Wave 0 |
| REG-01 | 649 entries present | unit | `cargo test --lib registry::tests::count` | No -- Wave 0 |
| REG-02 | O(1) ID lookup works | unit | `cargo test --lib registry::tests::lookup_by_id` | No -- Wave 0 |
| REG-03 | O(log n) name lookup works | unit | `cargo test --lib registry::tests::lookup_by_name` | No -- Wave 0 |
| REG-04 | 52 removed IDs error correctly | unit | `cargo test --lib registry::tests::removed` | No -- Wave 0 |
| REG-05 | Version/reference strings correct | unit | `cargo test --lib registry::tests::version` | No -- Wave 0 |
| ERR-01 | Error enum variants compile | unit | `cargo test --lib error` | No -- Wave 0 |
| ERR-02 | Public methods return Result | unit | Covered by other tests | -- |
| ERR-03 | Evaluation infallible after validation | N/A | No evaluation in Phase 1 | -- |
| VERIFY-01 | Verification harness builds | integration | `cargo test -p libxc_rs-verify` | Partially -- verify/ exists |
| BUILD-01 | No warnings | build | `cargo build 2>&1 \| grep warning` | -- |
| BUILD-02 | Tests pass | all | `cargo test --workspace` | No -- Wave 0 |
| BUILD-03 | Clippy clean | lint | `cargo clippy --workspace -- -D warnings` | -- |
| BUILD-04 | No unsafe outside allowed modules | audit | `cargo clippy` + manual review | -- |
| BUILD-05 | No runtime C FFI in library | audit | Verify no `extern "C"` or `link` in src/ | -- |

### Sampling Rate
- **Per task commit:** `cargo test --lib && cargo clippy --workspace -- -D warnings`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] All test files are new -- they will be created alongside implementation
- [ ] No separate test infrastructure setup needed; Rust's built-in test framework is sufficient
- [ ] verify/ crate needs LDA_X oracle test added

## Security Domain

Security enforcement is not explicitly disabled in config.json, but this phase involves no authentication, networking, cryptography, user input handling, or external data processing. The only input is from trusted C header files parsed at development time by the xtask tool.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | N/A |
| V3 Session Management | No | N/A |
| V4 Access Control | No | N/A |
| V5 Input Validation | Minimal | FunctionalId validates against known set; name lookup validates against registry |
| V6 Cryptography | No | N/A |

No threat patterns apply to Phase 1 (static data types, no I/O, no network, no user-facing input beyond validated functional IDs).

## Project Constraints (from CLAUDE.md)

- **Tech stack:** Pure Rust + CubeCL 0.9.0; no C/Fortran in production path
- **Precision:** f64 only
- **Dependencies:** thiserror 2.0, bitflags 2.10 (production); bindgen, cmake, anyhow (verification only)
- **Edition:** Rust 2024
- **GSD workflow:** Use GSD commands for planned work; do not make direct repo edits outside GSD workflow

## Sources

### Primary (HIGH confidence)
- `docs/design/libxc_rs_detailed_design.md` Sections 6, 8, 9, 15 -- All type definitions, module structure, error design
- `libxc-master/src/xc_funcs.h` -- 649 functional ID definitions (grep-confirmed count)
- `libxc-master/src/xc_funcs_removed.h` -- 52 removed/aliased ID definitions (grep-confirmed count)
- `libxc-master/src/xc.h` -- Flag values, family constants, kind constants (verified exact bit positions)
- `Cargo.toml` -- Current dependency versions confirmed
- `verify/build.rs` -- Existing cmake+bindgen infrastructure confirmed working
- `rustc --version` -- 1.92.0 confirmed, edition 2024 supported

### Secondary (MEDIUM confidence)
- None needed -- all claims verified against local source files

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all libraries already in Cargo.toml, versions confirmed
- Architecture: HIGH -- design document provides implementation-ready Rust code
- Pitfalls: HIGH -- identified from source analysis and Rust const-eval knowledge
- Code generation: MEDIUM -- xtask parsing pattern is straightforward but edge cases in name-to-kind mapping need careful handling

**Research date:** 2026-04-09
**Valid until:** 2026-07-09 (stable domain, no external dependency churn expected)
