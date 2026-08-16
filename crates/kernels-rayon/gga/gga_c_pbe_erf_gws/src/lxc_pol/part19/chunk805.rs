//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 805/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk805(t338: f64, t348: f64, t6594: f64, t2123: f64, t6183: f64, t326: f64, t6469: f64, t2200: f64, t855: f64, t859: f64, t854: f64, t899: f64, t912: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6597 = 455.0_f64 / 1296.0_f64 * t348 * t6594 * t338;
    let t6605 = t6183 * t2123;
    let t6608 = t326 * t6469;
    let t6616 = t855 * t2200 * t859;
    let t6617 = t854 * t6616;
    let t6627 = t899 * t912 * t923;
    (t6597, t6605, t6608, t6616, t6617, t6627)
}
