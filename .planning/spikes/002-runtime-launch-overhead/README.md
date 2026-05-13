---
spike: 002
name: runtime-launch-overhead
type: standard
validates: "Given generated CubeCL kernels and direct functional dispatch, when optimizing execution time, then the highest-leverage refactor boundary is identified with observable repository evidence"
verdict: PARTIAL
related: [001-kernel-build-time]
tags: [runtime, performance, cubecl, dispatch, resident-buffers]
---

# Spike 002: Runtime Launch Overhead

## What This Validates

**Given:** generated CubeCL kernels and direct LDA/GGA/MGGA dispatch paths  
**When:** optimizing execution time through a kernel refactor  
**Then:** identify whether to refactor generated kernels, host launch orchestration, or benchmark infrastructure first

## Research

### Prior spike context

Spike 001 validated `sccache` for build time and found CubeCL proc-macro expansion to be the dominant repeated-build cost. This spike does not revisit build caching; it asks where runtime execution time should be attacked.

### CubeCL manual guidance consulted

The project CubeCL manual says to keep launch surface narrow, use plain `#[cube]` helpers for reusable internal logic, push policy constants to `comptime!`, and measure before refactoring. For this repository, those rules imply two constraints:

| Approach | Tool/Library | Pros | Cons | Status |
|---|---|---|---|---|
| Host orchestration refactor | Existing `src/kernel/launch.rs`, `src/eval/*`, resident modules | Shared change point; avoids hand-editing generated kernels; aligns with workspace/cache constraint | Requires resident buffer lifetime design | Chosen |
| Generated-kernel math rewrite | CubeCL `#[cube]` functions in `crates/kernel-*` | Can improve per-thread arithmetic once measured | Large generated surface; high correctness risk; needs oracle and benchmarks first | Deferred |
| Kernel fusion across derivative/order arms | CubeCL launch API and generated kernel signatures | Could reduce launches/readbacks for repeated derivative requests | Would multiply signature complexity and macro fan-out if done naively | Deferred |
| Benchmark-first harness | Existing `benches/*` placeholders | Required to quantify future changes | Current benches are placeholders, so it cannot alone optimize runtime | Required prerequisite |

**Chosen approach:** audit the host launch surface and existing benchmark/resident boundaries, then document a concrete refactor sequence.

## How to Run

```bash
bash .planning/spikes/002-runtime-launch-overhead/scan_runtime_surface.sh
```

This writes:

- `.planning/spikes/002-runtime-launch-overhead/surface-report.md`
- `.planning/spikes/002-runtime-launch-overhead/surface-report.json`

## What to Expect

The report should show whether dispatch repeatedly creates clients, uploads host inputs, allocates zeroed outputs, and reads back results per call. It should also identify whether resident execution and runtime benches are implemented or only placeholders.

## Investigation Trail

### 2026-05-03: Prior context and manual pass

- Read existing spike manifest, conventions, and Spike 001.
- Read the GSD spike workflow and referenced UI output guidance.
- Read the CubeCL macro fan-out manual before making CubeCL-related recommendations.
- Inspected `src/kernel/launch.rs`, `src/eval/dispatch.rs`, `src/eval/gga_dispatch/mod.rs`, `src/eval/mgga_dispatch/mod.rs`, `src/functional/evaluate.rs`, resident input/output modules, and benchmark files.

### 2026-05-03: Static runtime surface audit

- Added `scan_runtime_surface.sh` to count launch entries, direct launch calls, upload/allocation/readback call sites, resident stubs, and placeholder benches.
- The audit is intentionally static because current benchmark files are placeholders and would not produce useful execution-time numbers yet.

## Results

**Verdict: PARTIAL**

The spike validated the refactor direction but did not validate an execution-time improvement yet.

### Evidence

`surface-report.md` shows:

| Signal | Count | Meaning |
|---|---:|---|
| Rust files under `crates/` | 4,307 | Manual generated-kernel edits are too broad for a first optimization pass. |
| Rust lines under `crates/` | 7,251,224 | Kernel math changes need generation support and oracle-backed tests. |
| Launchable CubeCL kernels in `crates/` | 3,797 | Launch entry surface is large enough that host orchestration overhead matters. |
| Host `launch_unchecked::<CpuRuntime>` calls in `src/` | 31 | Launches are centralized enough to optimize in shared dispatch/launch layers. |
| `cpu_client()` calls in `src/` | 7 | Direct dispatch creates clients inside hot paths. |
| Input upload call sites in `src/eval` | 10 | Direct dispatch re-uploads inputs per evaluation. |
| Zero output allocation call sites in `src/eval` | 28 | Direct dispatch allocates fresh zeroed output buffers per evaluation. |
| Readback call sites in `src/eval` | 12 | Direct dispatch synchronizes/readbacks per requested output field. |
| Resident input/output stubs | 2 | Resident execution is planned but not implemented. |
| Placeholder benchmark files | 7 | Runtime performance is not currently measured. |

### Conclusion

The first runtime refactor should be a resident execution path, not generated-kernel math surgery:

1. Implement resident input/output buffers around `src/input/resident.rs`, `src/output/resident.rs`, and `src/kernel/launch.rs`.
2. Teach direct LDA/GGA/MGGA dispatch to accept a reusable execution context or workspace-owned client/handles.
3. Add real `benches/init.rs`, `benches/transfer.rs`, `benches/resident.rs`, and family benches before changing generated kernel math.
4. Only consider kernel fusion or generated math refactors after the resident path shows host orchestration is no longer the dominant cost.

### Remaining gap

No speedup number is claimed. The existing benchmark files only print placeholder text, so the next build step must add benchmark coverage before declaring the runtime optimization validated.
