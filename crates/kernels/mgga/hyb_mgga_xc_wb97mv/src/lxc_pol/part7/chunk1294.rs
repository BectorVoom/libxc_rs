//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1294/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1294<F: Float>(t11487: F, t7415: F, t11323: F, t2474: F, t956: F, t2511: F, t4319: F, t11490: F, t2475: F, t7376: F, t11491: F, t7371: F, t11322: F, t2519: F, t2517: F, t4318: F, t7405: F) -> (F, F, F, F, F, F, F, F) {
    let t31682 = 4.0 * t7415 * t11487;
    let t31685 = 4.0 * t2474 * t11323 * t956;
    let t31688 = 2.0 * t2474 * t4319 * t2511;
    let t31691 = 0.96491876992155210402e2 * t7376 * t11490 * t2475;
    let t31693 = 0.32163958997385070134e2 * t7371 * t11491;
    let t31694 = t11322 * t2519;
    let t31697 = 0.32163958997385070134e2 * t2517 * t31694 * t956;
    let t31700 = 0.16081979498692535067e2 * t2517 * t11490 * t2511;
    let t31701 = t4318 * t7405;
    (t31682, t31685, t31688, t31691, t31693, t31697, t31700, t31701)
}
