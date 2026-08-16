//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 718/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk718(t7038: f64, t839: f64, t1946: f64, t846: f64, t233: f64, t64: f64) -> (f64, f64, f64) {
    let t7039 = t7038 * t839;
    let t7041 = t1946 * t846;
    let t7042 = 0.20007875121765877254e-2_f64 * t7041;
    let t7043 = t233 * t64;
    (t7039, t7042, t7043)
}
