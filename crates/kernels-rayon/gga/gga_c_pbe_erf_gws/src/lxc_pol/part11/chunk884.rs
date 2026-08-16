//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 884/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk884(t220: f64, t7776: f64, t211: f64, t156: f64, t5926: f64, t670: f64, t1999: f64, t542: f64, t196: f64, t5174: f64, t188: f64, t10: f64, t225: f64, t5902: f64) -> (f64, f64, f64, f64, f64) {
    let t16488 = t7776 * t220;
    let t16490 = 112.0_f64 / 1215.0_f64 * t211 * t16488;
    let t16498 = 0.43284165449459373508e0_f64 * t670 * t156 * t5926;
    let t16501 = 0.38474813732852776452e0_f64 * t670 * t542 * t1999;
    let t16531 = 1.0_f64 / t5174 / t196;
    let t16532 = t188 * t16531;
    let t16553 = 0.43284165449459373508e0_f64 * t670 * t10 * t225 * t5902;
    (t16490, t16498, t16501, t16532, t16553)
}
