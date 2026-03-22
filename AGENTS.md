# Project Agents Guide

## Project

`libxc_rs` is a Rust re-architecture of the public `libxc 7.0.0` API surface. The library keeps upstream capability reachability, but replaces the original C-style surface with a three-layer Rust design: compatibility shims, a typed safe core, and ergonomic high-level APIs for host and device execution.

**Core value:** deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

## Current Focus

- Active roadmap phase: **Phase 1 - Catalog & Metadata Lockdown**
- Immediate goal: lock generated registries and metadata tables so every ID, alias, and descriptor mirrors the upstream inventory before downstream layers depend on them.

## Key Constraints

- All numerical execution paths must use CubeCL, including CPU.
- The redesign cannot silently drop public functions, IDs, metadata paths, or removed-ID diagnostics.
- Public APIs must use typed Rust boundaries and `thiserror` v2 errors.
- libxc is an oracle for verification only; it is not part of the production runtime.
- Repeated workloads must reuse workspaces, resident buffers, and caches rather than reallocating on hot paths.

## Workflow

Before making file changes, route work through the GSD workflow so `.planning/` stays in sync with implementation.

- Use `/gsd:quick` for small fixes or doc updates.
- Use `/gsd:debug` for investigation and bug fixing.
- Use `/gsd:plan-phase <n>` to plan roadmap work.
- Use `/gsd:execute-phase <n>` to execute planned work.

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
