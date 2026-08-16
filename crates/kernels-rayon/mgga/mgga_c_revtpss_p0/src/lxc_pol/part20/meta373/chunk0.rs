//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1354/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354(t2760: f64, t2783: f64, t786: f64, t2801: f64, t10069: f64, t10920: f64, t231: f64, t2782: f64, t39709: f64, t10910: f64, t233: f64, t689: f64, t869: f64) -> (f64, f64, f64, f64) {
    let t40297 = t786 * t2783 * t2760;
    let t40298 = t40297 * t2801;
    let t40303 = t10069 * t10920;
    let t40307 = t2782 * t2783 * t39709 * t231;
    let t40311 = t689 * t869 * t233 * t10910;
    (t40298, t40303, t40307, t40311)
}
