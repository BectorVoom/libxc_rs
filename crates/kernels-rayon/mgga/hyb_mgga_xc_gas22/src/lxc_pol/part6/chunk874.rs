//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 874/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk874(t2639: f64, t2674: f64, t1095: f64, t1097: f64, t1096: f64, t7238: f64, t12: f64, t442: f64, t448: f64, t461: f64, t2655: f64, t6610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7323 = t2674 * t2639;
    let t7324 = t7323 * t1095;
    let t7327 = t1097 * t2674;
    let t7330 = t7238 * t1096;
    let t7336 = 1.0_f64 / t442 / t448 * t12 / 4.0_f64;
    let t7337 = t7336 * t461;
    let t7339 = t2655 * t6610;
    (t7323, t7324, t7327, t7330, t7336, t7337, t7339)
}
