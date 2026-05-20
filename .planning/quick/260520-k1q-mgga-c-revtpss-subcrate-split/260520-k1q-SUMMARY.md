---
quick_id: 260520-k1q
slug: mgga-c-revtpss-subcrate-split
date: 2026-05-20
status: complete
outcome: SUCCESS — mgga_c_revtpss compiles under jobs=1 via facade + 7 shard sub-crates (recipe replay of 260520-eem)
predecessor: 260520-eem
duration: ~1h (incl. ~26 min compile sweep at jobs=1)
commits:
  - c484ef470e   # feat(mgga_c_revtpss): regen hier + shard lxc_pol into sub-crates (260520-k1q)
  - 3639342c69   # docs(260520-k1q): compile sweep logs + disposition
follow_up:
  - Revisit the 11 revtpss lxc_pol entries in tools/kernel_size_exceptions.txt (lines 71-81) — they reference the pre-split flat partN.rs paths (part20-24, 28-33) which no longer exist as oversized flat files. Clean up only if a size audit passes. Not executed here.
  - Same hier+split recipe still available for the remaining D-LOCK-B candidates (gga_c_ft97 lxc_pol, mgga_c_kcis/kcisk/rmggac lxc_pol, lda_c_pk09 kxc_pol) if/when they hit the wall.
  - revtpss was ALREADY in default-members and stays there (unlike tpssloc) — no default-members change needed; it compiles.
---

# Quick task 260520-k1q — mgga_c_revtpss sub-crate split (recipe replay of 260520-eem)

## Outcome: SUCCESS — `cargo build -p libxc-kernel-mgga_c_revtpss` compiles under jobs=1

Direct replay of the proven mgga_c_tpssloc fix (quick task 260520-eem) applied to
mgga_c_revtpss, the next dense D-LOCK-B functional. Same two-layer recipe:
1. **Hierarchical CSE** (`LIBXC_RS_HIERARCHICAL_CSE=1`) bounds per-`#[cube]-fn` proc-macro RAM.
2. **Facade + shard sub-crate split** (`tools/split_per_functional_subcrate.py`) bounds
   aggregate rustc state per crate.

The splitter tool already existed (built in 260520-eem), so this task skipped tool-building
entirely — it was just: adapt the regen driver → regen → split lxc_pol → compile sweep.

## revtpss vs tpssloc — near-identical

| | tpssloc (eem) | revtpss (k1q) |
|---|---|---|
| lxc_pol parts | 122 | 113 |
| Worst flat part (pre-fix) | 9698 L | **12,649 L** (part22) |
| Pre-regen files | 30,795 | 19,970 |
| Post-hier-regen files | 63,360 | 62,858 |
| Largest single .rs after regen | 4487 L | 4463 L |
| Shards at budget 10000 | 7 | 7 |
| Worst shard RSS | 16.5 GB (68 parts) | 15.3 GB (61 parts) |
| Facade RSS | 9.0 GB | 9.5 GB |
| In default-members? | NO (excluded 260520-a0c) | **YES** (stays) |
| Secondary large output | — | kxc_pol (20 parts, 2871 files) — left in facade |

## What landed

### Regen + shard (commit c484ef470e)

- `run_regen_revtpss.py` (adapted from the tpssloc driver — only func name + c_file path
  differ; reuses `translate_mgga.emit_per_functional`, not a new entry point).
- Regen with hier ON produced 62,858 files; the 11 dense parts (20-24, 28-33) became meta
  dirs; largest single `.rs` dropped **12,649L → 4463L**.
- Split lxc_pol at budget 10000 → **7 shards** (p0..p6):

| shard | parts | files |
|-------|-------|-------|
| p0 | 0..=21 (22) | 9256 |
| p1 | 22..=23 (2) | 9320 |
| p2 | 24..=28 (5) | 9273 |
| p3 | 29..=31 (3) | 8493 |
| p4 | 32..=38 (7) | 9612 (most files) |
| p5 | 39..=51 (13) | 8919 |
| p6 | 52..=112 (61) | 5108 (most parts) |

- All 7 sanity checks passed. Facade `lxc_pol/mod.rs`: 0 `mod partN`, 113 shard-sourced
  `use` lines, 113 call statements intact. Facade keeps package name
  `libxc-kernel-mgga_c_revtpss`. kxc_pol (20 parts) stayed in the facade. revtpss remains
  in default-members; 7 shards added as deps (not to default-members).

### Compile sweep (commit 3639342c69) — all PASS, no OOM, no retry, no contingency

| build | parts | files | peak RSS | wall |
|-------|-------|-------|----------|------|
| p6 (part-count canary) | 61 | 5108 | **15.3 GB (worst)** | 7:58 |
| p4 (file-count canary) | 7 | 9612 | 7.5 GB | 2:06 |
| p0 | 22 | 9256 | 11.0 GB | 3:39 |
| p1 | 2 | 9320 | 7.5 GB | 2:07 |
| p2 | 5 | 9273 | 6.7 GB | 1:41 |
| p3 | 3 | 8493 | 6.3 GB | 1:32 |
| p5 | 13 | 8919 | 8.5 GB | 2:46 |
| **facade** | links 113 + kxc_pol 20pt in-crate | — | **9.5 GB** | 4:03 |

**`cargo build -p libxc-kernel-mgga_c_revtpss` succeeds at 9.5 GB peak** (cached re-verify
`Finished` 0.26s).

## Confirmations of the eem findings

- **RSS scales with PART COUNT, not file count** — re-confirmed: p6 has the FEWEST files
  (5108) but MOST parts (61) → worst RSS 15.3 GB; p4 has MOST files (9612) but 7 parts →
  7.5 GB. The 61-vs-68-part analog held (15.3 GB vs eem's 16.5 GB).
- **Facade links cheaply** — 9.5 GB confirms cross-crate `#[cube]` calls link against shard
  `expand` fns rather than re-expanding bodies. The watched risk (kxc_pol's 20 parts in the
  facade) resolved with margin; the kxc_pol-split contingency did NOT trigger.

## What this changes about the project

- **mgga_c_revtpss compiles under jobs=1** — the second dense D-LOCK-B functional fixed
  with the same recipe. It was already in default-members and stays there (so the default
  `cargo build` now builds it cleanly via the facade + transitive shard deps).
- The hier+split recipe is now demonstrated as a repeatable pattern (2/2 functionals). The
  ≲70-parts/shard budget heuristic held again (61 parts → 15.3 GB).
- New shard crates: `libxc-kernel-mgga_c_revtpss_p0..p6` (workspace members via root
  path-deps; not in default-members).
- `tools/kernel_size_exceptions.txt` still lists the 11 pre-split revtpss flat parts
  (lines 71-81) — they now reference paths that are meta dirs, not oversized flat files;
  cleanup is a deferred follow-up (size audit).

## Out of scope (unchanged)

- Numeric correctness / oracle parity — G3/G4 covers it; no oracle/verify tests run.
- kernel_size_exceptions.txt cleanup; the remaining D-LOCK-B functionals.

## Artifacts

| File | Purpose |
|------|---------|
| `260520-k1q-PLAN.md` | The 2-task plan |
| `run_regen_revtpss.py` | Adapted regen driver |
| `260520-k1q-regen.log` | Hier regen output (62,858 files) |
| `260520-k1q-split.log` | Splitter output (7-shard plan + sanity) |
| `260520-k1q-compile.log` | Per-build peak RSS (all PASS) |
