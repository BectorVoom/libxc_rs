//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1202/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1202<F: Float>(t653: F, t9838: F, t2952: F, t10161: F, t1157: F, t1126: F, t518: F, t8020: F, t2822: F, t516: F, t10045: F, t1117: F, t10098: F, t1148: F, t10053: F, t2860: F) -> (F, F, F, F, F, F, F, F) {
    let t28406 = t9838 * t653;
    let t28407 = t2952 * t28406;
    let t28410 = t1157 * t10161;
    let t28430 = t1126 * t518 * t8020;
    let t28434 = t516 * t2822 * t8020;
    let t28488 = t1117 * t10045;
    let t28500 = t1148 * t10098;
    let t28563 = t2860 * t10053;
    (t28406, t28407, t28410, t28430, t28434, t28488, t28500, t28563)
}
