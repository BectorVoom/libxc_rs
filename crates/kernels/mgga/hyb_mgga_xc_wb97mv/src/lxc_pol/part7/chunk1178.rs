//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1178/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1178<F: Float>(t13: F, t21983: F, t2986: F, t191: F, t21986: F, t25: F, t2223: F, t6736: F, t2037: F, t3155: F, t3159: F, t799: F, t6811: F, t8538: F, t762: F, t8529: F) -> (F, F, F, F, F, F, F) {
    let t26161 = t21983 * t13 * t2986;
    let t26163 = t25 * t21986 * t191;
    let t26171 = t2223 * t6736 * t191;
    let t26179 = t3155 * t799 * t2037 * t191 * t3159;
    let t26187 = t6811 * t2037 * t191;
    let t26189 = t3155 * t26187 * t8538;
    let t26198 = t8529 * t762;
    (t26161, t26163, t26171, t26179, t26187, t26189, t26198)
}
