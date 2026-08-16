//! Safe fractional power functions for DFT calculations.
//!
//! All functions are `#[cube]`-annotated for CubeCL compilation.
//! Generic over `<F: Float>` to support both f64 (oracle correctness) and f32 (performance).
//! `safe_cbrt` handles negative inputs correctly (cbrt(-8) == -2, not NaN).
//!
//! # Why these are built on a real cube root, not `powf(x, 1/3)`
//!
//! libxc's `util.h` defines the whole POW_n_3 family in terms of `cbrt`:
//!
//! ```c
//! #ifdef HAVE_CBRT
//! #define POW_1_3(x) cbrt(x)
//! #define POW_2_3(x) (cbrt(x)*cbrt(x))
//! #define POW_4_3(x) ((x)*cbrt(x))
//! #define POW_5_3(x) ((x)*cbrt(x)*cbrt(x))
//! #define POW_7_3(x) ((x)*(x)*cbrt(x))
//! #else   /* only when libm has no cbrt -- never, in practice */
//! #define POW_1_3(x) pow((x), 1.0/3.0)
//! ```
//!
//! `HAVE_CBRT` is an autoconf probe for `cbrt` in libm, so every real build of
//! libxc takes the first branch. `pow(x, 1.0/3.0)` is NOT the same function:
//! `1.0/3.0` is not exactly 1/3, so the result carries a relative error of
//! roughly `eps * ln(x)`, which grows without bound as x leaves [0.1, 10].
//! Measured against glibc's `cbrt` over 3e6 samples:
//!
//! | implementation        | rho in [1e-10, 1e3]        | full f64 range              |
//! |-----------------------|----------------------------|-----------------------------|
//! | `powf(x, 1.0/3.0)`    | 1.15 ulp mean, 4 ulp max   | 43.2 ulp mean, 118 ulp max  |
//! | `cbrt_f64` (below)    | 0.085 ulp mean, 1 ulp max  | 0.086 ulp mean, 2 ulp max   |
//!
//! `powf` also got exact cubes wrong -- `pow(1e9, 1.0/3.0)` returned
//! 999.9999999999997 rather than 1000. Since `pow_2_3` .. `pow_7_3` all square
//! or cube this value, the error compounds.

use cubecl::prelude::*;

// 2^(k/3) rescaling factors, selected by `xe % 3` in {-2, -1, 0, 1, 2}.
const CBRT_F_M2: f64 = 0.629960524947436582384; // 2^(-2/3)
const CBRT_F_M1: f64 = 0.793700525984099737376; // 2^(-1/3)
const CBRT_F_P1: f64 = 1.259921049894873164767; // 2^( 1/3)
const CBRT_F_P2: f64 = 1.587401051968199474752; // 2^( 2/3)

/// True cube root of an `f64`, branch-free and without any transcendental call.
///
/// Algorithm (the classical libm shape): reduce `x = xm * 2^xe` with
/// `xm` in [0.5, 1), evaluate a degree-6 minimax seed on `xm`, apply one Halley
/// step (cubically convergent), rescale by `2^(xe/3)`, then take one Newton
/// step whose residual `y^3 - x` is formed with an FMA so it is not destroyed
/// by cancellation. The result is within 1 ulp of a correctly rounded cube root
/// over the whole normal range and is exact on exact cubes.
///
/// Handles negatives natively (`cbrt(-8) == -2`), and passes 0, +-inf and NaN
/// through unchanged, matching C's `cbrt`.
#[cube]
pub fn cbrt_f64(x: f64) -> f64 {
    let a = f64::abs(x);
    let bits = u64::reinterpret(a);

    // frexp: a == xm * 2^xe with xm in [0.5, 1). Subnormals are scaled by 2^54
    // first, then the exponent is corrected, so they reduce like normals.
    let raw = (bits >> 52u64) & 0x7ffu64;
    let is_sub = raw == 0u64;
    let scaled = a * 18014398509481984.0f64; // 2^54
    let bits_u = select(is_sub, u64::reinterpret(scaled), bits);
    let raw_u = i32::cast_from((bits_u >> 52u64) & 0x7ffu64) + select(is_sub, -54i32, 0i32);
    let xm = f64::reinterpret((bits_u & 0x800fffffffffffffu64) | (1022u64 << 52u64));
    let xe = raw_u - 1022i32;

    // Degree-6 minimax seed for xm^(1/3) on [0.5, 1).
    let u = 0.354895765043919860f64
        + (1.50819193781584896f64
            + (-2.11499494167371287f64
                + (2.44693122563534430f64
                    + (-1.83469277483613086f64
                        + (0.784932344976639262f64 - 0.145263899385486377f64 * xm) * xm)
                        * xm)
                    * xm)
                * xm)
            * xm;

    // One Halley step, then undo the part of the exponent not divisible by 3.
    let t2 = u * u * u;
    let r = xe % 3i32;
    let fac = select(
        r == -2i32,
        CBRT_F_M2,
        select(
            r == -1i32,
            CBRT_F_M1,
            select(r == 0i32, 1.0f64, select(r == 1i32, CBRT_F_P1, CBRT_F_P2)),
        ),
    );
    let ym = u * (t2 + 2.0f64 * xm) / (2.0f64 * t2 + xm) * fac;

    // ldexp(ym, xe / 3). `xe / 3` lands in [-358, 341], so the exponent field
    // is always in range and the power of two is exact.
    let n = xe / 3i32;
    let pow2 = f64::reinterpret(u64::cast_from(n + 1023i32) << 52u64);
    let y0 = ym * pow2;

    // Newton polish. `fma` keeps the residual y^3 - a accurate; computing it as
    // y*y*y - a would lose every significant digit to cancellation.
    let t = y0 * y0;
    let err = fma(t, y0, -a);
    let y = y0 - err / (3.0f64 * t);

    let signed = select(x < 0.0f64, -y, y);
    // 0, +-0, +-inf and NaN all fall through correctly as x + x.
    let degenerate = (x == 0.0f64) || f64::is_nan(x) || f64::is_inf(x);
    select(degenerate, x + x, signed)
}

/// Compute the cube root of x, correctly handling negative values.
///
/// Maps to libxc's `CBRT(x)` / `POW_1_3(x)`, i.e. C's `cbrt`. The reduction is
/// carried out in `f64` regardless of `F`; for `F = f64` (every kernel today)
/// the casts are exact no-ops, and for a narrower `F` computing in `f64` and
/// rounding once is strictly more accurate than reducing in `F`.
#[cube]
pub fn safe_cbrt<F: Float>(x: F) -> F {
    F::cast_from(cbrt_f64(f64::cast_from(x)))
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

    /// ulp distance between two f64 values.
    fn ulp_diff(a: f64, b: f64) -> i64 {
        if a == b {
            return 0;
        }
        ((a.to_bits() as i64) - (b.to_bits() as i64)).abs()
    }

    /// `safe_cbrt` must agree with libm's `cbrt` to within 1 ulp across the
    /// density range a DFT grid actually spans -- roughly 13 decades. The old
    /// `powf(x, 1.0/3.0)` implementation failed this at up to 4 ulp.
    #[test]
    fn test_safe_cbrt_within_one_ulp_of_libm() {
        let n = 20_000usize;
        let inputs: Vec<f64> = (0..n)
            .map(|i| {
                let t = i as f64 / (n - 1) as f64;
                10f64.powf(-10.0 + 13.0 * t)
            })
            .collect();

        let results = run_cbrt(&inputs);

        let mut worst = 0i64;
        let mut worst_x = 0.0f64;
        for (&result, &x) in results.iter().zip(inputs.iter()) {
            let d = ulp_diff(result, libm::cbrt(x));
            if d > worst {
                worst = d;
                worst_x = x;
            }
        }
        assert!(
            worst <= 1,
            "safe_cbrt deviates from libm::cbrt by {worst} ulp (worst input {worst_x:e}); \
             expected <= 1 ulp"
        );
    }

    /// The cube root of an exact cube must be exact. `powf(x, 1.0/3.0)` got
    /// this wrong -- e.g. it returned 999.9999999999997 for cbrt(1e9).
    #[test]
    fn test_safe_cbrt_exact_on_exact_cubes() {
        let inputs: Vec<f64> = vec![
            1.0, 8.0, 27.0, 64.0, 125.0, 216.0, 1000.0, 1e9, 1e-9, 1e12, -8.0, -27.0, -1000.0,
        ];
        let results = run_cbrt(&inputs);
        let expected: Vec<f64> = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 10.0, 1000.0, 0.001, 10000.0, -2.0, -3.0, -10.0,
        ];
        for ((&got, &want), &x) in results.iter().zip(expected.iter()).zip(inputs.iter()) {
            assert_eq!(
                got, want,
                "safe_cbrt({x}) = {got}, expected exactly {want}"
            );
        }
    }

    /// 0, +-0, +-inf and NaN must pass through as C's `cbrt` does.
    #[test]
    fn test_safe_cbrt_degenerate_inputs() {
        let inputs = [0.0f64, -0.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN];
        let results = run_cbrt(&inputs);
        assert_eq!(results[0].to_bits(), 0.0f64.to_bits(), "cbrt(0) must be +0");
        assert_eq!(results[1].to_bits(), (-0.0f64).to_bits(), "cbrt(-0) must be -0");
        assert!(results[2].is_infinite() && results[2] > 0.0, "cbrt(+inf) must be +inf");
        assert!(results[3].is_infinite() && results[3] < 0.0, "cbrt(-inf) must be -inf");
        assert!(results[4].is_nan(), "cbrt(NaN) must be NaN");
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
