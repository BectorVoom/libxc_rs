//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 782/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk782(t343: f64, t6439: f64, t858: f64, t867: f64, t866: f64, t2164: f64, t2197: f64, t2192: f64, t2074: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6440 = t6439 * t343;
    let t6442 = t867 * t858 * t6440;
    let t6444 = t866 * t6442 / 96.0_f64;
    let t6445 = t2164 * t2197;
    let t6446 = 7.0_f64 / 96.0_f64 * t6445;
    let t6447 = t2164 * t2192;
    let t6448 = 7.0_f64 / 96.0_f64 * t6447;
    let t6449 = t2074 * t810;
    (t6440, t6442, t6444, t6446, t6448, t6449)
}
