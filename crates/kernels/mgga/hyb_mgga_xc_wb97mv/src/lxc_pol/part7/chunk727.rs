//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 727/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk727<F: Float>(t1522: F, t522: F, t1137: F, t1537: F, t2905: F, t2909: F, t1148: F, t532: F, t1106: F, t1514: F) -> (F, F, F, F, F, F) {
    let t3774 = t522 * t1522;
    let t3775 = t1137 * t3774;
    let t3778 = t1537 * t2905;
    let t3781 = t1537 * t2909;
    let t3784 = t1148 * t532;
    let t3785 = t1106 * t1514;
    (t3774, t3775, t3778, t3781, t3784, t3785)
}
