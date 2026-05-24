---
status: partial
phase: 05-functional-lifecycle-and-hybrid-properties
source: [05-VERIFICATION.md]
started: 2026-05-25T00:00:00Z
updated: 2026-05-25T00:00:00Z
---

## Current Test

[awaiting human testing — run on a warm-cache machine with f64 oracle available]

## Tests

### 1. b3lyp_gga_vxc_matches_libxc
expected: rust port output matches libxc xc_gga_vxc within 1e-12 relative tolerance for B3LYP at 16 grid points
why_human: Test compiles via verify/ crate that pulls 700+ kernel sub-crates; cold-cache cargo test compile estimated >15 hours. Source code, metadata, and FFI rc-checks are all in place; only execution awaits a warm-cache run.
result: [pending]

### 2. cam_b3lyp_gga_vxc_matches_libxc_default
expected: rust port matches libxc with default _omega=0.33 within 1e-12
why_human: Same compile-cascade reason. Tests are unignored and statically wired; only execution deferred.
result: [pending]

### 3. cam_b3lyp_gga_vxc_matches_libxc_omega_0_5
expected: after set_ext_param('_omega', 0.5), rust port matches libxc oracle within 1e-12
why_human: Validates ext_param propagation runtime path against live FFI. Only execution deferred.
result: [pending]

### 4. hse03_gga_vxc_matches_libxc
expected: HSE03 (id 427) rust port matches libxc within 1e-12
why_human: Same compile-cascade reason. Code in place.
result: [pending]

## Summary

total: 4
passed: 0
issues: 0
pending: 4
skipped: 0
blocked: 0

## Gaps
