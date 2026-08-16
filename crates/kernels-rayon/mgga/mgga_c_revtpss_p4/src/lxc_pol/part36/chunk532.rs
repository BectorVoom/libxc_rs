//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 532/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk532(t1469: f64, t83: f64, t1544: f64, t221: f64, t2675: f64, t2674: f64, t1558: f64, t243: f64, t231: f64) -> (f64, f64, f64, f64) {
    let t4335 = t83 * t1469;
    let t4349 = t2675 * t221 * t1544;
    let t4350 = t2674 * t4349;
    let t4352 = t243 * t1558;
    let t4353 = t4352 * t231;
    (t4335, t4349, t4350, t4353)
}
