//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 732/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk732<F: Float>(t43: F, t1211: F, t1226: F, t3875: F, t3876: F, t3912: F, t72: F, t88: F, t29: F, t125: F, t26: F, t1238: F) -> (F, F, F, F, F) {
    let t44 = 0.135e1 <= t43;
    let t3916 = piecewise3(t44, t3875, -8.0 / 3.0 * t3876 * t88 - 16.0 / 3.0 * t1211 * t1226 - 8.0 / 3.0 * t72 * t3912);
    let t3917 = t29 * t3916;
    let t3918 = t3917 * t125;
    let t3919 = t26 * t3918;
    let t3925 = t1238 * t1238;
    (t3916, t3917, t3918, t3919, t3925)
}
