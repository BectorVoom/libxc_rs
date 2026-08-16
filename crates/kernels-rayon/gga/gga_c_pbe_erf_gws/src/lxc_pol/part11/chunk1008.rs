//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1008/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1008(t12588: f64, t572: f64, t12537: f64, t5283: f64, t587: f64, t12485: f64, t586: f64, t12452: f64, t583: f64, t12813: f64, t5129: f64, t12702: f64, t185: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40422 = t12588 * t572;
    let t40474 = t587 * t5283 * t12537;
    let t40493 = t12485 * t586;
    let t40498 = t12452 * t583;
    let t40527 = t587 * t5129 * t12813;
    let t40547 = t185 * t582 * t12702;
    (t40422, t40474, t40493, t40498, t40527, t40547)
}
