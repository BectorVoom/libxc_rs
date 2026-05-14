//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1286/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1286<F: Float>(t11096: F, t2322: F, t11206: F, t11212: F, t11221: F, t11232: F, t2300: F, t2333: F, t30801: F, t31245: F, t31248: F, t31250: F, t31253: F, t3447: F, t3461: F, t847: F, t848: F, t855: F, t9219: F, t9237: F) -> (F,) {
    let t31518 = t2322 * t11096;
    let t31524 = t31245 + t31248 - t31250 - t31253 + 0.46785788981077169656e1 * t3447 * t9219 - 0.11696447245269292414e1 * t2333 * t11232 + 0.23392894490538584828e1 * t2333 * t11212 + 0.23392894490538584828e1 * t855 * t2300 * t30801 * t847 + 0.23392894490538584828e1 * t855 * t2300 * t11096 * t848 + 0.46785788981077169656e1 * t2333 * t11206 - 0.70178683471615754484e1 * t2333 * t11221 - 0.34631718211362927518e2 * t855 * t31518 * t3461 - 0.70178683471615754484e1 * t3447 * t9237;
    (t31524,)
}
