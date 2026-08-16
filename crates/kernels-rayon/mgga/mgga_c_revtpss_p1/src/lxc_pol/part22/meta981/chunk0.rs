//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3311/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3311(t5977: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t18657: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t18750: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t62760 = t860 * t5977;
    let t62763 = t2782 * t2783 * t62760 * t231;
    let t62775 = t689 * t869 * t233 * t18657;
    let t62777 = t10069 * t18750;
    let t62788 = t822 * t18657;
    (t62760, t62763, t62775, t62777, t62788)
}
