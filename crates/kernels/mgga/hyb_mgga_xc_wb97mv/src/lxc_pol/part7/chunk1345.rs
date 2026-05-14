//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1345/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1345<F: Float>(t27899: F, t3736: F, t10143: F, t10152: F, t11936: F, t1544: F, t16064: F, t28729: F, t32702: F, t32798: F, t32834: F, t32838: F, t32842: F, t32845: F, t32848: F, t32851: F, t32861: F, t32870: F, t32875: F, t3718: F, t3771: F, t505: F, t529: F, t7818: F, t7897: F, t7903: F, t7938: F, t9862: F, t9868: F, t9873: F, t9992: F) -> (F, F) {
    let t32878 = t3736 * t27899;
    let t32881 = -0.3072e-2 * t32834 * t9862 - 0.10666666666666666667e1 * t32838 * t32798 + 0.6336e-2 * t7903 * t32842 - 0.16128e-1 * t16064 * t32845 + 0.5632e-2 * t9868 * t32848 - 0.5632e-2 * t9873 * t32851 - 0.384e-3 * t28729 * t11936 - 8000.0 / 3.0 * t7938 * t1544 * t10152 + 4000.0 * t32702 * t10143 + 5600.0 * t32861 * t10152 - 22400.0 / 3.0 * t529 * t9992 * t10143 - 400.0 / 3.0 * t505 * t3718 * t10143 + 0.2016e-2 * t7818 * t32870 - 0.33792e-7 * t3771 * t32875 - 0.192e-3 * t7897 * t32878;
    (t32878, t32881)
}
