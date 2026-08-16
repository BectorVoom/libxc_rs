//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 993/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk993(t209: f64, t6475: f64, t6481: f64, t6607: f64, t758: f64, t6529: f64, t1909: f64, t201: f64, t7159: f64, t9412: f64, t9416: f64, t559: f64, t6322: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21907 = 0.4274e0_f64 * t209 * t6481 * t6475;
    let t21911 = t6607 * t758;
    let t21913 = t6529 * t758;
    let t21915 = t1909 * t201;
    let t21920 = t9412 * t7159;
    let t21929 = t9416 * t7159;
    let t21931 = t6322 * t559;
    (t21907, t21911, t21913, t21915, t21920, t21929, t21931)
}
