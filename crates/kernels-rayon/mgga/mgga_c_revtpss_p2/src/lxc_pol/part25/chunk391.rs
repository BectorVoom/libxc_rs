//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 391/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk391(t1426: f64, t225: f64, t545: f64, t555: f64, t869: f64, t689: f64, t546: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t1427 = t225 * t1426;
    let t1428 = t545 * t555;
    let t1429 = t869 * t1428;
    let t1431 = 0.54878743191129263322e-2_f64 * t689 * t1429;
    let t1432 = t786 * t546;
    (t1427, t1428, t1429, t1431, t1432)
}
