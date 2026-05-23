//! Safe fractional power functions for DFT calculations.
//!
//! All functions are `#[cube]`-annotated for CubeCL compilation.
//! Generic over `<F: Float>` to support both f64 (oracle correctness) and f32 (performance).
//! `safe_cbrt` handles negative inputs correctly (cbrt(-8) == -2, not NaN).

use cubecl::prelude::*;

/// Compute the cube root of x, correctly handling negative values.
///
/// Standard `powf(x, 1/3)` returns NaN for negative x. This function
/// extracts the sign, computes `|x|^(1/3)`, and restores the sign.
#[cube]
pub fn safe_cbrt<F: Float>(x: F) -> F {
    let abs_x = F::abs(x);
    let cbrt_abs = F::powf(abs_x, F::new(1.0) / F::new(3.0));
    let sign = select(x < F::new(0.0), F::new(-1.0), F::new(1.0));
    select(x == F::new(0.0), F::new(0.0), sign * cbrt_abs)
}

/// x^(1/3) -- cube root via safe_cbrt
#[cube]
pub fn pow_1_3<F: Float>(x: F) -> F {
    safe_cbrt::<F>(x)
}

/// x^(2/3) = cbrt(x)^2
#[cube]
pub fn pow_2_3<F: Float>(x: F) -> F {
    let c = safe_cbrt::<F>(x);
    c * c
}

/// x^(4/3) = x * cbrt(x)
#[cube]
pub fn pow_4_3<F: Float>(x: F) -> F {
    x * safe_cbrt::<F>(x)
}

/// x^(5/3) = x * cbrt(x)^2
#[cube]
pub fn pow_5_3<F: Float>(x: F) -> F {
    let c = safe_cbrt::<F>(x);
    x * c * c
}

/// x^(3/2) = x * sqrt(x)
/// Maps to C macro: POW_3_2(x) = (x)*sqrt(x)
#[cube]
pub fn pow_3_2<F: Float>(x: F) -> F {
    x * F::sqrt(x)
}

/// x^(1/4) = sqrt(sqrt(x))
/// Maps to C macro: POW_1_4(x) = sqrt(sqrt(x))
#[cube]
pub fn pow_1_4<F: Float>(x: F) -> F {
    F::sqrt(F::sqrt(x))
}

/// x^(7/3) = x * x * cbrt(x)
/// Maps to C macro: POW_7_3(x) = (x)*(x)*cbrt(x)
#[cube]
pub fn pow_7_3<F: Float>(x: F) -> F {
    x * x * safe_cbrt::<F>(x)
}

/// x^2 = x * x
/// Maps to C macro: POW_2(x) = (x)*(x)
/// Named function for grep-ability matching maple2c POW_2 references.
#[cube]
pub fn pow_2<F: Float>(x: F) -> F {
    x * x
}

/// x^3 = x * x * x
/// Maps to C macro: POW_3(x) = (x)*(x)*(x)
/// Named function for grep-ability matching maple2c POW_3 references.
#[cube]
pub fn pow_3<F: Float>(x: F) -> F {
    x * x * x
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    /// Test kernel that applies safe_cbrt element-wise.
    #[cube(launch_unchecked)]
    fn test_safe_cbrt_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = safe_cbrt::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_2_3 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_2_3_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_2_3::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_4_3 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_4_3_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_4_3::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_5_3 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_5_3_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_5_3::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_3_2 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_3_2_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_3_2::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_1_4 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_1_4_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_1_4::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_7_3 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_7_3_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_7_3::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_2 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_2_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_2::<f64>(input[idx]);
    }

    /// Test kernel that applies pow_3 element-wise.
    #[cube(launch_unchecked)]
    fn test_pow_3_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = pow_3::<f64>(input[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_cbrt(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_safe_cbrt_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_2_3(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_2_3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_4_3(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_4_3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_5_3(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_5_3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_safe_cbrt_known_values() {
        let inputs = [8.0, -8.0, 0.0, 1.0, 27.0, -27.0, 1000.0];
        let expected = [2.0, -2.0, 0.0, 1.0, 3.0, -3.0, 10.0];
        let results = run_cbrt(&inputs);

        for (i, (&result, &expect)) in results.iter().zip(expected.iter()).enumerate() {
            let err = if expect == 0.0 { result.abs() } else { ((result - expect) / expect).abs() };
            assert!(err < 1e-14,
                "safe_cbrt({}) = {}, expected {}, rel_err = {}",
                inputs[i], result, expect, err);
        }
    }

    #[test]
    fn test_safe_cbrt_libm_sweep() {
        let n = 1000;
        let mut inputs = Vec::with_capacity(n);
        for i in 0..n {
            let x = -100.0 + 200.0 * (i as f64) / ((n - 1) as f64);
            inputs.push(x);
        }

        let results = run_cbrt(&inputs);

        for (i, (&result, &x)) in results.iter().zip(inputs.iter()).enumerate() {
            let expected = libm::cbrt(x);
            let err = if expected == 0.0 {
                result.abs()
            } else {
                ((result - expected) / expected).abs()
            };
            assert!(err < 1e-14,
                "safe_cbrt({}) = {}, libm::cbrt = {}, rel_err = {} at index {}",
                x, result, expected, err, i);
        }
    }

    #[test]
    fn test_pow_1_3_is_safe_cbrt() {
        // pow_1_3 delegates to safe_cbrt, just verify a sample
        let results = run_cbrt(&[27.0, -8.0]);
        approx::assert_relative_eq!(results[0], 3.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], -2.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_2_3_known_values() {
        // 8^(2/3) = (cbrt(8))^2 = 2^2 = 4
        // 27^(2/3) = 3^2 = 9
        let results = run_pow_2_3(&[8.0, 27.0, 1.0]);
        approx::assert_relative_eq!(results[0], 4.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 9.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_4_3_known_values() {
        // 8^(4/3) = 8 * cbrt(8) = 8 * 2 = 16
        // 27^(4/3) = 27 * 3 = 81
        let results = run_pow_4_3(&[8.0, 27.0, 1.0]);
        approx::assert_relative_eq!(results[0], 16.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 81.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_5_3_known_values() {
        // 8^(5/3) = 8 * cbrt(8)^2 = 8 * 4 = 32
        // 27^(5/3) = 27 * 9 = 243
        let results = run_pow_5_3(&[8.0, 27.0, 1.0]);
        approx::assert_relative_eq!(results[0], 32.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 243.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    fn run_pow_3_2(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_3_2_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_1_4(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_1_4_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_7_3(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_7_3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_2(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_2_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_pow_3(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_pow_3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_pow_3_2_known_values() {
        // 4^(3/2) = 4 * sqrt(4) = 4 * 2 = 8
        // 9^(3/2) = 9 * sqrt(9) = 9 * 3 = 27
        let results = run_pow_3_2(&[4.0, 9.0, 1.0]);
        approx::assert_relative_eq!(results[0], 8.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 27.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_1_4_known_values() {
        // 16^(1/4) = sqrt(sqrt(16)) = sqrt(4) = 2
        // 81^(1/4) = sqrt(sqrt(81)) = sqrt(9) = 3
        let results = run_pow_1_4(&[16.0, 81.0, 1.0]);
        approx::assert_relative_eq!(results[0], 2.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 3.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_7_3_known_values() {
        // 8^(7/3) = 8 * 8 * cbrt(8) = 64 * 2 = 128
        // 27^(7/3) = 27 * 27 * cbrt(27) = 729 * 3 = 2187
        let results = run_pow_7_3(&[8.0, 27.0, 1.0]);
        approx::assert_relative_eq!(results[0], 128.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 2187.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_2_known_values() {
        // 3^2 = 9, 7^2 = 49
        let results = run_pow_2(&[3.0, 7.0, 1.0]);
        approx::assert_relative_eq!(results[0], 9.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 49.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_pow_3_known_values() {
        // 2^3 = 8, 3^3 = 27
        let results = run_pow_3(&[2.0, 3.0, 1.0]);
        approx::assert_relative_eq!(results[0], 8.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[1], 27.0, max_relative = 1e-14);
        approx::assert_relative_eq!(results[2], 1.0, max_relative = 1e-14);
    }
}
