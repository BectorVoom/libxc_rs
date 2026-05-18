//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1229/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1229<F: Float>(t22124: F, t22152: F, t22274: F, t22277: F, t22281: F, t22285: F, t22290: F, t22293: F, t55994: F, t55997: F, t56006: F, t56012: F) -> F {
    let t56258 = t22124 + t22152 + t22274 + t22277 + t22281 + t22285 + t55994 + t55997 - t22290 + t22293 + t56006 + t56012;
    t56258
}
