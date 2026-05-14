//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 937/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk937<F: Float>(t26: F, t8847: F, t1237: F, t6715: F, t683: F, t191: F, t685: F, t214: F, t3174: F, t676: F, t1285: F, t2003: F, t136: F, t1234: F, t2022: F, t2967: F, t764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8848 = t26 * t8847;
    let t8852 = t683 * t6715 * t1237;
    let t8854 = t685 * t191;
    let t8855 = t8854 * t214;
    let t8860 = t676 * t3174 / 32.0;
    let t8861 = t2003 * t1285;
    let t8862 = t136 * t8861;
    let t8864 = t1234 * t2022;
    let t8866 = t2967 * t764;
    (t8848, t8852, t8854, t8855, t8860, t8861, t8862, t8864, t8866)
}
