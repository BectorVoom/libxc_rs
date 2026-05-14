//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 449/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk449<F: Float>(t2044: F, t674: F, t2013: F, t687: F, t707: F, t140: F, t1847: F, t35: F, t1852: F, t703: F, t137: F, t696: F, t17: F) -> (F, F, F, F, F, F, F) {
    let t2045 = t2044 * t674;
    let t2049 = t687 * t2013;
    let t2053 = t707 * t707;
    let t2058 = 2.0 / 81.0 * t35 * t1847 * t140;
    let t2059 = t1852 * t703;
    let t2062 = 1.0 / t696 / t137;
    let t2063 = t17 * t2062;
    (t2045, t2049, t2053, t2058, t2059, t2062, t2063)
}
