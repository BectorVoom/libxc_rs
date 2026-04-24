# Phase 5: Functional Lifecycle and Hybrid Properties - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-24
**Phase:** 05-functional-lifecycle-and-hybrid-properties
**Areas discussed:** Registry metadata population; Ext_params plumbing to kernels; Functional struct shape and evaluation API; Hybrid + auxiliary recursion

---

## Gray Area Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Registry metadata population | Where ext_params, hybrid_terms, nlc_params, auxiliaries come from. Blocks FUNC-02 and all HYB-*. | ✓ |
| Ext_params plumbing to kernels | How user-set ext_params reach #[cube] kernel. Currently dispatch_lda takes only LdaFunctionalParams { alpha }. | ✓ |
| Functional struct shape and evaluation API | Module location, FunctionalParams trait object vs enum, Send+Sync, Functional::evaluate_* composition with dispatch_*. | ✓ |
| Hybrid + auxiliary recursion | HybridType + CAM + NLC + aux-funcs representation; eager vs lazy aux construction; _omega propagation; cycle detection. | ✓ |

**User's choice:** All four (multiSelect).

---

## Registry Metadata Population

### Q1 — Source of truth for hybrid_terms/nlc_params/auxiliaries/ext_params specs

| Option | Description | Selected |
|--------|-------------|----------|
| Extend xtask: parse _init() C source | Regex/parse each xc_*_*.c file for xc_hyb_init_* and xc_func_set_ext_params_name calls. Pure static Rust; no FFI at runtime; brittle. | |
| xtask links libxc, snapshots at generation time | xtask links via bindgen, calls xc_func_init per ID, reads xc_func_type fields, emits static Rust. Authoritative; committed output. | ✓ |
| Hand-curated from libxc-master docs + per-functional .c inspection | 270 files eyeballed by human. Slow, error-prone. | |
| Hybrid: FFI snapshot for hybrid/aux/nlc; hand-curate ext_params specs | Split hard problem — FFI for structured fields, hand for freeform. | |

**User's choice:** xtask links libxc, snapshots at generation time.
**Notes:** Authoritative; runtime stays FFI-free; build-time dependency on libxc confined to xtask.

### Q2 — Snapshot scope

| Option | Description | Selected |
|--------|-------------|----------|
| Only Phase 5 load-bearing fields | ext_params + hybrid_terms + nlc_params + auxiliaries + flags. Leave references empty. | |
| All fields including references/DOI/bibtex | Full populate; closes Phase 1 D-05 carry-over. | ✓ |
| Load-bearing now + flags + references later via follow-up todo | Partial. | |

**User's choice:** All fields including references/DOI/bibtex.

### Q3 — xtask target location

| Option | Description | Selected |
|--------|-------------|----------|
| New xtask target `generate-metadata`; reuses verify/'s libxc build | Factor libxc-sys into workspace crate used by both xtask and verify/. One cmake. | ✓ |
| New xtask target; xtask has own libxc FFI copy | Two cmake builds; simpler dependency graph. | |
| Fold into existing xtask `generate-registry` | Extend existing subcommand. Most cohesive. | |

**User's choice:** New xtask target `generate-metadata`; reuses verify/'s libxc build via factored libxc-sys.

### Q4 — Validation gate

| Option | Description | Selected |
|--------|-------------|----------|
| verify/ test round-trip every FunctionalMeta against live libxc FFI | metadata_oracle.rs iterates all 649 IDs. Highest confidence. | ✓ |
| Snapshot + checksum; xtask regen fails if libxc version changes | Version-tag file; no runtime verification. | |
| Both | Verify round-trip AND version checksum. | |

**User's choice:** verify/ test round-trip only.

### Q5 — Regen policy

| Option | Description | Selected |
|--------|-------------|----------|
| Manual regen, committed Rust output | Matches Phase 1 D-04 pattern. | ✓ |
| Auto-regen via build.rs, cached | Requires libxc toolchain for every build. | |

**User's choice:** Manual regen, committed Rust output.

---

## Ext_params Plumbing to Kernels

### Q1 — Functional runtime ext_params storage

| Option | Description | Selected |
|--------|-------------|----------|
| Option<Box<[f64]>> — None for zero-param functionals | Heap alloc only when needed; preserves EVAL-04 for common case. | ✓ |
| Box<[f64]> always (even zero-length) | Design doc literal; extra alloc. | |
| Cow<'static, [f64]> starting borrowed from meta | Zero alloc until mutation; lifetime complexity. | |

**User's choice:** Option<Box<[f64]>>.

### Q2 — Dispatch signature

| Option | Description | Selected |
|--------|-------------|----------|
| dispatch_* takes `&[f64]` slice | Most flexible; relies on stable ext_params order. | |
| dispatch_* takes `&FunctionalParams` trait object | Matches design doc §6.8; per-functional concrete type. | ✓ |
| Per-family enum LdaParams::LdaX { alpha: f64 } | Exhaustive match; huge enum (229 variants). | |
| Unchanged; Functional wraps it | Incremental, uses libxc defaults for un-plumbed. | |

**User's choice:** `&dyn FunctionalParams` trait object.

### Q3 — Derived parameter computation location

| Option | Description | Selected |
|--------|-------------|----------|
| FunctionalParams trait object stored in Functional | Box<dyn FunctionalParams>; per-functional impl; design-doc literal. | ✓ |
| Typed enum per family | Static dispatch; large enums. | |
| Derivation inline in kernel, no cached state | Recompute every call; breaks PERF-01 for hybrid hot paths. | |
| Derive lazily + cache OnceCell | Mix of trait + no-dyn. | |

**User's choice:** FunctionalParams trait object.

### Q4 — Wiring rollout scope

| Option | Description | Selected |
|--------|-------------|----------|
| Wire all 229 compiled functionals in Phase 5 | Full coverage; FUNC-02 works uniformly; big plan. | ✓ |
| Only ext_params-sensitive functionals | Smaller plan; creates silent-default trap. | |
| LDA + GGA + canonical hybrids; MGGA non-hybrids deferred | Partial. | |

**User's choice:** Wire all 229 compiled functionals in Phase 5.

---

## Functional Struct Shape and Evaluation API

### Q1 — Module location

| Option | Description | Selected |
|--------|-------------|----------|
| New `src/func/` module (design doc literal) | Matches §9.11 naming. | |
| New `src/functional/` module | Idiomatic; avoids abbreviation. | ✓ |
| Put in `src/model/` | Domain types together; mod.rs grows. | |

**User's choice:** `src/functional/`.

### Q2 — Evaluation flow

| Option | Description | Selected |
|--------|-------------|----------|
| Functional methods call free dispatch_* internally | Both public; verify/ tests unchanged. | ✓ |
| Move dispatch into Functional methods; free functions removed | Forces single canonical path; breaks verify tests. | |
| Keep both: free dispatch_* public, Functional wrapper | Two-tier API like libxc. | |

**User's choice:** Functional methods delegate to free dispatch_*.

### Q3 — Mixed GGA/MGGA paths

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 5 builds evaluate_mixed_gga + evaluate_mixed_mgga | Full mirror of LDA pattern. | ✓ |
| Phase 5 makes detect-and-dispatch only; mixed stays LDA-only | Defers B3LYP; fails HYB-* evaluation. | |
| Phase 5 builds mixed machinery + routing; less-common aux in Phase 6 | 90% coverage. | |

**User's choice:** Full GGA+MGGA mixed paths materialized.

### Q4 — Thread safety

| Option | Description | Selected |
|--------|-------------|----------|
| Require FunctionalParams: Send + Sync | Uniform guarantee. | ✓ |
| Don't require; Functional is Send but not Sync | Users clone for per-thread. | |
| Defer decision; let auto-traits decide | Phase 7 handles. | |

**User's choice:** Require Send + Sync on FunctionalParams.

---

## Hybrid + Auxiliary Recursion

### Q1 — HybridType classification source

| Option | Description | Selected |
|--------|-------------|----------|
| Pre-compute at xtask time, store HybridType in meta | Direct field read; redundant with hybrid_terms. | |
| Store only hybrid_terms; port xc_hyb_type logic to Rust | Single source of truth. | |
| Both — snapshot AND keep logic in Rust for validation | Defense in depth. | ✓ |

**User's choice:** Both — snapshot + Rust port + verify/ test confirms agreement.

### Q2 — Auxiliary construction timing

| Option | Description | Selected |
|--------|-------------|----------|
| Eager: recursively Functional::new each aux at parent construction | Design doc §10.1 flow. | ✓ |
| Lazy: build on first evaluation | Saves upfront cost; complicates state. | |
| Eager with Arc sharing for shared aux | Over-engineered. | |

**User's choice:** Eager recursive construction.

### Q3 — Aux ext_params propagation mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| xtask snapshots propagation map alongside hybrid_terms | No per-functional Rust code; table-driven. | ✓ |
| Per-functional init callbacks, one function per hybrid | 270 hand-ported Rust init fns. | |
| Propagate by convention: _-prefix params copy to matching aux names | Convention-driven; may miss cases. | |

**User's choice:** xtask-generated propagation map.

### Q4 — Aux depth / cycle detection

| Option | Description | Selected |
|--------|-------------|----------|
| Finite depth by construction; xtask validates max ≤ 2 | Static-data validation; no runtime check. | ✓ |
| Assume finite, crash loudly on recursion >8 | Trust libxc. | |
| Cycle detection via visited-set in Functional::new | Belt-and-braces. | |

**User's choice:** xtask validates max aux depth ≤ 2 at snapshot time.

---

## Wrap-up Prompt

| Option | Description | Selected |
|--------|-------------|----------|
| Explore plan decomposition (3 plans) | Discuss how to split into 3 roadmap plans. | |
| Explore ext_params API + error variants | FUNC-02 name/index API; error variant design. | |
| I'm ready for context | Write CONTEXT.md. | ✓ |

**User's choice:** Ready for context.

---

## Claude's Discretion (noted in CONTEXT.md)

- Plan decomposition across the 3 roadmap plans (suggested split in CONTEXT.md §decisions).
- FunctionalParams trait exact shape (by-name vs by-index getters, error semantics).
- Per-functional FunctionalParams impl generation: hand-written vs macro vs xtask-emitted.
- Internal file layout of `src/functional/`.
- New LibxcRsError variant names/messages.
- GgaScratch/MggaScratch exact split_at_mut offsets.
- `pub` vs `pub(crate)` for free dispatch_* functions.

## Deferred Ideas (noted in CONTEXT.md)

- Enabling deferred LDA (4) + MGGA (6) functionals (unchanged from Phase 4).
- FunctionalBuilder + BatchEvaluator + ergonomic evaluate() auto-dispatch (Phase 6).
- extern "C" compat layer (Phase 6).
- GPU backends (Phase 7).
- Performance benchmarks (Phase 7).
- Runtime `references()` getter API polish (Phase 10).
- Non-Copy propagation transforms (if any exist in libxc).
