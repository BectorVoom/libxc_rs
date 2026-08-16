//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1256/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1256(t136: f64, t2457: f64, t7778: f64, t25299: f64, t1568: f64, t786: f64, t25410: f64, t25375: f64, t99365: f64, t10073: f64, t1579: f64, t1958: f64, t25390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99412 = t25375 * t99365;
    let t99423 = t10073 * t25390 * t1958 * t1579;
    (t99380, t99381, t99403, t99404, t99412, t99423)
}
