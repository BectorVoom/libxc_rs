//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 952/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk952<F: Float>(t1399: F, t7009: F, t2485: F, t7025: F, t1405: F, t2213: F, t238: F) -> (F, F, F, F, F) {
    let t9151 = t7009 * t1399;
    let t9152 = t9151 * t2485;
    let t9154 = t7025 * t1399;
    let t9155 = t9154 * t2485;
    let t9159 = t238 * t2213 * t1405;
    (t9151, t9152, t9154, t9155, t9159)
}
