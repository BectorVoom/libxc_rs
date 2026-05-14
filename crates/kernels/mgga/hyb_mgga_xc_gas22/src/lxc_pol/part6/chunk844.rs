//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 844/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk844<F: Float>(t6966: F, t7034: F, t2569: F, t997: F, t2598: F, t993: F, t2597: F, t383: F) -> (F, F, F, F, F) {
    let t7082 = 0.93011851851851851854e0 * t6966;
    let t7089 = 0.36514074074074074075e0 * t7034;
    let t7099 = t2569 * t997;
    let t7104 = t993 * t2598;
    let t7108 = 1.0 / t2597 / t383;
    (t7082, t7089, t7099, t7104, t7108)
}
