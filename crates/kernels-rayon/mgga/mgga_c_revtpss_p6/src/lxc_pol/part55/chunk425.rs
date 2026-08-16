//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 425/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk425(t2113: f64, t2115: f64, t572: f64, t573: f64, t55: f64, t61: f64, t68: f64, t72: f64) -> (f64, f64, f64) {
    let t2118 = t2113 * t573 + 3.0_f64 * t2115 * t572;
    let t2121 = t55 * t61 - t68;
    let t2122 = t2121 * t72;
    (t2118, t2121, t2122)
}
