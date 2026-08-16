//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2076/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2076<F: Float>(t39358: F, t756: F, t706: F, t9448: F, t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t2652: F, t9874: F) -> (F, F, F, F, F, F) {
    let t40708 = F::cast_from(0.18989649058080861537e-2_f64) * t756 * t39358;
    let t40709 = t706 * t9448;
    let t40712 = t187 * t268;
    let t40714 = F::cast_from(0.1301229756036208781e0_f64) * t40712 * t39322;
    let t40716 = F::cast_from(0.19263893255070628431e1_f64) * t40712 * t39347;
    let t40721 = F::cast_from(0.21053605041484726346e2_f64) * t761 * t39336;
    let t40722 = t2652 * t9874;
    (t40708, t40709, t40714, t40716, t40721, t40722)
}
