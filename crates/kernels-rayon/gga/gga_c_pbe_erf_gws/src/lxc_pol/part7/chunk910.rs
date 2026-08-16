//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 910/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk910(t5217: f64, t735: f64, t5221: f64, t211: f64, t5098: f64, t582: f64, t1655: f64, t4991: f64, t587: f64, t5351: f64, t586: f64, t645: f64) -> (f64, f64, f64, f64) {
    let t17139 = t5217 * t735;
    let t17140 = t17139 * t5221;
    let t17141 = 128.0_f64 / 45.0_f64 * t17140;
    let t17143 = t211 * t582 * t5098;
    let t17144 = 16.0_f64 / 45.0_f64 * t17143;
    let t17146 = t587 * t4991 * t1655;
    let t17147 = 16.0_f64 / 135.0_f64 * t17146;
    let t17148 = t5351 * t586;
    let t17150 = 32.0_f64 / 15.0_f64 * t17148 * t645;
    (t17141, t17144, t17147, t17150)
}
