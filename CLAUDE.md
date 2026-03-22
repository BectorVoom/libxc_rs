<!-- GSD:project-start source:PROJECT.md -->
## Project

**libxc_rs**

`libxc_rs` is a Rust re-architecture of the full public `libxc` API surface for the `libxc 7.0.0` inventory captured in `docs/libxc_rs_detailed_design.md`. It preserves reachability to libxc public capabilities, but replaces the original C-style surface with a layered Rust API: a low-level compatibility layer, a typed safe core, and an ergonomic high-level interface for host and device execution.

This project is explicitly designed around a unified CubeCL compute path. All numerical evaluation on CPU and GPU goes through CubeCL kernels, while libxc itself remains an oracle used only by verification tooling.

**Core Value:** Deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

### Constraints

- **Tech stack**: Rust crate with CubeCL runtimes for CPU/CUDA/HIP/WGPU — all compute paths must share the same kernel substrate.
- **Compatibility**: Full reachability to the libxc public API inventory — the redesign cannot silently drop public functions, IDs, or metadata paths.
- **Safety**: Public APIs use typed Rust boundaries and `thiserror` v2 errors — unsafe code is confined to compat/raw-handle internals, CubeCL launch bridging, and verification FFI.
- **Validation**: libxc remains the oracle — acceptance requires oracle comparisons across family, derivative order, spin mode, and runtime combinations.
- **Performance**: Repeated workloads must reuse workspaces, resident buffers, and caches — hidden hot-path allocations and unnecessary transfers are not acceptable.
- **Scope discipline**: The design is a from-scratch public API redesign, not a transliteration of the original C header.
<!-- GSD:project-end -->

<!-- GSD:stack-start source:research/STACK.md -->
## Technology Stack

## Recommended Stack
### Core Technologies
| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| CubeCL compute stack (cubecl + cubecl-core/cubecl-runtime + optional cubecl-cpu/cubecl-cuda/cubecl-hip/cubecl-wgpu + cubecl-ir) | 0.9.0 | Compile and auto-vectorize a single kernel set that runs on CPU and every GPU runtime with shared launch logic | CubeCL 0.9.0 documents show the CPU/GPU runtimes share the same compute substrate, and Burn 0.20’s release notes stress that CubeCL unifies CPU and GPU kernels so teams avoid duplicate formulas while keeping JIT/autotune optimizations per backend. citeturn0search0turn4search0 |
| bindgen | 0.72.0 | Auto-generate the Rust side of libxc’s hundreds of bezier-ed C APIs so code generation stays synchronized with upstream headers | The bindgen project and Effective Rust both advocate ‘let the compiler generate the bindings’ for large C APIs, and the packaged CLI is now at 0.72.0 in major distributions, which keeps regen tooling reproducible. citeturn7search2turn7search3turn4search5 |
| thiserror | 2.0 | Define typed, descriptive library errors that callers can convert into their own error graphs | Best-practice articles recommend `thiserror` to keep library boundaries strongly typed while leaving application layers free to wrap them with dynamic contexts. citeturn5search5turn9search7 |
### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| ndarray | 0.17.2 | Host-side multi-dimensional buffers, shared views, and chunkable slicing/zip helpers before handing data off to CubeCL | Latest releases emphasize lightweight views and chunking, which prevent extra allocations when shaping libxc’s rho/sigma/lapl/tau arrays. citeturn8search0turn8search1 |
| rayon | 1.11.0 | Parallelize layout validation, score computation, and verification batches before launching CubeCL kernels | Rayon’s work-stealing scheduler is the de facto way to express data-parallel preprocessing on Rust hosts, keeping CPU prep cheap. citeturn6search1 |
| anyhow | 1.0.102 | Propagate context-rich errors in verification tooling, benchmarks, and xtask automation without defining new enums | Guides on designing Rust error types encourage `anyhow` for applications so the verification harness can log details while keeping the library’s public error enum focused. citeturn5search5turn9search7 |
### Development Tools
| Tool | Purpose | Notes |
|------|---------|-------|
| cargo xtask pattern (referencing tracel-xtask 4.13.4 for reusable command helpers) | Orchestrate code generation, CubeCL kernel builds, verification runs, and benchmarks from workspace scripts | Matklad’s cargo-xtask README describes this pattern as cross-platform automation, and the recent tracel-xtask 4.13.4 release embodies a modern implementation that layers CLI helpers on top of `anyhow`, `clap`, and tracing-friendly crates. citeturn2search0turn10search0 |
| Criterion benchmarks | Stabilize throughput, latency, and transfer-cost measurements for CPU vs. GPU vs. resident flows | Criterion 0.8.2 is the current release for reproducible benchmarking, and performance guides push for consistent tooling because ad-hoc measurement will hide regressions. citeturn11search3turn6search0 |
| bindgen-cli | Regenerate bindings as xtask steps so the libxc inventory remains the source-of-truth after header changes | The CLI is the practical entry point for the bindgen pattern and is at version 0.72.0 in current packaging, matching the library release. citeturn7search2turn4search5 |
## Installation
# Core tooling
# Supporting libraries
# Development/verification helpers
## Alternatives Considered
| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| CubeCL unified kernels | Maintain separate WGPU/HIP/CUDA handwritten kernels | Only worth the added maintenance cost if you need very vendor-specific features that CubeCL cannot express (very rare now that Burn 0.20 uses CubeCL for both CPU and GPU). citeturn4search0turn4search2 |
| bindgen for ingoing libxc headers | cbindgen’s reverse direction | cbindgen shines when exposing a Rust API to C; for consuming a large C API like libxc, bindgen keeps the code in sync automatically. citeturn7search0turn6search1 |
## What NOT to Use
| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Hand-rolled per-backend CUDA/HIP/Metal kernels | Duplicates logic and risks CPU/GPU drift while CubeCL’s 0.9.0 release plus the Burn 0.20/Phoronix coverage show teams consolidating kernels into one codebase | CubeCL gives a shared kernel infrared plus autotuning so you only write formulas once. citeturn0search0turn4search0 |
| cbindgen for libxc’s API | cbindgen is for generating C headers from Rust; using it to consume a C library would force you to reverse-engineer bindings manually | Use bindgen (and the Rust Patterns bindgen pattern) to import libxc’s inventory safely. citeturn6search1turn7search0 |
## Stack Patterns by Variant
- Use the cargo-xtask pattern described by Matklad and the tracel-xtask helpers so you can run bindgen, compile CubeCL kernels, and run oracle comparisons from one script. citeturn2search0turn10search0
- Because the Rust Patterns FFI chapter explicitly calls out bindgen as the automation you want for large C APIs, pairing it with xtasks keeps the generated inventory always synced with the headers you validate against. citeturn7search0
- Use ndarray’s views (0.17.2) together with Rayon’s work-stealing pool for parallel validation, copying, or reading libxc/Oracle outputs before handing them to CubeCL. citeturn8search1turn6search1
- Because ndarray already provides zero-copy chunking and Rayon is the canonical host-level data-parallel pool, you avoid duplicating buffer logic and keep CPU prep consistent. citeturn8search0
## Version Compatibility
| Package | Compatible With | Notes |
|---------|-----------------|-------|
| cubecl 0.9.0 | cubecl-core, cubecl-runtime, cubecl-ir, cubecl-cpu/cuda/hip/wgpu all at 0.9.0 | Docs stress that the optional runtime crates are meant to be version-locked together so proc macros and runtimes generate consistent kernels. citeturn0search0 |
| bindgen 0.72.0 | bindgen-cli 0.72.0 | The SUSE packaging (0.72.0) mirrors the code generator release, which keeps build scripts deterministic. citeturn4search5 |
| tracel-xtask 4.13.4 | tracel-xtask-macros 4.13.4 / serde / clap etc. | The 4.13.4 release shines because it already depends on the same `anyhow`, `clap`, and logging crates we rely on, avoiding version conflicts. citeturn10search0 |
## Sources
- `cubecl` 0.9.0 docs — multi-platform runtime + optional CPU/GPU feature list. citeturn0search0
- Burn 0.20 release blog — CubeCL unifies CPU + GPU kernels for high performance. citeturn4search0
- Phoronix / Burn 0.20 coverage — CubeCL-based CubeK shows the practical benefit of a single kernel stack. citeturn4search1
- Burn end-of-year review — CubeCL CPU/Metal/MLIR expansion validates the cross-platform claim. citeturn4search2
- Matklad’s cargo-xtask README — layout for xtask automation. citeturn2search0
- tracel-xtask 4.13.4 docs — up-to-date reusable xtask helpers. citeturn10search0
- bindgen GitHub + documentation — auto-generate large C APIs. citeturn7search2
- Rust Patterns FFI chapter — bindgen pattern for massive C interfaces. citeturn7search0
- Effective Rust Item 35 — prefer bindgen over manual declarations. citeturn7search3
- SUSE bindgen 0.72.0 packaging note. citeturn4search5
- oneuptime/how-to guide — thiserror for libraries, anyhow for applications tooling. citeturn9search7
- Effective Rust Item 4 — typed enums for libraries; `thiserror` + `anyhow` distinctions. citeturn5search5
- ndarray 0.17.2 release notes — documented arrays for chunking + slicing. citeturn8search1
- Rayon data-parallel article — work-stealing best practice. citeturn6search1
- Criterion 0.8.2 docs — current benchmarking tool. citeturn11search3
- Rust performance guide on benchmarking — emphasize consistent tooling. citeturn6search0
- cbindgen documentation — alternative direction to avoid. citeturn6search1
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
