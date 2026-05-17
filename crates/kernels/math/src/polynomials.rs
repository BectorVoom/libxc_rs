//! Polynomial and rational function evaluation using Horner's method.
//!
//! Used internally by the erf implementation and by GGA/MGGA functionals.
//! Generic over `<F: Float>` to support both f64 and f32.

use cubecl::prelude::*;

/// Evaluate a polynomial using Horner's method.
///
/// Coefficients are ordered highest-degree first:
/// `coeffs = [a_n, a_{n-1}, ..., a_1, a_0]`
/// computes `a_n * x^n + a_{n-1} * x^{n-1} + ... + a_1 * x + a_0`
///
/// Horner form: `((a_n * x + a_{n-1}) * x + ...) * x + a_0`
#[cube]
pub fn poly_eval<F: Float>(x: F, coeffs: &Array<F>, #[comptime] n: usize) -> F {
    let mut result = coeffs[0usize];
    let mut i = 1usize;
    while i < n {
        result = result * x + coeffs[i];
        i += 1;
    }
    result
}

/// Evaluate a rational function P(x)/Q(x) using Horner's method for both.
///
/// Both `p` and `q` coefficient arrays are highest-degree first.
#[cube]
pub fn rational_eval<F: Float>(
    x: F,
    p: &Array<F>,
    q: &Array<F>,
    #[comptime] np: usize,
    #[comptime] nq: usize,
) -> F {
    poly_eval(x, p, np) / poly_eval(x, q, nq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    #[cube(launch_unchecked)]
    fn test_poly_eval_kernel(
        x_arr: &Array<f64>,
        coeffs: &Array<f64>,
        output: &mut Array<f64>,
        #[comptime] nc: usize,
    ) {
        let idx = ABSOLUTE_POS;
        output[idx] = poly_eval(x_arr[idx], coeffs, nc);
    }

    #[cube(launch_unchecked)]
    fn test_rational_eval_kernel(
        x_arr: &Array<f64>,
        p: &Array<f64>,
        q: &Array<f64>,
        output: &mut Array<f64>,
        #[comptime] np: usize,
        #[comptime] nq: usize,
    ) {
        let idx = ABSOLUTE_POS;
        output[idx] = rational_eval(x_arr[idx], p, q, np, nq);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_poly_eval(x_values: &[f64], coeffs: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = x_values.len();
        let nc = coeffs.len();

        let h_x = client.create_from_slice(bytemuck::cast_slice(x_values));
        let h_coeffs = client.create_from_slice(bytemuck::cast_slice(coeffs));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_poly_eval_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&h_x, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_coeffs, nc, 1),
                ArrayArg::from_raw_parts::<f64>(&h_out, n, 1),
                nc,
            ).unwrap();
        }

        let bytes = client.read_one(h_out);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_rational_eval(x_values: &[f64], p: &[f64], q: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = x_values.len();
        let np = p.len();
        let nq = q.len();

        let h_x = client.create_from_slice(bytemuck::cast_slice(x_values));
        let h_p = client.create_from_slice(bytemuck::cast_slice(p));
        let h_q = client.create_from_slice(bytemuck::cast_slice(q));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_rational_eval_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&h_x, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_p, np, 1),
                ArrayArg::from_raw_parts::<f64>(&h_q, nq, 1),
                ArrayArg::from_raw_parts::<f64>(&h_out, n, 1),
                np,
                nq,
            ).unwrap();
        }

        let bytes = client.read_one(h_out);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_poly_eval_quadratic() {
        // 3x^2 + 2x + 1 at x=2: 3*4 + 2*2 + 1 = 17
        let results = run_poly_eval(&[2.0], &[3.0, 2.0, 1.0]);
        approx::assert_relative_eq!(results[0], 17.0, max_relative = 1e-15);
    }

    #[test]
    fn test_poly_eval_constant() {
        // Constant polynomial: 5.0
        let results = run_poly_eval(&[3.0, 7.0], &[5.0]);
        assert_eq!(results[0], 5.0);
        assert_eq!(results[1], 5.0);
    }

    #[test]
    fn test_poly_eval_linear() {
        // 2x + 3 at x=5: 13
        let results = run_poly_eval(&[5.0], &[2.0, 3.0]);
        approx::assert_relative_eq!(results[0], 13.0, max_relative = 1e-15);
    }

    #[test]
    fn test_poly_eval_cubic() {
        // x^3 + 0*x^2 + 0*x + 0 at x=3: 27
        let results = run_poly_eval(&[3.0], &[1.0, 0.0, 0.0, 0.0]);
        approx::assert_relative_eq!(results[0], 27.0, max_relative = 1e-15);
    }

    #[test]
    fn test_poly_eval_at_zero() {
        // Any polynomial at x=0 equals its constant term
        let results = run_poly_eval(&[0.0], &[99.0, 88.0, 77.0, 42.0]);
        approx::assert_relative_eq!(results[0], 42.0, max_relative = 1e-15);
    }

    #[test]
    fn test_rational_eval_basic() {
        // P(x) = 2x + 1, Q(x) = x + 1
        // At x=3: P(3) = 7, Q(3) = 4, result = 7/4 = 1.75
        let results = run_rational_eval(&[3.0], &[2.0, 1.0], &[1.0, 1.0]);
        approx::assert_relative_eq!(results[0], 1.75, max_relative = 1e-15);
    }

    #[test]
    fn test_rational_eval_identity() {
        // P(x) = x, Q(x) = 1 => result = x
        let results = run_rational_eval(&[5.0, 10.0], &[1.0, 0.0], &[1.0]);
        approx::assert_relative_eq!(results[0], 5.0, max_relative = 1e-15);
        approx::assert_relative_eq!(results[1], 10.0, max_relative = 1e-15);
    }
}
