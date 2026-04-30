# Phase 09 Plan 09-05 — Deferred-GGA Coverage Audit

This report is **auto-generated** by `tools/audit_deferred_gga.py` per CONTEXT D-12 (script-driven audit). Do not edit by hand. Re-run the script after any translator/regen pass to refresh.

## Summary

- Canonical functional count: **25** (must be 25)
- OK: **25**
- GAP: **0**
- FORBIDDEN_GATE: **0**

## Coverage Status

| Functional | Status | Sub-crates | Covered tuples | Gaps |
|---|---|---|---|---|
| `gga_c_acgga` | `OK` | kernel-gga-9c | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_acggap` | `OK` | kernel-gga-8a, kernel-gga-8b, kernel-gga-8c | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_ft97` | `OK` | kernel-gga-1a, kernel-gga-1b, kernel-gga-1c, kernel-gga-1d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_gapc` | `OK` | kernel-gga-4a, kernel-gga-4b, kernel-gga-4c, kernel-gga-4d, kernel-gga-4e, kernel-gga-4f | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_gaploc` | `OK` | kernel-gga-5a, kernel-gga-5b, kernel-gga-5c, kernel-gga-5d, kernel-gga-5e, kernel-gga-5f | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_hcth_a` | `OK` | kernel-gga-7d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_optc` | `OK` | kernel-gga-6a, kernel-gga-6b, kernel-gga-6c | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_pbe_erf_gws` | `OK` | kernel-gga-3a, kernel-gga-3b, kernel-gga-3c, kernel-gga-3d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_pbeloc` | `OK` | kernel-gga-11 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_pw91` | `OK` | kernel-gga-13 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_q2d` | `OK` | kernel-gga-7a, kernel-gga-7b, kernel-gga-7c | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_regtpss` | `OK` | kernel-gga-12 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_revtca` | `OK` | kernel-gga-3e | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_sg4` | `OK` | kernel-gga-10a, kernel-gga-10b | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_sogga11` | `OK` | kernel-gga-12 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_zpbeint` | `OK` | kernel-gga-13 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_zvpbeint` | `OK` | kernel-gga-11 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_c_zvpbeloc` | `OK` | kernel-gga-12 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_x_ft97` | `OK` | kernel-gga-14 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_x_hjs` | `OK` | kernel-gga-10c, kernel-gga-10d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_x_hjs_b88_v2` | `OK` | kernel-gga-9a, kernel-gga-9b | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_x_lcgau` | `OK` | kernel-gga-11 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_x_wpbeh` | `OK` | kernel-gga-2a, kernel-gga-2b, kernel-gga-2c, kernel-gga-2d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `gga_xc_b97` | `OK` | kernel-gga-13 | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |
| `hyb_gga_xc_wb97` | `OK` | kernel-gga-8d | exc/pol, exc/unpol, fxc/pol, fxc/unpol, kxc/pol, kxc/unpol, lxc/pol, lxc/unpol, vxc/pol, vxc/unpol | 0 |

## Detailed Gap Reports

All 25 canonical functionals are at status `OK`. No gaps.

