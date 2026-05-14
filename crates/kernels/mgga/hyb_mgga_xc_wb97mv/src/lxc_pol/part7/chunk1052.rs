//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1052/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1052<F: Float>(t10885: F, t136: F, t4110: F, t549: F, t10854: F, t10862: F, t10869: F, t10874: F, t10878: F, t10881: F, t10883: F, t1234: F, t216: F, t2966: F, t3291: F, t3305: F, t4069: F, t4111: F, t4115: F, t676: F, t765: F) -> (F, F) {
    let t10886 = t136 * t10885;
    let t10888 = t549 * t4110;
    let t10889 = t136 * t10888;
    let t10891 = -3.0 / 32.0 * t136 * t10854 - 3.0 / 64.0 * t676 * t4111 - 3.0 / 64.0 * t676 * t4069 - 3.0 / 64.0 * t10862 * t216 - 3.0 / 64.0 * t4115 * t765 - 3.0 / 32.0 * t1234 * t3291 + 3.0 / 16.0 * t2966 * t10869 - 3.0 / 32.0 * t1234 * t3305 + 3.0 / 16.0 * t2966 * t10874 - 3.0 / 64.0 * t136 * t10878 - t10881 / 32.0 - t10883 / 32.0 - t10886 / 32.0 - t10889 / 64.0;
    (t10888, t10891)
}
