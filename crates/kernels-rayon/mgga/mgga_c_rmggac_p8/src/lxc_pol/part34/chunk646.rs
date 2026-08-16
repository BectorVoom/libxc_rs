//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 646/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk646(t338: f64, t3814: f64, t1296: f64, t20: f64, t1318: f64, t21: f64, t41: f64, t1342: f64, t117: f64, t25809: f64, t128: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25877 = t3814 * t338;
    let t25987 = t1296 * t20;
    let t26004 = t1318 * t1318;
    let t26007 = t21 / t41 / t26004;
    let t26077 = t1342 * t1342;
    let t26078 = 1.0_f64 / t26077;
    let t26087 = t117 * t25809;
    let t26115 = t348 * t128;
    (t25877, t25987, t26004, t26007, t26078, t26087, t26115)
}
