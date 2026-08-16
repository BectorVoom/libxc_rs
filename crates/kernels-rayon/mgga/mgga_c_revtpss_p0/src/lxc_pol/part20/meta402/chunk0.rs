//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1491/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1491(t1053: f64, t11788: f64, t11783: f64, t3215: f64, t11817: f64, t3211: f64, t1025: f64, t1026: f64, t2434: f64, t371: f64, t11901: f64, t993: f64) -> (f64, f64, f64, f64, f64) {
    let t42265 = t11788 * t1053;
    let t42268 = t11783 * t3215;
    let t42270 = t3211 * t11817;
    let t42274 = t1025 * t371 * t2434 * t1026;
    let t42277 = t11901 * t993;
    (t42265, t42268, t42270, t42274, t42277)
}
