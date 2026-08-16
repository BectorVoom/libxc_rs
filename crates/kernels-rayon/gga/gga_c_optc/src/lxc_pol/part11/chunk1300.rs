//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1300/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1300(t57135: f64, t57148: f64, t57164: f64, t57179: f64, t828: f64, t837: f64, t845: f64, t39411: f64, t49385: f64, t49387: f64, t56966: f64, t56978: f64, t56981: f64, t56984: f64, t57024: f64, t57057: f64, t57060: f64, t57063: f64) -> (f64, f64, f64) {
    let t57181 = t57135 + t57148 + t57164 + t57179;
    let t57185 = 0.58482233974552040708e0_f64 * t845 * t828 * t57181 * t837;
    let t57197 = -0.92708333333333333333e-2_f64 * t57057 + 0.2225e0_f64 * t57060 - 0.33375e0_f64 * t56978 + 0.55625000000000000001e-1_f64 * t57063 - 0.49444444444444444444e-1_f64 * t49385 + 0.74166666666666666668e-1_f64 * t49387 + 0.74166666666666666668e-1_f64 * t56981 - 0.24722222222222222222e-1_f64 * t56984 - 0.24722222222222222222e-1_f64 * t39411 - 0.22249999999999999999e0_f64 * t57024 + 0.22249999999999999999e0_f64 * t56966;
    (t57181, t57185, t57197)
}
