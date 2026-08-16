//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1237/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1237(t12987: f64, t2138: f64, t13036: f64, t13038: f64, t13040: f64, t26842: f64, t12808: f64, t29096: f64, t12898: f64, t2139: f64, t12851: f64, t2134: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97193 = t12987 * t2138;
    let t97211 = t13036 * t13038 * sigma2 * t13040;
    let t97215 = t13036 * t26842 * t13040;
    let t97261 = t12808 * t29096;
    let t97272 = 0.1270341277572436651e-3_f64 * t2139 * t12898;
    let t97296 = 5.0_f64 / 1296.0_f64 * t2134 * t12851;
    (t97193, t97211, t97215, t97261, t97272, t97296)
}
