//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 757/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk757(t1378: f64, t7274: f64, t930: f64, t11073: f64, t953: f64, t322: f64, t3882: f64, t3881: f64, t1382: f64, t864: f64, t116: f64, t2718: f64, t2719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11191 = t7274 * t1378;
    let t11192 = t930 * t11191;
    let t11199 = t953 * t11073;
    let t11325 = t3882 * t322;
    let t11326 = t3881 * t11325;
    let t11327 = t864 * t1382;
    let t11368 = t2718 * t2719 * t116;
    (t11192, t11199, t11325, t11326, t11327, t11368)
}
