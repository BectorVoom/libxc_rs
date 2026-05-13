---
title: Phase 10 paused — workspace cargo build OOMs on dev machine
date: 2026-05-08
phase: 10-workspace-level-modular-split
status: paused-pending-phase-9
tags: [oom, kernel-build-time, blocker, infra]
---

# Phase 10 paused at Wave 1 Task 1

## Why we stopped

Wave 0 Task 1 (`cargo check --workspace` baseline capture) cannot complete on the
current dev machine. Two consecutive attempts were killed by the Linux global OOM
killer mid-build:

| Attempt | Config | Process killed at | Last crate logged |
|---------|--------|-------------------|-------------------|
| 1 | `CARGO_BUILD_JOBS=2`, no rustc-wrapper | rustc 30 GB anon-rss / 65 GB total-vm | mid-MGGA batch (around `kernel-mgga-6`) |
| 2 | `CARGO_BUILD_JOBS=1`, `RUSTC_WRAPPER=sccache` | rustc 30 GB anon-rss / 48 GB total-vm | `kernel-mgga-31` |

Source (`journalctl -k`):
```
Out of memory: Killed process 492148 (rustc) total-vm:48826928kB,
  anon-rss:30014860kB ... oom_score_adj:100
```

## Hardware vs requirement

- **Hardware:** 31 GB RAM + 8 GB swap = ~39 GB total commit
- **Single-rustc floor:** at least one kernel sub-crate needs >30 GB anon-rss to
  expand its `#[cube]` proc-macros and type-check the resulting AST. `jobs=N`
  does not help — the floor is per-process.
- **sccache stats:** `Compile requests 0` after both runs. `RUSTC_WRAPPER` either
  did not propagate to the kernel sub-crate rustc invocations, or the sccache
  server itself was killed by the same OOM (its stats reset on death). Even with
  a working sccache, a fresh checkout has no warm cache for the current
  toolchain, so the first build of every kernel sub-crate is still a full
  rustc run.
- **Workspace shape:** 109 kernel sub-crates under `crates/kernel-*/`, each
  CubeCL-proc-macro-heavy. Phase 8 SUMMARY already mitigated by rebatching MGGA
  from 7 → 37 sub-crates (first-fit-decreasing bin packing). Phase 9
  ("reduce-kernel-build-time") is in flight to push this further.

## Why Phase 10 cannot proceed without it

`10-VALIDATION.md` `## Wave 0 Requirements` ties success criteria SC-6 / SC-7 /
SC-8 and the bisectability invariant to a green pre-refactor `cargo check` /
`cargo test` / `cargo build` over the **whole workspace**. Without those
anchors, post-refactor verification has nothing to diff against — we cannot
honestly claim "test parity preserved" or "zero new warnings" without the
baseline captured first.

## What we DID capture (kept in `log/`, NOT committed)

These survived and are safe to reuse on the next attempt — they don't depend
on cargo:

- `log/10-pre-baseline-public-surface.log` — 33 unique `use libxc_rs::…` paths
  from `verify/` `tests/` `examples/` `benches/`. Required namespaces present
  (math: 5, eval: 4, model: 7).
- `log/10-xtask-baseline/{generated.rs, generated_hybrid.rs,
  generated_propagation.rs, by_id.rs, by_name.rs, removed.rs}` — byte-equivalent
  snapshots of the 6 xtask-generated source files (~1.3 MB total).
- `log/10-pre-baseline-cargo-check.log` — **partial** log of the second OOM run,
  ending at `Checking libxc-kernel-mgga-31`. Useful as evidence; do **not** use
  as a baseline anchor.

## Workarounds discovered (for reference)

- `bindgen` for `libxc-sys` needs the clang resource directory. The system
  `libclang1-18` package is installed, but the include directory with
  `stddef.h` is missing. The ROCm distribution at
  `/opt/rocm-7.1.1/lib/llvm/lib/clang/20/include/` provides a working
  resource dir. To make bindgen happy, export:
  ```
  LIBCLANG_PATH=/opt/rocm-7.1.1/lib/llvm/lib
  BINDGEN_EXTRA_CLANG_ARGS="-resource-dir=/opt/rocm-7.1.1/lib/llvm/lib/clang/20"
  ```
  This is environmental, not a tracked file change. `cargo check -p libxc-sys`
  with these env vars completes in ~3 min.

## Resume conditions

Phase 10 can restart when **at least one** of the following becomes true:

1. **Phase 9 ships kernel-build-time reduction** that brings the per-rustc peak
   for every kernel sub-crate well under ~25 GB. (Preferred — fixes the root
   cause for everyone.)
2. **A bigger-RAM host is used** — 64 GB physical or a dev container with
   substantial swap. The `log/cargo-check-prev-oom.log` shows historical builds
   running on `/workspace/...` paths with mold + sccache + clang linker, i.e. a
   different host already exists.
3. **A swap budget of 32 GB+ is added on this machine** AND the user accepts a
   multi-hour build. Swap-thrashing rustc is functional but slow; not a great
   fit for repeated baseline captures.

## Stash status

The pre-flight stash is intact and will be popped:
```
stash@{0}: On main: phase-10-preflight-stash 2026-05-08T06:23:39+09:00
```
Contains the in-flight Phase 6 work on `src/compat/raw_handle.rs`,
`.cargo/config.toml`, `AGENTS.md`, etc. Restoring it returns the working tree to
where the user left off.

## Suggested next moves

```
/gsd-progress                       # See current roadmap state
/gsd-execute-phase 9                # Continue Phase 9 (kernel build time) on this host
                                    # — only Plan 09-07 oracle-parity-sweep remains per STATE.md
```

Phase 6 is also still in flight per STATE.md (Plan 06-02b mid-stream); resuming
that on the bigger-RAM host is also an option once available.
