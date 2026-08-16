//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 835/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk835(t467: f64, t560: f64, t9097: f64, t182: f64, t310: f64, t129: f64, t5: f64, t2162: f64, t814: f64, t2354: f64, t813: f64, t1077: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9098 = t560 * t467;
    let t9099 = t9097 * t9098;
    let t10098 = t310 * t182;
    let t10146 = t129 * t5;
    let t10409 = t814 * t2162;
    let t10956 = t814 * t2354;
    let t11882 = t813 * t813;
    let t11883 = 1.0_f64 / t11882;
    let t12473 = t435 * t1077;
    (t9098, t9099, t10098, t10146, t10409, t10956, t11883, t12473)
}
