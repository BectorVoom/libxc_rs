# Phase 11: Splitter v2 — Discussion Log (2026-05-18 Reconsideration)

> **Audit trail only.** Decisions are captured in CONTEXT.md. This log preserves the reconsideration of D-02 and the final decision to lock Option A.

**Date:** 2026-05-18 (second session)
**Phase:** 11-splitter-v2-unified-5k-cap
**Session Type:** Discuss-phase reconsideration; D-02 decision revisited

---

## D-02 Reconsideration: Option C → Option A

| Decision Point | First Decision | Reconsideration | Final Decision |
|---|---|---|---|
| **D-02 ABI for `<F: Float>` chunks** | Option C (cast-at-call-site in translator) | User requested Option A reconsideration | **Option A (generic helpers via Python tooling)** |
| **Rationale for reconsideration** | N/A | "I think that modifying math scripts by python tools." — User recognizes Phase 2 script errors are fixable via improved Python tooling, not architectural blockers. | Option A is architecturally cleaner (proper generics, not workarounds). Python tooling improvements are the right investment. |
| **Timeline constraint** | Not discussed initially | User stated: "I do not concern timeline." | Quality over speed. Spike has **open-ended time-box** — no 1–2 day constraint. |
| **Spike scope** | Validate Option C via mgga_c_b94 | N/A | D-14 spike: (1) analyze 11 problematic Phase 2 files, (2) improve Python refactoring tooling, (3) refactor all 38 helpers to generic, (4) validate on mgga_c_b94. |

---

## Updated D-14 Spike Scope (Option A)

**Task:** Improve Phase 2 `_refactor_helper_*` Python scripts and refactor all 38 helpers to generic `<F: Float>`.

**Steps:**
1. **Analyze the 11 problematic files** — Identify the specific regex/pattern failures in the Phase 2 refactoring scripts
2. **Improve the Python tooling** — Fix the `_refactor_helper_*` scripts to handle the identified patterns correctly
3. **Refactor all 38 helpers** in `crates/kernels/math/src/`:
   - `piecewise.rs` (6 fns)
   - `powers.rs` (20 fns)
   - `erf.rs` (6 fns)
   - `lambert_w.rs` (3 fns)
   - `bspline.rs` (7 fns)
   - `br89.rs` (3 fns)
   - `bessel.rs` (14 fns)
   - `dft_quantities.rs` (8 fns)
   - `spin.rs` (10 fns)
   - `integrate.rs` (11 fns)
   - `polynomials.rs` (4 fns)
   - `special.rs` (6 fns)
   - `expint_e1.rs` (8 fns)
   - `mbrxc.rs` (3 fns)
4. **Validate on mgga_c_b94:**
   - `cargo build -p libxc-kernel-mgga_c_b94` PASS
   - Parity vs libxc oracle: 1e-12 relative error on energy + routed derivatives
   - Idempotency: re-run translator, no git diff

**Validation gates (all three must PASS):**
- Compile gate: `cargo build -p libxc-kernel-mgga_c_b94` under CubeCL 0.10, `jobs = 1`
- Parity gate: 1e-12 relative error on energy AND all routed derivatives (with one-shot `is_deferred(id)` bypass for mgga_c_b94)
- Idempotency gate: re-run translator from Maple, `git diff` is empty

**Time-box:** Open-ended. Quality is the priority.

---

## Why Option A Over Option C

| Dimension | Option A (Generic Helpers) | Option C (Cast-at-call-site) |
|-----------|--------------------------|------------------------------|
| **Architecture** | Proper generic abstractions; helpers are reusable across `<F: Float>` and f64 contexts | Workaround; cast boilerplate at every call site (~581K times) |
| **Code cleanliness** | Helpers are improved/generalized; future-proof | Generated code carries cast boilerplate; harder to reason about |
| **Root cause** | Fixes the Phase 2 script limitations via Python tooling | Circumvents the problem; leaves scripts broken for future use |
| **Long-term investment** | Strengthens the translation pipeline; better tools for future phases | One-off workaround specific to this phase |
| **Timeline** | Unconstrained; time to do it right | Fast (1–2 days); but with technical debt |
| **User preference** | "Modifying math scripts by python tools" — quality, not speed | Fast unblock, but not the chosen path |

---

## Confirmed: Next Steps

The replan structure **remains unchanged** in shape, but now targets **Option A** with commitment to quality:

1. **11-04 (retroactive SUMMARY only)** — commit `39eb75f93` (D-05 structural fix). Pause documented.
2. **11-05 (D-14 spike for Option A)** — Analyze Phase 2 scripts, improve tooling, refactor 38 helpers, validate on mgga_c_b94. Open-ended time-box.
3. **11-06 (D-16 translator update)** — Translator emit stays minimal (helpers now generic, no cast wrappers). Fix `from_raw_parts` API drift in `crates/kernels/math/tests/`.
4. **11-07 (D-15 entry-gate on full regen)** — Regen 266 subcrates. Compile-first: mgga_c_b94 gate passes all three legs.
5. **11-08 (per-`-p` sweep + audits + close)** — Incremental per-subcrate verify. Close phase.

**No timeline constraint.** The spike proceeds at whatever pace is needed to improve the Python tooling and get it right.

---

*Log date: 2026-05-18 (second session)*
*Session: Phase 11 discuss-phase reconsideration (D-02 reconsidered; Option A locked)*
