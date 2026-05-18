//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1210/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1210<F: Float>(t1181: F, t5878: F, t20685: F, t222: F, t3: F, t2951: F, t2970: F, t7848: F, t7857: F, t19746: F, t35: F, t7979: F) -> (F, F, F, F, F) {
    let t23124 = t1181 * t5878;
    let t23127 = t3 * t20685 * t222;
    let t23128 = t23127 * t2951;
    let t23139 = t2970 * t7848 * t7857;
    let t23253 = t35 * t19746 * t7979;
    (t23124, t23127, t23128, t23139, t23253)
}
