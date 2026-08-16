//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 940/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk940(t13200: f64, t1841: f64, t13182: f64, t29439: f64, t13179: f64, t7137: f64, t1897: f64, t3270: f64, t8942: f64, t1022: f64, t3234: f64) -> (f64, f64, f64, f64, f64) {
    let t43098 = t1841 * t13200;
    let t43099 = 0.2563508743380741428e-2_f64 * t43098;
    let t43100 = t29439 * t13182;
    let t43101 = 0.64087718584518535698e-3_f64 * t43100;
    let t43102 = t7137 * t13179;
    let t43106 = 0.76905262301422242837e-2_f64 * t1897 * t3270 * t8942;
    let t43107 = t3234 * t1022;
    (t43099, t43101, t43102, t43106, t43107)
}
