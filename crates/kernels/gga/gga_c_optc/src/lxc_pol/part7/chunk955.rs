//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 955/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk955<F: Float>(t559: F, t6316: F, t6319: F, t539: F, t6525: F, t1986: F, t6825: F, t1860: F, t1993: F, t601: F, t1: F, t598: F, t6735: F, t1874: F, t2042: F, t1963: F, t2048: F) -> (F, F, F, F, F, F, F, F) {
    let t22631 = t6316 * t559;
    let t22632 = 96.0 * t22631;
    let t22633 = t6319 * t559;
    let t22634 = 576.0 * t22633;
    let t22635 = t539 * t6525;
    let t22636 = 16.0 * t22635;
    let t22637 = t1986 * t6825;
    let t22638 = 0.14035736153892489771e2 * t22637;
    let t22641 = 0.21053604230838734656e2 * t601 * t1993 * t1860;
    let t22643 = t6735 * t1 * t598;
    let t22644 = 0.73246220147012639764e-3 * t22643;
    let t22645 = t2042 * t1874;
    let t22646 = 240.0 * t22645;
    let t22647 = t2048 * t1963;
    (t22632, t22634, t22636, t22638, t22641, t22644, t22646, t22647)
}
