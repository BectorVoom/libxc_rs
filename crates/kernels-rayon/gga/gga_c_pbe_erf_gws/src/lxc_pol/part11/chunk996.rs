//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 996/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk996(t20948: f64, t3754: f64, t28195: f64, t3128: f64, t11412: f64, t904: f64, t3805: f64, t6616: f64, t3717: f64, t3808: f64, t11609: f64, t2306: f64, t360: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37377 = t20948 * t3754;
    let t37380 = t3128 * t28195;
    let t37396 = t904 * t11412;
    let t37507 = t3805 * t6616;
    let t37632 = t3717 * param_a_c;
    let t37645 = t904 * t3808;
    let t37701 = t2306 * t11609 * t360;
    (t37377, t37380, t37396, t37507, t37632, t37645, t37701)
}
