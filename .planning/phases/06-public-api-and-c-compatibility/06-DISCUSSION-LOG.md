# Phase 6: Discussion Log

**Date:** 2026-05-06
**Mode:** discuss (default → batched on user request)

This log captures the question/answer trail for human reference (audits, retrospectives). Downstream agents (researcher, planner, executor) consume only `06-CONTEXT.md`.

---

## Top-Level Selection

**Q:** Which areas of Phase 6 (Public API & C Compatibility) do you want to discuss?
**Options:**
1. xc_func_type C struct layout & lifecycle
2. BatchEvaluator scope & ownership
3. Ergonomic evaluate() auto-dispatch
4. Compat-layer error/panic boundary

**Selected:** All four (multi-select).

---

## Area 1 — xc_func_type C struct & lifecycle

Originally asked one-at-a-time, then re-presented as a batch on user request ("batch function"). Final answers:

**Q1.** How should `xc_func_type` be exposed across the FFI boundary?
**A:** Opaque pointer (Rust-owned) — recommended.
*Captured as D-A1-1.*

**Q2.** How should `xc_func_alloc` / `xc_func_init` / `xc_func_end` / `xc_func_free` map onto a Rust-owned Functional?
**A:** Two-phase: alloc returns a typed slot — recommended. Mirrors libxc; init can be re-run after end.
*Captured as D-A1-2.*

**Q3.** What thread-safety contract does `xc_func_type*` carry across the FFI?
**A:** Match libxc: single-threaded per handle — recommended. Caller's responsibility; matches libxc's de-facto contract.
*Captured as D-A1-3.*

**Q4.** How should `xc_func_info_type*` be exposed?
**A:** Opaque, accessor-only — recommended. Symmetric with xc_func_type.
*Captured as D-A1-4.*

---

## Area 2 — BatchEvaluator scope & ownership

Asked as a 4-question batch.

**Q1.** What does `BatchEvaluator` own?
**Initial answer:** Workspace only — Functional + buffers per call.
**User requested clarification of options 1 and 2.** After detailed walkthrough (workspace-only vs Functional+workspace), confirmed:
**A:** Workspace only.
*Captured as D-A2-1.*

**Q2.** How does BatchEvaluator handle changing grid sizes (np)?
**A:** Fixed at construction; error on overflow. Preserves PERF-05 zero-alloc-in-hot-path.
*Captured as D-A2-2.*

**Q3.** Should one BatchEvaluator handle multiple functionals on the same grid?
**A:** Workspace shared across functionals. The motivation for the workspace-only ownership choice.
*Captured as D-A2-3.*

**Q4.** How does the ergonomic `evaluate()` auto-dispatch (API-03) wire in?
**A:** On BatchEvaluator: `be.evaluate(&functional, &input, order, &mut out)`.
*Captured as D-A2-4.*

---

## Area 3 — Ergonomic evaluate() auto-dispatch

**Q1.** What should the `EvaluateInput` sealed trait look like?
**User requested clarification of options 1 and 2.** After detailed walkthrough (vocabulary trait + match-in-evaluator vs dispatch trait + trivial-evaluator), confirmed:
**A:** Dispatch trait — option 2. Each impl owns family-specific call; BatchEvaluator stays trivial; zero `unsafe`.
*Captured as D-A3-1.*

**Q2.** Should `BatchEvaluator::evaluate` take `&Functional` or `&mut Functional`?
**A:** &Functional — read-only — recommended. Allows Arc-shared functionals; matches Phase 5's `&self` evaluate methods.
*Captured as D-A3-2.*

---

## Area 4 — Compat-layer error/panic boundary

Asked as a 4-question batch.

**Q1.** What error convention should extern "C" functions use?
**A:** int return codes everywhere + thread-local errno. Signature-breaking departure from strict drop-in (libxc void functions become int) — accepted trade for type-checked error reporting.
*Captured as D-A4-1.*

**Q2.** How do extern "C" functions handle Rust panics crossing the FFI boundary?
**A:** catch_unwind at every entry point — recommended.
*Captured as D-A4-2.*

**Q3.** How should NULL output pointers in C evaluate functions translate?
**A:** NULL → None at the FFI boundary — recommended.
*Captured as D-A4-3.*

**Q4.** What should evaluate on an alloc'd-but-not-init'd handle do?
**A:** Return error code (or set errno) — recommended.
*Captured as D-A4-4.*

---

## Notes / Deferred Ideas Captured During Discussion

- Multi-functional batch with summed output mentioned and explicitly deferred (scope creep beyond API-02).
- GPU backend selection deferred to Phase 7 (GPU-07).
- `extern "C-unwind"` deferred until a concrete C++ interop need arises.
- Resizable BatchEvaluator workspace deferred until Phase 7 benchmarks reveal a real DX problem.
- Documentation polish (DOC-01..03) deferred to a dedicated documentation phase.

## Claude's Discretion (planner owns)

- Plan decomposition across the 3 plans (suggested split in CONTEXT.md).
- FunctionalBuilder chain ergonomics (owned-self vs &mut self).
- Hand-written vs codegen'd extern "C" function bodies.
- C header generation strategy and committed location.
- Filenames inside `src/compat/`.
- Integration test mechanism (Rust-only FFI exercise vs cc-built C harness).
- Exact `LibxcRsError` discriminant → int errno mapping table.
- `removed.rs` error surfacing strategy.
