//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 942/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk942<F: Float>(t6760: F, t6762: F, t6765: F, t8908: F, t8912: F, t8929: F, t251: F, t260: F, t3413: F, t2318: F, t3452: F, t1364: F, t6981: F, t2326: F, t8911: F, t1330: F, t6786: F) -> (F, F, F, F, F, F, F) {
    let t8931 = -t6760 + 0.47488888888888888888e-1 * t6762 - 0.17808333333333333333e-1 * t6765 + 0.23744444444444444444e-1 * t8908 - t8912 + 0.53425e-1 * t8929;
    let t8933 = 0.621814e-1 * t8931 * t251;
    let t8934 = t260 * t3413;
    let t8937 = t3452 * t2318;
    let t8940 = t6981 * t1364;
    let t8941 = t8940 * t2326;
    let t8947 = 0.60385e0 * t8911;
    let t8951 = t6786 * t1330;
    (t8931, t8933, t8934, t8937, t8941, t8947, t8951)
}
