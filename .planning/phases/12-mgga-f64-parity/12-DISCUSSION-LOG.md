# Phase 12: MGGA f64 Parity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-25
**Phase:** 12-mgga-f64-parity
**Areas discussed:** work_mgga regularization, Fix-location policy, Verify harness & loop, Completion gate & scope

---

## Area selection

| Option | Description | Selected |
|--------|-------------|----------|
| work_mgga regularization | σ-down vs τ-up clamp mismatch; correct shared driver vs per-functional | ✓ |
| Fix-location policy | Permitted fix surfaces under AP-3 | ✓ |
| Verify harness & loop | Canary vs family oracle; regression proof | ✓ |
| Completion gate & scope | all-6-or-HALT vs document; 2D + derivative scope | ✓ |

**User's choice:** All four areas.

---

## work_mgga input regularization

| Option | Description | Selected |
|--------|-------------|----------|
| Mirror libxc exactly | σ floor → τ floor → σ←min(σ,8ρτ) at the dispatch chokepoint, replacing the τ-up clamp | ✓ |
| Minimal swap first | Only τ-up→σ-down; add floors only if still failing | |
| Per-functional only | Leave shared driver untouched | |

**User's choice:** Mirror libxc exactly → D-01, D-02.
**Notes:** Verified during discussion that `work_mgga_inc.c:67` clamps σ down while `prepare.rs:43` clamps τ up — confirmed mismatch. Revisits the Phase-11 G-1 τ-clamp decision; b94 canary host must be updated and re-pass.

---

## Fix-location policy

| Option | Description | Selected |
|--------|-------------|----------|
| Root-cause-routed | Driver→dispatch/prepare.rs; per-functional bug→translator+regen; math bug→math/src | ✓ |
| Translator-only | All fixes through translator, incl. moving regularization into kernels | |
| Dispatch-only this phase | Only dispatch layer; defer kernel-translation bugs | |

**User's choice:** Root-cause-routed → D-03.
**Notes:** Respects AP-3 (no hand-edit generated kernels) while allowing hand-edits to non-generated dispatch code.

---

## Regen discipline

| Option | Description | Selected |
|--------|-------------|----------|
| Selective loop, full-tree close | Single-functional regen while iterating; full-tree idempotent regen as close | ✓ |
| Full-tree every iteration | Always regen all families | |
| Single-functional only | Never full-tree | |

**User's choice:** Selective loop, full-tree close → D-04 (D-LOCK-D idempotency at close).

---

## Regression guard (SC #2)

| Option | Description | Selected |
|--------|-------------|----------|
| Full family oracle re-run | oracle-mgga (all routed, incl. 6 passing) + cheap oracle-lda/gga confirm | ✓ |
| MGGA-only re-run | Only oracle-mgga | |
| Canary + spot-check | Canaries + small spot-check set | |

**User's choice:** Full family oracle re-run → D-06.

---

## Verify loop harness & gate

| Option | Description | Selected |
|--------|-------------|----------|
| Canary loop + oracle gate | Per-functional verify-canary loop; family mgga_oracle authoritative gate; 6 canaries become permanent regression tests | ✓ |
| Family oracle only | Iterate directly on oracle-mgga | |
| Canary only | Canaries as both loop and gate | |

**User's choice:** Canary loop + oracle gate → D-05.
**Notes:** Avoids the Phase-11.1 b94 hollow-gate trap.

---

## Completion gate & iteration bound

| Option | Description | Selected |
|--------|-------------|----------|
| All-6 target, capped + escalate | Target all 6 at 1e-12; N=3 cycles/functional then HALT-to-discuss | ✓ |
| Hard all-6, uncapped | No iteration cap | |
| Document residuals | Fix tractable, document/route stubborn residuals | |

**User's choice:** All-6 target, capped + escalate → D-07 (mirrors Phase-11.1 D-13).

---

## mgga_x_2d_js17 (2D functional)

| Option | Description | Selected |
|--------|-------------|----------|
| Attempt, defer if 2D-structural | Try under the shared fix; de-route + document if inherently 2D | ✓ |
| Defer now | De-route immediately as out-of-scope 2D | |
| Fix fully | Must hit 1e-12 regardless | |

**User's choice:** Attempt, defer if 2D-structural → D-13.
**Notes:** PROJECT.md lists 1D/2D as out-of-scope; this is the escape hatch if the residual is dimensionality-bound.

---

## Derivative/spin scope

| Option | Description | Selected |
|--------|-------------|----------|
| exc-unpol only | exc to 1e-12; vxc-unpol no-regression; pol/Fxc/Kxc/Lxc deferred | ✓ |
| exc + vxc unpol | Also drive vxc to 1e-10 as explicit target | |
| Include polarized | Un-defer polarized MGGA | |

**User's choice:** exc-unpol only → D-08.

---

## Claude's Discretion

- D-09 — Debug entry order (regularization first, then mgga_x_th).
- D-10 — Sub-Fermi-hole test-point selection for canaries.
- D-11 — ext_params reproduction mechanism in canaries.
- D-12 — Whether to match libxc's isfinite fallback.

## Deferred Ideas

- Polarized MGGA dispatch; Fxc/Kxc/Lxc tiers; genuine 2D-dimensionality path; broader MGGA parity sweeps; f32/generic kernels.
