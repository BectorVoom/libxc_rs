//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1040/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1040(t694: f64, t8379: f64, t1674: f64, t8373: f64, t10409: f64, t1679: f64, t560: f64, t2354: f64, t469: f64, t301: f64, t11883: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36575 = 6.0_f64 * t694 * t8379;
    let t36592 = 12.0_f64 * t1674 * t8373;
    let t36601 = 2.0_f64 * t1679 * t10409 * t560;
    let t36602 = t2354 * t469;
    let t36605 = 6.0_f64 * t694 * t36602 * t301;
    let t36610 = t624 * t11883;
    (t36575, t36592, t36601, t36602, t36605, t36610)
}
