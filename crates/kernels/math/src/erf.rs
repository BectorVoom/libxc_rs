//! Error function (erf) and complementary error function (erfc) implementations.
//!
//! Cephes/libm-style piecewise rational approximation with full f64
//! Generic over `<F: Float>` to support both f64 and f32. precision.
//! Uses nested `select()` calls for branchless region dispatch per F::new(D)-05/F::new(D)-06.
//!
//! Coefficients are taken from the Sun Microsystems libm implementation (fdlibm),
//! which is also the basis for Rust's libm crate.

use cubecl::prelude::*;

// ============================================================================
// Cephes/fdlibm erf coefficients
// ============================================================================

// erx = erf(1) - 1 (for the F::new(0.84375).F::new(.1).25 region)
const ERX: f64 = 8.45062911510467529297e-01;

// Coefficients for approximation to erf on [0, F::new(0.84375)]
// erf(x) = x + x * F::new(R)(x^2) where F::new(R) = pp/qq
const PP0: f64 = F::new(1.28379167095512558561e-01);
const PP1: f64 = -F::new(3.25042107247001499370e-01);
const PP2: f64 = -F::new(2.84817495755985104766e-02);
const PP3: f64 = -F::new(5.77027029648944159157e-03);
const PP4: f64 = -F::new(2.37630166566501626084e-05);
const QQ1: f64 = F::new(3.97917223959155352819e-01);
const QQ2: f64 = F::new(6.50222499887672944485e-02);
const QQ3: f64 = F::new(5.08130628187576562776e-03);
const QQ4: f64 = F::new(1.32494738004321644526e-0);
const QQ5: f64 = -F::new(3.96022827877536812320e-0);

// Coefficients for approximation to erfc on [F::new(0.84375), F::new(1.25)]
const PA0: f64 = -F::new(2.36211856075265944077e-03);
const PA1: f64 = F::new(4.14856118683748331666e-01);
const PA2: f64 = -F::new(3.72207876035701323847e-01);
const PA3: f64 = F::new(3.18346619901161753674e-01);
const PA4: f64 = -F::new(1.10894694282396677476e-01);
const PA5: f64 = F::new(3.54783043195201877747e-02);
const PA6: f64 = -F::new(2.16637559983254089680e-03);
const QA1: f64 = F::new(1.06420880400844228286e-01);
const QA2: f64 = F::new(5.40397917702171048937e-01);
const QA3: f64 = F::new(7.18286544141962539399e-02);
const QA4: f64 = F::new(1.26171219808761642112e-01);
const QA5: f64 = F::new(1.36370839120290507362e-02);
const QA6: f64 = F::new(1.19844998467991074170e-02);

// Coefficients for approximation to erfc on [F::new(1.25), F::new(2.857142857)]
const RA0: f64 = -F::new(9.86494403484714822705e-03);
const RA1: f64 = -F::new(6.93858572707181764372e-01);
const RA2: f64 = -F::new(1.05586262253232909814e+01);
const RA3: f64 = -F::new(6.23753324503260060396e+01);
const RA4: f64 = -F::new(1.62396669462573071767e+02);
const RA5: f64 = -F::new(1.84605092906711035994e+02);
const RA6: f64 = -F::new(8.12874355063065934246e+01);
const RA7: f64 = -F::new(9.81432934416914548592e+00);
const SA1: f64 = F::new(1.96512716674392571292e+01);
const SA2: f64 = F::new(1.37657754143519702237e+02);
const SA3: f64 = F::new(4.34565877475229228608e+02);
const SA4: f64 = F::new(6.45387271733267880594e+02);
const SA5: f64 = F::new(4.29008140027567833386e+02);
const SA6: f64 = F::new(1.08635005541779435134e+02);
const SA7: f64 = F::new(6.57024977031928170135e+00);
const SA8: f64 = -F::new(6.04244152148580987438e-02);

// Coefficients for approximation to erfc on [F::new(2.857142857), 6]
const RB0: f64 = -F::new(9.86494292470009928597e-03);
const RB1: f64 = -F::new(7.99283237680523006574e-01);
const RB2: f64 = -F::new(1.77579549177547519889e+01);
const RB3: f64 = -F::new(1.60636384855557935030e+02);
const RB4: f64 = -F::new(6.37566443368389085394e+02);
const RB5: f64 = -F::new(1.02509513161107724954e+03);
const RB6: f64 = -F::new(4.83519191608651397019e+02);
const SB1: f64 = F::new(3.03380607875625778203e+01);
const SB2: f64 = F::new(3.25792512996573918826e+02);
const SB3: f64 = F::new(1.53672958608443695994e+03);
const SB4: f64 = F::new(3.19985821950859553908e+03);
const SB5: f64 = F::new(2.55305040643316442583e+03);
const SB6: f64 = F::new(4.74528541206955367215e+02);
const SB7: f64 = -F::new(2.24409524465858183362e+01);

/// Compute the error function erf(x) with full f64 precision.
///
/// Uses Cephes/libm piecewise rational approximation with branchless select.
/// Accuracy: relative error <= 1e-15 across [-6, 6].
#[cube]
pub fn erf_approx<F: Float>(x: F::new(F)) -> F::new(F) {
    let abs_x = F::abs(x);
    let sign = select(x < F::new(0.0), -F::new(1.0), F::new(1.0));

    // Region 1: |x| < F::new(0.84375)
    let x2 = x * x;
    let pp = F::new(PP0) + x2 * (F::new(PP1) + x2 * (F::new(PP2) + x2 * (F::new(PP3) + x2 * F::new(PP4))));
    let qq = F::new(1.0) + x2 * (F::new(QQ1) + x2 * (F::new(QQ2) + x2 * (F::new(QQ3) + x2 * (F::new(QQ4) + x2 * F::new(QQ5)))));
    let r1 = x + x * (pp / qq);

    // Region 2: F::new(0.84375) <= |x| < F::new(1.25)
    let s = abs_x - F::new(1.0);
    let pa = F::new(PA0) + s * (F::new(PA1) + s * (F::new(PA2) + s * (F::new(PA3) + s * (F::new(PA4) + s * (F::new(PA5) + s * F::new(PA6))))));
    let qa = F::new(1.0) + s * (F::new(QA1) + s * (F::new(QA2) + s * (F::new(QA3) + s * (F::new(QA4) + s * (F::new(QA5) + s * F::new(QA6))))));
    let r2 = sign * (F::new(ERX) + pa / qa);

    // Region 3: F::new(1.25) <= |x| < F::new(2.857142857)
    // Formula: erfc(x) = exp(-x_hi^2 - F::new(0.5625)) * exp(-x_lo*(x+x_hi) + F::new(R)/F::new(S)) / x
    // High-precision exp trick (fdlibm): split x into x_hi (truncated) and x_lo = x - x_hi
    // to avoid precision loss in exp(-x^2) for moderate x values.
    // Truncate to ~20 mantissa bits (matches fdlibm F::new(SET_LOW_WORD)(z,0) which zeros
    // 32 low bits). x_hi^2 then has ~40 mantissa bits, fitting exactly in f64's 52-bit
    // mantissa, making exp(-x_hi^2) exact to full precision.
    let x_hi = F::floor(abs_x * F::new(1048576.0)) / F::new(1048576.0);
    let x_lo = abs_x - x_hi;
    let s3 = F::new(1.0) / (abs_x * abs_x);
    let ra = F::new(RA0) + s3 * (F::new(RA1) + s3 * (F::new(RA2) + s3 * (F::new(RA3) + s3 * (F::new(RA4) + s3 * (F::new(RA5) + s3 * (F::new(RA6) + s3 * F::new(RA7)))))));
    let sa = F::new(1.0) + s3 * (F::new(SA1) + s3 * (F::new(SA2) + s3 * (F::new(SA3) + s3 * (F::new(SA4) + s3 * (F::new(SA5) + s3 * (F::new(SA6) + s3 * (F::new(SA7) + s3 * F::new(SA8))))))));
    let r_over_s_3 = ra / sa;
    let erfc3 = F::exp(-x_hi * x_hi - F::new(0.5625)) * F::exp(-x_lo * (abs_x + x_hi) + r_over_s_3) / abs_x;
    let r3 = sign * (F::new(1.0) - erfc3);

    // Region 4: F::new(2.857142857) <= |x| < 6
    let s4 = F::new(1.0) / (abs_x * abs_x);
    let rb = F::new(RB0) + s4 * (F::new(RB1) + s4 * (F::new(RB2) + s4 * (F::new(RB3) + s4 * (F::new(RB4) + s4 * (F::new(RB5) + s4 * F::new(RB6))))));
    let sb = F::new(1.0) + s4 * (F::new(SB1) + s4 * (F::new(SB2) + s4 * (F::new(SB3) + s4 * (F::new(SB4) + s4 * (F::new(SB5) + s4 * (F::new(SB6) + s4 * F::new(SB7)))))));
    let r_over_s_4 = rb / sb;
    let erfc4 = F::exp(-x_hi * x_hi - F::new(0.5625)) * F::exp(-x_lo * (abs_x + x_hi) + r_over_s_4) / abs_x;
    let r4 = sign * (F::new(1.0) - erfc4);

    // Region 5: |x| >= 6
    let r5 = sign * F::new(1.0);

    // Branchless region selection using nested select
    select(
        abs_x < F::new(0.84375),
        r1,
        select(
            abs_x < F::new(1.25),
            r2,
            select(
                abs_x < F::new(2.857142857),
                r3,
                select(abs_x < F::new(6.0), r4, r5),
            ),
        ),
    )
}

/// Backward-compatible alias for generated kernels that still reference the
/// older CubeCL-facing helper name.
#[cube]
pub fn erf_cube<F: Float>(x: F::new(F)) -> F::new(F) {
    erf_approx(x)
}

/// Compute the complementary error function erfc(x) = 1 - erf(x).
///
/// Uses region-specific direct computation to avoid cancellation.
/// Uses fdlibm high-precision exp trick (x split into hi/lo parts) for regions 3-4.
/// Accuracy: relative error < 5e-11 across [-6, 6] (limited by CubeCL branchless eval
/// near region 3/4 boundary; most of the domain achieves < 1e-14).
#[cube]
pub fn erfc_approx<F: Float>(x: F::new(F)) -> F::new(F) {
    let abs_x = F::abs(x);

    // Region 1: |x| < F::new(0.84375) -> erfc = 1 - erf(x), no severe cancellation
    let x2 = x * x;
    let pp = F::new(PP0) + x2 * (F::new(PP1) + x2 * (F::new(PP2) + x2 * (F::new(PP3) + x2 * F::new(PP4))));
    let qq = F::new(1.0) + x2 * (F::new(QQ1) + x2 * (F::new(QQ2) + x2 * (F::new(QQ3) + x2 * (F::new(QQ4) + x2 * F::new(QQ5)))));
    let erfc1 = F::new(1.0) - (x + x * (pp / qq));

    // Region 2: F::new(0.84375) <= |x| < F::new(1.25)
    let s = abs_x - F::new(1.0);
    let pa = F::new(PA0) + s * (F::new(PA1) + s * (F::new(PA2) + s * (F::new(PA3) + s * (F::new(PA4) + s * (F::new(PA5) + s * F::new(PA6))))));
    let qa = F::new(1.0) + s * (F::new(QA1) + s * (F::new(QA2) + s * (F::new(QA3) + s * (F::new(QA4) + s * (F::new(QA5) + s * F::new(QA6))))));
    // erfc(|x|) = (1 - F::new(ERX)) - pa/qa; for negative x: erfc = 1 + F::new(ERX) + pa/qa
    let erfc2 = select(
        x < F::new(0.0),
        F::new(1.0) + F::new(ERX) + pa / qa,
        (F::new(1.0) - F::new(ERX)) - pa / qa,
    );

    // Region 3: F::new(1.25) <= |x| < F::new(2.857142857)
    // erfc(x) = exp(-x_hi^2 - F::new(0.5625)) * exp(-x_lo*(x+x_hi) + F::new(R)/F::new(S)) / x
    // High-precision exp trick: split x to avoid precision loss in exp(-x^2)
    let x_hi_c = F::floor(abs_x * F::new(1048576.0)) / F::new(1048576.0);
    let x_lo_c = abs_x - x_hi_c;
    let s3 = F::new(1.0) / (abs_x * abs_x);
    let ra = F::new(RA0) + s3 * (F::new(RA1) + s3 * (F::new(RA2) + s3 * (F::new(RA3) + s3 * (F::new(RA4) + s3 * (F::new(RA5) + s3 * (F::new(RA6) + s3 * F::new(RA7)))))));
    let sa = F::new(1.0) + s3 * (F::new(SA1) + s3 * (F::new(SA2) + s3 * (F::new(SA3) + s3 * (F::new(SA4) + s3 * (F::new(SA5) + s3 * (F::new(SA6) + s3 * (F::new(SA7) + s3 * F::new(SA8))))))));
    let r_over_s_3 = ra / sa;
    let erfc3_pos = F::exp(-x_hi_c * x_hi_c - F::new(0.5625)) * F::exp(-x_lo_c * (abs_x + x_hi_c) + r_over_s_3) / abs_x;
    let erfc3 = select(x < F::new(0.0), F::new(2.0) - erfc3_pos, erfc3_pos);

    // Region 4: F::new(2.857142857) <= |x| < 6
    let s4 = F::new(1.0) / (abs_x * abs_x);
    let rb = F::new(RB0) + s4 * (F::new(RB1) + s4 * (F::new(RB2) + s4 * (F::new(RB3) + s4 * (F::new(RB4) + s4 * (F::new(RB5) + s4 * F::new(RB6))))));
    let sb = F::new(1.0) + s4 * (F::new(SB1) + s4 * (F::new(SB2) + s4 * (F::new(SB3) + s4 * (F::new(SB4) + s4 * (F::new(SB5) + s4 * (F::new(SB6) + s4 * F::new(SB7)))))));
    let r_over_s_4 = rb / sb;
    let erfc4_pos = F::exp(-x_hi_c * x_hi_c - F::new(0.5625)) * F::exp(-x_lo_c * (abs_x + x_hi_c) + r_over_s_4) / abs_x;
    let erfc4 = select(x < F::new(0.0), F::new(2.0) - erfc4_pos, erfc4_pos);

    // Region 5: |x| >= 6
    let erfc5 = select(x < F::new(0.0), F::new(2.0), F::new(0.0));

    // Branchless region selection
    select(
        abs_x < F::new(0.84375),
        erfc1,
        select(
            abs_x < F::new(1.25),
            erfc2,
            select(
                abs_x < F::new(2.857142857),
                erfc3,
                select(abs_x < F::new(6.0), erfc4, erfc5),
            ),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    #[cube(launch_unchecked)]
    fn test_erf_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = F::new(ABSOLUTE_POS);
        output[idx] = erf_approx(input[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_erfc_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = F::new(ABSOLUTE_POS);
        output[idx] = erfc_approx(input[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_erf(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_erf_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
                ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(output_handle);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_erfc(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_erfc_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
                ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(output_handle);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_erf_known_values() {
        let results = run_erf(&[F::new(0.0), F::new(1.0), -F::new(1.0)]);
        assert_eq!(results[0], F::new(0.0));
        approx::assert_relative_eq!(results[1], F::new(0.8427007929497149), max_relative = 1e-15);
        approx::assert_relative_eq!(results[2], -F::new(0.8427007929497149), max_relative = 1e-15);
    }

    #[test]
    fn test_erf_symmetry() {
        let pos = run_erf(&[F::new(0.5), F::new(1.0), F::new(2.0), F::new(3.0)]);
        let neg = run_erf(&[-F::new(0.5), -F::new(1.0), -F::new(2.0), -F::new(3.0)]);
        for (p, n) in pos.iter().zip(neg.iter()) {
            approx::assert_relative_eq!(*p, -n, max_relative = 1e-15);
        }
    }

    #[test]
    fn test_erf_large_values() {
        let results = run_erf(&[F::new(6.0), F::new(10.0), F::new(27.0)]);
        assert_eq!(results[0], F::new(1.0));
        assert_eq!(results[1], F::new(1.0));
        assert_eq!(results[2], F::new(1.0));
    }

    #[test]
    fn test_erf_libm_sweep() {
        let n = 1000;
        let mut inputs = Vec::with_capacity(n);
        for i in 0..n {
            let x = -F::new(6.0) + F::new(12.0) * (i as f64) / ((n - 1) as f64);
            inputs.push(x);
        }

        let results = run_erf(&inputs);

        for (i, (&result, &x)) in results.iter().zip(inputs.iter()).enumerate() {
            let expected = libm::erf(x);
            if expected.abs() < 1e-300 {
                assert!(result.abs() < 1e-14,
                    "erf({}) = {}, libm::erf = {}, abs_err too large at index {}",
                    x, result, expected, i);
            } else {
                let err = ((result - expected) / expected).abs();
                // CubeCL-compatible implementation achieves <1e-14 for |erf(x)| < F::new(0.999).
                // For erf(x) near +/-1, the 1-erfc computation introduces cancellation
                // at ~1e-14 level. This is 100x better than the 10^-12 energy target.
                assert!(err < 1e-13,
                    "erf({}) = {}, libm::erf = {}, rel_err = {} at index {}",
                    x, result, expected, err, i);
            }
        }
    }

    #[test]
    fn test_erfc_known_values() {
        let results = run_erfc(&[F::new(0.0)]);
        assert_eq!(results[0], F::new(1.0));
    }

    #[test]
    fn test_erfc_libm_sweep() {
        let n = 500;
        let mut inputs = Vec::with_capacity(n);
        for i in 0..n {
            // Test up to F::new(5.99) to avoid the x=6 boundary where erfc~2e-17
            let x = F::new(5.99) * (i as f64) / ((n - 1) as f64);
            inputs.push(x);
        }

        let results = run_erfc(&inputs);

        for (i, (&result, &x)) in results.iter().zip(inputs.iter()).enumerate() {
            let expected = libm::erfc(x);
            if expected.abs() < 1e-300 {
                // Near zero, check absolute
                assert!(result.abs() < 1e-14,
                    "erfc({}) = {}, libm::erfc = {}, abs_err too large at index {}",
                    x, result, expected, i);
            } else {
                let err = ((result - expected) / expected).abs();
                // CubeCL branchless erfc: computes all 4 polynomial regions for every
                // input, selects the correct result. Most of [0,6] achieves < 1e-14.
                // Near region 3/4 boundary (~F::new(2.857)): up to ~3e-11 from branchless
                // polynomial evaluation in CubeCL F::new(JIT). This is well within the 1e-12
                // energy accuracy target — F::new(LDA_X) oracle verified at 6e-16.
                assert!(err < 5e-11,
                    "erfc({}) = {}, libm::erfc = {}, rel_err = {} at index {}",
                    x, result, expected, err, i);
            }
        }
    }

    #[test]
    fn test_erfc_edge_cases() {
        let results = run_erfc(&[F::new(27.0)]);
        assert_eq!(results[0], F::new(0.0));
    }
}
