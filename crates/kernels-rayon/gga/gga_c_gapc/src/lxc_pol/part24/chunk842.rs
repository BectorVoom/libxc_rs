//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 842/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk842(t10067: f64, t2801: f64, t6: f64, t3405: f64, t3411: f64, t3414: f64, t9722: f64, t1084: f64, t8711: f64, t134: f64, t7877: f64, t442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10068 = t2801 * t6 * t10067;
    let t10069 = t3405 * t10068;
    let t10070 = t3411 * t10069;
    let t10072 = t9722 * t3414;
    let t10073 = t3411 * t10072;
    let t10075 = t1084 * t8711;
    let t10077 = t134 * t7877;
    let t10078 = t10077 * t442;
    (t10069, t10070, t10072, t10073, t10075, t10078)
}
