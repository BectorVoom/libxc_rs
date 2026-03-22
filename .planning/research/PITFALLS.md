# Pitfalls Research

**Domain:** Rust re-architecture of libxc with unified CubeCL CPU/GPU compute paths  
**Researched:** March 22, 2026  
**Confidence:** MEDIUM

## Critical Pitfalls

### Pitfall 1: Generated API coverage drift

**What goes wrong:** Generated registries fall out of sync with the upstream libxc headers, so some functional IDs, constants, or lifecycle hooks simply never appear in the Rust surface and downstream lookups break.  
**Why it happens:** The generator depends on parsing macros and structs; using an older bindgen/libclang combo or failing to re-run the pipeline after a header tweak can silently drop definitions, which is exactly what happens when bindgen cannot translate a new header update and the build fails with “cannot find struct…in crate” errors when the missing symbol is referenced. citeturn2search0  
**How to avoid:** Automate the parser/code-gen in Phase 0, lock the toolchain versions, and treat the generated count of public APIs/IDs as a regression check so any deviation fails the build before release.  
**Warning signs:** The API catalog test flags missing names, CI pulls in header changes without a matching generated diff, or `docs/libxc_rs_detailed_design.md` counts no longer match the generated registry table.  
**Phase to address:** Phase 0 (codegen) + Phase 1 (metadata/registry completeness)

---

### Pitfall 2: CPU/GPU semantic divergence

**What goes wrong:** Without tight coordination, the CPU CubeCL kernel can drift from the GPU path, leading to different numerical outputs or resource starvation when one backend accelerates more aggressively than the other. citeturn0search3  
**Why it happens:** GPUs have complex pipelines that expose subtle differences in scheduling, precision, and memory traffic; industry parity efforts already add parity checks to catch pipeline discrepancies because the hardware itself can behave differently from the CPU pipeline. citeturn0search1  
**How to avoid:** Share as much kernel logic as possible (Phase 3/4), run parity tests on every derivative order across CPU, CUDA, HIP, and WGPU backends, and make divergence detection part of the resident/verification flows.  
**Warning signs:** GPU utilization curves stay flat while CPU runs faster, libxc comparison tolerances wander, or CubeCL logs show different internal branches executed per backend.  
**Phase to address:** Phase 3 (shared CubeCL substrate) + Phase 4 (family kernels) with follow-up verifications in Phase 5.

---

### Pitfall 3: Verification gaps

**What goes wrong:** Heterogeneous systems lack enough regression/acceptance tests, so changes slip through without the rigorous HPC-style verification demanded by cross-device compute. citeturn13view0  
**Why it happens:** Building verification infrastructure is harder than adding features, and teams cut corners until a real mismatch surfaces; the AI infrastructure testing community stresses the same point—without dedicated clustered validation frameworks, production issues (and costly downtime) emerge unexpectedly. citeturn0search4  
**How to avoid:** Use Phase 7’s verification harness to compare every API path against libxc, run nightly CPU vs GPU parity suites, and log abs/rel/ULP discrepancies so regressions and backend-specific failures are immediately visible.  
**Warning signs:** Verification harness was skipped for a release, oracle comparison metrics are stale, or local tests only cover a single backend.  
**Phase to address:** Phase 6/7 (safe API completion and verification harness)

---

### Pitfall 4: Runtime capability mismatches

**What goes wrong:** CubeCL’s CPU backend still throws “not yet implemented” for common operations, so enabling that backend causes panics instead of graceful fallbacks. citeturn10view0  
**Why it happens:** The runtime exposes broad APIs, but the CPU compiler visitor has unimplemented operations, meaning new kernels trigger these gaps before they surface in production.  
**How to avoid:** Treat capability probing as mandatory—before allowing a CPU runtime, query CubeCL for supported operations, and if the new kernel uses an unsupported operation, revert to a well-defined error rather than exposing a panic.  
**Warning signs:** CI logs show “not yet implemented: This operation (…)” even though the kernel compiles for CUDA, or CubeCL CPU tests are skipped.  
**Phase to address:** Phase 3/4 (shared kernel compile + per-family launches)

---

### Pitfall 5: Transfer costs and small-batch overhead

**What goes wrong:** Data movement dominates low-batch execution, so transferring small slices per launch (and not overlapping transfers) leaves GPUs idle for 56 % of their cycles and wastes 27.9 % of runtime on data operations. citeturn1search1turn1search12  
**Why it happens:** Small batches force frequent launches with little amortization, so bandwidth stalls and synchronous checkpoints (data operations) swamp compute unless transfers are overlapped and resident buffers reused.  
**How to avoid:** Emphasize Phase 5/6 resident execution, make the workspace planner aware of dirty ranges, and use buffered transfer pipelines and cache reuse before exposing the high-level API.  
**Warning signs:** Benchmark transfer tests spike, small-batch profiling shows >25 % transfer time, or resident APIs default to synchronous readbacks.  
**Phase to address:** Phase 5/6/8 (resident flow, safe API, and benchmark stabilization)

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Skip GPU/CPU parity runs to keep CI fast | CI builds are quicker | Silent semantic divergence that only surfaces in production | Never; heterogeneity requires regression coverage citeturn13view0 |
| Skip transfer instrumentation or profiling dashboards | Save time on metrics work | Unable to detect data-starvation drags, which already cost 56 % of GPU cycles waiting for data citeturn1search1 | Only for quick prototypes |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| CubeCL CPU runtime | Enabling the backend without capability probing causes “not yet implemented” panics citeturn10view0 | Probe CubeCL’s unsupported operations map, gate CPU runs behind capability checks, and keep a fallback error path. |
| Storage/data layer | Assuming storage keeps up; the GPU/storage imbalance already makes compute sit idle | Prioritize transfer overlap, caching, and multi-tier storage planning so hosts feed the kernels without starvation citeturn6view0 |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Small batches without overlapped transfers | Transfer time ≳25 % of runtime, GPU idle | Resident execution + asynchronous staging for small launches citeturn1search12turn1search1 | When batch size drops below the threshold that amortizes kernel launch + transfer |
| Treating storage as “fast enough” | GPUs stall waiting for data, reports of 56 % idle cycles citeturn1search1 | Instrument I/O, add caching layers, or throttle storage demand to match the GPU rhythm citeturn6view0 | At petabyte-scale or multi-tenant clusters |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Leaving GPU/resident buffers populated | Sensitive data survives in device memory, creating a data-leak risk in shared environments citeturn1search6 | Zero-out/stage GPU buffers after use and limit device access to the runtime’s trusted scope. |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Surface raw CubeCL or libxc panics through the ergonomic API | Users lose confidence if the API explodes in production-only runtimes | Wrap low-level failures with explicit diagnostics, and document which devices/backends are safe to call; treat environment parity like a testing practice citeturn0search4 |

## "Looks Done But Isn't" Checklist

- [ ] **API coverage:** Generated inventory numbers still match libxc’s 85 functions/649 IDs; a single missing ID means the registry wasn’t regenerated. citeturn2search0  
- [ ] **Parity testing:** GPU-only results were compared against CPU and libxc oracle before release; heterogeneity testing requires this to avoid divergence. citeturn13view0  
- [ ] **Transfer profiling:** Small-batch runs still spend ≳25 % of time moving data; the transfer instrumentation was reviewed. citeturn1search12  
- [ ] **CubeCL CPU runtime:** Every code path that claims CPU support has passed the capability probe; unimplemented operations are fatal to parity. citeturn10view0

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Semantic divergence revealed after release | HIGH | Pause new feature work, rerun cross-backend parity harness, and rebuild the shared kernel with the corrected specialization. citeturn0search3 |
| Missing IDs drop from the generated registry | MEDIUM | Re-run the parser/codegen, refresh the generated tables, and extend the API catalog test to count regression coverage. citeturn2search0 |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Generated API coverage drift | Phase 0 (codegen) + Phase 1 (metadata/registry) | API catalog test that counts IDs and functions. citeturn2search0 |
| CPU/GPU semantic divergence | Phase 3/4 (shared kernel + family kernels) | Cross-backend parity suite per derivative order, resident mode comparisons. citeturn0search1 |
| Verification gaps | Phase 6/7 (safe API + verification harness) | libxc oracle runs + HPCTESTS-style regression tests. citeturn13view0 |
| Runtime capability mismatches | Phase 3/4 (CubeCL substrate + kernels) | Capability probe log, failure injection for unsupported operations. citeturn10view0 |
| Transfer & small-batch trap | Phase 5/6/8 (resident flow, safe API, benchmarks) | Transfer-time profiling dashboards, transfer-to-compute ratio targets. citeturn1search12 |

## Sources

- Bindgen/version mismatch breaking generated APIs (Rust issue report). citeturn2search0  
- GPU acceleration requires orchestration, and GPU pipelines already need parity checks. citeturn0search3turn0search1  
- AI infrastructure validation frameworks and HPC testing demands. citeturn0search4turn13view0  
- CubeCL CPU backend still throws “not yet implemented” for common ops. citeturn10view0  
- GPU cycles idle waiting for data; storage bottlenecks starve compute. citeturn1search1turn6view0  
- Data operations already consume ~27.9 % of runtime in small batches. citeturn1search12  
- GPU memory leaks can expose sensitive data in shared runtimes. citeturn1search6

---
