//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1231/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1231<F: Float>(t1283: F, t3155: F, t3159: F, t8542: F, t10558: F, t2039: F, t10563: F, t10869: F, t21910: F, t25681: F, t25684: F, t25687: F, t25689: F, t26171: F, t26220: F, t29909: F, t29970: F, t29973: F, t29976: F, t29985: F, t29989: F, t29991: F, t29996: F, t3156: F, t3157: F, t3288: F, t8440: F, t8528: F, t8530: F, t8548: F) -> (F, F, F) {
    let t30000 = t3155 * t8542 * t1283 * t3159;
    let t30006 = t10558 * t2039;
    let t30010 = t10563 * t2039;
    let t30014 = -t25681 / 72.0 + t25684 / 24.0 + t25687 / 72.0 + t29970 / 144.0 + t29973 / 288.0 + t29976 / 216.0 + 3.0 / 16.0 * t8440 * t10869 + 7.0 / 18.0 * t8528 * t26171 * t29909 + t25689 / 16.0 + t29985 / 144.0 + t21910 / 96.0 - 7.0 / 36.0 * t29989 + 7.0 / 18.0 * t26220 * t8530 * t29991 - 7.0 / 216.0 * t29996 - t30000 / 36.0 - t3155 * t3156 * t3288 * t3159 / 12.0 + t8548 * t3157 * t30006 / 8.0 + t8548 * t3157 * t30010 / 16.0;
    (t30006, t30010, t30014)
}
