//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1251/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1251<F: Float>(t10599: F, t2025: F, t683: F, t4109: F, t762: F, t10594: F, t1312: F, t3288: F, t10613: F, t8605: F, t10588: F, t10598: F, t10854: F, t1234: F, t136: F, t139: F, t2013: F, t2015: F, t2035: F, t2037: F, t2038: F, t2039: F, t214: F, t26: F, t26241: F, t26244: F, t26247: F, t26250: F, t2966: F, t2967: F, t29759: F, t30669: F, t3290: F, t4073: F, t453: F, t674: F, t676: F, t686: F, t687: F, t8848: F) -> (F,) {
    let t30685 = t683 * t2025 * t10599;
    let t30687 = t762 * t4109;
    let t30709 = t683 * t2025 * t10594;
    let t30711 = t3288 * t1312;
    let t30718 = t683 * t8605 * t10613;
    let t30723 = -3.0 / 32.0 * t1234 * t8848 + 3.0 / 8.0 * t2966 * t2967 * t3290 - 3.0 / 64.0 * t136 * t26 * t139 * t30669 * t214 - 3.0 / 32.0 * t2015 * t4073 - 3.0 / 16.0 * t676 * t10854 - t2035 * t29759 * t2037 * t687 * t453 / 6.0 - t30685 / 96.0 - t683 * t686 * t30687 * t674 / 32.0 - t683 * t686 * t10598 * t2013 / 64.0 - t2035 * t2038 * t10598 * t2039 / 48.0 - t683 * t686 * t10588 * t2013 / 64.0 - t2035 * t2038 * t10588 * t2039 / 48.0 - t30709 / 48.0 - t683 * t686 * t30711 * t674 / 16.0 - t26241 / 48.0 - 7.0 / 48.0 * t30718 + t26244 / 72.0 - t26247 / 96.0 - t26250 / 72.0;
    (t30723,)
}
