//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 941/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk941<F: Float>(t7: F, t8911: F, t1173: F, t6768: F, t2181: F, t3: F, t1874: F, t544: F, t1861: F, t1877: F, t3319: F, t3322: F, t457: F, t775: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t8912 = 0.35616666666666666666e-1 * t8911;
    let t8913 = t6768 * t1173;
    let t8916 = t2181 * t3;
    let t8917 = t1874 * t544;
    let t8927 = piecewise3(t8, 0.0, -28.0 / 27.0 * t8913 * t1861 + 16.0 / 9.0 * t8916 * t8917 + 4.0 / 9.0 * t3319 * t1877 - 2.0 / 3.0 * t775 * t1874 + 2.0 * t3322 * t457);
    let t8929 = t222 * t37 * t8927;
    (t8912, t8913, t8917, t8927, t8929)
}
