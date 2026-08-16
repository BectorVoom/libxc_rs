//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1221/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1221(t251: f64, t281: f64, t93238: f64, t1950: f64, t2769: f64, t786: f64, t25404: f64, t40270: f64, t10115: f64, t1951: f64, t7058: f64, t92871: f64) -> (f64, f64, f64, f64, f64) {
    let t93240 = t281 * t93238 * t251;
    let t93261 = t786 * t1950 * t2769;
    let t93272 = 0.96373646535613327356e-3_f64 * t40270 * t25404;
    let t93276 = 0.11044544084478153697e-3_f64 * t10115 * t1951;
    let t93278 = 0.22487184191643109717e-1_f64 * t7058 * t92871;
    (t93240, t93261, t93272, t93276, t93278)
}
