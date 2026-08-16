//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 757/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk757(t7174: f64, t943: f64, t5230: f64, t883: f64, t732: f64, t2553: f64, t2060: f64, t2558: f64, t2559: f64, t731: f64, t2547: f64, t481: f64, t685: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7175 = t943 * t7174;
    let t7177 = t883 * t5230;
    let t7178 = t732 * t7177;
    let t7179 = t2553 * t7178;
    let t7181 = t2060 * t2558;
    let t7182 = t943 * t7181;
    let t7184 = t731 * t2559;
    let t7187 = t481 * t2547 * t685;
    (t7175, t7177, t7179, t7182, t7184, t7187)
}
