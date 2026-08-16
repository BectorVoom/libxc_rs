//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1090/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1090(t185: f64, t186: f64, t47428: f64, t47458: f64, t47487: f64, t47500: f64, t598: f64, t30407: f64, t3465: f64, t3553: f64, t5522: f64, t639: f64) -> (f64, f64, f64) {
    let t47506 = 2.0_f64 / 15.0_f64 * t185 * t186 * t598 * (t47428 + t47458 + t47487 + t47500);
    let t47507 = 32.0_f64 / 45.0_f64 * t30407;
    let t47511 = 8.0_f64 / 9.0_f64 * t639 * t5522 * t3465 * t3553;
    (t47506, t47507, t47511)
}
