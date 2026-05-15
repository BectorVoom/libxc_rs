# Phase 11 — Pre-Phase Baseline Snapshot

**Captured:** 2026-05-13 at start of Wave 0 (plan 11-01).
**Source of truth:** outputs of the commands below, run on the unmodified tree.

## Build envelope
See `.cargo/config.toml` (D-08, D-09 invariants — values NOT duplicated here).
Active settings at capture time: `jobs = 1`, `target-dir = .cache/cargo-target`,
`RUST_MIN_STACK = 67108864`.

## Oversized kernel files (P11-INV-2 baseline)
Command: `python3 tools/audit_kernel_size.py`
Result: **235**  (target after Phase 11: 0)

> Note: plan 11-01 originally projected 237. The −2 delta is drift from
> uncommitted work-in-progress that was stashed at start of Wave 0
> (stash entry `260512-q03-wrong-abi-chunker-spike`). The cap of 5,000
> lines remains the hard contract; the count just starts 2 lower.

## Maximum file size (P11-INV-2 baseline)
Command: `python3 tools/audit_kernel_size.py` (max_path field)
Result: **16703  crates/kernels/mgga-2/src/mgga_c_b94/kxc_pol.rs**
(target after Phase 11: ≤ 5000)

## Numbered subcrate count (P11-INV-1 baseline)
Command: `bash tools/audit_subcrate_collapse.sh` (FAIL output line)
Result: **27**  (target after Phase 11: 0)

> Note: plan 11-01 originally projected 22. The +5 delta is because the plan
> author was counting only `gga-N` + `mgga-N` (8 + 17 = 25) but the tree also
> has `lda-1` + `lda-2`. Full enumeration:
> `gga-1..8` (8), `lda-1`, `lda-2` (2), `mgga-1..7` (7), `mgga-8a`, `mgga-8b`,
> `mgga-9a`, `mgga-9b`, `mgga-10`, `mgga-11a`, `mgga-11b`, `mgga-12`,
> `mgga-13`, `mgga-14` (10) = 27 total.

## #[cube(launch_unchecked)] count (P11-INV-5 baseline)

> **SUPERSEDED 2026-05-15 by D-13 (11-CONTEXT.md).** The flat `≤23` count
> below was unsatisfiable against the D-10b per-functional dispatch design
> (`audit_cube_launch.sh` reports 1677 — the dispatch macros call
> `.launch_unchecked()` per `(functional × output)`). P11-INV-5 is revised to
> a per-design budget; `audit_cube_launch.sh` is rewritten in the replanned
> 11-03. The pre-W0 figures below are retained for history only — they are
> NOT the Phase-11 acceptance contract. See D-13 and the 2026-05-15 re-plan
> note in 11-CONTEXT.md.

Command: `bash tools/audit_cube_launch.sh` against pre-W0 tree
Pre-W0 baseline: **22**  (recorded in plan)
Post-W0 measurement: **23**  (+1 from this plan's Wave-0 spike test at
`crates/kernels/math/tests/spike_tuple_return_cube.rs:22`)

`audit_cube_launch.sh`'s `BASELINE` constant is set to **23** (post-W0),
not 22, so the script returns PASS for the post-W0 tree. This is documented
in `11-01-SUMMARY.md` as deviation D2. ~~**Phase 11 acceptance contract: count
MUST NOT increase further** beyond 23 in plans 11-02..06.~~ *(superseded — see
the D-13 note above.)*

## Workspace member count
Command: `cargo metadata --format-version 1 --no-deps | python3 -c "import sys,json; print(len(json.load(sys.stdin)['workspace_members']))"`
Result: **35**
(target after Phase 11: ≤ 11 — see plan 11-06 cleanup; 22 numbered subcrates
are removed and the family façades stay, leaving root + libxc-sys + verify +
math + lda + gga + mgga + a few tooling crates)

## cargo build --workspace peak RSS (P11-INV-3 baseline)
Run inline (no worktree dispatch — D-07).
Command: `/usr/bin/time -v cargo build --workspace 2>&1 | tail -25 | grep 'Maximum resident'`
Result: **TODO — to be filled by user once a safe-RAM moment exists**

This metric is captured for OOM regression check after Wave 4 collapse. It
requires a successful `cargo build --workspace` which itself requires the
dispatch-tree staleness (B1, see `11-DISPATCH-AUDIT.md`) to be resolved.
On the pre-Phase-11 tree the workspace build FAILS at path resolution, so a
peak-RSS measurement is not capturable today. Plan 11-05 closes B1 and lands
a successful workspace build; the user will run this command post-11-05 and
patch in the value, OR (if a clean-tree build is desired at Wave 0) capture
`cargo build -p libxc-kernel-math` peak RSS as a leaf-crate proxy.

## phase11_smoke parity baseline
Command: `cargo test -p libxc_rs-verify --test parity_phase11 phase11_smoke -- --test-threads=1 --nocapture`
Result: **TODO — to be filled by user**

Per `[[feedback_verify_crate_oom]]`, running `cargo test -p libxc_rs-verify`
OOMs this machine because the verify crate's dev-dependencies
(`libxc-kernel-lda` + `libxc-kernel-mgga`) trigger a full kernel-tree compile
during the test build. The user runs this command in a higher-RAM environment
(e.g., after closing other apps) and pastes the PASS/SKIP/FAIL summary here.

Expected shape:
```
PARITY_TUPLE: smoke lda lda_x ... PASS
PARITY_TUPLE: smoke lda lda_c_pw ... PASS
PARITY_TUPLE: smoke lda lda_xc_teter93 ... PASS
PARITY_TUPLE: smoke gga gga_x_pbe ... PASS
PARITY_TUPLE: smoke gga gga_c_pbe ... PASS
PARITY_TUPLE: smoke gga gga_x_b88 ... PASS
PARITY_TUPLE: smoke mgga mgga_x_lta ... PASS
PARITY_TUPLE: smoke mgga mgga_x_tpss ... PASS
PARITY_TUPLE: smoke mgga mgga_x_pkzb ... PASS
PARITY_TUPLE: smoke mgga mgga_x_th ... PASS
PARITY_SUMMARY: smoke total=10 pass=10 skip=0 fail=0
```

If any MGGA entry FAILs at strict 1e-12, substitute per the W-D10-6
selection rule (executor picks first 4 of 8 widened candidates that PASS).
Document the substitution in `11-01-SUMMARY.md` and re-run.

## D-02 ABI spike (P11-INV-A1 baseline)
Command: `cargo test -p libxc-kernel-math --test spike_tuple_return_cube -- --nocapture`
Result: **PASS** (user-verified 2026-05-13)

`test spike_tuple_return_f64_cpu ... ok` — Wave 0 HARD GATE GREEN.

The full chunked-emission ABI for plans 11-02..05 is empirically proven on
this machine: cubecl-macros 0.10 round-trips `#[cube] fn f<F: Float>(...) -> (F, F)`
through parser → IR → cubecl-cpu runtime correctly. P11-INV-A1 satisfied.

Spike location: `crates/kernels/math/tests/spike_tuple_return_cube.rs`
(relocated from `verify/tests/` per OOM avoidance, see 11-01-SUMMARY.md D1).
Commit: `d17e2968`.
