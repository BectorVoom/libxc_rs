//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2895/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2895(t41267: f64, t41275: f64, t41672: f64, t51921: f64, t51923: f64, t51927: f64, t51932: f64, t51935: f64, t51937: f64, t51940: f64, t51942: f64, t51945: f64) -> f64 {
    let t52562 = 0.69463333333333333334e-1_f64 * t51921 + 0.92617777777777777778e-1_f64 * t51923 - 0.104195e0_f64 * t51927 - 0.13892666666666666667e0_f64 * t51932 - 0.34731666666666666667e-1_f64 * t51935 - 0.41678000000000000001e0_f64 * t51937 - 0.125034e1_f64 * t51940 + 0.125034e1_f64 * t51942 + 0.250068e1_f64 * t51945 + t41672 - 0.41678000000000000001e0_f64 * t41267 + 0.41678000000000000001e0_f64 * t41275;
    t52562
}
