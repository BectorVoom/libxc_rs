//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 791/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk791(t4818: f64, t7672: f64, t176: f64, t4848: f64, t998: f64, t4786: f64, t7512: f64, t7557: f64, t4895: f64, t778: f64, t2569: f64, t5053: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13900 = t4818 * t7672;
    let t13911 = t176 * t4848;
    let t13912 = t13911 * t998;
    let t13939 = t7512 * t4786;
    let t13947 = t7557 * t4786;
    let t13998 = t4895 * t778;
    let t14029 = t5053 * t2569;
    (t13900, t13912, t13939, t13947, t13998, t14029)
}
