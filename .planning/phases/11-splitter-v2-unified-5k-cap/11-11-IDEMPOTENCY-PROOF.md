# Phase 11 · Plan 11 — D-LOCK-D Translator Idempotency Proof

**Verdict: SATISFIED** (264 non-sharded zero-diff + sharded pair handled per Option A)
**Date:** 2026-05-22
**Method:** Direct `translate --family all` + `git diff` (the stale `test_idempotency.sh` was NOT used — it references the pre-D-10a `crates/kernels/lda/src` path deleted in 11-03). Translator-only; **no cargo build**.

---

## Task 1 — 264 non-sharded functionals: ZERO DIFF

**Procedure**
1. Confirmed clean tree: `git status --porcelain -- crates/kernels/` → 0 lines (pre-regen).
2. Vanilla regen (hier-CSE default OFF): `python3 tools/maple_to_kernels.py translate --family all` → **rc=0**.
   - `[lda] ok=43 skipped=0 failed=0`
   - `[gga] ok=131 skipped=0 failed=0`
   - `[mgga] ok=92 skipped=0 failed=0`
   - Total **266** functionals emitted, 0 failed, 0 skipped.
3. Captured the diff EXCLUDING the two sharded functionals:

```
$ git diff --stat -- crates/kernels/ \
    ':(exclude)crates/kernels/mgga/mgga_c_tpssloc/**' \
    ':(exclude)crates/kernels/mgga/mgga_c_revtpss/**'
(no output — 0 lines)
```

**Cross-checks (catch untracked churn that `git diff` hides):**
- Tracked-modified files outside the sharded pair: `git diff --name-only -- crates/kernels/ | grep -v sharded` → **0**.
- Untracked files created outside the sharded pair: `git status --porcelain | grep '^??' | grep -v sharded` → **0**.

**Result:** the **264 non-sharded functionals** (266 emitted − 2 sharded) are **byte-stable** under a fresh re-translate — no modified tracked files, no new untracked files. **D-LOCK-D / ROADMAP SC #6 satisfied for the 264.** No codegen non-determinism found (none to root-cause).

4. Tree restored: `git checkout -- crates/kernels/ && git clean -fd crates/kernels/` removed the 235 transient regen artifacts; `git status --porcelain -- crates/kernels/` → 0. The committed facade/shard layout is intact.

---

## Task 2 — Sharded pair (mgga_c_tpssloc + mgga_c_revtpss): Option A (exclude-and-document)

**Why excluded from the 264 scope (by design).** A vanilla `translate --family all` (hier-CSE OFF) emits these two functionals **FLAT**, which is the *wrong* layout: they were converted to a **hierarchical-CSE regen + `split_per_functional_subcrate.py` shard** layout (facade + 7 `_pK` shards each) to clear a per-`#[cube]`-fn proc-macro OOM (memory `project_tpssloc_oom_resolution`; quick tasks `260520-eem` tpssloc, `260520-k1q` revtpss). The flat re-emit observed in Task 1 (122 untracked under `mgga_c_tpssloc/` + 113 under `mgga_c_revtpss/`, plus tracked facade churn) was **discarded** by the Step-4 restore — the committed OOM-fix layout was preserved verbatim and never committed flat.

**The sharded layout is a DETERMINISTIC two-stage post-process:**
1. `LIBXC_RS_HIERARCHICAL_CSE=1` regen of the functional, then
2. `python3 tools/split_per_functional_subcrate.py mgga <func> lxc_pol --budget 10000` → facade + 7 `_pK` shards (public crate name kept).

This recipe was proven reproducible end-to-end **2/2** (tpssloc `260520-eem`, revtpss `260520-k1q`).

**Split-stage determinism — demonstrated now (no heavy regen needed):**

- **Tool selftest** — `python3 tools/split_per_functional_subcrate.py --selftest` → **SELFTEST PASS**:
  > "synthetic facade sharded; each part in exactly one shard; facade mod.rs re-sourced (zero `mod partN;`); #[cube] body unchanged; shard re-exports resolve; **double-run byte-identical**."
  The tool has an explicit *deterministic write* path and verifies its own double-run byte-identity.

- **Idempotency guard on the ACTUAL committed layout** — re-running the split on the committed shards is a verified no-op (the `_already_split` guard fires before any filesystem write):
  ```
  $ python3 tools/split_per_functional_subcrate.py mgga mgga_c_tpssloc lxc_pol --budget 10000 --dry-run
  NOTICE: mgga_c_tpssloc/lxc_pol already split (facade mod.rs has zero `mod partN;` lines) — nothing to do.
  $ python3 tools/split_per_functional_subcrate.py mgga mgga_c_revtpss lxc_pol --budget 10000 --dry-run
  NOTICE: mgga_c_revtpss/lxc_pol already split (facade mod.rs has zero `mod partN;` lines) — nothing to do.
  ```
  `git status --porcelain` → 0 after the demos (true no-op).

**Disposition:** the split is a deterministic, idempotent post-process layered on the (byte-stable) translator output. The sharded pair is therefore **excluded from the vanilla-translate zero-diff scope and documented** as a reproducible two-stage process, with the OOM-fix facade/shard layout preserved verbatim. (Option B — full pipeline replay diffed against the committed shards — was not required; Option A's lower-risk evidence is sufficient and avoids regenerating the two heaviest functionals on a RAM-constrained machine.)

---

## SATISFIED

D-LOCK-D is **SATISFIED**: the 264 non-sharded functionals are proven byte-stable under re-translate (zero diff, including untracked-file cross-checks), and the 2 sharded functionals (mgga_c_tpssloc, mgga_c_revtpss) are dispositioned per Option A as a deterministic, idempotent post-process whose split stage is demonstrated reproducible (selftest double-run byte-identical + committed-layout idempotency-guard no-op), with the OOM-fix layout preserved. Working tree restored clean; no cargo build performed.
