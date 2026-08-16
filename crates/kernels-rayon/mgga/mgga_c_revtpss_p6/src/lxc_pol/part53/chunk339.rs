//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 339/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk339(t1493: f64, t77: f64, t1471: f64, t1487: f64, t71: f64, t85: f64) -> (f64, f64) {
    let t1494 = t77 * t1493;
    let t1497 = -t1471 * t85 / 12.0_f64 + t1487 * t85 / 24.0_f64 + t71 * t1494 / 24.0_f64;
    (t1494, t1497)
}
