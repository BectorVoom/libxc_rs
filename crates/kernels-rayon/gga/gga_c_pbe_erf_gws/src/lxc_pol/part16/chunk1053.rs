//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1053/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1053(t3224: f64, t6402: f64, t2307: f64, t3252: f64, t2112: f64, t816: f64, t3258: f64, t3257: f64, t3287: f64, t6203: f64, t745: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9539 = 7.0_f64 / 576.0_f64 * t6402 * t3224;
    let t9540 = t3252 * t2307;
    let t9543 = t816 * t2112;
    let t9544 = t3258 * t9543;
    let t9545 = t3257 * t9544;
    let t9549 = 7.0_f64 / 288.0_f64 * t6203 * t3287;
    let t9550 = t851 * t745;
    (t9539, t9540, t9544, t9545, t9549, t9550)
}
