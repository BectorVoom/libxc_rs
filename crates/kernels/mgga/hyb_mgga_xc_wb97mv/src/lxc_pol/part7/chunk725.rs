//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 725/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk725<F: Float>(t1111: F, t1291: F, t3746: F, t1114: F, t513: F, t516: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t3747 = t1291 * t1111;
    let t3748 = t3746 * t3747;
    let t3751 = t1291 * t1114;
    let t3752 = t3746 * t3751;
    let t3759 = t513 * sigma2;
    let t3760 = t516 * t3759;
    (t3747, t3748, t3751, t3752, t3759, t3760)
}
