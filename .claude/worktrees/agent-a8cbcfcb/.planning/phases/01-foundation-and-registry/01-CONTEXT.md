# Phase 1: Foundation and Registry - Context

**Gathered:** 2026-04-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Establish all domain types (Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags, Dimensions, Thresholds), error hierarchy (LibxcRsError with thiserror v2), static registry with 649 functional metadata entries, and the oracle verification harness infrastructure. The project compiles, tests pass, and any functional can be looked up by ID or name with correct metadata.

</domain>

<decisions>
## Implementation Decisions

### Registry Data Structure
- **D-01:** Use a sparse array indexed by raw functional ID (`&[Option<&'static FunctionalMeta>; 1024]`) for O(1) ID lookup. The design doc specifies this approach and IDs are u16 values with known bounds.
- **D-02:** Use a sorted `&[(&str, FunctionalId)]` slice with binary search for O(log n) name lookup.
- **D-03:** Removed ID table is a separate `&[(u16, u16)]` mapping removed_id to replacement_id, checked before the main registry.

### Metadata Generation
- **D-04:** Build an xtask code generator (or standalone Rust script) that parses `libxc-master/src/xc_funcs.h` and `libxc-master/src/xc_funcs_removed.h` to produce Rust source files with all 649 `FunctionalMeta` const entries and the 52 removed ID entries. Generated source is committed to the repo — no build.rs dependency on C headers in the main crate.
- **D-05:** For this initial phase, only functional ID, name, family, and kind need to be populated from xc_funcs.h. Full metadata (references, ext_params, hybrid terms, flags) will be populated incrementally in later phases as kernel translation proceeds.

### Module Organization
- **D-06:** Follow the design doc's Section 8 module decomposition exactly: `model/`, `meta/`, `registry/`, `error/` as the Phase 1 modules. Each module has its own directory with `mod.rs`.
- **D-07:** Convert the crate from binary (`main.rs`) to library (`lib.rs`) with public re-exports of all domain types.

### Error Design
- **D-08:** Use the full `LibxcRsError` enum from design doc Section 15 with thiserror v2 derives. All public API methods return `Result<T, LibxcRsError>`.
- **D-09:** For Phase 1, implement the lookup-related error variants: `UnknownFunctionalId`, `RemovedFunctionalId`, `UnknownFunctionalName`. Other variants (buffer mismatch, GPU errors) are defined but not yet used.

### Dimension Calculation
- **D-10:** Implement the `Dimensions` struct with `lda()`, `gga()`, `mgga()` constructors matching the dimension table from design doc Section 6.5. This is pure computation — no CubeCL dependency needed.

### Oracle Harness
- **D-11:** The verify/ crate already has a build.rs that builds vendored libxc via cmake and generates bindings via bindgen. Extend it to support basic oracle comparison for LDA_X as a smoke test.
- **D-12:** The verify/ crate uses `anyhow` for errors, keeping `thiserror` at the library boundary per the design doc.

### Claude's Discretion
- Exact file organization within each module directory
- Test organization (inline #[cfg(test)] vs separate tests/ directory)
- Specific const naming conventions for the generated registry tables
- Whether to use `phf` (perfect hash function) crate or stick with sorted binary search for name lookup

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Domain Model and Types
- `docs/design/libxc_rs_detailed_design.md` §6 — Complete data structure definitions (enums, FunctionalId, FunctionalMeta, Dimensions, Thresholds)
- `docs/design/libxc_rs_detailed_design.md` §6.5 — Dimension values table confirmed from libxc util.c

### API Mapping
- `docs/design/libxc_rs_detailed_design.md` §5 — All 85 C-to-Rust API mappings (Phase 1 covers discovery/info functions)

### Error Design
- `docs/design/libxc_rs_detailed_design.md` §15 — Complete error enum variants, error flow diagram, boundary rules

### Module Structure
- `docs/design/libxc_rs_detailed_design.md` §8 — Module decomposition with directory layout
- `docs/design/libxc_rs_detailed_design.md` §9.1-9.4 — Responsibilities for model/, meta/, registry/, error/

### Registry Data Sources
- `libxc-master/src/xc_funcs.h` — 649 functional ID definitions (the source of truth)
- `libxc-master/src/xc_funcs_removed.h` — 52 removed functional IDs with replacements
- `libxc-master/src/xc.h` — Public API declarations, struct definitions, constant values

### Verification Infrastructure
- `verify/build.rs` — Existing cmake + bindgen setup for oracle comparison
- `docs/design/libxc_rs_detailed_design.md` §17 — Oracle verification plan, error metrics, tolerance thresholds

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `verify/build.rs`: Already builds vendored libxc via cmake and generates FFI bindings with bindgen. This is functional infrastructure that Phase 1 extends.
- `Cargo.toml`: Dependencies already configured — bitflags 2.10, bytemuck 1.25, cubecl 0.9.0, thiserror 2.0.18. Workspace includes verify/ crate.

### Established Patterns
- The project uses Rust edition 2024
- Workspace structure with main crate + verify/ crate is already in place
- cmake crate handles cross-platform libxc compilation in verify/

### Integration Points
- `src/main.rs` needs to be replaced with `src/lib.rs` — currently just hello world
- `verify/` depends on the main crate via `libxc_rs = { path = ".." }` — types defined in Phase 1 will be immediately usable in verification tests
- `libxc-master/src/` contains all source files needed for metadata extraction

</code_context>

<specifics>
## Specific Ideas

- The design doc is implementation-ready with exact Rust code for all types — follow it closely rather than redesigning
- For the xtask generator, parse `#define XC_LDA_X 1` patterns from xc_funcs.h and map to Family based on the ID prefix naming convention
- The 649 entries can be validated by grep-counting xc_funcs.h lines matching `#define XC_`

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-foundation-and-registry*
*Context gathered: 2026-04-09*
