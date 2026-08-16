//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1538/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538(t126: f64, t3181: f64, t1003: f64, t3080: f64, t221: f64, t346: f64, t68: f64, t345: f64, t1014: f64, t2852: f64, t245: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11725 = t126 * t3181;
    let t11732 = t1003 * t3080;
    let t11735 = t221 * t68 * t346;
    let t11737 = 5.0_f64 / 1296.0_f64 * t345 * t11735;
    let t11765 = t1014 * t2852;
    let t11772 = t3089 * t245;
    (t11725, t11732, t11735, t11737, t11765, t11772)
}
