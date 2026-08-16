//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 448/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk448(t1730: f64, t619: f64, t1406: f64, t220: f64, t186: f64, t616: f64, t633: f64, t663: f64, t582: f64, t611: f64, t185: f64, t1687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1732 = 8.0_f64 / 15.0_f64 * t1730 * t619;
    let t1733 = -t1406;
    let t1734 = t220 * t1733;
    let t1735 = t186 * t1734;
    let t1737 = 4.0_f64 / 15.0_f64 * t616 * t1735;
    let t1739 = 4.0_f64 / 15.0_f64 * t633 * t663;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1742 = 8.0_f64 / 45.0_f64 * t1741;
    let t1743 = 0.25188888888888888889e-2_f64 * t1687;
    (t1732, t1733, t1734, t1735, t1737, t1739, t1740, t1741, t1742, t1743)
}
