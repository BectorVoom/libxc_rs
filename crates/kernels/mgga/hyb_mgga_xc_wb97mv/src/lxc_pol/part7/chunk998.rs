//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 998/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk998<F: Float>(t535: F, t9872: F, t513: F, t1126: F, t1127: F, t7818: F, t7838: F, t7848: F, t7897: F, t7903: F, t7918: F, t9715: F, t9723: F, t9826: F, t9827: F, t9832: F, t9835: F, t9839: F, t9843: F, t9846: F, t9850: F, t9853: F, t9857: F, t9862: F, t9865: F, t9868: F, t9869: F, sigma0: F) -> (F, F, F, F) {
    let t9873 = t535 * t9872;
    let t9878 = t513 * sigma0;
    let t9879 = t1126 * t9878;
    let t9886 = 0.58666666666666666667e-1 * t1127 * t9715 - 0.72e-1 * t7918 * t9723 + 0.512e-3 * t9826 * t9827 + 0.144e-2 * t7848 * t9832 - 0.1728e-2 * t7903 * t9835 + 0.53333333333333333334e0 * t9839 * t9843 - 0.1728e-2 * t7903 * t9846 - 0.53333333333333333333e-3 * t9850 * t9853 - 0.192e-3 * t7897 * t9857 - 0.1728e-2 * t7903 * t9857 + 0.2016e-2 * t7818 * t9862 + 0.53333333333333333334e0 * t9865 * t9843 - 0.1536e-2 * t9868 * t9869 + 0.1536e-2 * t9873 * t9827 - 0.192e-3 * t7897 * t9835 + 0.17777777777777777778e0 * t9879 * t9843 - 0.192e-3 * t7897 * t9846 + 0.288e-3 * t7838 * t9862;
    (t9873, t9878, t9879, t9886)
}
