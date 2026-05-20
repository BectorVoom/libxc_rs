---
quick_id: 260520-eem
slug: mgga-c-tpssloc-subcrate-split
date: 2026-05-20
status: complete
outcome: SUCCESS — mgga_c_tpssloc compiles under jobs=1 via facade + 7 shard sub-crates
predecessor: 260520-c91
duration: ~1.5h (incl. ~25 min compile sweep at jobs=1)
commits:
  - 769912377c   # feat(tools): per-functional sub-crate splitter (260520-eem)
  - 5118f47708   # feat(mgga_c_tpssloc): regen hier + shard lxc_pol into sub-crates (260520-eem)
  - edbe3c5b39   # docs(260520-eem): compile sweep logs + disposition
follow_up:
  - Re-add libxc-kernel-mgga_c_tpssloc to workspace default-members (now compiles cleanly). Separate decision; not executed here.
  - Revisit the 9 tpssloc partN.rs entries in tools/kernel_size_exceptions.txt (size audit after the split). Not executed here.
  - Plan 11.1-03 G4 (full-649 f32 oracle) is now unblocked w.r.t. tpssloc COMPILATION. Numeric parity is still G3/G4's job.
  - The splitter + the hierarchical CSE infra now cover the other D-LOCK-B candidates (gga_c_ft97 lxc_pol, mgga_c_kcis/kcisk/revtpss/rmggac lxc_pol, lda_c_pk09 kxc_pol) — same recipe applies if/when they need it.
---

# Quick task 260520-eem — mgga_c_tpssloc sub-crate split with facade (Option A)

## Outcome: SUCCESS — `cargo build -p libxc-kernel-mgga_c_tpssloc` now compiles under jobs=1

This is the chosen "Option A" follow-up to quick task 260520-c91. The predecessor solved
the per-`#[cube]-fn` proc-macro OOM (hierarchical CSE) but uncovered a NEW bottleneck:
one rustc process holding parse + IR + monomorphization state across ~63K modules of a
single crate exceeds the 30 GB box. This task splits the functional into a thin **facade**
crate plus **7 shard sub-crates**, each compiling in its own rustc process — bounding the
aggregate state. Every shard AND the facade now compile under jobs=1 with no OOM.

User-locked design (both honored): hierarchical layout INSIDE the shards; facade re-export
crate keeps the public name `libxc-kernel-mgga_c_tpssloc`.

## What landed

### Task 1 — the splitter tool (commit 769912377c)

`tools/split_per_functional_subcrate.py` (723 lines) — a generic POST-PROCESS splitter
(it does NOT call the translator; it reorganizes an already-regenerated facade crate).
CLI: `python3 tools/split_per_functional_subcrate.py <family> <func> <output> --budget <files-per-shard> [--dry-run] [--selftest]`.

- Scans an output's part UNITS (a flat `partN.rs` file OR a `partN/` directory); weight =
  recursive `*.rs` count.
- Bin-packs CONTIGUOUS part ranges into shards under `--budget`; a single part exceeding
  budget gets its own solo shard (parts are atomic, never split).
- Names shards `<func>_pK`; materializes each shard's `Cargo.toml` + `src/lib.rs`
  (`pub use <output>::<partfn>;` re-exports) + thin `src/<output>/mod.rs` index; MOVES
  (shutil.move) each owned part unit in.
- Rewrites the facade `<output>/mod.rs`: drops `mod partN;`, re-sources each
  `use partN::fn;` to `use <shard_ident>::fn;`, leaves the `#[cube]` header + all call
  statements VERBATIM.
- Patches facade `Cargo.toml` (one dep per shard) + root workspace `Cargo.toml` (one
  path-dep per shard; shards intentionally NOT added to `default-members`).
- `--selftest` (synthetic facade in a tempdir) + `--dry-run` + idempotency guard +
  D-LOCK-D determinism (re-run byte-identical).

### Task 2 — regen + shard (commit 5118f47708)

Regen via the predecessor's `run_regen.py` with both env vars
(`LIBXC_RS_HIERARCHICAL_CSE=1 LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1`) produced the expected
hier shape: **63,360 `.rs` files, 7366 meta dirs, largest single `.rs` 4487L** — exact
match to 260520-c91's documented shape.

Split at **budget 10000** → **7 shards**, all under budget (no over-budget solo shard
needed):

| shard | parts | files |
|-------|-------|-------|
| p0 | 0..=20 | 7818 |
| p1 | 21..=23 | 9629 |
| p2 | 24..=28 | 9776 |
| p3 | 29..=31 | 8815 |
| p4 | 32..=39 | 9902 |
| p5 | 40..=53 | 9943 (largest by files) |
| p6 | 54..=121 | 4971 |

All 7 sanity checks passed: 7 shard dirs each with Cargo.toml + lib.rs + lxc_pol/mod.rs;
all ≤ budget; facade `lxc_pol/mod.rs` has 0 `mod partN;` + 122 shard-sourced `use` lines +
122 part-call statements unchanged; facade keeps name `libxc-kernel-mgga_c_tpssloc`;
per-shard deps in facade + root Cargo.toml; 0 shards in `default-members`; splitter is
idempotent.

### Task 3 — compile sweep (commit edbe3c5b39)

Single pass, jobs=1, no retry loop. All builds PASS (exit 0):

| build | parts | files | peak RSS | wall |
|-------|-------|-------|----------|------|
| p5 (file-count canary) | 14 | 9943 | 9.0 GB | 2:10 |
| p0 | 21 | 7818 | 9.7 GB | 2:38 |
| p1 | 3 | 9629 | 7.5 GB | 1:29 |
| p2 | 5 | 9776 | 6.9 GB | 1:24 |
| p3 | 3 | 8815 | 6.6 GB | 1:17 |
| p4 | 8 | 9902 | 7.5 GB | 1:41 |
| **p6** | **68** | **4971** | **16.5 GB (worst)** | 6:24 |
| **facade** | links 122 | — | **9.0 GB** | 3:05 |

**`cargo build -p libxc-kernel-mgga_c_tpssloc` SUCCEEDS at ~9.0 GB peak, 3:05.**
Re-verified post-commit: cached build returns `Finished` in 0.26s.

## Key finding — RSS scales with PART COUNT, not file count

The most load-bearing insight: per-rustc peak RSS tracks the **number of distinct `#[cube]`
functions** that one rustc process must expand + monomorphize, NOT the raw `.rs` file
count.

- **p6** has the FEWEST files (4971) but the MOST parts (68 — it absorbed the tail
  parts 54..=121, most of which are small single-output components) → the WORST RSS at
  16.5 GB and the longest compile (6:24).
- **p5** has the MOST files (9943) but only 14 parts (it holds dense 4th-derivative parts
  whose bulk is meta/chunk sub-modules, not top-level `#[cube]` entry fns) → only 9.0 GB.

So the file-count budget (10000) was a *conservative proxy* — it happened to bound
part-count adequately because dense parts have huge file/part ratios, but the
correlation isn't perfect. **For future splits of the other D-LOCK-B candidates, budget on
PART COUNT (target ≲ 70 parts/shard given the 16.5 GB observed at 68 parts), or on a blend,
rather than pure file count.** Every shard here still landed comfortably under 30 GB, so no
re-shard was needed.

## Facade compile confirms the linchpin (key_risk 1 / verified_structural_fact 3)

The facade compiled at ~9.0 GB — cheap relative to the shards. This confirms the design
assumption: a `#[cube]` fn calling another `#[cube]` fn (even cross-crate) **LINKS against
the callee's already-generated `expand` fn; it does NOT re-expand the body**. The facade's
`mgga_c_tpssloc_lxc_pol` wrapper type-checks 122 cross-crate calls cheaply; the ~9 GB is
the facade's own 9 remaining output modules (exc/vxc/fxc/kxc × unpol/pol + lxc_unpol)
compiling in the same crate, NOT the lxc_pol parts (which now live in the shards). Same
mechanism by which tpssloc already links `pow_1_3`/`piecewise3` from the math crate.

## Structure delivered

```
crates/kernels/mgga/mgga_c_tpssloc/          ← FACADE (libxc-kernel-mgga_c_tpssloc, name UNCHANGED)
  Cargo.toml                                  ← + 7 shard deps
  src/lib.rs, src/lxc_pol/mod.rs (#[cube] wrapper, shard-sourced use), + 9 other outputs
crates/kernels/mgga/mgga_c_tpssloc_p0../p6/  ← 7 shard sub-crates (lxc_pol parts)
Cargo.toml (root)                             ← + 7 shard path-deps (workspace members; NOT in default-members)
```

## What this changes about the project

- **mgga_c_tpssloc compiles again** under jobs=1 on the 30 GB box — the long-standing OOM
  (open since 260520-a0c) is resolved end-to-end: hierarchical CSE (260520-c91) fixed the
  per-fn macro RAM; this sub-crate split (260520-eem) fixed the aggregate rustc state.
- **New reusable tool** `tools/split_per_functional_subcrate.py` — generic per-functional
  sub-crate splitter for any oversized functional in the D-10 layout.
- **New per-functional split pattern** established: facade crate (keeps the D-10
  one-crate-per-functional public name) + `_pK` shard sub-crates. First instance in the
  per-functional layout (the older `_a`/`_b` splits were on the batched layout via
  `split_oversized_kernel.py`).
- mgga_c_tpssloc is still OUT of `default-members` (deferral from 260520-a0c unchanged) —
  re-adding it is a candidate follow-up now that it compiles.
- `tools/kernel_size_exceptions.txt` unchanged — its 9 tpssloc entries reference the
  pre-split partN.rs paths; revisiting them is a follow-up (size audit) once the split is
  permanent.

## Out of scope (unchanged)

- Numeric correctness / oracle parity — Plan 11.1-03 G3/G4 covers it; no oracle tests run
  (also avoids the verify/ crate OOM per memory `feedback_verify_crate_oom`).
- Full-tree regen; other D-LOCK-B functionals.
- Re-adding to default-members; kernel_size_exceptions cleanup — both deferred.

## Artifacts

| File | Purpose |
|------|---------|
| `260520-eem-PLAN.md` | The 3-task plan |
| `260520-eem-regen.log` | Hier regen output (63,360 files) |
| `260520-eem-split.log` | Splitter output (7-shard plan + sanity) |
| `260520-eem-compile.log` | Per-build peak RSS + the part-count-vs-file-count finding |
