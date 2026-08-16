//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 309/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk309(t475: f64, t470: f64, t1035: f64, t1037: f64, t1041: f64, t1044: f64) -> (f64, f64, f64, f64) {
    let t1073 = t475 * t475;
    let t1074 = 1.0_f64 / t1073;
    let t1075 = t470 * t1074;
    let t1080 = -0.1176575e1_f64 * t1035 - 0.516475e0_f64 * t1037 - 0.2103875e0_f64 * t1041 - 0.104195e0_f64 * t1044;
    (t1073, t1074, t1075, t1080)
}
