//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 144/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk144(t221: f64, t462: f64, t65: f64, t225: f64, t460: f64, t355: f64, t424: f64, t452: f64, t454: f64) -> (f64, f64, f64) {
    let t464 = t221 * t65 * t462;
    let t467 = t460 * t225;
    let t471 = f64::exp(-(-t424 + t452 + t454) * t225 * t355);
    (t464, t467, t471)
}
