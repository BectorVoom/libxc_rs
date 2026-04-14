# Phase 2: Math Core and CubeCL Substrate - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-04-09
**Phase:** 02-math-core-and-cubecl-substrate
**Areas discussed:** CubeCL integration approach, erf/erfc implementation strategy, Math function testing strategy, LDA_X canary kernel scope

---

## CubeCL Integration Approach

### CubeCL Dependency

| Option | Description | Selected |
|--------|-------------|----------|
| CPU-only for now | Add cubecl with only cpu feature. GPU backends deferred to Phase 7. | ✓ |
| All backends feature-gated from day one | Add all backends as optional features immediately. | |
| Separate math-core crate | Isolate CubeCL dependency in a separate crate. | |

**User's choice:** CPU-only for now
**Notes:** Keeps compile times down and avoids GPU toolchain requirements during math core development.

### Reference Implementations

| Option | Description | Selected |
|--------|-------------|----------|
| Dual implementations | #[cube] version + plain-Rust version for each function | |
| CubeCL CPU backend only | Trust CubeCL CPU backend directly, no dual reference impl | ✓ |
| Reference impl in verify/ crate | Plain-Rust references in verify/ alongside oracle | |

**User's choice:** CubeCL CPU backend only

### Module Layout

| Option | Description | Selected |
|--------|-------------|----------|
| Flat math/ module | src/math/mod.rs with submodules matching design doc Section 9.5 | ✓ |
| math/ + kernel/shared/ split | Separate pure math from DFT-specific compositions | |
| Single math.rs file | All functions in one file, split later | |

**User's choice:** Flat math/ module

### Kernel Location

| Option | Description | Selected |
|--------|-------------|----------|
| src/kernel/lda/lda_x.rs | Follow design doc Section 9.9 structure from day one | ✓ |
| src/math/canary.rs | Keep in math/ as validation artifact | |
| tests/canary_lda_x.rs | Integration test only, not production module | |

**User's choice:** src/kernel/lda/lda_x.rs

---

## erf/erfc Implementation Strategy

### Approximation Method

| Option | Description | Selected |
|--------|-------------|----------|
| Abramowitz & Stegun rational | Classic rational approximation, max error ~1.5e-7 or piecewise for full precision | |
| Chebyshev polynomial expansion | Clenshaw recurrence, systematically improvable | |
| Cephes/libm-style implementation | Port libm's erf.c piecewise rational approximation | ✓ |

**User's choice:** Cephes/libm-style implementation
**Notes:** Multiple coefficient sets per interval, proven f64 precision.

### Precision Target

| Option | Description | Selected |
|--------|-------------|----------|
| Full f64 (~1e-15 relative error) | Match libm precision, no shortcuts | ✓ |
| 1e-12 relative error | Match overall oracle tolerance | |
| You decide | Claude picks best tradeoff | |

**User's choice:** Full f64 (~1e-15 relative error)

---

## Math Function Testing Strategy

### Test Location

| Option | Description | Selected |
|--------|-------------|----------|
| Inline #[cfg(test)] in each submodule | Tests next to code in each math submodule | ✓ |
| Separate tests/math/ directory | Integration-style tests in separate directory | |
| In verify/ crate | All math validation alongside oracle comparison | |

**User's choice:** Inline #[cfg(test)]

### Reference Values

| Option | Description | Selected |
|--------|-------------|----------|
| Hardcoded known values | Hand-computed/Wolfram Alpha values for key points | |
| libm crate as dev-dependency | Systematic comparison against libm functions | |
| Both hardcoded + libm sweep | Hardcoded for key points + sweep tests across ranges | ✓ |

**User's choice:** Both hardcoded + libm sweep

### Cross-Backend Consistency

| Option | Description | Selected |
|--------|-------------|----------|
| Defer to Phase 7 | Only meaningful when GPU backends are available | ✓ |
| Test CubeCL CPU vs native Rust now | Catch CubeCL CPU codegen quirks early | |
| You decide | Claude determines approach | |

**User's choice:** Defer to Phase 7

---

## LDA_X Canary Kernel Scope

### Derivative Orders

| Option | Description | Selected |
|--------|-------------|----------|
| All orders through 4th (exc, vxc, fxc, kxc, lxc) | Full translation matching maple2c_order=4 | ✓ |
| Just exc and vxc | Prove energy and first derivative | |
| exc only | Minimal canary | |

**User's choice:** All orders through 4th

### Translation Pattern

| Option | Description | Selected |
|--------|-------------|----------|
| Manual translation of lda_x.c | Hand-translate preserving variable names and operation order | ✓ |
| Automated translator tool | Build maple2c-to-Rust translator in xtask/ | |
| LLM-assisted bulk translation | Use Claude for Phase 4, manual canary for pattern | |

**User's choice:** Manual translation

### Spin Modes

| Option | Description | Selected |
|--------|-------------|----------|
| Both spin modes (unpolarized + polarized) | Validates spin handling early per SC-3 | ✓ |
| Unpolarized only | Simpler first | |

**User's choice:** Both spin modes

### Launch Wrapper Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal launch in kernel/launch.rs | Basic wrapper, just enough for LDA_X | |
| Full launch infrastructure | Backend selection, buffer management, dispatch traits | ✓ |
| You decide | Claude determines scope | |

**User's choice:** Full launch infrastructure

---

## Claude's Discretion

- CubeCL ComputeClient initialization pattern and lifetime management
- CubeCount/CubeDim calculation strategy
- poly_eval/rational_eval const generics vs slices
- Internal organization of kernel/launch.rs
- libm dev-dependency version selection

## Deferred Ideas

None -- discussion stayed within phase scope
