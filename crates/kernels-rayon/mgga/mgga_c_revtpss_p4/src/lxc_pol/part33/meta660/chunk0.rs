//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2137/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137(t18657: f64, t1955: f64, t18797: f64, t25399: f64, t1579: f64, t231: f64, t4423: f64, t1580: f64, t27194: f64, t689: f64, t29690: f64, t25411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106404 = t1955 * t18657;
    let t106407 = t25399 * t18797;
    let t106410 = t1579 * t4423 * t231;
    let t106423 = t689 * t27194 * t1580;
    let t106430 = t29690 * t689;
    let t106431 = t25411 * t106430;
    (t106404, t106407, t106410, t106423, t106430, t106431)
}
