//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 880/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk880<F: Float>(t1057: F, t2712: F, t1086: F, t2683: F, t2702: F, t1099: F, t2689: F, t1084: F, t2693: F, t10: F, t2807: F, t1096: F, t2778: F, t2782: F, t1089: F, t458: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7760 = t2712 * t1057;
    let t7765 = t2702 * t2683 * t1086;
    let t7767 = 0.35089341735807877242e1 * t1099 * t7765;
    let t7768 = t2689 * t2683;
    let t7769 = t2693 * t1084;
    let t7770 = t7768 * t7769;
    let t7772 = 0.51947577317044391277e2 * t1099 * t7770;
    let t7773 = t2807 * t10;
    let t7774 = t7773 * t1096;
    let t7777 = t2778 * t2782;
    let t7779 = t2807 * t1089;
    let t7780 = t458 * t7779;
    (t7760, t7765, t7767, t7768, t7769, t7770, t7772, t7773, t7774, t7777, t7779, t7780)
}
