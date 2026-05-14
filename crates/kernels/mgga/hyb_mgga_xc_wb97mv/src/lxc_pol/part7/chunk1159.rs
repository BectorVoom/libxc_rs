//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1159/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1159<F: Float>(t1041: F, t7779: F, t2790: F, t575: F, t2798: F, t699: F, t2712: F, t2776: F, t2715: F, t2808: F, t2775: F, t2807: F, t458: F, t1057: F, t7487: F, t1090: F) -> (F, F, F, F, F, F, F, F) {
    let t24036 = t1041 * t7779;
    let t24042 = 1.0 / t2790 / t575;
    let t24057 = 1.0 / t2798 / t699;
    let t24075 = t2712 * t2776;
    let t24077 = t2715 * t2808;
    let t24080 = t458 * t2807 * t2775;
    let t24084 = t7487 * t1057;
    let t24087 = 480.0 * t7487 * t1090;
    (t24036, t24042, t24057, t24075, t24077, t24080, t24084, t24087)
}
