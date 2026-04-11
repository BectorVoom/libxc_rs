## 19. Source Tree


```text
├── docs/
src/
├── lib.rs
├── api/
│   ├── mod.rs
│   ├── functional.rs
│   ├── builder.rs
│   ├── batch.rs
│   ├── resident.rs
│   ├── meta.rs
│   └── compat.rs
├── compat/
│   ├── mod.rs
│   ├── raw_handle.rs
│   ├── c_layout.rs
│   ├── legacy_eval.rs
│   ├── ids.rs
│   └── removed.rs
├── meta/
│   ├── mod.rs
│   ├── library.rs
│   ├── reference.rs
│   ├── functional_meta.rs
│   ├── ext_param.rs
│   ├── hybrid.rs
│   ├── nlc.rs
│   └── auxiliary.rs
├── registry/
│   ├── mod.rs
│   ├── current.rs
│   ├── legacy.rs
│   ├── internal.rs
│   ├── by_id.rs
│   ├── by_name.rs
│   ├── families.rs
│   └── generated.rs
├── model/
│   ├── mod.rs
│   ├── family.rs
│   ├── kind.rs
│   ├── spin.rs
│   ├── derivative.rs
│   ├── flags.rs
│   ├── thresholds.rs
│   ├── precision.rs
│   └── feature_requirements.rs
├── layout/
│   ├── mod.rs
│   ├── dims.rs
│   ├── packed.rs
│   ├── strided.rs
│   ├── soa.rs
│   ├── tiles.rs
│   └── validation.rs
├── input/
│   ├── mod.rs
│   ├── lda.rs
│   ├── gga.rs
│   ├── mgga.rs
│   ├── owned.rs
│   ├── borrowed.rs
│   └── resident.rs
├── output/
│   ├── mod.rs
│   ├── request.rs
│   ├── lda.rs
│   ├── gga.rs
│   ├── mgga.rs
│   ├── bundle.rs
│   └── resident.rs
├── workspace/
│   ├── mod.rs
│   ├── planner.rs
│   ├── host.rs
│   ├── resident.rs
│   └── scratch_map.rs
├── runtime/
│   ├── mod.rs
│   ├── device.rs
│   ├── cpu.rs
│   ├── cuda.rs
│   ├── hip.rs
│   ├── wgpu.rs
│   ├── cache.rs
│   ├── streams.rs
│   └── capability.rs
├── kernel/
│   ├── mod.rs
│   ├── launch.rs
│   ├── dispatch_key.rs
│   ├── shared/
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   ├── math.rs
│   │   ├── thresholds.rs
│   │   ├── spin.rs
│   │   ├── ext_params.rs
│   │   ├── output_mask.rs
│   │   └── aux_accumulate.rs
│   ├── lda/
│   │   ├── mod.rs
│   │   ├── order0.rs
│   │   ├── order1.rs
│   │   ├── order2.rs
│   │   ├── order3.rs
│   │   └── order4.rs
│   ├── gga/
│   │   ├── mod.rs
│   │   ├── order0.rs
│   │   ├── order1.rs
│   │   ├── order2.rs
│   │   ├── order3.rs
│   │   └── order4.rs
│   ├── mgga/
│   │   ├── mod.rs
│   │   ├── order0.rs
│   │   ├── order1.rs
│   │   ├── order2.rs
│   │   ├── order3.rs
│   │   └── order4.rs
│   └── mix/
│       ├── mod.rs
│       ├── aux_eval.rs
│       ├── weighted_sum.rs
│       ├── hybrid_terms.rs
│       └── nlc_terms.rs
├── eval/
│   ├── mod.rs
│   ├── dispatcher.rs
│   ├── prepare.rs
│   ├── execute.rs
│   ├── finalize.rs
│   └── policy.rs
├── error/
│   ├── mod.rs
│   ├── public.rs
│   ├── internal.rs
│   └── ffi.rs
└── generated/
    ├── mod.rs
    ├── functional_registry.rs
    ├── legacy_aliases.rs
    ├── removed_ids.rs
    ├── ext_param_specs.rs
    └── dispatch_tables.rs

xtask/
├── main.rs
├── parse_xc_h.rs
├── parse_functionals.rs
├── generate_registry.rs
└── generate_dispatch.rs

tests/
├── api_catalog.rs
├── registry_roundtrip.rs
├── ext_params.rs
├── shape_validation.rs
├── oracle_lda.rs
├── oracle_gga.rs
├── oracle_mgga.rs
├── oracle_hybrid.rs
├── cpu_gpu_parity.rs
├── nan_inf.rs
└── removed_ids.rs

verify/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── dataset.rs
    ├── oracle_ffi.rs
    ├── compare.rs
    ├── report.rs
    └── thresholds.rs

benches/
├── registry.rs
├── init.rs
├── lda.rs
├── gga.rs
├── mgga.rs
├── resident.rs
└── transfer.rs
```
