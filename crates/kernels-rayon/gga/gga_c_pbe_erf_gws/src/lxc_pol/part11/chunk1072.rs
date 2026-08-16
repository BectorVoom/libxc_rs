//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1072/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1072(t3047: f64, t9955: f64, t13112: f64, t22493: f64, t13650: f64, t4414: f64, t13700: f64, t2053: f64, t3703: f64, t10424: f64, t1820: f64, t1821: f64, t3346: f64, param_gamma: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47084 = t9955 * t3047;
    let t47087 = t22493 * t13112;
    let t47143 = t4414 * t13650;
    let t47169 = t13700 * t2053;
    let t47181 = param_gamma * t3703;
    let t47293 = 16.0_f64 / 15.0_f64 * t1820 * t1821 * t10424 * t3346;
    (t47084, t47087, t47143, t47169, t47181, t47293)
}
