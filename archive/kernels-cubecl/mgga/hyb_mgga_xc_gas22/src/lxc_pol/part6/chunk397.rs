//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 397/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk397<F: Float>(t1028: F, t1050: F, t1054: F, t1059: F, t1102: F, t1109: F, t1114: F, t1386: F, t1449: F, t1523: F, t1524: F, t462: F, t493: F) -> F {
    let t1528 = t1028 + t1050 - t1054 - t1059 + t462 * t1524 + t1102 + F::cast_from(0.19751673498613801407e-1_f64) * t1523 * t493 - t1109 - t1114 - t1386 - t1449;
    t1528
}
