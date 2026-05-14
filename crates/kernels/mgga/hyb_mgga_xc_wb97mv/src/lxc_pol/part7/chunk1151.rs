//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1151/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1151<F: Float>(t1019: F, t1037: F, t222: F, t3038: F, t2660: F, t7650: F, t1846: F, t2664: F, t2668: F, t566: F, t7606: F, t7608: F, t2631: F, t2634: F, t7582: F, t7587: F) -> (F, F, F, F, F, F) {
    let t23832 = 0.22161481481481481481e0 * t222 * t3038 * t1019 * t1037;
    let t23835 = 0.14246666666666666666e0 * t222 * t7650 * t2660;
    let t23839 = 0.22911460125803964958e1 * t222 * t1846 * t2664 * t2668;
    let t23843 = 0.68734380377411894876e1 * t222 * t566 * t7606 * t7608;
    let t23847 = 0.28493333333333333333e0 * t222 * t1846 * t2631 * t2634;
    let t23851 = 0.3684616320282908548e2 * t222 * t566 * t7582 * t7587;
    (t23832, t23835, t23839, t23843, t23847, t23851)
}
