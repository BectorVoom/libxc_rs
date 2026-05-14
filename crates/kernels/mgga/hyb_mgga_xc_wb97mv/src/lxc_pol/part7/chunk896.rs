//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 896/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk896<F: Float>(t2869: F, t522: F, t2849: F, t2873: F, t1141: F) -> (F, F, F, F, F) {
    let t7984 = t522 * t2869;
    let t7995 = t522 * t2849;
    let t7999 = t522 * t2873;
    let t8019 = t1141 * t1141;
    let t8020 = 1.0 / t8019;
    (t7984, t7995, t7999, t8019, t8020)
}
