//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1000/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1000<F: Float>(t516: F, t7831: F, t1126: F, t3759: F, t646: F, t198: F, t3677: F, t1795: F, t3687: F, t3686: F, t8020: F, t1291: F, t297: F, t1801: F, t2873: F, t3711: F, sigma2: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9890 = t516 * t7831;
    let t9893 = t1126 * t3759;
    let t9896 = t646 * tau1;
    let t9897 = t9896 * t198;
    let t9898 = t3677 * t9897;
    let t9901 = t3687 * t1795;
    let t9902 = t3686 * t9901;
    let t9913 = t8020 * sigma2;
    let t9914 = t1291 * t297;
    let t9915 = t9914 * t1801;
    let t9916 = t9913 * t9915;
    let t9921 = t3711 * t2873;
    (t9890, t9893, t9896, t9897, t9898, t9901, t9902, t9913, t9915, t9916, t9921)
}
