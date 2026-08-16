//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3186/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3186(t1196: f64, t12487: f64, t1756: f64, t45187: f64, t45190: f64, t16784: f64, t3543: f64, t58322: f64, t58325: f64, t58327: f64, t58330: f64, t58333: f64, t58341: f64, t58344: f64, t58462: f64, t58464: f64, t58468: f64, t58658: f64, t58660: f64, t58662: f64, t58664: f64, t58669: f64, t58671: f64, t58675: f64, t58678: f64) -> (f64, f64, f64) {
    let t58683 = 0.91082604192152556044e5_f64 * t1196 * t45187 * t1756 * t45190 * t12487;
    let t58685 = 0.51947577317044391276e2_f64 * t16784 * t3543;
    let t58686 = t58658 - t58322 + t58325 + t58660 + t58327 + t58330 + t58333 - t58662 - t58664 + t58669 - t58671 - t58341 - t58344 - t58675 - t58678 - t58683 + t58462 + t58464 + t58468 - t58685;
    (t58683, t58685, t58686)
}
