//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 966/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk966(t17697: f64, t438: f64, t935: f64, t450: f64, t8915: f64, t3107: f64, t16236: f64, t8511: f64, t894: f64, t1136: f64, t16241: f64, t8951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17699 = t17697 * t935 * t438;
    let t17700 = t450 * t17699;
    let t17704 = t17697 * t8915 * t935;
    let t17705 = t450 * t17704;
    let t17709 = t17697 * t3107 * t935;
    let t17710 = t450 * t17709;
    let t17713 = t8511 * t16236;
    let t17714 = t894 * t17713;
    let t17719 = t1136 * t16241;
    let t17720 = t894 * t17719;
    let t17723 = t8951 * t16236;
    (t17699, t17700, t17704, t17705, t17709, t17710, t17713, t17714, t17719, t17720, t17723)
}
