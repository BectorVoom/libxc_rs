# Spike Manifest

## Idea

Reduce build and execution time in translated CubeCL kernel paths by identifying low-risk shared refactor boundaries before downstream layers depend on slow workflows.

## Requirements

- Build time must be reduced significantly (target: >50% improvement on repeated builds)
- Must work with existing CubeCL proc-macro architecture
- Must not break existing tests or functionality
- Runtime optimization must preserve the single CubeCL execution path for CPU and GPU.
- Execution-time claims require real benchmarks; static launch-surface evidence can only justify refactor priority.

## Spikes

| # | Name | Type | Validates | Verdict | Tags |
|---|------|------|-----------|---------|------|
| 001 | kernel-build-time | standard | Given a workspace with 200+ kernel crates, when using sccache caching, then subsequent builds are 92% faster | ✓ VALIDATED | build, performance, cubecl, sccache |
| 002 | runtime-launch-overhead | standard | Given generated CubeCL kernels and direct functional dispatch, when optimizing execution time, then the highest-leverage refactor boundary is identified with observable repository evidence | ⚠ PARTIAL | runtime, performance, cubecl, dispatch, resident-buffers |

## Key Findings

- sccache provides 92% build time reduction on repeated builds (4m34s → 22.77s)
- First build has ~30s sccache overhead
- CubeCL proc-macro expansion is the main bottleneck, sccache caches this expensive compilation
- Runtime optimization should first target resident host orchestration: current dispatch creates clients, uploads inputs, allocates zeroed outputs, and reads back per call while resident modules and benches are placeholders
