# Phase 10: Workspace-Level Modular Split - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-07
**Phase:** 10-workspace-level-modular-split
**Areas discussed:** error/ placement, math/ disposition, Generated-files + xtask flow, libxc-compat crate-type + cdylib

---

## Gray-area selection

| Option | Description | Selected |
|--------|-------------|----------|
| error/ placement | libxc-core (default), separate libxc-error micro-crate, or split (typed enum in core + FFI errno layer in compat). Resolves blocker todo audit-error-math-placement. | ✓ |
| math/ deletion vs absorption | src/math/mod.rs is 12 lines of pure re-exports of libxc_kernel_math with ZERO callsites in src/. Delete entirely, keep as a tiny shim in libxc-core, or fold into kernel-math. | ✓ |
| Generated-files + xtask flow | xtask writes hard-coded src/... paths today. Post-split: write directly into crates/libxc-core/src/{meta,registry}/, into a shared location and re-export, or move generators into a libxc-codegen crate. Resolves research question logged 2026-05-07. | ✓ |
| libxc-compat crate-type + cdylib | Does libxc-compat produce the cdylib (drop-in libxc.so) or stay rlib + separate cdylib target? Also staticlib? Affects libxc-compat Cargo.toml [lib], how C/Fortran consumers link, and where the C header lands. | ✓ |

**User's choice:** All four areas selected.
**Notes:** Architecture mapping in `.planning/notes/workspace-modular-architecture.md` already locks crate names + per-module mapping + one-way dependency direction; discussion narrowed to the four implementation gaps.

---

## error/ placement

| Option | Description | Selected |
|--------|-------------|----------|
| libxc-core (Recommended) | Move src/error/ into crates/libxc-core/src/error/. Both libxc-eval and libxc-compat depend on libxc-core anyway, so they get LibxcRsError for free. The original counter-argument (spare kernels from heavy metadata) doesn't apply — no kernel constructs errors today. Simplest, least churn. | ✓ |
| Separate libxc-error micro-crate | Forces libxc-core to also depend on libxc-error to construct typed errors that reference its own types — and we'd have to move Family/Spin/FunctionalId out of model/ into a smaller layer too, OR have LibxcRsError use string/integer fields instead of typed enum payloads. Strictly worse than option 1 unless we predict future kernel-* error use. | |
| Split: typed enum in core, FFI errno layer in compat | LibxcRsError in libxc-core; the int errno table + thread-local + xc_rs_last_error_* accessors in libxc-compat (already de facto the case in today's src/compat/errno.rs and macros.rs). Just confirms today's de facto split is preserved. | |

**User's choice:** libxc-core (Recommended).
**Notes:** Pre-question scout showed (a) zero `crates/kernel-*/` files contain `LibxcRsError`, and (b) `LibxcRsError` already imports `crate::model::{DerivativeOrder, Family, FunctionalId, Spin}` — both libxc-core types — so the micro-crate alternative would still need libxc-core. The de facto Phase 6 split (typed enum vs FFI errno) is preserved as a sub-decision (D-01a) regardless of where the typed enum lives.

---

## math/ disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Delete it (Recommended) | No callsites in src/. Kernel sub-crates already import libxc_kernel_math directly. Deleting src/math/ removes a dead re-export shim and one less thing to relocate during the split. Verify by trying cargo check post-deletion. | ✓ |
| Keep as tiny shim in libxc-core | Move src/math/mod.rs into crates/libxc-core/src/math.rs as a re-export shim. Useful only if downstream consumers depend on libxc_rs::math::* paths. Worth checking verify/ and benches before deleting. | |
| Fold into kernel-math directly | Add the 12 re-export lines to crates/kernel-math/src/lib.rs so libxc_kernel_math itself exposes the same surface. Same effect as delete-and-import-libxc_kernel_math-directly, but doesn't require touching downstream callers. | |

**User's choice:** Delete it (Recommended).
**Notes:** `grep -rn "use crate::math" src/` returned empty. Delete is safe pending the verification recipe in CONTEXT.md (pre/post `cargo check --workspace` log diff).

---

## Generated-files + xtask flow

### Sub-question A: xtask write paths

| Option | Description | Selected |
|--------|-------------|----------|
| Direct: xtask writes to crates/libxc-core/src/... (Recommended) | Simplest. Just update the hard-coded paths in xtask/src/main.rs and xtask/src/generate_metadata.rs from `src/meta/generated.rs` to `crates/libxc-core/src/meta/generated.rs` (etc.). xtask stays a thin script. Rebuild only triggers libxc-core (and downstream) on metadata changes — libxc-compat untouched. | ✓ |
| Shared output location + re-export | Write to e.g. xtask/output/ then `include!()` from libxc-core. Adds indirection without solving any real problem; rebuilds are the same. Strictly worse than direct. | |
| Carve out a libxc-codegen library crate | Move the generator logic from xtask/src/generate_metadata.rs and xtask/src/main.rs's generate_registry into a new crates/libxc-codegen/ library; xtask becomes a thin CLI wrapper. Buys testability of the generators. Pure scope creep relative to the modular split unless you've been wanting it. | |

**User's choice:** Direct (Recommended).
**Notes:** 7 hard-coded paths total to update (4 in xtask/src/main.rs, 3 in xtask/src/generate_metadata.rs).

### Sub-question B: future-cross guard

| Option | Description | Selected |
|--------|-------------|----------|
| Defer — cross that bridge if ids.rs ever becomes generated (Recommended) | src/compat/ids.rs is still a 2-line stub today. Phase 6 may populate it by hand, by codegen, or not at all. Phase 10's xtask refactor only handles today's outputs; if a future phase generates a compat-side file, that phase's plan handles the second target path (small change). Keeps Phase 10 narrowly scoped. | ✓ |
| Pre-emptively support multiple targets | Restructure xtask now so each generator declares its target crate via a small map (codegen_targets: { meta_generated: "libxc-core", ids: "libxc-compat" }), even though only libxc-core is used today. Buys forward-compat at the cost of an unnecessary abstraction. | |

**User's choice:** Defer (Recommended).
**Notes:** YAGNI. Add when needed.

### Sub-question C: xtask path-dep on libxc-core

| Option | Description | Selected |
|--------|-------------|----------|
| No — keep xtask as a string emitter (Recommended) | xtask's Cargo.toml comment is explicit: it's the metadata oracle, the registry it generates is its output, depending on the crate it generates from would be circular. Even if libxc-core compiles fast, xtask doesn't NEED the types — it emits text. Status quo wins. | ✓ |
| Yes — path-dep on libxc-core for type-checked emission | Construct typed FunctionalMeta values in xtask, then serialize them (via something like quote!) to source. Catches type drift at xtask-compile time. Adds a circular-feeling dep (xtask outputs land in libxc-core's src/, but xtask depends on libxc-core). Workable but more moving parts. | |

**User's choice:** No — string emitter (Recommended).
**Notes:** Preserves the xtask/Cargo.toml NOTE comment's stance — xtask is the metadata oracle, not a consumer of generated types.

---

## libxc-compat crate-type + cdylib

### Sub-question A: crate-type

| Option | Description | Selected |
|--------|-------------|----------|
| rlib + cdylib + staticlib (Recommended) | Single crate, multi-output. `cargo build -p libxc-compat` produces .so (dynamic), .a (static), and .rlib (Rust dep). All three from one [lib] section. Matches libxc's own dual lib output (libxc has both .so and .a). Adds ~zero build cost — same compilation, just emits more artifact files. | ✓ |
| rlib + cdylib only | Skip staticlib. Practical for users that link via .so. Matches what most Rust C-FFI crates do. Add staticlib later if anyone asks. | |
| rlib-only here; separate libxc-shared/libxc-static sibling crates produce cdylib/staticlib | Cleanest separation — libxc-compat is pure code, the shared/static crates are 5-line wrappers. Lets cdylib name be 'libxc.so' (drop-in) without renaming libxc-compat itself. Adds 1-2 trivial crates to the workspace. | |

**User's choice:** rlib + cdylib + staticlib (Recommended).

### Sub-question B: cdylib name

| Option | Description | Selected |
|--------|-------------|----------|
| libxc_rs.so (Rust default, Recommended) | Default Rust naming. Downstream links via -lxc_rs (or extern crate libxc_rs in Rust). Avoids confusion / collision with system libxc. CLAUDE.md describes drop-in at the SOURCE level (recompile against our header), not BINARY level (Phase 6 D-A4-1 changed void→int signatures, so binary drop-in is impossible by design). | ✓ |
| libxc.so (binary-drop-in) | Set [lib] name = "xc" so cargo emits libxc.so / libxc.a. Downstream uses -lxc same as system libxc. Risk: collides with libxc-master/ or system libxc on the same linker path. Phase 6 D-A4-1 already broke binary drop-in (signature changes), so the name match is cosmetic at best. | |
| Both (configurable via Cargo.toml) | Ship both libxc_rs.so AND libxc.so via two cdylib outputs. More complexity for marginal benefit. | |

**User's choice:** libxc_rs.so (Recommended).
**Notes:** Phase 6 D-A4-1 already broke binary drop-in via void→int signature changes; matching the libxc.so name would be cosmetic and risks linker collisions.

### Sub-question C: C header

| Option | Description | Selected |
|--------|-------------|----------|
| Hand-written, committed at crates/libxc-compat/include/xc_rs.h (Recommended) | Phase 6 context already lands here. Move the location during the split: crates/libxc-compat/include/xc_rs.h sits next to the cdylib it documents. Mirrors libxc-master/src/xc.h structure. Low churn on libxc version bumps because the surface is small (~100 decls). | ✓ |
| Generated via cbindgen at build time | build.rs invokes cbindgen on libxc-compat's source. Catches drift automatically but adds a build dep, generates header into target/, and produces noisy output for our small surface. Worth it only if Phase 7+ adds many more extern-C symbols. | |
| Defer to a later phase — ship Phase 10 without a header | If Phase 6 hasn't shipped a header yet, Phase 10 doesn't need one either. The .so is callable from Rust tests (verify/) without a header. | |

**User's choice:** Hand-written at crates/libxc-compat/include/xc_rs.h (Recommended).
**Notes:** Phase 6 context already named this file as a Phase 6 deliverable but with a planner-discretion location; Phase 10 confirms and pins the location to live alongside the cdylib it documents. Header generation is gated on Phase 6 actually producing one; if Phase 6 hasn't, Phase 10 does NOT block on writing one (D-09a).

---

## Continuation check

| Option | Description | Selected |
|--------|-------------|----------|
| I'm ready for context | Write CONTEXT.md now. The remaining items (root-facade re-export curation, plan atomicity, verify/ re-pointing, default-members policy, src/main.rs disposition) get marked as planner discretion. | ✓ |
| Explore more gray areas | Open up: Root-facade re-export strategy, Plan atomicity, verify/ re-pointing, default-members policy, src/main.rs Hello-world disposition. | |

**User's choice:** Ready for context.
**Notes:** Five additional candidate gray areas (root-facade re-export curation, plan atomicity, verify/ re-pointing, workspace default-members policy, src/main.rs Hello-world disposition) were identified during the analyze_phase step but the user opted to leave them as planner-discretion items rather than discuss further.

---

## Claude's Discretion

Items left to the planner / researcher (called out explicitly in CONTEXT.md `### Claude's Discretion`):

- Plan decomposition across the 3 roadmap-allocated plans (recommend: extract libxc-core first → libxc-eval → libxc-compat + facade reduction; each commit must keep `cargo check --workspace` green).
- Root-facade re-export curation strategy (recommend: line-for-line preserve today's `src/lib.rs:24-39`).
- verify/ + integration test re-pointing (recommend: stay through root facade).
- Workspace `[default-members]` post-split (recommend: add the three new crates; leave the kernel-* enumeration alone).
- `src/main.rs` "Hello, world!" disposition (recommend: delete; safe to leave).
- Per-directory move tactics (recommend: `git mv` for blame preservation).
- Per-crate exact `Cargo.toml` `[dependencies]` partitioning (planner reads root Cargo.toml).
- Disposition of empty `src/error/{ffi,internal,public}.rs` stubs (recommend: delete).
- Whether root `libxc_rs` adds any `[lib] crate-type` override (recommend: no — root stays rlib-only).

---

## Deferred Ideas

(Captured in CONTEXT.md `<deferred>` section in full; summary here.)

- Pre-emptive multi-target xtask abstraction (D-05 alternative).
- libxc-codegen library crate carve-out (D-06 alternative).
- xtask path-dep on libxc-core for type-checked emission (D-06 alternative).
- Phase directory rename (cosmetic).
- libxc.so binary-drop-in cdylib name (D-08 alternative).
- cbindgen-generated header (D-09 alternative).
- libxc-error micro-crate (D-01 alternative).
- src/main.rs deletion (planner discretion).
- Workspace [default-members] glob (Cargo doesn't support).
- libxc-core staticlib target (speculative).
- libxc-eval cdylib target (FFI is libxc-compat's concern).
- Test reorganization (tests move with code; no separate plan).
- Dropping kernel-* default-members enumeration (still need to compile by default).

### Reviewed Todos (not folded)

None — `audit-error-math-placement` was the only Phase-10-relevant todo and it was folded (resolved via D-01 + D-02).

---

# Restructure Update — Discussion Log (2026-05-21)

> **Audit trail only.** Re-discussion of Phase 10 against the post-Phase-11 281-crate reality. Decisions captured as D-10–D-14 in CONTEXT.md. Original D-01..D-09a unchanged.

**Date:** 2026-05-21
**Trigger:** Plans 10-00..10-03 + original CONTEXT assumed ~170 flat umbrella kernel crates; Phase 11 D-10a restructured to 281 per-functional crates, added `libxc-sys`, left workspace partially red mid-11.1.
**Areas discussed:** A kernel wiring, B deferred relocation, C libxc-sys/verify, D execution gate (all 4 selected via multiSelect).

---

## Gate: existing CONTEXT.md found

| Option | Selected |
|--------|----------|
| Update it | ✓ |
| View it first | |
| Skip — use as-is | |

**User's choice:** Update it.

## A. libxc-eval kernel wiring → default-members composition

| Option | Selected |
|--------|----------|
| Add core+eval+root, exclude compat (cdylib link = OOM risk at jobs=1) | ✓ |
| Add all 3 new crates | |
| Kernels-only (add none) | |

**User's choice:** Add core+eval+root, exclude libxc-compat. → **D-10, D-10a, D-10b.**

## B. libxc-core purity vs `is_deferred`

| Option | Selected |
|--------|----------|
| Relocate `deferred` to libxc-core (pure metadata) | ✓ |
| Split model across core+eval | |
| Relax SC2 (allow core→kernel-math) — NOT ADVISED (pulls CubeCL) | |

**User's choice:** Relocate to libxc-core. → **D-11.**
**Notes:** Verified no kernel crate consumes `deferred`; `deferred.rs` is hand-written (not xtask-generated); kernel-math depends on cubecl (so relaxing SC2 is non-viable, not merely undesirable).

## C. libxc-sys + verify wiring

| Option | Selected |
|--------|----------|
| Leave both untouched (libxc-sys outside layering; verify via root facade) | ✓ |
| Re-point verify to inner crates | |

**User's choice:** Leave untouched. → **D-12, D-12a.**

## D. Green-gate vs Phase 11.1

| Option | Selected |
|--------|----------|
| Hard-block execution on 11.1 green | ✓ |
| Re-baseline gate, start now | |

**User's choice:** Hard-block on 11.1 green. → **D-13.** (cubecl 0.9→0.10 dep refresh folded into **D-14** regardless.)

## Wrap-up

| Option | Selected |
|--------|----------|
| Write updated CONTEXT | ✓ |
| Revisit an area | |

**User's choice:** Write updated CONTEXT.
