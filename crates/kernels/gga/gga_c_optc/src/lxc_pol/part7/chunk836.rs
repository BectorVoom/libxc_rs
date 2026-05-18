//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 836/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk836<F: Float>(t1659: F, t7969: F, t2780: F, t924: F, t2778: F, t2769: F, t2774: F, t2773: F, t2606: F, t297: F) -> (F, F, F, F, F, F, F) {
    let t7970 = t1659 * t7969;
    let t7973 = t924 * t2780;
    let t7974 = t2778 * t7973;
    let t7976 = t2769 * t7969;
    let t7979 = t924 * t2774;
    let t7980 = t2773 * t7979;
    let t7982 = t2606 * t297;
    (t7970, t7973, t7974, t7976, t7979, t7980, t7982)
}
