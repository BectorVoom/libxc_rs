# Phase 7: GPU Backends and Performance — Context

**Gathered:** 2026-04-24
**Updated:** 2026-05-27 — **RE-SCOPED to f64-only.** The 2026-05-07 multi-precision substrate (old D-01–D-14: generic `<F: Float>` kernels, `Precision` enum, `LIBXC_RS_PRECISION`, two-track f32/f16 verification) is **SUPERSEDED** — f32/f16 was re-deferred milestone-scale on 2026-05-23 because the kernels are f64-concrete by design (2491 files `&Array<f64>`, 0 generic). Backend-selection decisions (old D-12/D-13/D-14) are carried forward and adapted to f64-only + the Phase-10 layered workspace. This phase returns to the original ROADMAP intent: f64-only GPU backends + a performance/benchmark layer.
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 7 delivers, at **f64 precision only**, against today's architecture (libxc-core ← libxc-eval with 281–305 per-functional kernel crates, cubecl **0.10**):

1. **Feature-gated GPU backends** layered onto the existing CubeCL CPU baseline. **HIP/ROCm is the single end-to-end-verified backend** (on the local `gfx1152` AMD iGPU). CUDA and WGPU are feature-gated stubs (CUDA: typed-error stub, no hardware locally; WGPU: wired enough to probe f64 capability).
2. **Runtime backend selection** — `Backend` enum, `LIBXC_RS_BACKEND` env var, `FunctionalBuilder::backend()` override; the cubecl `Runtime` threaded from today's `CpuRuntime`-concrete launch layer to backend-selectable.
3. **GPU-resident buffer management** (`GpuBuffer<R>`) as an internal optimization minimizing host↔device transfers.
4. **A criterion benchmark suite** (the 7 `benches/*.rs` placeholders become real) covering CPU performance vs libxc C, cold-start init, and GPU transfer/resident costs, with regression detection.
5. **GPU-vs-CPU correctness** at 1e-14 on a representative per-family subset (the existing oracle witnesses), via HIP.

**This phase does NOT add:**
- f32/f16 or any non-f64 precision (re-deferred milestone-scale — kernels are f64-concrete).
- Per-precision C FFI; the C ABI stays 1:1 with libxc at f64 (old D-04b preserved).
- A `>5x` GPU throughput claim (old SC#2/PERF-02 dropped — no datacenter-class f64 GPU available).
- CUDA or WGPU *runtime correctness* verification (no NVIDIA hardware; WGPU is f64-probe-only this phase).
- Full-roster GPU correctness verification (the all-281 GPU build OOMs the box; deferred to equipped hardware).
- New functional families or oracle fixtures.

**Preserved from Phases 1–6, 10–12:**
- f64 oracle tolerances (exc ≤1e-12, vxc ≤1e-10, fxc ≤1e-8, kxc ≤1e-6, lxc ≤1e-4).
- cubecl 0.10 as the compute substrate (migrated in 11-14).
- Per-family dispatch (`LdaFunctional`/`GgaFunctional`/`MggaFunctional`) and the layered libxc-core ← libxc-eval ← libxc-compat split (Phase 10).
- Process discipline: jobs=1, **no umbrella/all-281 builds**, **never edit `.cargo/config.toml`**, per-`-p` compile as the entry gate, USER runs heavy compiles.

</domain>

<decisions>
## Implementation Decisions

### Scope & GPU Verification Strategy (Area 1)

- **D-01:** **One backend, fully verified.** HIP/ROCm on the local `gfx1152` iGPU is the single backend verified end-to-end (correctness + benches). CUDA and WGPU are feature-gated stubs (see D-09). Rationale: no NVIDIA hardware (CUDA untestable locally); a single real GPU path is the fastest route to a green, honest phase.
- **D-02:** **Correctness scale = representative subset per family.** GPU(HIP)-vs-CPU agreement at **1e-14** is verified on the existing oracle witnesses (~2 LDA + 2 GGA + 2 MGGA), NOT the full roster — the all-281 GPU build OOMs the 30 GB box. Full-roster GPU verification is documented as needing equipped hardware + a chunked sweep.
- **D-03:** **The `>5x`-on-ROCM throughput target is DROPPED from Phase 7 scope** (old SC#2 / PERF-02). An f64-rate-limited iGPU cannot beat the Ryzen CPU by 5x; real-GPU throughput becomes a separate future task. The benchmark suite still *measures* GPU-vs-CPU throughput (D-12), but there is no pass/fail `>5x` gate.
- **D-04:** **HIP feasibility spike precedes the threading refactor.** Before planning the 133-file-scale change, a cheap single-kernel spike must confirm (a) cubecl-hip 0.10 launches an f64 kernel on `gfx1152` at all (likely needs `HSA_OVERRIDE_GFX_VERSION` — gfx115x ROCm support is unofficial), and (b) generic `launch_unchecked::<R>` monomorphizes cleanly under cubecl 0.10. If HIP cannot run f64 on this iGPU, the phase re-evaluates the verified-backend choice (WGPU fallback) before committing.

### Runtime Threading (Area 2)

- **D-05:** **Generic over `R: Runtime`, with the `Backend`→concrete-`Runtime` match localized to the launch chokepoint.** The generated dispatch + the buffer helpers in `launch.rs` become generic over `R: Runtime` (today they are `CpuRuntime`-concrete); a single wrapper at the launch boundary (`launch.rs` / the dispatch macro) matches the selected `Backend` to its concrete runtime (`CpuRuntime` / `HipRuntime`) and calls `launch_unchecked::<R>`. Per-functional kernel files stay backend-agnostic. Backend `#[cfg]` gates are confined to the launch layer.
- **D-06:** **Research maps the generated call-site shape FIRST, then chooses regen-vs-localized.** The `CpuRuntime` literal is emitted by `translate_lda.py` + `generate_{gga,mgga}_dispatch.py` and appears in 133 eval files. The researcher must determine whether per-functional files call launch directly or route through a stable wrapper, then pick the path that **minimizes build/OOM risk**: (a) change generators + full regen (AP-3-aligned, durable, but ~133-file recompile under jobs=1/no-umbrella/USER-run), or (b) confine the change to hand-written `launch.rs`/`dispatch.rs` if the call-sites already route through a stable-signature wrapper (cheapest). Decision deferred to research findings.
- **D-07:** **Client resolved once per `Functional` at `build()`/`new()`.** `FunctionalBuilder::backend()` / `LIBXC_RS_BACKEND` resolves the `ComputeClient<R>` once; it is stored on the `Functional` and reused across `evaluate()` calls. Aligns with resident buffers (D-15) and the <100 ms cold-start budget (PERF-03). No per-call client creation.

### Backend Selection Surface (Area 3)

- **D-08:** **`cpu`-as-default-feature shape (old D-12).** `cpu` becomes a Cargo feature (today cubecl/cpu is unconditional); `cuda`/`hip`/`wgpu` are additive opt-in features in libxc-eval forwarding `cubecl/<backend>`, re-forwarded by the root facade (mirrors the existing `oracle-*` pattern). The `Backend` enum is `#[cfg]`-gated: `Cpu` under `cpu`, `Cuda`/`Hip`/`Wgpu` under their features.
  - **Constraint to resolve in planning:** the new `default` must include `cpu` and reconcile with the existing `default = ["oracle-lda","oracle-gga","oracle-mgga"]` (likely `default = ["cpu","oracle-lda","oracle-gga","oracle-mgga"]`). An **"at least one backend"** invariant is required — `launch.rs` needs a concrete runtime to compile, so `--no-default-features` with no backend feature must fail with a clear compile error or a documented requirement.
- **D-09:** **WGPU wired enough to probe f64; CUDA is a light typed-error stub.**
  - **WGPU:** pulls cubecl-wgpu, instantiates a client, and probes `SHADER_FLOAT64`. Satisfies **GPU-04 / SC#3** (typed error if f64 absent, no silent fallback) even though kernel correctness is not verified this phase.
  - **CUDA:** a `#[cfg(feature = "cuda")]` `Backend::Cuda` variant that returns a typed error at `build()` — **no cubecl-cuda dependency** (the CUDA toolkit can't build on this box). Documented as requiring the toolkit + NVIDIA hardware.
- **D-10:** **`LIBXC_RS_BACKEND` env var + builder override, strict parsing (carry old D-14).** Lowercase exact match on `cpu`|`cuda`|`hip`|`wgpu`; unset → `Backend::Cpu`; invalid name → `Error::InvalidBackendEnvVar(String)`; valid name whose feature isn't compiled in → `Error::BackendNotCompiledIn { backend_name }` (string-keyed because the `#[cfg]`-gated variant doesn't exist). `FromStr`/`Display` mirror the grammar. `FunctionalBuilder::backend(Backend)` always wins over the env var. **f64 capability** failures (WGPU without `SHADER_FLOAT64`) surface as a typed error at `build()` — **no precision axis** (f64-only), so the old `BackendPrecisionUnsupported` simplifies to a single f64-unsupported variant (exact name = Claude's discretion).
- **D-11:** **Placement (research confirms): `Backend` type + `FromStr` + error variants in libxc-core; `ComputeClient<R>` construction + `#[cfg]` backend deps in libxc-eval.** The public `Backend` type lives in the core types layer everything depends on; cubecl-touching client construction stays in libxc-eval (which owns the cubecl dep). The builder method lives wherever `FunctionalBuilder` currently lives. Researcher verifies exact module homes against the Phase-10 split before locking.

### Benchmarks & Performance (Area 4)

- **D-12:** **Benchmark a representative few per family** (e.g. `lda_x` + a GGA like `gga_x_pbe` + one MGGA), reused across PERF-01/03/04. Keeps the bench crate within jobs=1/no-umbrella build limits while giving real per-family signal. (criterion is **not yet a dependency**; the 7 `benches/*.rs` are `println!` placeholders — both are net-new work under PERF-04.)
- **D-13:** **Reuse verify/'s bindgen libxc FFI inside a criterion bench** to compute the PERF-01 ratio (Rust CPU within 1.5x of libxc C) directly and reproducibly. Cost accepted: the bench pulls the libxc C build + cubecl-cpu.
- **D-14:** **PERF-05 (zero heap allocation in the non-mixed hot path) is gated by an allocation-counting test** — a counting global allocator (or dhat) wraps a warmed `evaluate()` and asserts zero allocations on the non-mixed path. Automated, regression-catching.
- **D-15:** **GPU-05 resident buffers are an INTERNAL optimization.** `GpuBuffer<R>` keeps data device-resident across `evaluate()` calls; measured by `resident.rs` / `transfer.rs`. **No new public API surface** in v1 — the C-FFI-stable facade is unchanged.

### Prerequisite Doc Amendments (Plan 07-01)

- **D-16:** **A first plan (07-01) amends the planning/spec docs before code work** (mirrors the old D-11 pattern, new f64-only content). Specific amendments:
  - **ROADMAP.md §Phase 7 Success Criteria:** SC#1 ("ROCM … all tested functionals") → "HIP/ROCm backend matches CPU to 1e-14 on a representative per-family subset; full-roster GPU verification deferred to equipped hardware." **SC#2 (">5x on ROCM") → removed** (recorded under Deferred). SC#3 (WGPU typed f64 error) → reword "no silent f32 fallback" to "no silent fallback; WGPU probes `SHADER_FLOAT64`." SC#4 (CPU within 1.5x libxc) and SC#5 (criterion suite) → keep.
  - **REQUIREMENTS.md:** **PERF-02** (GPU batch >5x) → move to an out-of-scope/deferred note ("requires datacenter-class f64 GPU"). **GPU-02** (CUDA) → annotate "typed-error stub, no local hardware." VERIFY-08 → scope to the per-family subset. GPU-06 unchanged (f64-only typed error). Add the `LIBXC_RS_BACKEND` / `Backend` / `FunctionalBuilder::backend()` requirements if not already present.
  - **CLAUDE.md:** correct the stack table `cubecl 0.9.0` → `0.10.0`; the "WGPU lacks f64" and "kernel compilation limits" risk rows stay relevant.
  - **Do NOT** re-introduce the old PREC-01..08 / `Precision` requirements — those belong to the deferred f32 milestone.

### Claude's Discretion

- **f64-unsupported error variant name/shape** (e.g. `Error::F64NotSupported { backend }` vs reusing a backend-error enum) — planner/researcher choose; the contract is "typed, at `build()`, no silent fallback."
- **`GpuBuffer<R>` API internals** — handle ownership, lifetime, and the resident-vs-transfer split are implementation choices, as long as host↔device transfers are demonstrably minimized for repeated `evaluate()` calls.
- **Counting-allocator mechanism for PERF-05** (custom global allocator vs dhat) — implementer's choice.
- **Bench batch sizes** beyond the locked PERF anchors (PERF-01 = 1000 CPU points; PERF-03 = cold-start <100 ms) — reasonable defaults at the planner's discretion.

### Folded Todos

None — the one pending todo (`audit-error-math-placement`, 2026-05-07) targets the Phase-10 workspace split (complete) and is unrelated to GPU/perf.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project-level (authoritative)
- `.planning/ROADMAP.md` §Phase 7 — goal + success criteria; **amended by Plan 07-01 per D-16** (drop SC#2/`>5x`, reword SC#1/SC#3).
- `.planning/REQUIREMENTS.md` — GPU-01..07, VERIFY-08, PERF-01..05. **Already f64-only** (the 2026-05-07 PREC amendments were NEVER applied — GPU-06 still reads "f64-only precision policy"). Plan 07-01 annotates PERF-02 (deferred) and GPU-02 (CUDA stub).
- `CLAUDE.md` — Recommended Stack (stack table says **cubecl 0.9.0 — STALE; actual is 0.10.0**, fix in 07-01) + Key Technical Risks rows "WGPU backend lacks f64 on many GPUs" and "CubeCL 0.9.0 kernel compilation limits" (both directly relevant). Project constraints confirm "Precision: f64 only" + the f32-is-a-milestone bullet.
- `.planning/STATE.md` — current progress; confirms Phase 7 is the last open piece of milestone v1.0.

### Architecture / chokepoints (the threading targets)
- `crates/libxc-eval/src/kernel/launch.rs` — the `CpuRuntime` chokepoint: `cpu_client() -> ComputeClient<CpuRuntime>`, buffer helpers typed `&ComputeClient<CpuRuntime>` (already generic over **element type** `F: Pod` from 11-12, NOT over runtime `R`), `calculate_launch_config`. This is the primary D-05 edit site.
- `crates/libxc-eval/src/eval/dispatch.rs` — the dispatch macro emitting `launch_unchecked::<CpuRuntime>` for exc/vxc/fxc/kxc/lxc (unpol + pol arms). The launch-boundary `Backend` match (D-05) lands here.
- 133 files under `crates/libxc-eval/src/eval/**` carry `use cubecl::cpu::CpuRuntime` — the regen-vs-localized scope (D-06) hinges on whether these call launch directly.
- `crates/libxc-eval/Cargo.toml` — `cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }` (cubecl/cpu currently unconditional) + the `[features]` oracle-* machinery. D-08 backend features land here.
- `Cargo.toml` (root facade) — `[features]` re-forwards `oracle-*` to libxc-eval; **load-bearing** `libxc-eval { default-features = false }` pin (Pitfall 1 — prevents all-306-kernel OOM). D-08 backend features re-forward here; `default` must be reconciled.

### Generators (D-06 regen targets, if chosen)
- `tools/translate_lda.py` — emits LDA dispatch incl. the `CpuRuntime` literal.
- `tools/generate_gga_dispatch.py`, `tools/generate_mgga_dispatch.py` — emit GGA/MGGA dispatch + `launch_unchecked::<CpuRuntime>` (these are the files 11-14 migrated to the 0.10 launch ABI — the 0.10 ABI recipe lives in `.planning/phases/11-.../11-14-*`).

### Benchmark + verification
- `benches/{registry,lda,gga,mgga,init,resident,transfer}.rs` — **all `println!` placeholders.** Map to: init→PERF-03 cold start, lda/gga/mgga→PERF-01 throughput, resident/transfer→GPU-05, registry→lookup. D-12/D-13 turn these into criterion benches.
- `verify/` crate — bindgen + vendored libxc oracle (rayon-parallel). Reused by D-13 (PERF-01 1.5x ratio) and D-02 (SC#1 GPU-vs-CPU witnesses). The per-family oracle command (memory `reference_per_family_oracle_command`) is the OOM-safe invocation.

### Prior-phase decisions (carry forward)
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-CONTEXT.md` — `Functional`/builder shape (gains `.backend()`).
- `.planning/phases/06-public-api-and-c-compatibility/06-CONTEXT.md` — C FFI is f64-only; backend selection is a Rust-side capability (no compat/ changes).
- **This file's SUPERSEDED predecessor** — the 2026-05-07 multi-precision decisions (old D-01–D-14) are archived in `07-DISCUSSION-LOG.md` / git history; do NOT resurrect the `Precision`/generic-`<F>` content.

### Relevant memories
- `project_kernels_f64_concrete_f32_milestone` — why f32/f16 is out of scope.
- `project_umbrella_cubecl010_launch_abi_drift` — the 0.10 launch-ABI recipe (`.clone()` handles, 2-arg `from_raw_parts`, `launch_unchecked` returns `()`).
- `feedback_ram_constraints`, `reference_kernelfree_check_gate`, `reference_per_family_oracle_command` — the OOM-safe build/test discipline this phase must obey.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **`launch.rs` buffer helpers are already generic over element type** (`F: Pod`, from 11-12) — extending them to generic `R: Runtime` (D-05) is an incremental change, not a rewrite.
- **`verify/` bindgen libxc oracle** — gives both the libxc-C timing baseline (PERF-01) and the GPU-vs-CPU correctness reference (SC#1) with zero new oracle infrastructure.
- **`benches/*.rs` filenames already map to the PERF reqs** — scaffolding/naming exists; bodies are placeholders.
- **The `oracle-*` family-feature pattern** in libxc-eval + root is the exact template for the additive backend features (D-08).
- **11-14's cubecl-0.10 launch-ABI migration** is already applied to the generators + hand-written dispatch — the GPU launch path inherits the correct 0.10 idiom.

### Established Patterns
- **Per-family dispatch enums + id-based constructors** — `Backend`/`R` threads through orthogonally to the functional id.
- **Layered workspace (libxc-core ← libxc-eval ← libxc-compat) + thin root facade** — D-11 placement respects this; public types in core, cubecl in eval.
- **`Array<f64>` SoA buffers** — become `Array<f64>` on whichever runtime; only the runtime/client type changes, not the buffer layout.

### Integration Points
- **`launch.rs` / `dispatch.rs`** — generic-`R` + the `Backend` match (D-05/D-06).
- **`Backend` enum + `FromStr` + error variants** — libxc-core (D-11).
- **`ComputeClient<R>` construction + cubecl backend deps** — libxc-eval, `#[cfg]`-gated (D-08/D-11).
- **`FunctionalBuilder`** — gains `.backend(Backend)`; `build()` resolves env var + constructs/stores the client (D-07).
- **Error enum** — add `InvalidBackendEnvVar(String)`, `BackendNotCompiledIn { backend_name }`, and an f64-unsupported variant.
- **`Cargo.toml` (root + libxc-eval)** — backend `[features]`; reconcile `default`; preserve the `default-features = false` eval pin.

### Non-Obvious Constraints
- **The all-281 GPU build OOMs** — same ceiling as the f64 CPU umbrella. GPU verification (D-02) and benches (D-12) MUST stay subset-scoped; any full-roster GPU work needs the chunked per-`-p` USER-run sweep.
- **gfx1152 ROCm support is unofficial** — cubecl-hip 0.10 on this iGPU may need `HSA_OVERRIDE_GFX_VERSION` and may not work at all; hence the D-04 spike-first gate.
- **f64 on the iGPU is rate-limited** — correctness is achievable, throughput is not (D-03 dropped `>5x`).
- **"At least one backend" invariant** — making `cpu` a feature (D-08) means `--no-default-features` with no backend leaves `launch.rs` with no runtime; planning must enforce/ document this.
- **`#[cfg]`-gated `Backend` variants force `#[cfg]` on every match arm** — keep the backend match in ONE wrapper (D-05) to contain the boilerplate; a `match_backend!` macro may help.

</code_context>

<specifics>
## Specific Ideas

- **Spike before refactor (D-04):** the cheapest possible HIP probe — one `#[cube]` kernel, `HipRuntime` client on `gfx1152`, f64 round-trip, compare to CPU — gates the whole threading effort. Try `HSA_OVERRIDE_GFX_VERSION=11.0.0` (or nearest supported gfx11 target) if the native gfx1152 target is rejected.
- **Backend match in one place (D-05):** generated dispatch calls a stable-signature `launch_<fam>::<R>(client, …)`; the `Backend → R` match + `#[cfg]` arms live only in that wrapper, so per-functional files never mention a runtime.
- **Strict env parsing (carry old D-14):** `LIBXC_RS_BACKEND=HIP` (uppercase), `= hip ` (whitespace), `=rocm`/`=amd` (aliases) all → `Error::InvalidBackendEnvVar`. Exact lowercase `cpu|cuda|hip|wgpu` only. `FromStr` needs `#[cfg]` arms: a valid-but-not-compiled name returns `BackendNotCompiledIn`, not `InvalidBackendEnvVar`.
- **WGPU f64 probe (D-09):** query `wgpu` adapter features for `SHADER_FLOAT64` before creating the device; absent → typed error at `build()`. This is the GPU-04/SC#3 deliverable even without verified kernel execution.
- **PERF-01 honesty:** if the naive CPU path misses 1.5x of libxc, document the gap rather than over-optimizing — Phase 9 (build-time) and any hot-path tuning are separate concerns; PERF-05's zero-alloc gate is the concrete hot-path lever here.

</specifics>

<deferred>
## Deferred Ideas

### Out of scope for Phase 7 (recorded, not lost)
- **`>5x` GPU throughput** (old SC#2 / PERF-02) — requires datacenter-class f64 GPU; the bench harness measures throughput but does not gate on it.
- **CUDA runtime verification** — no NVIDIA hardware locally; CUDA is a typed-error stub. Revisit when NVIDIA hardware + toolkit are available.
- **WGPU kernel-correctness verification** — WGPU is f64-probe-only this phase; verifying WGPU compute results is future work (and depends on a driver exposing `SHADER_FLOAT64`).
- **Full-roster GPU correctness sweep** — the all-281 GPU build OOMs; needs equipped hardware + a chunked per-`-p` sweep.
- **f32 / f16 / bf16 precision, generic-`<F>` kernels, `Precision` enum, two-track verification** — the entire 2026-05-07 multi-precision program (old D-01–D-14 precision parts) is a separate MILESTONE-scale effort (translator re-arch + ~2491-file regen + FP-order reconciliation). See memory `project_kernels_f64_concrete_f32_milestone`.
- **Mixed-precision / runtime precision switching / per-precision C FFI** — all tied to the deferred f32 milestone.
- **DSP / AIE-ML accelerator** (the box also exposes an AIE-ML DSP) — not a cubecl backend; out of scope.

### Reviewed Todos (not folded)
- `audit-error-math-placement` (2026-05-07) — targets the Phase-10 workspace split (complete); unrelated to GPU/perf.

</deferred>

---

*Phase: 07-gpu-backends-and-performance*
*Context gathered: 2026-04-24*
*Re-scoped to f64-only: 2026-05-27 — supersedes the 2026-05-07 multi-precision substrate; backend-selection decisions (old D-12/13/14) carried forward.*
