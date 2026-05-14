//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1254/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1254<F: Float>(t10648: F, t6497: F, t10652: F, t6574: F, t24926: F, t29079: F, t29081: F, t29384: F, t29387: F, t29392: F, t29394: F, t29396: F, t29398: F, t29400: F, t29402: F, t29404: F, t29406: F, t29408: F, t29411: F, t29414: F, t29418: F, t8785: F) -> (F, F, F) {
    let t29420 = 4.0 * t6497 * t10648;
    let t29422 = 0.32163958997385070134e2 * t6574 * t10652;
    let t29423 = t29079 + t29081 - t29384 + t29387 + 24.0 * t24926 * t8785 + t29392 - t29394 + t29396 + t29398 - t29400 + t29402 - t29404 - t29406 + t29408 - t29411 - t29414 - t29418 + t29420 - t29422;
    (t29420, t29422, t29423)
}
