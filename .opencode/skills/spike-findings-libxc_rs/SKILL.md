---
name: spike-findings-libxc_rs
description: Implementation blueprint from spike experiments. Requirements, proven patterns, and verified knowledge for building libxc_rs. Auto-loaded during implementation work.
---

<context>
## Project: libxc_rs

Rust re-architecture of libxc 7.0.0 exchange-correlation functional library. Build optimization focused on 200+ kernel crates using CubeCL proc-macros.

Spike sessions wrapped: 2026-05-03
</context>

<requirements>
## Requirements

- Build time must be reduced significantly (target: >50% improvement on repeated builds)
- Must work with existing CubeCL proc-macro architecture
- Must not break existing tests or functionality
</requirements>

<findings_index>
## Feature Areas

| Area | Reference | Key Finding |
|------|-----------|-------------|
| Build Optimization | references/build-optimization.md | sccache provides 92% build time reduction |

## Source Files

Original spike source files are preserved in `sources/` for complete reference.
</findings_index>

<metadata>
## Processed Spikes

- 001-kernel-build-time
</metadata>