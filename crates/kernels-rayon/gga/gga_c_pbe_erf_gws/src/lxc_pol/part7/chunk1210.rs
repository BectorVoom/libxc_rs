//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1210/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1210(t21498: f64, t21500: f64, t6587: f64, t899: f64, t900: f64, t935: f64, t6045: f64, t855: f64, t863: f64, t888: f64, t2327: f64, t6505: f64) -> (f64, f64, f64, f64) {
    let t21502 = t21498 * t21500 / 12.0_f64;
    let t21507 = t899 * t900 * t6587;
    let t21508 = t21507 * t935;
    let t21511 = t863 * t855 * t6045;
    let t21512 = t21511 * t888;
    let t21513 = 455.0_f64 / 162.0_f64 * t21512;
    let t21514 = t6505 * t2327;
    (t21502, t21508, t21513, t21514)
}
