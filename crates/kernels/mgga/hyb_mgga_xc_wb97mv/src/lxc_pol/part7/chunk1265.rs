//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1265/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1265<F: Float>(t4189: F, t6909: F, t11192: F, t786: F, t810: F, t2244: F, t4157: F, t2248: F, t1341: F, t26440: F, t3370: F, t9115: F, t11041: F, t2240: F, t3369: F, t2199: F, t809: F) -> (F, F, F, F, F, F, F, F) {
    let t30967 = 1.0 * t6909 * t4189;
    let t30968 = t11192 * t786;
    let t30970 = 2.0 * t30968 * t810;
    let t30971 = t4157 * t2244;
    let t30973 = 0.16081979498692535067e2 * t30971 * t2248;
    let t30975 = 2.0 * t26440 * t1341;
    let t30977 = 4.0 * t9115 * t3370;
    let t30979 = 1.0 * t11041 * t2240;
    let t30989 = t3369 * t3369;
    let t30992 = 4.0 * t2199 * t30989 * t809;
    (t30967, t30970, t30973, t30975, t30977, t30979, t30989, t30992)
}
