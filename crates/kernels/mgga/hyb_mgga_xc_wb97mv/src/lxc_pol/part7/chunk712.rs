//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 712/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk712<F: Float>(t2893: F, t1291: F, t646: F, t1153: F, t1519: F, sigma0: F, sigma2: F) -> (F, F, F, F, F) {
    let t3686 = t2893 * sigma2;
    let t3687 = t1291 * sigma0;
    let t3688 = t3687 * t646;
    let t3689 = t3686 * t3688;
    let t3697 = t1153 * t1519;
    (t3686, t3687, t3688, t3689, t3697)
}
