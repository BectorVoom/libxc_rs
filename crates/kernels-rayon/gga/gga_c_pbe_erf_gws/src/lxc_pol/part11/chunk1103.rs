//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1103/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1103(t33105: f64, t3414: f64, t7062: f64, t40604: f64, t31102: f64, t40655: f64, t31200: f64, t1827: f64, t41514: f64, t587: f64, t950: f64, t40672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47675 = 32.0_f64 / 15.0_f64 * t7062 * t33105 * t3414;
    let t47676 = 128.0_f64 / 45.0_f64 * t40604;
    let t47677 = 32.0_f64 / 135.0_f64 * t31102;
    let t47678 = 64.0_f64 / 45.0_f64 * t40655;
    let t47679 = 8.0_f64 / 45.0_f64 * t31200;
    let t47683 = 16.0_f64 / 45.0_f64 * t587 * t1827 * t41514 * t950;
    let t47684 = 32.0_f64 / 15.0_f64 * t40672;
    (t47675, t47676, t47677, t47678, t47679, t47683, t47684)
}
