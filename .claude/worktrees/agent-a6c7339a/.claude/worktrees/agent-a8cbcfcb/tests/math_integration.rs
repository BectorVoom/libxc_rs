//! Integration tests for cross-module math function composition through CubeCL.
//!
//! These tests verify that #[cube] functions from different modules compose correctly
//! when called through the CubeCL JIT compilation pipeline. This catches:
//! - CubeCL failing to inline cross-module #[cube] function calls
//! - Type mismatches between modules only visible after JIT compilation
//! - CubeCL IR generation failures when composing multiple #[cube] functions
//!
//! Individual function correctness is tested by inline unit tests in each module.
//! These integration tests focus on the composition chains.

use cubecl::cpu::CpuRuntime;
use cubecl::prelude::*;

use libxc_rs::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};
use libxc_rs::math::constants::{KF_CONST, RS_CONST};
use libxc_rs::math::dft_quantities::{reduced_gradient_s, tf_kinetic, wigner_seitz_rs};
use libxc_rs::math::erf::erf_approx;
use libxc_rs::math::powers::pow_1_3;
use libxc_rs::math::spin::spin_scaling;

// =============================================================================
// Test kernels: each exercises a cross-module composition chain
// =============================================================================

/// DFT quantities composition kernel.
/// Exercises: dft_quantities -> powers -> safe_cbrt chain.
#[cube(launch_unchecked)]
fn dft_quantities_kernel(rho: &Array<f64>, output: &mut Array<f64>) {
    let ip = ABSOLUTE_POS;
    if ip < output.len() {
        output[ip] = wigner_seitz_rs(rho[ip]);
    }
}

/// Spin scaling composition kernel.
/// Exercises: spin -> powers -> safe_cbrt chain.
#[cube(launch_unchecked)]
fn spin_scaling_kernel(zeta: &Array<f64>, output: &mut Array<f64>) {
    let ip = ABSOLUTE_POS;
    if ip < output.len() {
        output[ip] = spin_scaling(zeta[ip]);
    }
}

/// Combined GGA-like kernel: computes rs * s from (rho, sigma).
/// Exercises: dft_quantities + powers + constants all composed in a single kernel.
#[cube(launch_unchecked)]
fn gga_like_kernel(rho: &Array<f64>, sigma: &Array<f64>, output: &mut Array<f64>) {
    let ip = ABSOLUTE_POS;
    if ip < output.len() {
        let rs = wigner_seitz_rs(rho[ip]);
        let s = reduced_gradient_s(rho[ip], sigma[ip]);
        output[ip] = rs * s;
    }
}

/// erf sweep kernel: runs erf_approx on many points through CubeCL.
/// Exercises: erf -> polynomials composition in batch.
#[cube(launch_unchecked)]
fn erf_sweep_kernel(x: &Array<f64>, output: &mut Array<f64>) {
    let ip = ABSOLUTE_POS;
    if ip < output.len() {
        output[ip] = erf_approx(x[ip]);
    }
}

/// Combined MGGA-like kernel: computes rs, s, and tf_kinetic together.
/// Simulates what a real MGGA kernel does: multiple DFT quantities computed
/// in sequence within a single #[cube] function.
#[cube(launch_unchecked)]
fn mgga_like_kernel(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    output_rs: &mut Array<f64>,
    output_s: &mut Array<f64>,
    output_tf: &mut Array<f64>,
) {
    let ip = ABSOLUTE_POS;
    if ip < output_rs.len() {
        output_rs[ip] = wigner_seitz_rs(rho[ip]);
        output_s[ip] = reduced_gradient_s(rho[ip], sigma[ip]);
        output_tf[ip] = tf_kinetic(rho[ip]);
    }
}

// =============================================================================
// Helper: relative error with absolute floor
// =============================================================================

fn rel_err(a: f64, b: f64) -> f64 {
    if b.abs() < 1e-300 {
        a.abs()
    } else {
        ((a - b) / b).abs()
    }
}

// =============================================================================
// Test 1: DFT quantities composition (dft_quantities -> powers -> safe_cbrt)
// =============================================================================

#[test]
fn test_dft_quantities_integration() {
    let n = 50;
    // Log-spaced densities from 0.001 to 100.0
    let rho: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            // 10^(-3 + 5*t) => range 0.001 to 100
            10.0_f64.powf(-3.0 + 5.0 * t)
        })
        .collect();

    let client = cpu_client();
    let input_handle = create_input_buffer(&client, &rho);
    let output_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        dft_quantities_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
        )
        .unwrap();
    }

    let results = read_output_buffer(&client, output_handle, n);

    // Verify against explicit computation: rs = RS_CONST * (1/rho)^(1/3)
    for (i, (&r, &rho_val)) in results.iter().zip(rho.iter()).enumerate() {
        let expected = RS_CONST * (1.0 / rho_val).cbrt();
        let err = rel_err(r, expected);
        assert!(
            err < 1e-14,
            "wigner_seitz_rs(rho={}) = {}, expected {}, rel_err = {} at index {}",
            rho_val, r, expected, err, i
        );
    }

    // Specific known value: rho closest to 1.0
    // At rho=1.0: rs = RS_CONST * 1.0 = 0.62035049089940002
    let idx_1 = rho
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| (*a - 1.0).abs().partial_cmp(&(*b - 1.0).abs()).unwrap())
        .unwrap()
        .0;
    let expected_rs_1 = RS_CONST * (1.0 / rho[idx_1]).cbrt();
    assert!(rel_err(results[idx_1], expected_rs_1) < 1e-14);
}

// =============================================================================
// Test 2: Spin scaling composition (spin -> powers -> safe_cbrt)
// =============================================================================

#[test]
fn test_spin_scaling_integration() {
    let zeta_values = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
    let n = zeta_values.len();

    let client = cpu_client();
    let input_handle = create_input_buffer(&client, &zeta_values);
    let output_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        spin_scaling_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
        )
        .unwrap();
    }

    let results = read_output_buffer(&client, output_handle, n);

    // Verify against explicit computation:
    // f(zeta) = ((1+zeta)^(4/3) + (1-zeta)^(4/3)) / 2
    for (i, (&f_zeta, &z)) in results.iter().zip(zeta_values.iter()).enumerate() {
        let expected = ((1.0 + z).powf(4.0 / 3.0) + (1.0 - z).powf(4.0 / 3.0)) / 2.0;
        let err = rel_err(f_zeta, expected);
        assert!(
            err < 1e-14,
            "spin_scaling(zeta={}) = {}, expected {}, rel_err = {} at index {}",
            z, f_zeta, expected, err, i
        );
    }

    // Known exact values:
    // f(0) = 1.0
    approx::assert_relative_eq!(results[0], 1.0, max_relative = 1e-14);
    // f(1) = 2^(1/3) = 1.2599210498948732
    approx::assert_relative_eq!(results[6], 2.0_f64.powf(1.0 / 3.0), max_relative = 1e-14);
}

// =============================================================================
// Test 3: Combined GGA-like kernel (multiple DFT quantities composed)
// =============================================================================

#[test]
fn test_gga_like_composition() {
    // Test with known (rho, sigma) pairs
    let rho = [0.1, 0.5, 1.0, 2.0, 10.0];
    let sigma = [0.01, 0.1, 1.0, 4.0, 100.0];
    let n = rho.len();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho);
    let sigma_handle = create_input_buffer(&client, &sigma);
    let output_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        gga_like_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
        )
        .unwrap();
    }

    let results = read_output_buffer(&client, output_handle, n);

    // Verify: output = rs * s
    // rs = RS_CONST * (1/rho)^(1/3)
    // s = sqrt(sigma) / (2 * KF_CONST * rho^(4/3))
    for (i, ((&rho_val, &sigma_val), &result)) in
        rho.iter().zip(sigma.iter()).zip(results.iter()).enumerate()
    {
        let rs = RS_CONST * (1.0 / rho_val).cbrt();
        let s = sigma_val.sqrt() / (2.0 * KF_CONST * rho_val.powf(4.0 / 3.0));
        let expected = rs * s;
        let err = rel_err(result, expected);
        assert!(
            err < 1e-13,
            "gga_like(rho={}, sigma={}) = {}, expected {}, rel_err = {} at index {}",
            rho_val, sigma_val, result, expected, err, i
        );
    }
}

// =============================================================================
// Test 4: erf sweep through CubeCL (erf -> polynomials in batch)
// =============================================================================

#[test]
fn test_erf_sweep_integration() {
    let n = 1000;
    let x: Vec<f64> = (0..n)
        .map(|i| -6.0 + 12.0 * (i as f64) / ((n - 1) as f64))
        .collect();

    let client = cpu_client();
    let input_handle = create_input_buffer(&client, &x);
    let output_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        erf_sweep_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
        )
        .unwrap();
    }

    let results = read_output_buffer(&client, output_handle, n);

    // Compare all 1000 outputs against libm::erf() with tolerance
    let mut max_err = 0.0_f64;
    for (i, (&result, &x_val)) in results.iter().zip(x.iter()).enumerate() {
        let expected = libm::erf(x_val);
        if expected.abs() < 1e-300 {
            assert!(
                result.abs() < 1e-14,
                "erf({}) = {}, expected ~0, abs_err too large at index {}",
                x_val, result, i
            );
        } else {
            let err = ((result - expected) / expected).abs();
            if err > max_err {
                max_err = err;
            }
            // The branchless CubeCL implementation achieves <1e-13 vs libm
            assert!(
                err < 1e-13,
                "erf({}) = {}, libm::erf = {}, rel_err = {} at index {}",
                x_val, result, expected, err, i
            );
        }
    }

    // Verify batch launch processed all 1000 points
    assert_eq!(results.len(), 1000);
    // Log the maximum error achieved
    eprintln!(
        "erf sweep max relative error across 1000 points: {:.2e}",
        max_err
    );
}

// =============================================================================
// Test 5: MGGA-like kernel (multiple DFT quantities in single kernel)
// =============================================================================

#[test]
fn test_mgga_like_multi_output() {
    let rho = [0.01, 0.1, 1.0, 10.0, 100.0];
    let sigma = [0.001, 0.01, 0.1, 1.0, 10.0];
    let n = rho.len();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho);
    let sigma_handle = create_input_buffer(&client, &sigma);
    let out_rs_handle = create_zero_output_buffer(&client, n);
    let out_s_handle = create_zero_output_buffer(&client, n);
    let out_tf_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        mgga_like_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&out_rs_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&out_s_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&out_tf_handle, n, 1),
        )
        .unwrap();
    }

    let rs_results = read_output_buffer(&client, out_rs_handle, n);
    let s_results = read_output_buffer(&client, out_s_handle, n);
    let tf_results = read_output_buffer(&client, out_tf_handle, n);

    for (i, ((&rho_val, &sigma_val), ((&rs, &s), &tf))) in rho
        .iter()
        .zip(sigma.iter())
        .zip(rs_results.iter().zip(s_results.iter()).zip(tf_results.iter()))
        .enumerate()
    {
        let expected_rs = RS_CONST * (1.0 / rho_val).cbrt();
        let expected_s = sigma_val.sqrt() / (2.0 * KF_CONST * rho_val.powf(4.0 / 3.0));
        let expected_tf = 0.3 * KF_CONST * KF_CONST * rho_val.powf(5.0 / 3.0);

        assert!(
            rel_err(rs, expected_rs) < 1e-14,
            "rs: rho={}, got={}, expected={}, err={:.2e} at {}",
            rho_val, rs, expected_rs, rel_err(rs, expected_rs), i
        );
        assert!(
            rel_err(s, expected_s) < 1e-13,
            "s: rho={}, sigma={}, got={}, expected={}, err={:.2e} at {}",
            rho_val, sigma_val, s, expected_s, rel_err(s, expected_s), i
        );
        assert!(
            rel_err(tf, expected_tf) < 1e-14,
            "tf: rho={}, got={}, expected={}, err={:.2e} at {}",
            rho_val, tf, expected_tf, rel_err(tf, expected_tf), i
        );
    }
}

// =============================================================================
// Test 6: Verify pow_1_3 cross-module call through a kernel
// (pow_1_3 -> safe_cbrt internal chain, called from integration test crate)
// =============================================================================

#[cube(launch_unchecked)]
fn pow_chain_kernel(input: &Array<f64>, output: &mut Array<f64>) {
    let ip = ABSOLUTE_POS;
    if ip < output.len() {
        // Call pow_1_3 which internally calls safe_cbrt
        output[ip] = pow_1_3(input[ip]);
    }
}

#[test]
fn test_pow_chain_cross_crate() {
    let inputs = [8.0, 27.0, 64.0, 125.0, 1000.0, 0.001, 0.008];
    let expected = [2.0, 3.0, 4.0, 5.0, 10.0, 0.1, 0.2];
    let n = inputs.len();

    let client = cpu_client();
    let input_handle = create_input_buffer(&client, &inputs);
    let output_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        pow_chain_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
        )
        .unwrap();
    }

    let results = read_output_buffer(&client, output_handle, n);

    for (i, (&result, &exp)) in results.iter().zip(expected.iter()).enumerate() {
        let err = rel_err(result, exp);
        assert!(
            err < 1e-14,
            "pow_1_3({}) = {}, expected {}, rel_err = {} at index {}",
            inputs[i], result, exp, err, i
        );
    }
}
