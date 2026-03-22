# Requirements: libxc_rs

**Defined:** 2026-03-22
**Core Value:** Deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

## v1 Requirements

### Catalog and Metadata

- [ ] **CATL-01**: Caller can resolve every current public libxc functional by ID and get its family classification.
- [ ] **CATL-02**: Caller can resolve functionals by canonical name and supported legacy aliases.
- [ ] **CATL-03**: Caller can query functional metadata including family, kind, flags, references, derivative support, and external-parameter specifications.
- [ ] **CATL-04**: Generated artifacts preserve complete inventory coverage for the targeted libxc surface, including 85 public functions, 649 current IDs, and explicit legacy or removed identifier handling.

### Configuration and Validation

- [ ] **CONF-01**: Caller can construct a functional by ID or name with selected spin, thresholds, external parameters, precision policy, and runtime binding.
- [ ] **CONF-02**: Caller receives typed errors for invalid thresholds, unknown or removed identifiers, and invalid external-parameter names, indexes, or counts.
- [ ] **CONF-03**: Caller can submit LDA, GGA, and MGGA inputs through family-typed input bundles that validate shape and layout before launch.
- [ ] **CONF-04**: MGGA evaluation rejects missing `tau` or `lapl` channels before launch when metadata marks them as required.
- [ ] **CONF-05**: Caller can request only the needed derivative outputs and receive typed output bundles for derivative orders 0 through 4.

### Execution and Runtime

- [ ] **EXEC-01**: Host evaluation for LDA, GGA, and MGGA derivative orders 0 through 4 runs through CubeCL CPU kernels without a handwritten CPU evaluator.
- [ ] **EXEC-02**: The same kernel logic can execute on at least one GPU backend through CubeCL and returns typed backend-unavailable or capability-mismatch errors when unsupported.
- [ ] **EXEC-03**: Dispatch specializes by family, derivative order, spin mode, required MGGA channels, and precision policy while masking unused outputs.
- [ ] **EXEC-04**: Auxiliary, hybrid, and nonlocal-correlation accumulation flows run through the same device-side execution path instead of host-side formula fallbacks.
- [ ] **EXEC-05**: Resident execution keeps functionals, inputs, outputs, and scratch buffers resident across repeated launches and uploads only dirty input regions.

### API and Compatibility

- [ ] **API-01**: Safe Rust APIs cover lifecycle, metadata access, configuration, host evaluation, batch evaluation, and resident evaluation.
- [ ] **API-02**: Safe or compatibility APIs together reach all 85 public libxc functions from the targeted inventory.
- [ ] **API-03**: Compatibility shims preserve legacy aggregate evaluation entry points and packed libxc-compatible layout behavior for migration-oriented callers.
- [ ] **API-04**: Public library errors are exposed through `thiserror` v2 while verification, benchmarking, and CLI tooling can use `anyhow`.

### Verification and Performance

- [ ] **VERI-01**: Verification tooling compares Rust results against libxc across family, derivative order, spin mode, and supported runtime combinations.
- [ ] **VERI-02**: Verification reports include per-functional abs/rel/ULP metrics, CPU-vs-GPU parity summaries, and removed-identifier handling summaries.
- [ ] **PERF-01**: Benchmarks measure lookup, initialization, CPU batch, GPU batch, resident reuse, transfer volume, and cold-vs-warm execution behavior.
- [ ] **PERF-02**: Runtime caches and workspace reuse keep repeated evaluation paths free of avoidable hidden allocations and unnecessary transfers.

## v2 Requirements

### Runtime Expansion

- **RTEX-01**: Caller can target multiple GPU runtime backends with documented support coverage and parity status.
- **RTEX-02**: Caller can opt into additional autotuning and stream-placement policies beyond the initial stable runtime defaults.

### Precision and Forward Compatibility

- **PREC-01**: Caller can use optional `f32` or mixed-precision execution modes through a stable public API once parity thresholds are validated.
- **PREC-02**: Future concrete `LCA` or `OEP` functionals can be integrated without redesigning the public type model.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Embedding libxc as the production evaluator | The runtime must remain Rust-native; libxc is reserved for oracle verification only |
| Separate handwritten CPU formula implementation | Conflicts with the CubeCL-only compute rule and increases semantic drift risk |
| C ABI structs as the primary user-facing API | The compatibility layer exists for reachability, not as the main ergonomic surface |
| Silent fallback from unsupported GPU runtime to unrelated execution path | Backend availability must fail explicitly so correctness and performance claims remain trustworthy |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CATL-01 | TBD | Pending |
| CATL-02 | TBD | Pending |
| CATL-03 | TBD | Pending |
| CATL-04 | TBD | Pending |
| CONF-01 | TBD | Pending |
| CONF-02 | TBD | Pending |
| CONF-03 | TBD | Pending |
| CONF-04 | TBD | Pending |
| CONF-05 | TBD | Pending |
| EXEC-01 | TBD | Pending |
| EXEC-02 | TBD | Pending |
| EXEC-03 | TBD | Pending |
| EXEC-04 | TBD | Pending |
| EXEC-05 | TBD | Pending |
| API-01 | TBD | Pending |
| API-02 | TBD | Pending |
| API-03 | TBD | Pending |
| API-04 | TBD | Pending |
| VERI-01 | TBD | Pending |
| VERI-02 | TBD | Pending |
| PERF-01 | TBD | Pending |
| PERF-02 | TBD | Pending |

**Coverage:**
- v1 requirements: 22 total
- Mapped to phases: 0
- Unmapped: 22 WARNING

---
*Requirements defined: 2026-03-22*
*Last updated: 2026-03-22 after initial definition*
