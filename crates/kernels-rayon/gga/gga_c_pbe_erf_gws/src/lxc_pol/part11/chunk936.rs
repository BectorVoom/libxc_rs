//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 936/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk936(t6401: f64, t6684: f64, t19561: f64, t274: f64, t346: f64, t2251: f64, t2300: f64, t2250: f64, t2170: f64, t332: f64, t2332: f64, t899: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20675 = t6684 * t6401;
    let t20692 = t19561 * t274;
    let t20693 = t20692 * t346;
    let t20732 = t2251 * t2300;
    let t20733 = t2250 * t20732;
    let t20833 = t332 * t2170;
    let t20839 = t899 * t912 * t2332;
    (t20675, t20692, t20693, t20733, t20833, t20839)
}
