//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2919/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919(t11223: f64, t379: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t42013: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> (f64, f64) {
    let t52927 = t11223 * t379;
    let t52946 = 0.11111111111111111111e-1_f64 * t51973;
    let t52954 = 0.2e0_f64 * t51849 - 0.55555555555555555555e-2_f64 * t51853 - 0.24691358024691358025e-1_f64 * t51858 + 0.50000000000000000001e-1_f64 * t51863 + 0.50000000000000000001e-1_f64 * t51867 + 0.16666666666666666667e-1_f64 * t51871 - 0.19999999999999999999e0_f64 * t51875 + t42013 + 0.99999999999999999999e-1_f64 * t51961 - 0.27777777777777777777e-1_f64 * t51965 + 0.83333333333333333334e-2_f64 * t51967 - 0.83333333333333333333e-2_f64 * t51971 - t52946 + 0.86419753086419753086e-2_f64 * t51978 + 0.25925925925925925926e-1_f64 * t41361 + 0.22222222222222222222e-1_f64 * t41363 - 0.11111111111111111111e-1_f64 * t41369 + 0.1e0_f64 * t52028 + 0.11111111111111111111e0_f64 * t52031 + 0.5e-1_f64 * t52033;
    (t52927, t52954)
}
