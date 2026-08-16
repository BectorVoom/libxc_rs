//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 958/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk958(t11677: f64, t14881: f64, t14883: f64, t14895: f64, t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t17412: f64, t17597: f64) -> f64 {
    let t17609 = -0.80768518518518518518e3_f64 * t17338 - 0.72691666666666666667e3_f64 * t17358 + 0.43614999999999999999e4_f64 * t17354 + 0.29076666666666666666e4_f64 * t17342 - 0.14538333333333333333e4_f64 * t17346 - 0.43614999999999999999e4_f64 * t17350 - 0.34962962962962962963e2_f64 * t17412 - 0.26222222222222222223e3_f64 * t11677 + 0.52444444444444444444e2_f64 * t14895 - 0.31466666666666666667e3_f64 * t14881 + 0.15733333333333333334e3_f64 * t14883;
    let t17610 = t17597 + t17609;
    t17610
}
