//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 744/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk744<F: Float>(t1924: F, t3931: F, t1945: F, t1937: F, t3926: F, t622: F, t626: F, t74: F, t79: F, t81: F, t82: F, t1205: F, t1217: F, t1929: F, t617: F, t72: F, t85: F) -> (F, F, F, F) {
    let t3932 = t1924 * t3931;
    let t3948 = t1945 * t3931;
    let t3959 = -2.0 * t1937 * t3931 * t81 + t622 * t3926 * t81 / 2.0 + t3948 * t81 / 4.0 - 4.0 * t3931 * t82 - t79 * t3931 * t81 - 4.0 * t626 * t3926 - t74 * t3926 * t81;
    let t3962 = -t3932 * t81 / 2.0 + 2.0 * t1929 * t3931 - t617 * t3926 + 2.0 * t3926 * t85 + 4.0 * t1205 * t1217 + 2.0 * t72 * t3959;
    (t3932, t3948, t3959, t3962)
}
