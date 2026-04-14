# Phase 4: Bulk Kernel Translation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-04-10
**Phase:** 04-bulk-kernel-translation
**Areas discussed:** Translation approach, Large kernel handling, Verification strategy, Translation ordering

---

## Translation Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Automated translator | Build an xtask tool that parses maple2c C and emits Rust #[cube] functions. The C is auto-generated and highly regular. | |
| Template + manual hybrid | Build templates/macros for boilerplate, hand-translate mathematical expressions. | |
| Fully manual translation | Hand-translate each file following the LDA_X pattern. Maximum control and understanding. | ✓ |

**User's choice:** Fully manual translation
**Notes:** None

### File Structure

| Option | Description | Selected |
|--------|-------------|----------|
| One file per functional | Each maple2c .c file becomes one Rust .rs file. Clear 1:1 correspondence. | ✓ |
| Group small functionals | Small functionals grouped into shared files by subfamily. | |

**User's choice:** One file per functional
**Notes:** None

### VXC Special Files

| Option | Description | Selected |
|--------|-------------|----------|
| Translate alongside | Include 4 special _vxc functionals in their family batches. | ✓ |
| Defer to end | Handle as separate cleanup pass after main _exc translation. | |

**User's choice:** Translate alongside
**Notes:** None

---

## Large Kernel Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Translate as-is, split if needed | Translate full file faithfully. Split into sub-kernels if CubeCL compilation fails. | ✓ |
| Pre-split by derivative order | Always split large kernels upfront. | |
| Size threshold split | Set a line-count threshold, split above it. | |

**User's choice:** Translate as-is, split if needed
**Notes:** None

### Testing Timing

| Option | Description | Selected |
|--------|-------------|----------|
| First in MGGA batch | Translate largest MGGA first to surface risks immediately. | ✓ |
| After a few simpler MGGAs | Build confidence with smaller MGGAs first. | |
| Last | Do all smaller kernels first. | |

**User's choice:** First in MGGA batch
**Notes:** None

---

## Verification Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Per-family batch tests | One test file per family iterating over all functionals. | ✓ |
| Per-functional test files | One test file per functional (262 files). | |
| Single parametric test | One test function parameterized over all 649 IDs. | |

**User's choice:** Per-family batch tests
**Notes:** None

### Failure Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Fix immediately | Each functional must pass before moving to next. | ✓ |
| Batch then fix | Translate batch, then fix all failures together. | |
| Track and fix later | Translate all, log failures, fix in dedicated pass. | |

**User's choice:** Fix immediately
**Notes:** None

### Derivative Order Testing

| Option | Description | Selected |
|--------|-------------|----------|
| Test each order independently | Run separate tests for exc, vxc, fxc, kxc, lxc. | ✓ |
| Test highest order only | Request max order, check all fields. | |

**User's choice:** Test each order independently
**Notes:** None

---

## Translation Ordering

| Option | Description | Selected |
|--------|-------------|----------|
| LDA -> GGA -> MGGA | Simplest-first, matches roadmap plan structure. | ✓ |
| By usage frequency | Most commonly used functionals first across all families. | |
| All families in parallel | Translate all families simultaneously. | |

**User's choice:** LDA -> GGA -> MGGA
**Notes:** None

### Dispatch Wiring

| Option | Description | Selected |
|--------|-------------|----------|
| Per-functional | Wire each functional into dispatch immediately after translation and verification. | ✓ |
| Per-batch | Translate all in a family, then wire all into dispatch. | |

**User's choice:** Per-functional
**Notes:** None

---

## Claude's Discretion

- Module structure under kernel/gga/ and kernel/mgga/
- Per-family oracle test file organization
- Launch wrapper adaptation for GGA/MGGA additional input arrays
- Commit granularity during translation

## Deferred Ideas

None -- discussion stayed within phase scope
