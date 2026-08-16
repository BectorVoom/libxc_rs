//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 916/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk916(t1063: f64, t11722: f64, t126: f64, t3181: f64, t247: f64, t2853: f64, t1007: f64, t3083: f64, t1003: f64, t3080: f64, t221: f64, t346: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t11723 = t1063 * t11722;
    let t11725 = t126 * t3181;
    let t11727 = t247 * t11725 * t2853;
    let t11728 = t1063 * t11727;
    let t11730 = t3083 * t1007;
    let t11732 = t1003 * t3080;
    let t11735 = t221 * t68 * t346;
    (t11723, t11728, t11730, t11732, t11735)
}
