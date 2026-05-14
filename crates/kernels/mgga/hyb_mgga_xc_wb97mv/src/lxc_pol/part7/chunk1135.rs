//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1135/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1135<F: Float>(t22570: F, t2293: F, t2322: F, t22472: F, t22511: F, t6858: F, t782: F, t2243: F, t230: F, t2246: F, t6981: F, t834: F, t2254: F, t2261: F, t6875: F, t815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22571 = 1.0 / t22570;
    let t22580 = t2293 * t2322;
    let t22588 = 0.13490888888888888889e1 * t22472;
    let t22601 = 0.31310740740740740741e1 * t22511;
    let t22620 = 0.13388493827160493828e1 * t22472;
    let t22633 = 0.31003950617283950618e1 * t22511;
    let t22649 = t782 * t6858;
    let t22652 = t2243 * t2243;
    let t22654 = t230 / t22652;
    let t22655 = t2246 * t2246;
    let t22656 = 1.0 / t22655;
    let t22725 = 0.96141975308641975307e-1 * t22511;
    let t22733 = 0.17757530864197530864e0 * t22511;
    let t22741 = 0.18467901234567901234e0 * t22511;
    let t22749 = t834 * t6981;
    let t22755 = t2254 * t2261;
    let t22758 = t815 * t6875;
    (t22571, t22580, t22588, t22601, t22620, t22633, t22649, t22654, t22656, t22725, t22733, t22741, t22749, t22755, t22758)
}
