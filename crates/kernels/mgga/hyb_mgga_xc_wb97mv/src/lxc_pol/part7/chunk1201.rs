//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1201/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1201<F: Float>(t2831: F, t535: F, t8020: F, t16063: F, t522: F, t10075: F, t653: F, t1157: F, t1291: F, t9872: F, t1126: F, t9825: F, t537: F, t7926: F, t10181: F, t2952: F) -> (F, F, F, F, F, F, F) {
    let t28356 = t535 * t2831 * t8020;
    let t28378 = t535 * t16063 * t522;
    let t28384 = t535 * t10075 * t653;
    let t28388 = t1157 * t9872 * t1291;
    let t28392 = t1126 * t9825 * t1291;
    let t28395 = t7926 * t537;
    let t28400 = t2952 * t10181;
    (t28356, t28378, t28384, t28388, t28392, t28395, t28400)
}
