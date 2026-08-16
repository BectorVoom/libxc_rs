//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 628/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk628(t266: f64, t331: f64, t265: f64, t1640: f64, t649: f64, t1661: f64, t597: f64, t1802: f64, t590: f64, t1: f64, t1952: f64, t119: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5519 = t266 * t331;
    let t5521 = 8.0_f64 / 405.0_f64 * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5543 = t1661 * t597;
    let t5548 = t590 * t1802;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    (t5519, t5521, t5522, t5543, t5548, t5559, t5560)
}
