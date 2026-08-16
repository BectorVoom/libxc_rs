//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1023/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1023<F: Float>(t22124: F, t22128: F, t22130: F, t22134: F, t22136: F, t22141: F, t22143: F, t22152: F, t22274: F, t22277: F, t22281: F, t22285: F) -> F {
    let t22286 = t22124 - t22128 - t22130 - t22134 - t22136 + t22141 - t22143 + t22152 + t22274 + t22277 + t22281 + t22285;
    t22286
}
