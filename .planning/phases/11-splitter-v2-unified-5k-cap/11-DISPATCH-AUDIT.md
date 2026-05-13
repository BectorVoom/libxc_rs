# Phase 11 Dispatch-Tree Freshness Audit (Blocker B1)

**Captured:** 2026-05-13 at start of Wave 0.
**Source-of-truth command:** `bash tools/audit_dispatch_tree.sh` (committed in this task).

## Snapshot

Pre-Phase-11, the dispatch tree (`src/eval/{gga,mgga}_dispatch/*.rs`) was scaffolded in
Phase 4-04 against a then-current MGGA topology of 37 numbered subcrates and a
GGA topology of 22 numbered subcrates. Subsequent rebatching (q05/q06/q08 quick tasks
+ bin-pack reductions) shrank the actual subcrate count to **8 GGA** + **17 MGGA**
children (current: lda-1, lda-2, gga-1..8, mgga-1..7, mgga-8a, mgga-8b, mgga-9a, mgga-9b,
mgga-10, mgga-11a, mgga-11b, mgga-12, mgga-13, mgga-14 — total 27 numbered subcrates
including LDA), but the dispatch tree was never regenerated against the smaller façade.

### Audit-script output (verbatim)

```text
GGA dispatch references: 10
GGA façade exposes:       8
GGA unresolved:           10
  batch13
  batch15
  batch16
  batch17
  batch18
  batch19
  batch20
  batch21
  batch22
  batch5g
MGGA dispatch references: 8
MGGA façade exposes:      17
MGGA unresolved:          8
  batch17
  batch21
  batch23
  batch28
  batch29
  batch30
  batch34
  batch35
FAIL: dispatch tree has unresolved batchN references against the current façade.
      This is pre-existing staleness from Phase 4-04; plan 11-05's collapse
      blast radius includes regenerating the dispatch tree to close it.
```

### Interpretation

- The 10 GGA unresolved names (`batch13`, `batch15..batch22`, `batch5g`) all point at
  batchN identifiers that no longer exist on the GGA façade (which exposes only `batch1..batch8`).
- The 8 MGGA unresolved names (`batch17`, `batch21`, `batch23`, `batch28`, `batch29`, `batch30`,
  `batch34`, `batch35`) all point at batchN identifiers above the current MGGA façade range
  (which exposes `batch1..batch7, batch8a, batch8b, batch9a, batch9b, batch10, batch11a,
  batch11b, batch12, batch13, batch14` — 17 aliases). Note: every GGA reference happens to be
  unresolved (`GGA unresolved: 10 == GGA references: 10`), which is a stark signal that the
  GGA dispatch was never updated after rebatching.

## Implication

`cargo check --workspace` fails today on the root crate (libxc_rs) at path resolution.
This is pre-existing brokenness from Phase 4-04, not Phase 11 regression. The Wave 0
audit is purely diagnostic — it does not attempt to fix the staleness.

## Resolution path

Plan 11-05 (subcrate collapse) is the natural place to close this:

- The dispatch tree must be regenerated to match whatever batchN identifiers the
  post-collapse façade exposes. Per D-10b (Strategy 1 obsolete), the per-family façade
  re-exports per-functional (`pub mod lda_x;`, `pub mod lda_c_pw;`, ...), not per-batch.
  The dispatch tree IS regenerated against this new shape: `src/eval/{gga,mgga}_dispatch/*.rs`
  paths become `crate::kernel::{family}::<func>::...` (no `batchN::` segment).
- The dispatch generator (`tools/generate_gga_dispatch.py`, plus the MGGA equivalent
  added in plan 11-02) must be re-run during the collapse operation, with output
  committed alongside the collapse changes.

Plan 11-05's `must_have` truths reflect this — the "preserve dispatch unchanged"
aspiration of an earlier draft is replaced by "regenerate dispatch as part of
collapse so it resolves against the post-collapse façade".

## Acceptance gate

`bash tools/audit_dispatch_tree.sh` exits 0 after plan 11-05 lands. Pre-Phase-11 it
exits 1 (expected; recorded here for posterity).
