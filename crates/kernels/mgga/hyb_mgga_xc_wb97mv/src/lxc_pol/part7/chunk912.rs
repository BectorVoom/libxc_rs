//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 912/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk912<F: Float>(t1937: F, t3085: F, t1205: F, t6285: F, t1945: F, t6296: F, t1917: F, t82: F, t79: F, t3090: F, t3103: F, t3104: F, t3110: F, t3113: F, t3116: F, t622: F, t6257: F, t626: F, t74: F, t81: F, t8344: F, t8363: F, t8385: F, t8386: F) -> (F, F, F, F, F) {
    let t8389 = t1937 * t3085;
    let t8392 = t6285 * t1205;
    let t8400 = t1945 * t3085;
    let t8405 = t6296 * t1205;
    let t8408 = t1917 * t82;
    let t8413 = t79 * t1917;
    let t8421 = 15.0 / 2.0 * t8385 * t8386 - 4.0 * t8389 * t3104 - 5.0 / 2.0 * t8392 * t8386 - 2.0 * t3103 * t6257 + t622 * t8344 * t81 / 2.0 + t8400 * t3104 / 2.0 + t3110 * t6257 / 4.0 + t8405 * t8386 / 8.0 - 4.0 * t8408 * t1205 - 8.0 * t3113 * t3085 - t8413 * t3090 - 2.0 * t3116 * t8363 - 4.0 * t626 * t8344 - t74 * t8344 * t81;
    (t8392, t8405, t8408, t8413, t8421)
}
