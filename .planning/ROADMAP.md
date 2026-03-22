# Roadmap: libxc_rs

## Overview

Route the libxc public surface through four layered capabilities: start by locking inventory + metadata, follow with configuration and validation, build the shared CubeCL execution substrate, expose the safe/compat APIs, and finish with verification plus performance baselines. Coarse granularity keeps the phases focused on capability completion before moving on to the next dependency.

## Phases

- [ ] **Phase 1: Catalog & Metadata Lockdown** - Deliver generated registries and metadata tables that mirror the upstream public inventory.
- [ ] **Phase 2: Configuration Safety & Input Validation** - Surface typed builders, thresholds, and input bundles that reject bad configurations before launching kernels.
- [ ] **Phase 3: Unified CubeCL Execution Substrate** - Build the CubeCL kernel family, dispatch keys, and resident execution plumbing so CPU and GPU share the same numeric path.
- [ ] **Phase 4: Safe & Compatibility APIs** - Layer the ergonomic safe API, compatibility shims, and resident flows on top of the validated metadata and kernels.
- [ ] **Phase 5: Verification & Performance Baselines** - Deliver the oracle comparison harness, reporting, and benchmarks that prove correctness and reuse.

## Phase Details

### Phase 1: Catalog & Metadata Lockdown
**Goal**: Guarantee every public functional ID, name, legacy alias, and metadata datum is available through generated registries before downstream layers rely on them.
**Depends on**: Nothing (first phase)
**Requirements**: CATL-01, CATL-02, CATL-03, CATL-04
**Success Criteria** (what must be TRUE):
  1. The generated registry lets callers resolve each current ID and report its family classification on demand.
  2. Canonical names and legacy aliases all resolve to the same metadata entry without loss.
  3. Metadata queries surface family, kind, flags, references, derivative support, and external-parameter specs.
  4. The generation pipeline reproduces the 85 public functions, 649 current IDs, and remaining legacy/removed identifiers explicitly.
**Plans**: TBD

### Phase 2: Configuration Safety & Input Validation
**Goal**: Provide typed builders, thresholds, and input/output bundles that guard launches with precise, family-aware validation.
**Depends on**: Phase 1
**Requirements**: CONF-01, CONF-02, CONF-03, CONF-04, CONF-05
**Success Criteria** (what must be TRUE):
  1. Functional builders accept IDs or names with spin, threshold, external-parameter, precision, and runtime bindings.
  2. Invalid thresholds, removed/unknown IDs, or bad external-parameter requests surface typed errors before execution reaches CubeCL.
  3. LDA/GGA/MGGA input bundles check shape/layout invariants at construction so every launch is structurally sound.
  4. MGGA inputs reject missing tau or lapl channels whenever the metadata mandates them.
  5. Output requests deliver typed bundles corresponding to the requested derivative orders 0 through 4 only.
**Plans**: TBD

### Phase 3: Unified CubeCL Execution Substrate
**Goal**: Compile the shared CubeCL kernel family, dispatch logic, and resident router so CPU and GPU share a single numeric engine.
**Depends on**: Phase 2
**Requirements**: EXEC-01, EXEC-02, EXEC-03, EXEC-04, EXEC-05
**Success Criteria** (what must be TRUE):
  1. Host evaluations for LDA, GGA, and MGGA orders 0–4 execute through CubeCL CPU kernels without a handwritten evaluator.
  2. The same kernel logic runs on at least one CubeCL GPU backend and surfaces backend-unavailable or capability-mismatch errors when unsupported.
  3. Dispatch keys specialize by family, derivative order, spin, required MGGA channels, precision policy, and output masks.
  4. Auxiliary, hybrid, and nonlocal-correlation accumulation flows execute entirely on the device-side path.
  5. Resident execution keeps functionals, inputs, outputs, and scratch buffers resident and uploads only dirty regions between launches.
**Plans**: TBD

### Phase 4: Safe & Compatibility APIs
**Goal**: Layer ergonomic safe APIs and compatibility shims on the validated metadata and unified kernels so callers can reach every libxc capability.
**Depends on**: Phase 3
**Requirements**: API-01, API-02, API-03, API-04
**Success Criteria** (what must be TRUE):
  1. Safe Rust APIs cover lifecycle, metadata access, configuration, host/batch/resident evaluation, and runtime policy controls.
  2. Safe or compatibility APIs together reach every one of the 85 public libxc functions.
  3. Compatibility shims preserve legacy aggregate evaluation entry points and packed layout behavior for migration callers.
  4. Public errors are surfaced through thiserror v2 while verification, benchmarking, and CLI tooling can still rely on anyhow.
**Plans**: TBD

### Phase 5: Verification & Performance Baselines
**Goal**: Prove correctness and performance by exercising every capability against libxc and measuring caching, resident reuse, and transfer costs.
**Depends on**: Phase 4
**Requirements**: VERI-01, VERI-02, PERF-01, PERF-02
**Success Criteria** (what must be TRUE):
  1. Verification tooling compares Rust outputs against libxc across family, derivative order, spin mode, and supported runtime combinations.
  2. Reports include per-functional abs/rel/ULP metrics, CPU-vs-GPU parity summaries, and removed-identifier handling summaries.
  3. Benchmarks cover lookup, initialization, CPU batch, GPU batch, resident reuse, transfer volume, and cold-vs-warm execution behavior.
  4. Runtime caches and workspace reuse keep repeated evaluation paths free of hidden allocations and unnecessary transfers.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Catalog & Metadata Lockdown | 0/TBD | Not started | - |
| 2. Configuration Safety & Input Validation | 0/TBD | Not started | - |
| 3. Unified CubeCL Execution Substrate | 0/TBD | Not started | - |
| 4. Safe & Compatibility APIs | 0/TBD | Not started | - |
| 5. Verification & Performance Baselines | 0/TBD | Not started | - |
