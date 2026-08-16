//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 417/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk417(t1225: f64, t606: f64, t1012: f64, t1204: f64, t225: f64, t480: f64, t1209: f64) -> (f64, f64, f64, f64, f64) {
    let t1226 = t1225 * t606;
    let t1227 = t1012 * t1226;
    let t1230 = t1204 * t225;
    let t1231 = t1230 * t480;
    let t1234 = t1209 * t225;
    (t1226, t1227, t1230, t1231, t1234)
}
