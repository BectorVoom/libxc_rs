//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1315/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1315(t13017: f64, t7607: f64, t13032: f64, t26843: f64, t13036: f64, t13038: f64, t13040: f64, t26842: f64, t12901: f64, t26844: f64, t13014: f64, t12998: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97204 = t7607 * t13017;
    let t97206 = t13032 * t26843;
    let t97211 = t13036 * t13038 * sigma2 * t13040;
    let t97215 = t13036 * t26842 * t13040;
    let t97218 = t26844 * t12901;
    let t97220 = t7607 * t13014;
    let t97222 = t7607 * t12998;
    (t97204, t97206, t97211, t97215, t97218, t97220, t97222)
}
