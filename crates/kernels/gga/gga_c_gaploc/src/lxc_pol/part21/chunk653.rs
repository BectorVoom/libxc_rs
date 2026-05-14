//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 653/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk653<F: Float>(t2287: F, t471: F, t6363: F, t6366: F, t6379: F, t6381: F, t6383: F, t64: F, t869: F, t90: F) -> (F,) {
    let t6393 = t6383 * t471 - 8.0 / 3.0 * t2287 * t64 + 4.0 / 3.0 * t869 * t90 + 63.0 / 512.0 * t6363 - 49.0 / 16384.0 * t6366 + 49.0 / 49152.0 * t6379 - 21.0 / 512.0 * t6381;
    (t6393,)
}
