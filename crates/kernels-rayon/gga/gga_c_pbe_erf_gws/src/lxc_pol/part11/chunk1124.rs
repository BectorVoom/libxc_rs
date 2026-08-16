//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1124/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1124(t32215: f64, t3479: f64, t3555: f64, t1033: f64, t12871: f64, t1896: f64, t47409: f64, t587: f64, t590: f64, t1661: f64, t1664: f64, t10843: f64, t3531: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47902 = 16.0_f64 / 45.0_f64 * t32215;
    let t47904 = 4.0_f64 / 5.0_f64 * t3479 * t3555;
    let t47906 = 8.0_f64 / 15.0_f64 * t1033 * t12871;
    let t47910 = 8.0_f64 / 15.0_f64 * t587 * t590 * t1896 * t47409;
    let t47914 = 4.0_f64 / 9.0_f64 * t587 * t1661 * t1664 * t47409;
    let t47916 = 16.0_f64 / 9.0_f64 * t10843 * t3531;
    (t47902, t47904, t47906, t47910, t47914, t47916)
}
