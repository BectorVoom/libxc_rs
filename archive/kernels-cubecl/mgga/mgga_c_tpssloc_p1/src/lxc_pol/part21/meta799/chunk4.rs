//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2783/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2783<F: Float>(t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40679: F, t40685: F, t40708: F, t40714: F, t40716: F, t57900: F, t57903: F, t57907: F, t57908: F, t57936: F, t57939: F, t57943: F, t57946: F, t57948: F) -> F {
    let t58964 = -t40679 + t57900 + t57903 - t40685 + t57907 + t57908 + t39373 - t39397 - t39400 + t40708 + t39408 + t39411 + t57936 + t57939 + t57943 + t57946 + t57948 - t40714 + t40716;
    t58964
}
