//! F-03 gate: proves the translator's emitted literal-wrap form
//! `F::cast_from(<lit>_f64)` (which replaced the old f32-narrowing `F::new(<lit>)`)
//! both COMPILES inside a generic tuple-returning `#[cube]` chunk fn AND preserves
//! the exact f64 value — i.e. there is no silent f32 truncation.
//!
//! Mirrors the exact shape emitted into the per-functional chunk bodies, e.g.
//! `mgga_c_kcisk_p3/src/lxc_pol/part6/chunk988.rs`:
//!   `-t.. - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t.. + ...`
//!
//! Located under `libxc-kernel-math` (cubecl-only deps) — compiling the real
//! affected crates (rmggac/kcisk/tpss, 1000+ files each) OOMs this box, so this
//! standalone spike is the OOM-safe gate for the emitted pattern.

#![allow(non_snake_case)]

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::prelude::*;

/// A chunk-like generic fn using the emitted `F::cast_from(<lit>_f64)` form for:
///  - integer-ratio coefficients (`4.0/9.0`, `2.0/3.0`) — exact in f32, must match;
///  - a non-exact-in-f32 coefficient (`0.82785e-1`) — the f32-truncating `F::new`
///    path would have produced 0.082785001… here; cast_from must yield exact f64.
#[cube]
fn chunk_like<F: Float>(x: F) -> (F, F) {
    let a = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * x
        + F::cast_from(0.82785e-1_f64);
    let b = x - F::cast_from(2.0_f64) / F::cast_from(3.0_f64)
        + F::cast_from(0.301925e0_f64);
    (a, b)
}

#[cube(launch_unchecked)]
fn chunk_like_kernel<F: Float>(input: &Array<F>, out_a: &mut Array<F>, out_b: &mut Array<F>) {
    let ip = ABSOLUTE_POS;
    if ip < out_a.len() {
        let (a, b) = chunk_like::<F>(input[ip]);
        out_a[ip] = a;
        out_b[ip] = b;
    }
}

#[test]
fn spike_cast_from_preserves_f64() {
    let input: Vec<f64> = vec![1.5_f64, 3.0_f64];
    let n = input.len();
    let client = CpuRuntime::client(&CpuDevice);

    let input_handle = client.create_from_slice(bytemuck::cast_slice(&input));
    let out_a_handle = client.create_from_slice(bytemuck::cast_slice(&vec![0.0_f64; n]));
    let out_b_handle = client.create_from_slice(bytemuck::cast_slice(&vec![0.0_f64; n]));

    let cube_dim = CubeDim::new_1d(256);
    let cube_count = CubeCount::new_1d((n as u32).div_ceil(256));
    let out_a_clone = out_a_handle.clone();
    let out_b_clone = out_b_handle.clone();

    unsafe {
        chunk_like_kernel::launch_unchecked::<f64, CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts(input_handle, n),
            ArrayArg::from_raw_parts(out_a_handle, n),
            ArrayArg::from_raw_parts(out_b_handle, n),
        );
    }

    let out_a: Vec<f64> = bytemuck::cast_slice(&client.read_one(out_a_clone).unwrap()).to_vec();
    let out_b: Vec<f64> = bytemuck::cast_slice(&client.read_one(out_b_clone).unwrap()).to_vec();

    // Expected computed in full f64 — must match to the last bit (no f32 narrowing).
    for (i, &x) in input.iter().enumerate() {
        let ea = 4.0_f64 / 9.0_f64 * x + 0.82785e-1_f64;
        let eb = x - 2.0_f64 / 3.0_f64 + 0.301925e0_f64;
        let da = (out_a[i] - ea).abs();
        let db = (out_b[i] - eb).abs();
        // 1e-15 absolute is far tighter than the ~8e-9 error an f32-narrowed
        // 0.82785e-1 would introduce — this assertion fails loudly if cast_from
        // ever silently truncates to f32.
        assert!(da < 1e-15, "out_a[{i}]={:.20} expected {:.20} (diff {:e})", out_a[i], ea, da);
        assert!(db < 1e-15, "out_b[{i}]={:.20} expected {:.20} (diff {:e})", out_b[i], eb, db);
    }
}
