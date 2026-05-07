# Phase 10: Workspace-Level Modular Split — Context

**Gathered:** 2026-05-07
**Status:** Ready for planning

<domain>
## Phase Boundary

Refactor the monolithic root `libxc_rs` crate into a layered Cargo workspace where types and metadata are separated from compute orchestration by **compiler-enforced** crate boundaries. After this phase:

1. **`crates/libxc-core/`** owns pure data: `model/`, `meta/` (incl. generated tables), `registry/`, `input/`, `output/`, `layout/`, `dims/`, `error/` (decided this discussion), and the existing `kernel-math` re-exports the previous `src/math/` shim used to provide. **Zero CubeCL imports. Zero compute logic.**
2. **`crates/libxc-eval/`** owns orchestration: `eval/`, `functional/`, `kernel/` glue, `workspace/`. Depends one-way on `libxc-core` and on the existing `crates/kernel-{lda,gga,mgga}*` family. Nothing in `libxc-core` may `use libxc_eval::...`.
3. **`crates/libxc-compat/`** owns the extern "C" shim — the ~85 entry points, `FunctionalSlot` machinery, thread-local errno, `xc_rs_last_error_*` accessors, `catch_unwind` wrapper, opaque `xc_func_type`/`xc_func_info_type`. Depends on `libxc-eval` + `libxc-core`. **Nothing depends on `libxc-compat` except its own cdylib output** (success criterion 4).
4. **Root `libxc_rs` crate** becomes a thin facade: `api/` (Phase 6 Layer 3 — `BatchEvaluator`, `FunctionalBuilder`, `EvaluateInput`) + curated public re-exports of `libxc-core` + `libxc-eval` types so downstream `use libxc_rs::...` paths still resolve.

The 170 existing kernel sub-crates (`kernel-{lda,gga,mgga}*` + `kernel-math`) are NOT touched. The Phase 8/9 sub-crate explosion stays exactly as-is.

**In scope:**

- Create `crates/libxc-core/`, `crates/libxc-eval/`, `crates/libxc-compat/` with `Cargo.toml` + `src/lib.rs`.
- Move `src/{model,meta,registry,input,output,layout,dims,error}/` into `crates/libxc-core/src/` (paths preserved within the crate).
- Move `src/{eval,functional,kernel,workspace}/` into `crates/libxc-eval/src/` (paths preserved).
- Move `src/compat/` into `crates/libxc-compat/src/` (paths preserved).
- Delete `src/math/mod.rs` (12-line dead re-export shim — D-02).
- Update `xtask/src/main.rs` and `xtask/src/generate_metadata.rs` to write into `crates/libxc-core/src/...` instead of root `src/...` (D-03).
- Reduce root `src/lib.rs` to: `pub mod api;` + curated `pub use libxc_core::...` / `pub use libxc_eval::...` re-exports preserving today's surface (planner picks exact curation strategy).
- Add `[lib] crate-type = ["rlib", "cdylib", "staticlib"]` to `crates/libxc-compat/Cargo.toml` with default name `libxc_rs` (so output is `libxc_rs.so` / `libxc_rs.a`).
- Hand-write `crates/libxc-compat/include/xc_rs.h` covering the extern "C" symbols Phase 6 ships.
- Update `verify/Cargo.toml`, integration tests, and `[workspace] default-members` to reflect the new crate names.
- Verify success criteria: `cargo tree -p libxc-core` shows zero CubeCL/kernel-* deps; `cargo tree -p libxc-eval` shows libxc-core but not libxc-compat; `cargo tree -p libxc-compat` shows both; root `libxc_rs::...` paths still resolve; `cargo test --workspace` matches pre-refactor pass/fail set; oracle parity at 1e-12 preserved on representative LDA/GGA/MGGA.

**Out of scope (deferred to future phases):**

- Splitting any kernel sub-crate further (Phase 8/9 territory).
- Renaming the phase directory (cosmetic, post-merge cleanup).
- Adding new functional or kernel — Phase 10 is a pure refactor.
- Moving `src/compat/ids.rs` to xtask-generated (currently a 2-line stub; if a future phase generates it AND it's a libxc-compat target, that phase's plan handles the second xtask path — D-05).
- A separate `libxc-codegen` library crate (xtask stays a thin string emitter — D-06).
- Binary drop-in compatibility with system libxc (Phase 6 D-A4-1 already broke this with `void → int` signatures).
- C header generation via cbindgen (D-09 — hand-written wins for our small surface).
- Performance benchmarks across the new boundary — Phase 7 territory.

</domain>

<decisions>
## Implementation Decisions

### Area 1 — `error/` placement (resolves blocker todo `audit-error-math-placement`, error half)

- **D-01:** `src/error/` moves into `crates/libxc-core/src/error/`. Both `libxc-eval` and `libxc-compat` already depend on `libxc-core` (one-way invariant), so they get `LibxcRsError` for free. The "spare kernels from heavy metadata" counter-position is moot — zero `crates/kernel-*/` source files construct `LibxcRsError` today (verified via `grep -rln "LibxcRsError" crates/`), and `LibxcRsError` already depends on `model::{DerivativeOrder, Family, FunctionalId, Spin}` so a separate micro-crate would still pull libxc-core anyway.
- **D-01a:** The de facto error split between typed enum (libxc-core) and FFI errno layer (libxc-compat) that Phase 6 implemented in `src/compat/errno.rs` + `src/compat/macros.rs` is preserved. The thread-local `RefCell<Option<CString>>`, `xc_rs_last_error_code`/`xc_rs_last_error_message` accessors, and `extern_c_wrapper!` macro stay in `libxc-compat`. Only the **typed enum** crosses into libxc-core.
- **D-01b:** The four current files in `src/error/` (`mod.rs` 382 lines + 2-line stubs `ffi.rs`, `internal.rs`, `public.rs`) move into `crates/libxc-core/src/error/`. Stubs may be deleted in the same plan if still empty.

### Area 2 — `math/` disposition (resolves blocker todo `audit-error-math-placement`, math half)

- **D-02:** **Delete `src/math/mod.rs` entirely.** It is 12 lines of pure `pub use libxc_kernel_math::{constants, powers, piecewise, polynomials, erf, spin, dft_quantities, bspline, lambert_w, expint_e1, special, integrate};` with **zero `crate::math::*` callsites in `src/`** (verified via `grep -rn "use crate::math" src/`). The kernel sub-crates already import `libxc_kernel_math` directly. Verify by running `cargo check --workspace 2>&1 | tee log/10-pre-math-delete.log`, deleting, then `cargo check --workspace 2>&1 | tee log/10-post-math-delete.log` to confirm no consumers.
- **D-02a:** Before deletion, check `verify/`, `benches/` (if any), and root `src/lib.rs` re-exports for `libxc_rs::math::*` paths. If found, replace with `pub use libxc_kernel_math::...` in the root facade so downstream consumers don't break.

### Area 3 — Generated-files + xtask flow (resolves research question `How to handle generated.rs files across the modular workspace split`)

- **D-03:** xtask writes generated outputs **directly to `crates/libxc-core/src/...`** post-split. Update hard-coded paths in `xtask/src/main.rs` (4 paths: `src/meta/generated.rs`, `src/registry/by_id.rs`, `src/registry/by_name.rs`, `src/registry/removed.rs`) and `xtask/src/generate_metadata.rs` (3 paths: `src/meta/generated.rs`, `src/meta/generated_hybrid.rs`, `src/meta/generated_propagation.rs`) to prefix `crates/libxc-core/`. Total: 7 path-string updates.
- **D-04:** No generated file currently crosses the libxc-core / libxc-eval / libxc-compat boundary. All 7 outputs land in libxc-core's domain. Rebuild behavior post-split: a metadata change triggers libxc-core rebuild + libxc-eval (depends on libxc-core) + libxc-compat (depends on both) + root + verify/. **libxc-compat does NOT need a rebuild** if only `meta/generated_propagation.rs` changes and libxc-compat doesn't reference it — but Cargo's crate-level granularity means if libxc-core's `lib.rs` re-exports change, libxc-compat does rebuild. Acceptable cost; this is not the critical-path item.
- **D-05:** **No pre-emptive multi-target xtask abstraction.** `src/compat/ids.rs` is a 2-line stub today and may never become xtask-generated. If a future phase generates a libxc-compat-bound file, that phase's plan adds the second target path (small change). Phase 10 stays narrowly scoped.
- **D-06:** xtask **stays a string emitter** — no path-dep on `libxc-core`. xtask/Cargo.toml's NOTE comment about avoiding the kernel-crate cascade still applies (transitively through `libxc_rs` if it depended on root). Even though libxc-core post-split is small, xtask doesn't need typed types for emission and a circular-feeling dep (xtask outputs into libxc-core's `src/`) adds confusion for no real win.
- **D-06a:** xtask's `find_workspace_root()` walking-up logic (`xtask/src/main.rs:265-278`, `xtask/src/generate_metadata.rs:138-`) is unchanged. Workspace root is still where the top-level `Cargo.toml` lives.

### Area 4 — `libxc-compat` crate-type + cdylib

- **D-07:** `crates/libxc-compat/Cargo.toml` declares `[lib] crate-type = ["rlib", "cdylib", "staticlib"]`. One crate, three artifact outputs. Matches libxc's own dual-output (`.so` + `.a`) plus retains rlib for Rust callers (verify/, root `libxc_rs` facade). Build cost: same compilation, just emits more artifact files. No `[bin]` target.
- **D-08:** **cdylib name = `libxc_rs`** (Rust default — no `[lib] name = "..."` override). Output files: `libxc_rs.so` (Linux) / `libxc_rs.dylib` (macOS) / `libxc_rs.dll` (Windows) and `libxc_rs.a`. Downstream consumers link via `-lxc_rs`. **Drop-in is source-level** (recompile against our header), not binary-level — Phase 6 D-A4-1 (`void → int` signature changes) already broke binary drop-in by design. A `libxc.so` name match would be cosmetic and risks linker collisions with system libxc on the same path.
- **D-09:** **C header is hand-written** at `crates/libxc-compat/include/xc_rs.h`, **committed**. Mirrors `libxc-master/src/xc.h` 1:1 minus the `void → int` signature changes. Surface is small (~100 declarations); cbindgen produces noisy output for a stable surface and adds a build-time dep. Phase 6 context already recommended this; Phase 10 confirms and pins the location to live alongside the cdylib it documents.
- **D-09a:** Phase 10 ships the header **only if Phase 6 has produced/committed it before Phase 10 starts**. If Phase 6 hasn't, Phase 10 does NOT block on writing one — the header generation is a Phase 6 deliverable per `06-CONTEXT.md` `### What Phase 6 Creates`, just relocated. Planner verifies state of the header at plan time.

### Claude's Discretion

These were left to the planner/researcher (not pinned by this discussion):

- **Plan decomposition across the 3 roadmap-allocated plans** (10-01 extract libxc-core, 10-02 extract libxc-eval, 10-03 extract libxc-compat + reduce root to facade). Sequencing: extract libxc-core first (smallest, leaf-most, highest churn ratio); libxc-eval second (depends on core); libxc-compat third (depends on both); root reduction last. **Each plan must leave the workspace `cargo check --workspace` green.** No big-bang atomic commit — atomic refactors of this size are bisect-hostile.
- **Root-facade re-export curation strategy** — three candidate shapes the planner picks among: (a) curated explicit re-exports matching today's `src/lib.rs:24-39` line-for-line (low churn for downstream); (b) blanket `pub use libxc_core::*; pub use libxc_eval::*;` (terse but exposes private-feeling internals); (c) split-by-module `pub mod model { pub use libxc_core::model::*; }` (preserves the namespace shape downstream callers see today). Recommend (a); planner can override.
- **Whether `verify/`, integration tests, and benches re-point to `libxc-core` directly or stay routed through root `libxc_rs`** — Phase-10-internal callers (verify/) probably stay through root since the root facade's whole point is to preserve those paths. But if a re-point would meaningfully clarify which layer is being tested (e.g., a unit test of `LdaInput` validation could go through libxc-core direct), the planner has discretion.
- **Workspace `[default-members]` post-split** — today the list explicitly enumerates ~100 kernel sub-crates. Add `crates/libxc-core`, `crates/libxc-eval`, `crates/libxc-compat`. Optional cleanup: collapse the kernel-* enumeration via `crates/kernel-*` glob (Cargo doesn't support glob in default-members today — workaround is the explicit list, so leave alone).
- **`src/main.rs` "Hello, world!" disposition** — 3-line vestigial binary in root. Either delete or leave alone (it's harmless). Planner picks; deleting is cleaner but is scope creep relative to the split itself.
- **Module-by-module move tactics within each plan** — `git mv` per directory (preserves blame), or copy-then-delete (loses blame but easier to bisect). Recommend `git mv`.
- **Cargo.toml dependency declarations** for the three new crates — exact dep list per crate (cubecl features, bytemuck features, thiserror, bitflags, kernel-* path-deps for libxc-eval). Planner reads existing root `Cargo.toml` and partitions.
- **Where `src/error/{ffi,internal,public}.rs` 2-line stubs go** — into libxc-core's `error/` if non-empty, deleted if empty. Planner picks.
- **Whether to also move the placeholder `src/math/` directory and any Cargo `[lib] crate-type` decisions down to the leaf level for libxc-core/libxc-eval** — they're rlib-only by default, no override needed unless future consumers want a libxc-core staticlib (not in scope).
- **Whether root `libxc_rs` adds any `[lib] crate-type` override** — recommend no; root stays rlib-only since the cdylib comes from libxc-compat (D-07). Confirms via success criterion 4 ("nothing depends on libxc-compat except the cdylib output").

### Folded Todos

- **`audit-error-math-placement`** (`.planning/todos/pending/audit-error-math-placement.md`) — both halves resolved by D-01 (error → libxc-core) and D-02 (delete math/). Mark this todo as resolved when CONTEXT.md is committed.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and pre-planning artifacts

- `.planning/ROADMAP.md` §Phase 10 (lines 207-229) — Goal, depends-on (Phase 6 stable seam), pre-planning blockers (both resolved by this discussion), success criteria 1-8, 3 plans allocated.
- `.planning/REQUIREMENTS.md` — project-level requirements; no Phase-10-specific entries beyond architecture (read for non-regression context).
- `.planning/PROJECT.md` — three-layer API constraint, 1e-12 oracle parity contract, "drop-in replacement for libxc in C/Fortran DFT codes" Constraint, BUILD-04 unsafe budget.
- `.planning/STATE.md` — current phase position; Phase 6 still executing, so Phase 10 must not collide with in-flight 06-02a/02b/03 work (depends-on note in ROADMAP).

### Phase 10 origin docs (resolved during this discussion)

- `.planning/notes/workspace-modular-architecture.md` — **Architecture lock.** Crate names, responsibilities (per-module mapping), one-way dependency invariant, and risks called out for planning. Read first.
- `.planning/research/questions.md` §"How to handle generated.rs files across the modular workspace split" — research question 1-5; resolved by D-03 through D-06.
- `.planning/todos/pending/audit-error-math-placement.md` — blocker todo; resolved by D-01 + D-02. Mark resolved on commit.

### Prior Phase Context (decisions that carry forward)

- `.planning/phases/01-foundation-and-registry/01-CONTEXT.md` — Phase 1 D-04 (xtask-generated committed Rust output — pattern preserved by D-03), D-08 (typed `LibxcRsError` shape).
- `.planning/phases/03-input-output-and-evaluation-framework/03-CONTEXT.md` — Phase 3 D-05 (OutputMask + Option<&mut> semantics — flows into libxc-core/output/), D-06 (caller-provided buffers, zero allocation).
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-CONTEXT.md` — Phase 5 D-13 (`Functional` is `Send + Sync` — preserved through libxc-eval).
- `.planning/phases/06-public-api-and-c-compatibility/06-CONTEXT.md` — Phase 6 D-A1-1 through D-A4-4 (compat layer architecture); the entire compat module being moved is the Phase 6 deliverable. **Read this file end-to-end before planning libxc-compat extraction.** Of particular note for Phase 10: Phase 6 puts `extern_c_wrapper!` macro in `compat/macros.rs`, thread-local errno in `compat/errno.rs`, opaque type forward decls in `compat/c_layout.rs` — these stay together and move as a unit (D-01a).
- `.planning/phases/09-reduce-kernel-build-time/09-CONTEXT.md` — Phase 9 sub-crate boundaries (170 kernel-* crates) are STABLE and NOT touched by Phase 10.

### Codebase entry points (read at plan time, not now)

- `src/lib.rs` — root re-exports, current public surface that the facade must preserve (D-domain). Lines 24-39 are the curated re-export list reference.
- `Cargo.toml` (root) — workspace + dependencies + `[default-members]` to update.
- `src/error/mod.rs` — 24-variant `LibxcRsError` enum; moves to `crates/libxc-core/src/error/mod.rs` per D-01.
- `src/math/mod.rs` — 12-line dead shim; deleted per D-02.
- `xtask/src/main.rs:291,329,355,387` — 4 hard-coded write paths; update per D-03.
- `xtask/src/generate_metadata.rs:445,595,643` — 3 hard-coded write paths; update per D-03.
- `xtask/Cargo.toml` — NOTE comment explains the no-libxc_rs-dep stance; preserve per D-06.
- `src/api/`, `src/compat/`, `src/eval/`, `src/functional/`, `src/kernel/`, `src/workspace/` — files that move into libxc-eval / libxc-compat per the architecture note.
- `verify/Cargo.toml`, `verify/tests/*.rs` — depend on root `libxc_rs::...` paths; stay through root facade per planner discretion above.

### libxc reference (read for non-regression context only)

- `libxc-master/src/xc.h` — authoritative C API surface; the hand-written `crates/libxc-compat/include/xc_rs.h` (D-09) mirrors this 1:1 minus `void → int` changes from Phase 6 D-A4-1.

### Logs convention (per user feedback memory)

- All `cargo check`/`cargo build` runs during execution log to `log/<descriptive>.log`. Suggested names: `log/10-01-libxc-core-cargo-check.log`, `log/10-02-libxc-eval-cargo-check.log`, `log/10-03-libxc-compat-cargo-check.log`, `log/10-final-cargo-check-workspace.log`, `log/10-final-cargo-tree-{core,eval,compat}.log`, `log/10-final-oracle-parity.log`.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **Architecture mapping is already done.** `.planning/notes/workspace-modular-architecture.md` enumerates which top-level `src/` module lands in which target crate. Planner does not need to re-derive this — they execute it.
- **xtask path-rewrite is small and isolated.** 7 hard-coded paths total across 2 files (`xtask/src/main.rs:291,329,355,387` + `xtask/src/generate_metadata.rs:445,595,643`). Confirmed via `grep -n` during scout. No xtask logic changes needed.
- **`LibxcRsError` already obeys the libxc-core boundary.** Its imports (`use crate::model::{DerivativeOrder, Family, FunctionalId, Spin};`) are all libxc-core types. Moving the file into `crates/libxc-core/src/error/` requires zero import changes — just `crate::model::` references resolve via the same crate.
- **Phase 6 already partitioned `src/compat/` into the right shape for extraction**: `c_layout.rs` (opaque types), `errno.rs` (thread-local), `ids.rs` (discovery — currently stub), `legacy_eval.rs` (33 evaluate functions), `macros.rs` (`extern_c_wrapper!`), `mod.rs`, `raw_handle.rs` (`FunctionalSlot` + alloc/init/end/free), `removed.rs`. The directory moves as a unit into `crates/libxc-compat/src/`.
- **`crates/kernel-{lda,gga,mgga}*` and `kernel-math` are stable.** Phase 8/9 finished the sub-crate explosion. `libxc-eval`'s Cargo.toml carries the path-deps that root `libxc_rs/Cargo.toml` carries today (`libxc-kernel-math`, `libxc-kernel-lda`, `libxc-kernel-gga`, `libxc-kernel-mgga`).

### Established Patterns to Continue

- **Edition 2024, MSRV 1.85+** (per CLAUDE.md and root Cargo.toml). All three new crates use Edition 2024.
- **Committed xtask-generated output** (Phase 1 D-04). The 6 generated files committed today move into `crates/libxc-core/src/` and remain committed.
- **`#![deny(warnings)]`** — root `src/lib.rs:1` sets this. Each new crate's `lib.rs` matches.
- **CARGO_TARGET_DIR convention** (per user memory) — shared `target/` at repo root. Workspace already enforces this; new crates inherit.
- **Cargo logs convention** (per user memory) — `log/<descriptive>.log` for every cargo run during execution; analyze from file, never terminal.
- **`cargo check` over `cargo build` for compile-error verification** (per user memory). Each plan's verification phase uses `cargo check --workspace 2>&1 | tee log/10-NN-<step>.log`.
- **`Send + Sync` everywhere** (Phase 5 D-13) — preserved through the move.
- **thiserror v2 at the library boundary** — `LibxcRsError` stays the typed-boundary type, now in libxc-core.

### Integration Points

- **`verify/` ↔ root `libxc_rs`** — `verify/Cargo.toml` has `libxc_rs = { path = ".." }`. Path stays the same; the curated re-exports through root preserve `use libxc_rs::LdaInput` etc. No verify/ test source changes expected if the facade re-exports are line-for-line preserved (planner discretion item).
- **`xtask/` ↔ libxc-core (post-split)** — xtask continues to write into a fixed crate's `src/`, just under a different prefix. xtask/Cargo.toml does NOT add a path-dep on libxc-core (D-06).
- **`libxc-compat` ↔ cdylib output** — `cargo build -p libxc-compat` produces `target/debug/libxc_rs.so` + `libxc_rs.a` + `libxc_rs.rlib`. The C header `crates/libxc-compat/include/xc_rs.h` documents the `.so` surface.
- **Root `libxc_rs` ↔ everything below** — `[lib]` is rlib-only (no override). Re-export curation makes downstream `use libxc_rs::...` paths resolve. The cdylib lives in libxc-compat (D-07).
- **`[default-members]` ↔ new crates** — adds `crates/libxc-core`, `crates/libxc-eval`, `crates/libxc-compat` to the existing kernel-* enumeration. `cargo build` (no `-p`) post-split builds all three plus the kernel-* crates plus root.

### What Phase 10 Creates

- `crates/libxc-core/Cargo.toml` (new)
- `crates/libxc-core/src/lib.rs` (new — `pub mod model; pub mod meta; pub mod registry; pub mod input; pub mod output; pub mod layout; pub mod dims; pub mod error;`)
- `crates/libxc-core/src/{model,meta,registry,input,output,layout,dims,error}/` (moved from root `src/...`)
- `crates/libxc-eval/Cargo.toml` (new)
- `crates/libxc-eval/src/lib.rs` (new — `pub mod eval; pub mod functional; pub mod kernel; pub mod workspace;`)
- `crates/libxc-eval/src/{eval,functional,kernel,workspace}/` (moved from root `src/...`)
- `crates/libxc-compat/Cargo.toml` (new — `[lib] crate-type = ["rlib", "cdylib", "staticlib"]`)
- `crates/libxc-compat/src/lib.rs` (new)
- `crates/libxc-compat/src/{compat module contents}` (moved from root `src/compat/`)
- `crates/libxc-compat/include/xc_rs.h` (hand-written, D-09 — only if not already produced by Phase 6 by the time Phase 10 starts)

### What Phase 10 Modifies

- `src/lib.rs` (root) — reduced to `pub mod api;` + curated re-exports of libxc-core + libxc-eval surfaces.
- `Cargo.toml` (root) — `[dependencies]` shrinks (most deps move to leaf crates); adds `libxc-core`, `libxc-eval`, `libxc-compat` path-deps; updates `[workspace] members` and `[workspace] default-members`.
- `xtask/src/main.rs` — 4 path-string updates per D-03.
- `xtask/src/generate_metadata.rs` — 3 path-string updates per D-03.
- `verify/tests/*.rs` — possibly zero changes if root facade re-exports preserve paths line-for-line.

### What Phase 10 Deletes

- `src/math/mod.rs` — 12-line dead re-export shim (D-02).
- `src/error/{ffi,internal,public}.rs` — 2-line stubs (only if still empty at execute time; planner-discretion item above).
- `src/main.rs` — 3-line "Hello, world!" (planner-discretion item above; safe to leave).

</code_context>

<specifics>
## Specific Ideas

- **D-01 reasoning grounding** — verified during scout that zero `crates/kernel-*/` source files contain the string `LibxcRsError` (`grep -rln "LibxcRsError" crates/` returned empty). The micro-crate counter-position from the audit-error-math-placement todo is therefore moot today. Future-proofing comment: if any future kernel-* crate genuinely needs to construct a typed error, add a path-dep on libxc-core. libxc-core's compile cost post-split is small (no kernels, no CubeCL), so the dep is cheap.
- **D-02 verification recipe** — pre-delete: `cargo check --workspace 2>&1 | tee log/10-pre-math-delete.log`. Delete `src/math/`. Post-delete: `cargo check --workspace 2>&1 | tee log/10-post-math-delete.log`. Diff the two logs — if no new errors, no consumers existed. If errors appear (likely 0 based on scout), surface them, decide on per-callsite basis.
- **D-03 mechanical update** — `xtask/src/main.rs:291,329,355,387` is `let path = root.join("src/...");`. Change to `let path = root.join("crates/libxc-core/src/...");`. Same for `xtask/src/generate_metadata.rs:445,595,643`. No other xtask logic changes.
- **D-07 Cargo.toml shape**:
  ```toml
  [package]
  name = "libxc-compat"
  version = "0.1.0"
  edition = "2024"

  [lib]
  crate-type = ["rlib", "cdylib", "staticlib"]
  # default name = "libxc_rs" (no override) → libxc_rs.so / libxc_rs.a / libxc_rs.rlib

  [dependencies]
  libxc-core = { path = "../libxc-core" }
  libxc-eval = { path = "../libxc-eval" }
  thiserror = "2.0"
  ```
- **D-09 header note** — phase-6 context already names this file in `06-CONTEXT.md` `### What Phase 6 Creates` as `compat/include/xc.h or target/include/xc.h, planner picks`. Phase 10 pins to `crates/libxc-compat/include/xc_rs.h` (renamed `xc.h → xc_rs.h` to avoid collision with libxc's own xc.h on the same include path). If Phase 6 already committed at a different path, Phase 10's plan moves it.
- **Plan sequencing rationale** — extract leaf-most first: libxc-core has no internal deps on root types post-extraction (already verified — its modules only import each other). libxc-eval depends on libxc-core, so it goes second. libxc-compat depends on both, so it goes third. Each plan must run a `cargo check --workspace` and an oracle parity sample (1 LDA + 1 GGA + 1 MGGA at order 0 & 2, both spin modes — small enough to be fast, broad enough to catch type-routing regressions). Final plan adds the cdylib + header + workspace tree assertions.
- **Bisectability** — every commit in this phase MUST leave `cargo check --workspace` green. No "WIP intermediate state" commits. If a single git mv produces too many failing imports to fix in one commit, split into smaller commits but keep each green. Planner's responsibility to decompose plans accordingly.

</specifics>

<deferred>
## Deferred Ideas

- **Pre-emptive multi-target xtask abstraction (D-05)** — only do it when a real second target appears.
- **`libxc-codegen` library crate (D-06 alternative)** — if xtask grows substantially or generators need unit testing, carve it out then.
- **xtask path-dep on libxc-core (D-06 alternative)** — if a future generator wants type-checked emission via `quote!`, revisit.
- **Phase directory rename** — `10-workspace-level-modular-split` is fine; cosmetic.
- **`libxc.so` binary-drop-in name (D-08 alternative)** — if a downstream user genuinely wants `-lxc` without recompile, set `[lib] name = "xc"`. Phase 6 D-A4-1 already broke binary drop-in by signature, so this would be cosmetic.
- **cbindgen-generated header (D-09 alternative)** — revisit if extern-C surface grows beyond ~150 declarations or if drift becomes a real problem.
- **`libxc-error` micro-crate (D-01 alternative)** — only worth carving if a kernel-* crate adds a real need to construct typed errors AND libxc-core's compile cost matters in that path.
- **`src/main.rs` deletion** — planner discretion; safe to leave, cleaner to delete.
- **Workspace `[default-members]` glob** — Cargo doesn't support glob there today; the explicit kernel-* enumeration stays.
- **`libxc-core` staticlib target** — only if a downstream consumer wants to embed the data layer without the rest. Speculative.
- **`libxc-eval` cdylib target** — same. The cdylib is a libxc-compat concern (FFI surface).
- **Per-module unit tests in libxc-core that are currently in `src/` test modules** — they move with the code; no separate test reorganization plan.
- **Updating root `Cargo.toml`'s `default-members` to drop the kernel-* enumeration in favor of the three new crates** — kernel-* crates still need to compile by default for `cargo build`. Leave the explicit list alone.

### Reviewed Todos (not folded)

None — `audit-error-math-placement` was the only Phase-10-relevant todo and it was folded (resolved via D-01 + D-02).

</deferred>

---

*Phase: 10-workspace-level-modular-split*
*Context gathered: 2026-05-07 (assistant: Opus 4.7 1M)*
*Discussion length: 4 selected gray areas (error/, math/, generated-files+xtask, libxc-compat crate-type+cdylib); 9 batched AskUserQuestion subquestions; all resolved on first answer with no scope creep*
