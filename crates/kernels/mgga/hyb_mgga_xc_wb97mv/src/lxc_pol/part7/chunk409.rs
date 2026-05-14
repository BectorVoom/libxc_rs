//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 409/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk409<F: Float>(t328: F, t95: F, t302: F, t104: F, t654: F, t123: F, t646: F, tau0: F) -> (F, F, F, F, F) {
    let t1805 = t95 * t328;
    let t1806 = 1.0 / t302;
    let t1808 = 1.0 / t654 / t104;
    let t1812 = t646 * t123;
    let t1815 = tau0 * tau0;
    (t1805, t1806, t1808, t1812, t1815)
}
