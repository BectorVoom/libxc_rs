//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1005/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1005(t22120: f64, t587: f64, t601: f64, t6405: f64, t2204: f64, t2229: f64, t1846: f64, t1863: f64, t6427: f64, t2040: f64, t8: f64, t108: f64, t117: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22124 = 0.1403573615389248977e2_f64 * t601 * t6405 * t22120 * t587;
    let t22126 = 70.0_f64 / 3.0_f64 * t2229 * t2204;
    let t22148 = 1.0_f64 / t1863 / t1846;
    let t22152 = 0.12304676425209353917e5_f64 * t601 * t22148 * t22120 * t6427;
    let t22154 = 1.0_f64 / t8 / t2040;
    let t22158 = 455.0_f64 / 243.0_f64 * t108 * t22154 * t56 * t117;
    (t22124, t22126, t22148, t22152, t22154, t22158)
}
