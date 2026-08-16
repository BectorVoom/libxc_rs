//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 501/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk501<F: Float>(t124: F, t9419: F, t3192: F, t574: F, t1201: F, t1390: F) -> (F, F, F) {
    let t9420 = t9419 * t124;
    let t9421 = t9420 * t3192;
    let t9422 = t574 * t9421;
    let t9438 = t1201 * t124 * t1390;
    (t9420, t9422, t9438)
}
