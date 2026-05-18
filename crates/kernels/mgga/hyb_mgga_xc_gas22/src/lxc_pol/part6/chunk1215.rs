//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1215/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1215<F: Float>(t23622: F, t2988: F, t555: F, t557: F, t13: F, t20075: F, t2969: F, t20078: F, t25: F, t92: F, t2212: F, t6184: F) -> (F, F, F, F) {
    let t23625 = t555 * t23622 * t557 * t2988;
    let t23647 = t20075 * t13 * t2969;
    let t23649 = t25 * t20078 * t92;
    let t23655 = t2212 * t6184 * t92;
    (t23625, t23647, t23649, t23655)
}
