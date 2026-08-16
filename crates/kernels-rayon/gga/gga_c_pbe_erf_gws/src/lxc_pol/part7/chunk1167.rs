//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1167/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1167(t2212: f64, t6480: f64, t2170: f64, t332: f64, t2122: f64, t6277: f64, t6678: f64, t2332: f64, t899: f64, t912: f64, t2348: f64, t336: f64, t9239: f64) -> (f64, f64, f64, f64) {
    let t20831 = t6480 * t2212;
    let t20832 = 35.0_f64 / 12.0_f64 * t20831;
    let t20833 = t332 * t2170;
    let t20835 = t20833 * t2122 * t6277;
    let t20837 = t6678 * t20835 / 4.0_f64;
    let t20839 = t899 * t912 * t2332;
    let t20840 = t20839 * t2348;
    let t20842 = t9239 * t336;
    (t20832, t20837, t20840, t20842)
}
