//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 684/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk684<F: Float>(t3507: F, t939: F, t1393: F, t2492: F, t941: F, t946: F, t1399: F, t238: F, t800: F) -> (F, F, F, F, F) {
    let t3508 = t939 * t3507;
    let t3513 = t2492 * t1393;
    let t3514 = t3513 * t941;
    let t3516 = t946 * t3507;
    let t3520 = t238 * t800 * t1399;
    (t3508, t3513, t3514, t3516, t3520)
}
