//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 749/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk749(t140: f64, t6916: f64, t1261: f64, t6956: f64, t1281: f64, t2105: f64, t6917: f64, t2029: f64, t3500: f64, t1278: f64, t7061: f64, t1291: f64, t7073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9771 = t6916 * t140;
    let t9782 = t6956 * t1261;
    let t9794 = t1281 * t2105;
    let t9839 = t6917 * t140;
    let t9896 = t3500 * t2029;
    let t9913 = t7061 * t1278;
    let t9915 = t7073 * t1291;
    (t9771, t9782, t9794, t9839, t9896, t9913, t9915)
}
