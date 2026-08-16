//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2827/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2827(t1011: f64, t3254: f64, t697: f64, t225: f64, t42051: f64, t1053: f64, t11788: f64, t11817: f64, t3211: f64, t1025: f64, t1026: f64, t2434: f64, t371: f64) -> (f64, f64, f64, f64, f64) {
    let t42257 = t1011 * t697 * t3254;
    let t42261 = t42051 * t225;
    let t42265 = t11788 * t1053;
    let t42270 = t3211 * t11817;
    let t42274 = t1025 * t371 * t2434 * t1026;
    (t42257, t42261, t42265, t42270, t42274)
}
