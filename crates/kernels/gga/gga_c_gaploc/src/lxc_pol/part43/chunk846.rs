//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 846/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk846<F: Float>(t13937: F, t2549: F, t12176: F, t2558: F, t943: F, t1841: F, t47484: F, t7289: F, t2576: F, t39347: F, t731: F, t1897: F, t1901: F, t47322: F, t13921: F, t7137: F) -> (F, F, F, F, F, F, F) {
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    let t47693 = t1841 * t7289 * t47484;
    let t47696 = t1841 * t39347 * t2576;
    let t47702 = t731 * t13937;
    let t47708 = 0.76905262301422242837e-2 * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    (t47687, t47690, t47693, t47696, t47702, t47708, t47709)
}
