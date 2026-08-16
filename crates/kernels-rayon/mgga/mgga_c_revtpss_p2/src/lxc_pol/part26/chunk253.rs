//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 253/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk253(t233: f64, t251: f64, t869: f64, t689: f64, t234: f64, t786: f64) -> (f64, f64, f64, f64) {
    let t870 = t233 * t251;
    let t871 = t869 * t870;
    let t873 = 0.54878743191129263322e-2_f64 * t689 * t871;
    let t874 = t786 * t234;
    (t870, t871, t873, t874)
}
