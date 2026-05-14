//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1032/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1032<F: Float>(t3931: F, t6285: F, t1937: F, t3926: F, t1945: F, t6296: F, t1205: F, t82: F, t79: F, t10434: F, t10457: F, t3085: F, t3103: F, t3104: F, t3110: F, t3113: F, t3116: F, t3932: F, t622: F, t626: F, t74: F, t81: F, t8363: F) -> (F, F, F, F, F, F, F) {
    let t10478 = t6285 * t3931;
    let t10481 = t1937 * t3926;
    let t10487 = t1945 * t3926;
    let t10492 = t6296 * t3931;
    let t10495 = t1205 * t82;
    let t10498 = t79 * t1205;
    let t10508 = 15.0 / 2.0 * t3932 * t3104 - 4.0 * t3103 * t8363 - 5.0 / 2.0 * t10478 * t3104 - 2.0 * t10481 * t3104 + t622 * t10434 * t81 / 2.0 + t10487 * t3104 / 4.0 + t3110 * t8363 / 2.0 + t10492 * t3104 / 8.0 - 8.0 * t10495 * t3085 - 2.0 * t10498 * t8363 - 4.0 * t3113 * t3926 - t3116 * t10457 - 4.0 * t626 * t10434 - t74 * t10434 * t81;
    (t10478, t10481, t10487, t10492, t10495, t10498, t10508)
}
