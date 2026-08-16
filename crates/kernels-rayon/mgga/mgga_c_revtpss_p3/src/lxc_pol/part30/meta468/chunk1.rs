//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1776/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1776(t25309: f64, t786: f64, t7060: f64, t233: f64, t25286: f64, t1957: f64, t11007: f64) -> (f64, f64, f64, f64, f64) {
    let t25310 = t786 * t25309;
    let t25311 = t25310 * t7060;
    let t25313 = t233 * t25286;
    let t25314 = t1957 * t25313;
    let t25317 = t11007 * t233;
    (t25310, t25311, t25313, t25314, t25317)
}
