//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 853/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk853(t3328: f64, t7115: f64, t9927: f64, t9926: f64, t3402: f64, t9253: f64, t1038: f64, t8140: f64, t3787: f64) -> (f64, f64, f64, f64, f64) {
    let t9928 = t7115 * t3328;
    let t9929 = t9927 * t9928;
    let t9930 = t9926 * t9929;
    let t9932 = t3402 * t9253;
    let t9933 = t1038 * t8140;
    let t9934 = t3787 * t9933;
    (t9929, t9930, t9932, t9933, t9934)
}
