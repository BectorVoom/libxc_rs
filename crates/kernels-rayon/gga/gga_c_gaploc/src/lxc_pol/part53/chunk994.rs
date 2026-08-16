//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 994/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk994(t2478: f64, t3695: f64, t6576: f64, t2482: f64, t9263: f64, t46850: f64, t4820: f64, t6824: f64, t107: f64, t47008: f64, t544: f64, t2375: f64) -> (f64, f64, f64, f64) {
    let t47829 = t6576 * t3695 * t2478;
    let t47832 = t9263 * t3695 * t2482;
    let t47835 = t6824 * t4820 * t46850;
    let t47838 = t544 * t47008 * t107;
    let t47839 = t47838 * t2375;
    (t47829, t47832, t47835, t47839)
}
