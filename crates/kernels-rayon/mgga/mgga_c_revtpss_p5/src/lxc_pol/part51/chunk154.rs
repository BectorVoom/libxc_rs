//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 154/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk154(t25: f64, t596: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t88: f64, t90: f64, t29: f64) -> (f64, f64, f64, f64) {
    let t598 = 6.0_f64 * t25 * t596;
    let t599 = t578 - t582 + t586 - t590 + t594 - t598;
    let t602 = 1.0_f64 / t90 / t88;
    let t603 = t29 * t602;
    (t598, t599, t602, t603)
}
