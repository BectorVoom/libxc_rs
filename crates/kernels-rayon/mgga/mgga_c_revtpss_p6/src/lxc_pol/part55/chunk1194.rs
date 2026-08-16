//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1194/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1194(t125: f64, t244: f64, t246: f64, t31838: f64, t4533: f64, t126273: f64, t248: f64, t8485: f64, t31767: f64, t31772: f64, t4364: f64, t119891: f64, t14686: f64, t1579: f64) -> (f64, f64, f64, f64) {
    let t126345 = t31838 * t244 * t246 * t125 * t4533;
    let t126358 = t126273 * t8485 * t248;
    let t126365 = t31767 * t4364 * t31772 * t4533;
    let t126375 = t14686 * t119891 * t1579;
    (t126345, t126358, t126365, t126375)
}
