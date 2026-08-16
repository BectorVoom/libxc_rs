//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1066/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1066(t1382: f64, t24407: f64, t1388: f64, t3843: f64, t893: f64, t1384: f64, t7894: f64, t874: f64, t24447: f64, t25217: f64, t1397: f64, t3902: f64, t913: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31579 = t24407 * t1382;
    let t31718 = t3843 * t1388;
    let t31719 = t893 * t31718;
    let t31765 = t874 * t7894 * t1384;
    let t32008 = t24447 * t1382;
    let t32131 = t25217 * t1382;
    let t32252 = t913 * t3902 * t1397;
    (t31579, t31718, t31719, t31765, t32008, t32131, t32252)
}
