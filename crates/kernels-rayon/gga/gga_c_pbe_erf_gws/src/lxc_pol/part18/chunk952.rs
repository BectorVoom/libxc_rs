//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 952/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk952(t10722: f64, t661: f64, t7216: f64, t1620: f64, t2576: f64, t7527: f64, t2612: f64, t2667: f64, t2674: f64, t2680: f64, t3403: f64, t7011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10723 = t10722 * t661;
    let t10724 = t7216 * t10723;
    let t10726 = 16.0_f64 / 15.0_f64 * t1620 * t10724;
    let t10728 = 16.0_f64 / 45.0_f64 * t7527 * t2576;
    let t10730 = 8.0_f64 / 45.0_f64 * t2612 * t2667;
    let t10732 = 16.0_f64 / 45.0_f64 * t2612 * t2674;
    let t10734 = 8.0_f64 / 27.0_f64 * t2612 * t2680;
    let t10736 = 16.0_f64 / 45.0_f64 * t7011 * t3403;
    (t10726, t10728, t10730, t10732, t10734, t10736)
}
