//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2586/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2586<F: Float>(t71255: F, t71313: F, t71315: F, t71317: F, t71319: F, t71543: F, t71545: F, t71547: F, t71655: F, t71657: F, t72045: F, t72047: F, t72050: F, t72052: F, t72058: F, t72061: F, t72065: F, t72067: F, t72071: F, t72073: F) -> F {
    let t72196 = t71255 + t72045 + t71313 + t71315 + t71317 + t71319 - t72047 + t72050 - t72052 + t72058 - t72061 - t72065 + t71543 - t71545 + t71547 + t71655 + t71657 + t72067 - t72071 - t72073;
    t72196
}
