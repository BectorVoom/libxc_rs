//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 89/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk89<F: Float>(t136: F, t8: F, t6: F, t95: F, t403: F, t406: F, t408: F, t90: F, t101: F, t387: F, t397: F, t400: F, t72: F, t75: F) -> (F, F, F) {
    let t411 = F::new(1.0) / t8 / t136;
    let t412 = t6 * t411;
    let t413 = t95 * t412;
    let t415 = F::cast_from(0.59778596625315888114e-2_f64) * t90 - F::new(0.17565e-2) * t403 + F::new(0.39625e-3) * t406 - F::cast_from(0.1294884726949076719e-4_f64) * t408 + F::cast_from(0.1260328125e-5_f64) * t413;
    let t417 = -F::cast_from(0.11713266981940447749e-2_f64) * t90 * t72 - F::cast_from(0.23426533963880895498e-2_f64) * t387 * t397 - t400 * t101 - t75 * t415;
    (t413, t415, t417)
}
