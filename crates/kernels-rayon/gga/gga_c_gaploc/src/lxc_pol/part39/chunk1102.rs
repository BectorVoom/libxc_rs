//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1102/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1102(t40942: f64, t40946: f64, t43361: f64, t43364: f64, t43368: f64, t43371: f64, t43374: f64, t43378: f64, t43384: f64, t43385: f64, t43387: f64, t43390: f64) -> f64 {
    let t47126 = 0.15337170381568299871e1_f64 * t40942;
    let t47127 = 0.38342925953920749677e0_f64 * t40946;
    let t47128 = t43361 - t43364 - t43368 - t47126 - t43371 - t43374 - t47127 - t43378 + t43384 - t43385 - t43387 + t43390;
    t47128
}
