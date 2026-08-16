//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 89/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk89(t136: f64, t8: f64, t6: f64, t95: f64, t403: f64, t406: f64, t408: f64, t90: f64, t101: f64, t387: f64, t397: f64, t400: f64, t72: f64, t75: f64) -> (f64, f64, f64) {
    let t411 = 1.0_f64 / t8 / t136;
    let t412 = t6 * t411;
    let t413 = t95 * t412;
    let t415 = 0.59778596625315888114e-2_f64 * t90 - 0.17565e-2_f64 * t403 + 0.39625e-3_f64 * t406 - 0.1294884726949076719e-4_f64 * t408 + 0.1260328125e-5_f64 * t413;
    let t417 = -0.11713266981940447749e-2_f64 * t90 * t72 - 0.23426533963880895498e-2_f64 * t387 * t397 - t400 * t101 - t75 * t415;
    (t413, t415, t417)
}
