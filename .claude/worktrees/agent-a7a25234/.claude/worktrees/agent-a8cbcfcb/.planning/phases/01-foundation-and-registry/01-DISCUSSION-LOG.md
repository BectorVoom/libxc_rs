# Phase 1: Foundation and Registry - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-09
**Phase:** 01-Foundation and Registry
**Mode:** auto
**Areas discussed:** Registry structure, Metadata generation, Module organization, Crate type, Error design, Dimension calculation, Oracle harness

---

## Registry Data Structure

| Option | Description | Selected |
|--------|-------------|----------|
| Sparse array indexed by raw ID | O(1) lookup, ~1024 entries with gaps, matches design doc | ✓ |
| HashMap<u16, &FunctionalMeta> | O(1) amortized, heap-allocated, runtime init | |
| Sorted Vec with binary search | O(log n), no gaps, smaller memory | |

**User's choice:** [auto] Sparse array indexed by raw ID (recommended default)
**Notes:** Design doc Section 9.3 specifies this approach. IDs are bounded u16 values making sparse array efficient.

---

## Metadata Generation

| Option | Description | Selected |
|--------|-------------|----------|
| xtask code generator | Parse C headers, produce committed Rust source | ✓ |
| Hand-write all 649 entries | Manual, error-prone at scale | |
| build.rs parsing at compile time | Adds C header dependency to main crate | |

**User's choice:** [auto] xtask code generator (recommended default)
**Notes:** Keeps build.rs clean, produces auditable committed source. C header dependency stays in xtask/, not the main crate.

---

## Module Organization

| Option | Description | Selected |
|--------|-------------|----------|
| Follow design doc Section 8 | Nested directories: model/, meta/, registry/, error/ | ✓ |
| Flat module structure | All types in fewer files | |

**User's choice:** [auto] Follow design doc Section 8 (recommended default)
**Notes:** Design doc provides complete module decomposition that's well thought out.

---

## Crate Type

| Option | Description | Selected |
|--------|-------------|----------|
| Library (lib.rs) | Replace main.rs with lib.rs and public re-exports | ✓ |
| Both lib.rs and main.rs | Library with optional CLI binary | |

**User's choice:** [auto] Library (lib.rs) (recommended default)
**Notes:** This is a library crate — no CLI binary needed.

---

## Auto-Resolved

- Registry structure: auto-selected sparse array (recommended)
- Metadata generation: auto-selected xtask generator (recommended)
- Module organization: auto-selected design doc structure (recommended)
- Crate type: auto-selected library (recommended)
