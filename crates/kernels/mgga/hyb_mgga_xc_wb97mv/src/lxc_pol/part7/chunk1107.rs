//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1107/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1107<F: Float>(t1111: F, t4077: F, t3746: F, t1114: F, t1142: F, t4550: F) -> (F, F, F, F) {
    let t11809 = t4077 * t1111;
    let t11810 = t3746 * t11809;
    let t11813 = t4077 * t1114;
    let t11814 = t3746 * t11813;
    let t11821 = t1142 * t4550;
    (t11809, t11810, t11814, t11821)
}
