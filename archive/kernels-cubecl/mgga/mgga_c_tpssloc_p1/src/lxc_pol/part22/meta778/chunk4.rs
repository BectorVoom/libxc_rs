//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2667/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2667<F: Float>(t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t74036: F, t74040: F, t74041: F, t74042: F, t74043: F, t74044: F, t74046: F) -> F {
    let t74469 = -t74036 + t39360 + t39364 - t74040 + t74041 + t74042 + t74043 + t39373 - t74044 - t39384 + t39393 - t39397 - t39400 + t39408 + t39411 - t74046;
    t74469
}
