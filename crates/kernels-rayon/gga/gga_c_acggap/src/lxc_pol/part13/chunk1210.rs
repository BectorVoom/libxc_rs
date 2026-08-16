//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1210/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1210(t11883: f64, t624: f64, t560: f64, t811: f64, t10956: f64, t1679: f64, t467: f64, t9099: f64, t1680: f64, t2166: f64, t29953: f64, t29955: f64, t29958: f64, t36587: f64, t36592: f64, t36593: f64, t36601: f64, t36605: f64, t567: f64, t7288: f64, t8021: f64, t9082: f64, t9096: f64, t9098: f64) -> f64 {
    let t36610 = t624 * t11883;
    let t36611 = t560 * t811;
    let t36617 = 2.0_f64 * t1679 * t10956 * t467;
    let t36619 = 4.0_f64 * t1679 * t9099;
    let t36620 = -t1680 * t567 * t8021 - 2.0_f64 * t2166 * t567 * t9082 + 4.0_f64 * t36587 * t9096 * t9098 + 6.0_f64 * t36593 * t567 * t7288 - 6.0_f64 * t36610 * t36611 * t9096 - t29953 + t29955 + 6.0_f64 * t29958 + t36592 - t36601 + t36605 - t36617 + t36619;
    t36620
}
