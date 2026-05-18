//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1102/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1102<F: Float>(t40942: F, t40946: F, t43361: F, t43364: F, t43368: F, t43371: F, t43374: F, t43378: F, t43384: F, t43385: F, t43387: F, t43390: F) -> F {
    let t47126 = F::new(0.15337170381568299871e1) * t40942;
    let t47127 = F::new(0.38342925953920749677e0) * t40946;
    let t47128 = t43361 - t43364 - t43368 - t47126 - t43371 - t43374 - t47127 - t43378 + t43384 - t43385 - t43387 + t43390;
    t47128
}
