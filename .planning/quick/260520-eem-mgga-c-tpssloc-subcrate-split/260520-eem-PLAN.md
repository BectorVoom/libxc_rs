---
quick_id: 260520-eem
slug: mgga-c-tpssloc-subcrate-split
type: quick
created: 2026-05-20
mode: inline-sequential
predecessor: 260520-c91
autonomous: false
files_modified:
  - tools/split_per_functional_subcrate.py            # NEW — the post-process splitter tool
  - crates/kernels/mgga/mgga_c_tpssloc/Cargo.toml      # + [dependencies] on shard crates
  - crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs  # re-source `use` from shards
  - crates/kernels/mgga/mgga_c_tpssloc_*/              # NEW — shard sub-crates (generated)
  - Cargo.toml                                         # + root [dependencies] path-deps for shards
  - .planning/quick/260520-eem-mgga-c-tpssloc-subcrate-split/260520-eem-SUMMARY.md  # at close

must_haves:
  truths:
    - "A facade crate named libxc-kernel-mgga_c_tpssloc still exists at crates/kernels/mgga/mgga_c_tpssloc with that exact package name (D-10 public-interface invariant preserved)."
    - "The 122 lxc_pol parts are physically distributed across N shard sub-crates (N>1); no single shard holds the whole lxc_pol part set."
    - "The facade's lxc_pol/mod.rs keeps its #[cube] pub fn mgga_c_tpssloc_lxc_pol(...) body with all 122 call statements unchanged in order; only the import sources change from partN::fn to <shard_crate>::fn."
    - "The largest shard sub-crate compiles under jobs=1 without OOM (peak RSS materially under the 30 GB box; canary target ~12 GB)."
    - "The facade crate itself compiles under jobs=1 without OOM (it links against shard expand fns; it does not re-expand part bodies)."
    - "Re-running the splitter on the same regen output produces a byte-identical crate layout (D-LOCK-D determinism)."
  artifacts:
    - path: "tools/split_per_functional_subcrate.py"
      provides: "Generic post-process splitter: bin-packs an output's parts across shard sub-crates, generates shard Cargo.toml + lib.rs, rewrites the facade output-wrapper imports + facade Cargo.toml, updates root workspace Cargo.toml. Includes --selftest and --dry-run."
      min_lines: 200
    - path: "crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs"
      provides: "Facade lxc_pol output wrapper with shard-sourced use imports and the unchanged 122-statement #[cube] body."
      contains: "pub fn mgga_c_tpssloc_lxc_pol"
    - path: "crates/kernels/mgga/mgga_c_tpssloc/Cargo.toml"
      provides: "Facade manifest with a [dependencies] entry per shard sub-crate (plus existing cubecl + libxc-kernel-math)."
      contains: "libxc-kernel-mgga_c_tpssloc"
    - path: "Cargo.toml"
      provides: "Root workspace manifest with one [dependencies] path-dep per shard sub-crate (makes them workspace members); shards intentionally absent from default-members."
      contains: "mgga_c_tpssloc"
  key_links:
    - from: "crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs"
      to: "shard sub-crate part fns"
      via: "use libxc_kernel_mgga_c_tpssloc_<shard>::mgga_c_tpssloc_lxc_pol_partN_<outputs>; then the existing call in the #[cube] body"
      pattern: "use libxc_kernel_mgga_c_tpssloc_"
    - from: "crates/kernels/mgga/mgga_c_tpssloc_<shard>/src/lib.rs"
      to: "the moved partN module(s)"
      via: "pub mod lxc_pol; pub use lxc_pol::partN::<fn>; re-export at a stable crate-root path"
      pattern: "pub use"
    - from: "Cargo.toml [dependencies]"
      to: "each shard crate dir"
      via: "path-dep entry — makes the shard a workspace member without adding it to default-members"
      pattern: "path = crates/kernels/mgga/mgga_c_tpssloc_"
---

<objective>
Make `cargo build -p libxc-kernel-mgga_c_tpssloc` succeed under jobs=1 on the 30 GB
dev box by splitting the functional into a thin **facade** crate plus N **shard**
sub-crates, each compiling in its own rustc process.

This is **Option A** from quick task 260520-c91. The predecessor solved the
per-`#[cube]-fn` proc-macro OOM (hierarchical CSE landed; largest single `.rs`
is 4487L) but uncovered a NEW bottleneck: one rustc process holding parse + IR +
monomorphization state across ~63K modules of a single crate exceeds 30 GB. Each
shard compiling in isolation bounds that aggregate state.

Purpose: unblock Plan 11.1-03 G4 (full-649 f32 oracle), which needs tpssloc to
compile. Numeric correctness/oracle parity is OUT OF SCOPE here (G3/G4 covers it).

Output:
- A new generic post-process splitter tool (`tools/split_per_functional_subcrate.py`).
- A regenerated-then-sharded `mgga_c_tpssloc` facade + N shard sub-crates.
- An empirical per-shard file budget validated by a canary compile.

LOCKED design decisions (from user, do NOT revisit):
- Hierarchical layout (`LIBXC_RS_HIERARCHICAL_CSE=1`) lives INSIDE the shard sub-crates.
- A facade re-export crate keeps the public name `libxc-kernel-mgga_c_tpssloc`.
</objective>

<execution_context>
@/home/user/Documents/workspace/libxc_rs/.claude/get-shit-done/workflows/execute-plan.md
@/home/user/Documents/workspace/libxc_rs/.claude/get-shit-done/templates/summary.md

RAM-constrained machine, jobs=1 (memory `feedback_ram_constraints`):
- Run inline-sequential. NO worktree isolation.
- Do NOT edit `.cargo/config.toml` (user manages jobs by hand).
- Each `cargo build -p` is minutes-long. Budget compiles; no speculative retry loops.

Path-scoped commits (memory `feedback_path_scoped_commits`): the working tree has
many pre-staged unrelated files (deleted .planning/spikes/, .cargo/config.toml,
.cache/, BUILD_ERROR_*.md, untracked partN/ dirs). EVERY commit MUST use
`git commit -m "msg" -- <explicit paths>` or `git commit --only -- <paths>`.
NEVER `git add -A` / `git add .`.
</execution_context>

<context>
@.planning/STATE.md
@.planning/quick/260520-c91-mgga-c-tpssloc-hierarchical-sub-wrapper/260520-c91-SUMMARY.md
@.planning/quick/260520-a0c-mgga-c-tpssloc-memory-spike-fix/260520-a0c-SUMMARY.md

Reference (read before Task 1; do NOT re-document the hier mechanism — cross-ref the SUMMARYs):
@tools/split_oversized_kernel.py          # bin-packing + Cargo.toml/lib.rs gen REFERENCE (old batched layout; adapt, do not reuse directly)
@tools/translate_v2/emit.py               # emit_output_dir — the part layout the splitter consumes
@.planning/quick/260520-a0c-mgga-c-tpssloc-memory-spike-fix/run_regen.py  # the regen driver — REUSE it, do not author a new translate entry point

<verified_structural_facts>
Established by orchestrator exploration — build on these, do NOT re-litigate:

1. Parts are the atomic shardable unit. Each
   `mgga_c_tpssloc_lxc_pol_partN_<outputs>(rho, sigma, lapl, tau, <its output buf(s)>,
   dens_threshold, zeta_threshold)` takes the SAME inputs and writes DISJOINT output
   buffers. No cross-part data dependency. The wrapper just calls all 122 in order.

2. Both parts and the lxc_pol wrapper are plain `#[cube]` (NOT `#[cube(launch)]`).

3. `#[cube]` fns expand per-DEFINITION, not per-call-site. A cross-crate `#[cube]`
   caller LINKS against the callee's already-generated `expand` fn; it does NOT
   re-expand the body. PROVEN today: tpssloc parts call `pow_1_3::<f64>`/`piecewise3::<f64>`
   from the `libxc-kernel-math` crate and compile by linking. THEREFORE the facade
   wrapper calling shard part fns compiles CHEAPLY. This is the linchpin and is confirmed.

4. The lxc_pol part fns are concrete-f64 (`rho: &Array<f64>`), NOT generic `<F: Float>`,
   so the facade calling them needs NO turbofish. (Turbofish only applies to generic
   cube fns — the math helpers, and the hier meta-fns INSIDE each part, which are
   internal to a part and unchanged by the split.)

5. lxc_pol dominates the file count (122 parts; the 9 dense 4th-derivative parts
   19-23/27-32 produced 7366 meta dirs). Splitting only at the output-module boundary
   would NOT help — lxc_pol alone OOMs. The split MUST shard lxc_pol's PARTS.

6. No live runtime dispatch depends on tpssloc yet (excluded from default-members;
   referenced only as a path-dep at root Cargo.toml:218). No routing table to update.

7. Workspace membership is by ROOT [dependencies] path-deps, NOT by an explicit
   members list. `members=["xtask","verify","libxc-sys"]` is the only literal list;
   every kernel subcrate is a member because it's a path-dep of the root package.
   So: add each shard as a root [dependencies] path-dep -> it becomes a workspace
   member. Leave it OUT of `default-members` -> bare `cargo build` skips it.

8. Current tpssloc on-disk + HEAD state: lxc_pol has 122 parts — 100 flat `partN.rs`
   files and 22 `partN/` directories (the CSE-chunked ones). The splitter MUST move
   whole part UNITS (file OR directory) intact. After Task 2 regen with hier ON, far
   MORE parts become directories.

9. The lxc_pol/mod.rs wrapper is generated deterministically by per_functional.py
   (lines ~340-367): a block of `mod part{i};`, then a block of `use part{i}::{pfn};`,
   then the `#[cube] pub fn` header, then one `{pfn}(args);` call per part, then `}`.
   The facade rewrite is mechanical: drop `mod part{i};`, change each
   `use part{i}::{pfn};` -> `use <shard_crate_ident>::{pfn};`, leave header + call body
   verbatim.
</verified_structural_facts>

<interfaces>
Facade lxc_pol/mod.rs current shape (the rewrite target):

```rust
//! ... lxc_pol (nested-by-output, 122 parts).
#![allow(...)]

mod part0;
mod part1;
// ...
mod part121;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_tpssloc_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
// ...
use part121::mgga_c_tpssloc_lxc_pol_part121_...;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol(rho: &Array<f64>, /* ... */ dens_threshold: f64, zeta_threshold: f64) {
    mgga_c_tpssloc_lxc_pol_part0_...(rho, sigma, lapl, tau, zk, /* ... */ dens_threshold, zeta_threshold);
    // ...
    mgga_c_tpssloc_lxc_pol_part121_...(rho, sigma, lapl, tau, /* ... */ dens_threshold, zeta_threshold);
}
```

Each part file/dir exports a single `#[cube] pub fn mgga_c_tpssloc_lxc_pol_partN_<outputs>(...)`.

Shard crate Cargo.toml shape (mirror emit.py CUBECL_DEP / math path-dep at the `../../` depth):
```toml
[package]
name = "libxc-kernel-mgga_c_tpssloc_<shard>"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../../math" }
```

Shard crate lib.rs must re-export each owned part fn at a STABLE crate-root path so the
facade's `use libxc_kernel_mgga_c_tpssloc_<shard>::<partfn>;` resolves. Recommended:
keep the `lxc_pol/` module subtree inside the shard and re-export:
```rust
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
pub mod lxc_pol;            // contains the owned partN modules + their own deps
pub use lxc_pol::mgga_c_tpssloc_lxc_pol_partN_<outputs>;   // one per owned part
```
where `lxc_pol/mod.rs` in the shard is a thin module index: `pub mod partN;` for each
owned part (plus `pub use partN::<fn>;`), and NO #[cube] wrapper (the wrapper stays in
the facade). Each moved `partN/` dir or `partN.rs` keeps its own `use libxc_kernel_math::...`
imports — they already do — so it links the math expand fns directly.
</interfaces>
</context>

<tasks>

<task type="auto">
  <name>Task 1: Build the per-functional sub-crate splitter (post-process) + selftest</name>
  <files>tools/split_per_functional_subcrate.py</files>
  <action>
Write a NEW generic post-process splitter `tools/split_per_functional_subcrate.py`.
It operates on an ALREADY-REGENERATED per-functional facade crate (it does NOT call
the translator). Adapt the bin-packing + boilerplate-gen logic from
`tools/split_oversized_kernel.py` (read it first) to the per-functional layout;
its `_a`/`_b` model targets the OLD batched layout, so do NOT reuse it directly.

Decide and implement POST-PROCESS over the regen output (key_risk 6: lower-risk than a
translator emit mode — it does NOT touch the proven hier emit path, and matches
split_oversized_kernel.py's model). Do NOT add a translator emit mode.

CLI: `python3 tools/split_per_functional_subcrate.py <family> <func> <output> --budget <files-per-shard> [--dry-run] [--selftest]`
For this task the concrete invocation will be:
`... mgga mgga_c_tpssloc lxc_pol --budget <N>` (Task 2 picks N).

Behavior:
1. SCAN `crates/kernels/<family>/<func>/src/<output>/`. Enumerate the part UNITS.
   A unit is a flat `partN.rs` FILE or a `partN/` DIRECTORY (verified_structural_fact 8).
   Count files recursively per unit (`partN.rs` = 1; `partN/` = recursive `*.rs` count).
   That file-count is the bin-packing weight (NOT line count — the predecessor OOM
   scaled with module/file count, 63,360 files > 30 GB; see 260520-c91-SUMMARY
   "Phase 2 result").
2. BIN-PACK part units into shards under `--budget` files-per-shard, using CONTIGUOUS
   part ranges (locked_design: keeps the facade `use` list readable). Walk parts 0..121
   in order; start a new shard when adding the next unit would exceed budget. A dense
   4th-derivative part dir can alone exceed budget; in that case it gets its OWN
   single-part shard — NEVER split a part across shards (verified_structural_fact 1:
   the part is atomic). Determinism (D-LOCK-D): shard assignment + naming is a pure
   function of (part order, file counts, budget) — re-runs byte-identical.
3. NAME shards `<func>_p0`, `<func>_p1`, ... (contiguous-range index). Package name
   `libxc-kernel-<func>_pK`; rust ident `libxc_kernel_<func>_pK`; directory
   `crates/kernels/<family>/<func>_pK/`.
4. MATERIALIZE each shard:
   - `Cargo.toml` per the <interfaces> shape (cubecl + `libxc-kernel-math = { path = "../../math" }`).
   - `src/lib.rs`: crate `#![allow(...)]`, `pub mod <output>;`, then one
     `pub use <output>::<partfn>;` per owned part (stable crate-root re-export).
   - `src/<output>/mod.rs`: thin index — `pub mod partN;` for each owned part; NO
     #[cube] wrapper (wrapper stays in the facade); optionally `pub use partN::<partfn>;`
     so the lib.rs re-export resolves in one hop.
   - MOVE (shutil.move) each owned `partN.rs` / `partN/` UNIT from the facade
     `src/<output>/` into the shard `src/<output>/`. Move (not copy) so the facade
     carries no duplicate part trees.
   Discover each part's `pub fn` name with regex `pub fn (\w+)\s*\(` in the part's
   `.rs` (for a dir unit, its `mod.rs`) — same approach as per_functional.py:359.
5. REWRITE the facade `src/<output>/mod.rs` (verified_structural_fact 9):
   - DROP every `mod part{i};` line.
   - REPLACE each `use part{i}::{pfn};` with `use <shard_ident_owning_i>::{pfn};`.
   - LEAVE the `#[cube] pub fn` header, ALL call statements, and `}` VERBATIM.
   - LEAVE the math/cubecl `use` lines the wrapper itself needs (M_PI, cubecl::prelude) untouched.
6. REWRITE the facade `Cargo.toml`: add one `[dependencies]` line per shard
   (`libxc-kernel-<func>_pK = { path = "../<func>_pK" }`).
7. UPDATE root workspace `Cargo.toml`: add one `[dependencies]` path-dep per shard
   (`libxc-kernel-<func>_pK = { path = "crates/kernels/<family>/<func>_pK" }`), inserted
   near the facade's existing path-dep (Cargo.toml:218 region). This makes each shard a
   workspace member (verified_structural_fact 7). Do NOT add shards to `default-members`
   (tpssloc is deferred; bare `cargo build` must skip them).
8. `--dry-run`: print the shard plan (count, per-shard part ranges, files-per-shard,
   the longest shard) and make NO filesystem changes.
9. `--selftest`: build a tiny synthetic facade in a tempdir (an output with a few flat
   parts + one dir part), run the split with a small budget, assert: (a) every part
   unit lands in exactly one shard; (b) the facade `mod.rs` has zero `mod partN;` lines
   and its `use` lines now point at shard idents; (c) the facade call body is unchanged;
   (d) shard lib.rs re-exports resolve (string check); (e) double-run is byte-identical.
   Mirror emit.py's `_selftest` tempdir + snapshot approach. Exit 0 success, 1 failure.

Idempotency guard: if the facade `<output>/mod.rs` already has zero `mod partN;` lines,
treat as already-split and exit 0 with a notice — do NOT double-move or crash.
  </action>
  <verify>
    <automated>python3 tools/split_per_functional_subcrate.py --selftest</automated>
  </verify>
  <done>
`--selftest` exits 0. A `--dry-run` against a synthetic input prints a sensible shard
plan. No real `crates/kernels/` files modified yet (tool only selftested + dry-run; the
real split happens in Task 2). Commit the tool (path-scoped):
`git commit -m "feat(tools): per-functional sub-crate splitter (260520-eem)" -- tools/split_per_functional_subcrate.py`
  </done>
</task>

<task type="checkpoint:human-verify" gate="blocking">
  <what-built>
A post-process splitter tool (`tools/split_per_functional_subcrate.py`) that will:
shard mgga_c_tpssloc's lxc_pol 122 parts into N contiguous-range sub-crates, generate
their Cargo.toml + lib.rs, MOVE the part units in, rewrite the facade lxc_pol/mod.rs
imports (keeping the #[cube] body verbatim), patch the facade Cargo.toml, and add root
workspace path-deps. Selftest + dry-run pass; NO real crates/kernels files changed yet.
  </what-built>
  <how-to-verify>
Before the RAM-heavy Task 2 regen + Task 3 compile sweep, review the approach:
1. Read `tools/split_per_functional_subcrate.py` — confirm it MOVES part units intact
   (file OR dir), keeps the facade package name `libxc-kernel-mgga_c_tpssloc` unchanged,
   and never rewrites the facade #[cube] call body.
2. Preview the shard shape at a chosen budget against the CURRENT (HEAD, pre-regen)
   facade:
   `python3 tools/split_per_functional_subcrate.py mgga mgga_c_tpssloc lxc_pol --budget 10000 --dry-run`
   (Reads the 30,795-file HEAD tree; the post-regen tree is ~2x denser, so the real
   Task 2 budget may differ — this is just a sanity preview.)
3. Confirm the proposed per-shard file budget + shard count look reasonable (target
   ~6 shards at <=~10-12K files/shard; the largest shard is the canary).
4. Confirm shards will NOT be added to `default-members`.
Approve to proceed to regen + sharding + compile sweep, or request changes.
  </how-to-verify>
  <resume-signal>Type "approved" (optionally with a budget override, e.g. "approved, budget 8000") or describe changes.</resume-signal>
</task>

<task type="auto">
  <name>Task 2: Regen tpssloc (hier ON) -> run splitter -> sanity-check layout</name>
  <files>crates/kernels/mgga/mgga_c_tpssloc/, crates/kernels/mgga/mgga_c_tpssloc_*/, Cargo.toml</files>
  <action>
Step A — REGEN. Reuse the predecessor's regen driver (constraint: do NOT add a new
translate entry point):
`LIBXC_RS_HIERARCHICAL_CSE=1 LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1 python3 .planning/quick/260520-a0c-mgga-c-tpssloc-memory-spike-fix/run_regen.py`
(run_regen.py asserts ACCEPT_OVERSIZED; HIERARCHICAL_CSE triggers the hier branch landed
at fde9608e00. Regenerates mgga_c_tpssloc into the real crates/kernels path: ~63K files,
9 problem parts get meta dirs, largest single `.rs` ~4487L; see 260520-c91-SUMMARY for
the expected shape.) Capture output to `260520-eem-regen.log` in the quick dir.

Step B — PICK THE BUDGET. Default = the value approved at the checkpoint, else 10000
files/shard. Justification (key_risk 2): predecessor data point is 63,360 files in ONE
crate -> >30 GB. Linear: ~20K files ~ 10 GB, so <=~10-12K files/shard targets ~6 shards
at <=~6 GB each, comfortably under the 30 GB box and the ~12 GB canary target. The
largest shard's compile in Task 3 is the empirical check.

Step C — SPLIT. Run the Task 1 tool on the real tree:
`python3 tools/split_per_functional_subcrate.py mgga mgga_c_tpssloc lxc_pol --budget <N>`
Capture output to `260520-eem-split.log`.

Step D — SANITY-CHECK the layout (NO compile yet — that is Task 3). Assert ALL of:
1. N shard dirs exist: `ls -d crates/kernels/mgga/mgga_c_tpssloc_p*/`. N>1.
2. Each shard has Cargo.toml + src/lib.rs + src/lxc_pol/mod.rs.
3. Files-per-shard <= budget (recursive `*.rs` count per shard dir). Report the LARGEST
   shard's file count (the Task 3 canary).
4. The facade `crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs` has ZERO
   `mod partN;` lines and its part `use` lines now reference `libxc_kernel_mgga_c_tpssloc_p*`
   idents. `grep -c '^mod part' .../lxc_pol/mod.rs` == 0.
5. The facade #[cube] body still has 122 `mgga_c_tpssloc_lxc_pol_partN_` call statements
   (count unchanged).
6. The facade Cargo.toml has a [dependencies] line per shard; root Cargo.toml has a
   path-dep per shard; NO shard appears in `default-members`.
7. Determinism: re-run the splitter (idempotency guard) -> it no-ops or reproduces
   byte-identical (D-LOCK-D). `git status --short` shows no NEW churn on a second run.

If sanity-check FAILS (facade still has `mod partN;`, a part unit duplicated, or imports
don't reference shards): STOP — do NOT proceed to Task 3. Document the failure in
260520-eem-split.log and the SUMMARY. Per `feedback_ram_constraints`, do NOT burn a
compile cycle on a known-bad layout.
  </action>
  <verify>
    <automated>test "$(grep -c '^mod part' crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs)" = "0" && ls -d crates/kernels/mgga/mgga_c_tpssloc_p*/ >/dev/null 2>&1 && test "$(grep -c 'mgga_c_tpssloc_lxc_pol_part' crates/kernels/mgga/mgga_c_tpssloc/src/lxc_pol/mod.rs)" -ge 122 && echo SANITY_OK</automated>
  </verify>
  <done>
Regen produced the hier layout; splitter sharded lxc_pol into N>1 contiguous-range
shards; all 7 sanity checks pass; the largest shard's file count is recorded (Task 3
canary target). Facade keeps name `libxc-kernel-mgga_c_tpssloc` and its #[cube] body
verbatim. NO compile attempted yet. Commit the sharded layout + regen artifacts
(path-scoped — enumerate each shard dir explicitly, do NOT `git add -A`):
`git commit -m "feat(mgga_c_tpssloc): regen hier + shard lxc_pol into sub-crates (260520-eem)" -- crates/kernels/mgga/mgga_c_tpssloc crates/kernels/mgga/mgga_c_tpssloc_p0 <...each shard...> Cargo.toml .planning/quick/260520-eem-mgga-c-tpssloc-subcrate-split/`
  </done>
</task>

<task type="auto">
  <name>Task 3: Compile sweep — largest shard (canary) first, then remaining shards, then facade</name>
  <files>crates/kernels/mgga/mgga_c_tpssloc/, crates/kernels/mgga/mgga_c_tpssloc_*/</files>
  <action>
Single-pass compile budget (constraint: jobs=1, minutes each, NO retry loop). Run each
build under `/usr/bin/time -v` and capture peak RSS to `260520-eem-compile.log`.

Step 1 — CANARY: compile the LARGEST shard first (most files, recorded in Task 2). It is
the worst case; if it fits, the rest fit (key_risk 2).
`/usr/bin/time -v cargo build -p libxc-kernel-mgga_c_tpssloc_<largest_shard>`
Disposition:
- PASS (peak RSS materially under 30 GB; ~12 GB target): per-shard budget validated.
  Proceed to Step 2.
- OOM (exit 137): the per-shard budget is STILL too high. STOP the sweep. Do NOT retry
  in-session (no runaway loop). Document the observed file count + the failing budget in
  the SUMMARY and recommend a SMALLER budget for a follow-up re-shard (re-run the Task 1
  tool with `--budget <smaller>` next session). The splitter is reusable, so re-sharding
  is cheap; the expensive part (translator regen) is already done and committed.

Step 2 — REMAINING SHARDS: compile each remaining shard, in order, under jobs=1.
`for s in <remaining shards>; do /usr/bin/time -v cargo build -p libxc-kernel-mgga_c_tpssloc_$s; done`
Any OOM here (unexpected, since the canary was the largest) -> STOP, document, same
re-shard recommendation.

Step 3 — FACADE: compile the facade LAST.
`/usr/bin/time -v cargo build -p libxc-kernel-mgga_c_tpssloc`
This is a DISTINCT risk (key_risk 1): theory says it LINKS against shard expand fns
(cheap, verified_structural_fact 3), but it type-checks 122 cross-crate #[cube] calls.
Disposition:
- PASS: the whole approach is validated. mgga_c_tpssloc now compiles under jobs=1.
- OOM: the facade itself is the bottleneck — the approach needs rework (shard the facade
  wrapper too, or 3-tier recursion / Option B). Document thoroughly; this is the
  load-bearing finding.

Do NOT run the verify/ crate or any oracle tests (out of scope; would re-trigger the
verify OOM per memory `feedback_verify_crate_oom`). Compile-only.

Post-compile, IF all three steps PASS: note (do NOT necessarily execute — separate
decision per constraints) that re-adding tpssloc to `default-members` is now a candidate
follow-up, and that the 9 tpssloc entries in `tools/kernel_size_exceptions.txt` could be
revisited only if a size audit passes. Leave both for a follow-up unless the user directs
otherwise here.
  </action>
  <verify>
    <automated>cargo build -p libxc-kernel-mgga_c_tpssloc 2>&1 | tail -5 | grep -qiE 'Finished|Compiling libxc-kernel-mgga_c_tpssloc' && echo FACADE_BUILD_OK</automated>
  </verify>
  <done>
Compile sweep ran single-pass: largest shard (canary) -> remaining shards -> facade.
Peak RSS per build captured in 260520-eem-compile.log. Disposition documented per
outcome. If all PASS: `cargo build -p libxc-kernel-mgga_c_tpssloc` succeeds under jobs=1
— the Option A split is validated and Plan 11.1-03 G4 is unblocked. If any OOM: the
failing budget + observed file count are recorded with a smaller-budget re-shard
recommendation (no in-session retry). Commit logs path-scoped:
`git commit -m "docs(260520-eem): compile sweep logs + disposition" -- .planning/quick/260520-eem-mgga-c-tpssloc-subcrate-split/`
  </done>
</task>

</tasks>

<verification>
Overall checks (compile-only; correctness is out of scope, G3/G4 covers it):
- `python3 tools/split_per_functional_subcrate.py --selftest` exits 0 (Task 1).
- Facade `lxc_pol/mod.rs` has zero `mod partN;` lines, 122 part-call statements, and
  shard-sourced `use` imports (Task 2 sanity).
- N>1 shard sub-crates exist; each <= budget files; largest recorded as canary (Task 2).
- Splitter is idempotent (second run byte-identical / no-op) — D-LOCK-D (Task 2).
- Largest shard compiles under jobs=1 without OOM (Task 3 canary).
- Facade compiles under jobs=1 without OOM — `cargo build -p libxc-kernel-mgga_c_tpssloc`
  succeeds (Task 3 final).
</verification>

<success_criteria>
- `cargo build -p libxc-kernel-mgga_c_tpssloc` succeeds under jobs=1 on the 30 GB box
  (PRIMARY — unblocks Plan 11.1-03 G4).
- The facade crate retains the exact package name `libxc-kernel-mgga_c_tpssloc` (D-10
  public-interface invariant).
- lxc_pol's 122 parts are distributed across N>1 shard sub-crates, each compiling in its
  own rustc process under budget.
- The splitter tool is generic, deterministic, selftested, and reusable for other
  oversized functionals.
- All commits are path-scoped (no sweep of pre-staged unrelated working-tree files).

OR, if a shard or the facade OOMs: a clear, documented disposition with the observed
file counts, the failing budget, and a concrete smaller-budget re-shard (or Option B
3-tier) recommendation — NO in-session retry loop.
</success_criteria>

<output>
After completion, create
`.planning/quick/260520-eem-mgga-c-tpssloc-subcrate-split/260520-eem-SUMMARY.md`
recording: the splitter design, the chosen budget + justification, the shard count +
files-per-shard, the canary + facade compile dispositions (with peak RSS), whether
`cargo build -p libxc-kernel-mgga_c_tpssloc` now succeeds, and the follow-up state
(default-members re-add candidate; kernel_size_exceptions.txt revisit; or re-shard /
Option B if OOM). Update STATE.md Quick Tasks Completed table + Session Continuity.
</output>
