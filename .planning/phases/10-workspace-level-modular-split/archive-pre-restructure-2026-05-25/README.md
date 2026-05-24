# Archived Phase 10 plans (pre-restructure, 2026-05-07/08)

These four plans (10-00..10-03) were written 2026-05-07/08 against the OLD kernel
topology: ~170 flat `crates/kernel-{lda,gga,mgga}*` crates behind 4 umbrella façade
deps (`libxc-kernel-{math,lda,gga,mgga}`).

They were **never executed** (first `/gsd-execute-phase 10` OOM-killed 2026-05-08).
Phase 11's clean-slate restructure (D-10a) deleted the umbrella crates and replaced
them with per-functional crates (now 306, under `crates/kernels/{lda,gga,mgga,math}/`),
and Phase 11-12 made the kernel deps `optional` behind `[features] oracle-{lda,gga,mgga}`.

They are factually stale (10-02/10-03 depend on `crates/kernel-{lda,gga,mgga}` which no
longer exist) and were archived on 2026-05-25 when the phase was re-planned against the
live topology per CONTEXT.md decisions D-10..D-14. Kept for the git-mv recipe detail and
audit trail. Do NOT execute these.
