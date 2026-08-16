//! P11-INV-A1: Wave 0 spike for D-02 tuple-return `<F: Float>` `#[cube]` ABI.
//! Proves cubecl-macros 0.10 round-trips `#[cube] fn f<F: Float>(...) -> (F, F)`
//! through parser -> IR -> cubecl-cpu runtime.
//! If this test fails, D-02 ABI is invalidated and Phase 11 pauses for re-discussion.
//!
//! Located under `libxc-kernel-math` rather than `verify/` (where plan 11-01 task 1
//! placed it) because the verify crate's dev-dependencies include the full kernel
//! tree (`libxc-kernel-lda` + `libxc-kernel-mgga`), and compiling those during
//! `cargo test -p libxc_rs-verify --test ...` OOMs this machine. `libxc-kernel-math`
//! depends only on `cubecl` and is the natural home for `#[cube]` ABI experiments.

#![allow(non_snake_case)]

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::prelude::*;

#[cube]
fn add_sub<F: Float>(x: F, y: F) -> (F, F) {
    (x + y, x - y)
}

#[cube(launch_unchecked)]
fn add_sub_kernel<F: Float>(
    input: &Array<F>,
    out_a: &mut Array<F>,
    out_b: &mut Array<F>,
) {
    let ip = ABSOLUTE_POS;
    if ip < out_a.len() {
        let (a, b) = add_sub::<F>(input[ip * 2], input[ip * 2 + 1]);
        out_a[ip] = a;
        out_b[ip] = b;
    }
}

#[test]
fn spike_tuple_return_f64_cpu() {
    let input: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    let pair_count = input.len() / 2;

    let client = CpuRuntime::client(&CpuDevice);

    let input_handle = client.create_from_slice(bytemuck::cast_slice(&input));
    let out_a_zeros = vec![0.0_f64; pair_count];
    let out_b_zeros = vec![0.0_f64; pair_count];
    let out_a_handle = client.create_from_slice(bytemuck::cast_slice(&out_a_zeros));
    let out_b_handle = client.create_from_slice(bytemuck::cast_slice(&out_b_zeros));

    let cube_dim = CubeDim::new_1d(256);
    let cube_count = CubeCount::new_1d((pair_count as u32).div_ceil(256));

    let input_len = input.len();
    let out_a_clone = out_a_handle.clone();
    let out_b_clone = out_b_handle.clone();

    unsafe {
        add_sub_kernel::launch_unchecked::<f64, CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts(input_handle, input_len),
            ArrayArg::from_raw_parts(out_a_handle, pair_count),
            ArrayArg::from_raw_parts(out_b_handle, pair_count),
        );
    }

    let out_a_bytes = client.read_one(out_a_clone).expect(
        "D-02 hard gate: read_one(out_a) failed — runtime error post-launch; Phase 11 must pause.",
    );
    let out_b_bytes = client.read_one(out_b_clone).expect(
        "D-02 hard gate: read_one(out_b) failed — runtime error post-launch; Phase 11 must pause.",
    );
    let out_a: Vec<f64> = bytemuck::cast_slice(&out_a_bytes).to_vec();
    let out_b: Vec<f64> = bytemuck::cast_slice(&out_b_bytes).to_vec();

    let expected_a = [3.0_f64, 7.0];
    let expected_b = [-1.0_f64, -1.0];

    assert_eq!(out_a.len(), pair_count);
    assert_eq!(out_b.len(), pair_count);

    for i in 0..pair_count {
        let da = (out_a[i] - expected_a[i]).abs();
        let db = (out_b[i] - expected_b[i]).abs();
        assert!(
            da < 1e-15,
            "D-02 hard gate: out_a[{i}] = {:e}, expected {:e} (abs diff {:e}); \
             tuple-return ABI returned wrong value — Phase 11 must pause.",
            out_a[i], expected_a[i], da,
        );
        assert!(
            db < 1e-15,
            "D-02 hard gate: out_b[{i}] = {:e}, expected {:e} (abs diff {:e}); \
             tuple-return ABI returned wrong value — Phase 11 must pause.",
            out_b[i], expected_b[i], db,
        );
    }

    assert_eq!(out_a.as_slice(), expected_a.as_slice());
    assert_eq!(out_b.as_slice(), expected_b.as_slice());
}
