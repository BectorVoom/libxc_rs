# Project Agents Guide

## Project

`libxc_rs` is a Rust re-architecture of the public `libxc 7.0.0` API surface. The library keeps upstream capability reachability, but replaces the original C-style surface with a three-layer Rust design: compatibility shims, a typed safe core, and ergonomic high-level APIs for host and device execution.

**Core value:** deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

## Skills

- **Spike findings for libxc_rs** (build optimization patterns, sccache setup) → `Skill("spike-findings-libxc_rs")`

## Current Focus

- Active roadmap phase: **Phase 1 - Catalog & Metadata Lockdown**
- Immediate goal: lock generated registries and metadata tables so every ID, alias, and descriptor mirrors the upstream inventory before downstream layers depend on them.


## Conventions

<<<<<<< HEAD
- Before creating any test code, read `\home\chemtech\workspace\libxc_rs\docs\rust_crate_test_guideline.md` and follow it when designing and implementing the tests.
=======
- Before creating any test code, read `\home\chemtech\workspace\cintx\docs\rust_crate_test_guideline.md` and follow it when designing and implementing the tests.
>>>>>>> origin/main


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
<<<<<<< HEAD


## Mandatory Manual for `cubecl` Implementation

When implementing, modifying, or generating code that uses the Rust `cubecl` crate, the agent must first read:

`/home/chemtech/workspace/libxc_rs/docs/manual/Cubecl`

This manual must be used as the primary reference for implementation patterns, architecture, configuration, and coding rules related to `cubecl`.

Do not write or propose `cubecl`-based code without consulting this manual first.


````md
## Crawl4AI Usage

When web content needs to be fetched, rendered, or extracted from dynamic pages, use the running Crawl4AI Docker service instead of writing ad hoc scraping code.

### Basic rules
- Prefer Crawl4AI for page crawling, markdown extraction, and multi-page content collection.
- Use it for JavaScript-rendered pages when simple HTTP fetch is insufficient.
- Save extracted results into project artifacts or intermediate files when they are used for later steps.
- If crawling fails, report the target URL, command used, and error briefly.

### Example commands

Fetch one page:
```bash
curl -X POST http://localhost:11235/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "formats": ["markdown"]
  }'
````

Crawl multiple pages:

```bash
curl -X POST http://localhost:11235/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "urls": ["https://example.com", "https://example.com/docs"],
    "formats": ["markdown"]
  }'
```

Save result to a file:

```bash
curl -X POST http://localhost:11235/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "formats": ["markdown"]
  }' > crawl-result.json
```

## Conventions

- Before creating any test code, read `\home\chemtech\workspace\libxc_rs\docs\rust_crate_test_guideline.md` and follow it when designing and implementing the tests.

When working on this Rust project, always save the full Cargo output to a log file under the `log/` directory before investigating any build issues.

### Required procedure

1. Ensure the `log/` directory exists.
2. Run the relevant Cargo command and redirect **both stdout and stderr** to a log file under `log/`.
3. Do **not** rely only on terminal output. Use the saved log file as the primary source for analysis.
4. After the log file is created, investigate all **errors** first, then review **warnings**.
5. In your report:
   - identify the root cause of each error
   - identify the source location when possible
   - distinguish confirmed causes from hypotheses
   - propose concrete fixes in priority order
   - list warnings separately and state whether each warning is actionable or low priority

### Command examples

For build investigation:
```bash
mkdir -p log && cargo build > log/cargo.log 2>&1

For test investigation:

mkdir -p log && cargo test > log/cargo-test.log 2>&1
Analysis requirements
Read the log file completely before making conclusions.
Quote the relevant error and warning lines exactly when useful.
Do not guess if the log does not support the conclusion.
If multiple errors appear, determine whether later errors are cascading effects of an earlier failure.
Prioritize the earliest primary error that likely caused the rest.
After analyzing errors, summarize warnings and recommend whether they should be fixed now or later.
Output format

Provide results in this order:

Log file used
Primary error summary
Root cause analysis
Recommended fix steps
Warning review
Remaining uncertainties
=======
>>>>>>> origin/main
