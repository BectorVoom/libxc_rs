//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2903/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2903(t41267: f64, t41275: f64, t41592: f64, t51921: f64, t51923: f64, t51927: f64, t51932: f64, t51935: f64, t51937: f64, t51940: f64, t51942: f64, t51945: f64) -> f64 {
    let t52690 = 0.54771111111111111111e-1_f64 * t51921 + 0.73028148148148148149e-1_f64 * t51923 - 0.82156666666666666668e-1_f64 * t51927 - 0.10954222222222222222e0_f64 * t51932 - 0.27385555555555555556e-1_f64 * t51935 - 0.32862666666666666667e0_f64 * t51937 - 0.98587999999999999998e0_f64 * t51940 + 0.98587999999999999998e0_f64 * t51942 + 0.197176e1_f64 * t51945 + t41592 - 0.32862666666666666666e0_f64 * t41267 + 0.32862666666666666666e0_f64 * t41275;
    t52690
}
