//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 927/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk927(t118: f64, t119: f64, t120: f64, t1477: f64, t1553: f64, t7236: f64, t502: f64, t7271: f64, t505: f64, t97: f64, t5772: f64, t131: f64, t137: f64, t5852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19355 = 70.0_f64 / 81.0_f64 * t118 * t119 * t1477 * t120;
    let t19357 = 0.29018074074074074074e1_f64 * t1553 * t7236;
    let t19359 = 0.25390814814814814815e1_f64 * t502 * t7271;
    let t19367 = 1.0_f64 / t505 / t97;
    let t19383 = t5772 * t120;
    let t19407 = t131 / t5852 / t137;
    (t19355, t19357, t19359, t19367, t19383, t19407)
}
