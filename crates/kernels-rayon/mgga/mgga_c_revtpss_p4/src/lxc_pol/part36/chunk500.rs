//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 500/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk500(t3596: f64, t474: f64, t3147: f64, t479: f64, t3594: f64, t471: f64) -> (f64, f64, f64, f64, f64) {
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    let t3599 = t3597 * t3598;
    let t3600 = t3594 * t3599;
    let t3603 = t471 * t471;
    (t3597, t3598, t3599, t3600, t3603)
}
