//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 941/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk941<F: Float>(t22366: F, t22422: F, t22485: F, t22617: F, t40: F, t60: F, t544: F, t6525: F, t1872: F, t2045: F, t22026: F, t88: F, t539: F, t1860: F, t1993: F, t601: F) -> (F, F, F, F, F, F) {
    let t22621 = t40 * t60 * (t22366 + t22422 + t22485 + t22617);
    let t22623 = 16.0 * t544 * t6525;
    let t22624 = t2045 * t1872;
    let t22625 = 72.0 * t22624;
    let t22626 = t22026 * t88;
    let t22627 = 384.0 * t22626;
    let t22635 = t539 * t6525;
    let t22636 = 16.0 * t22635;
    let t22641 = 0.21053604230838734656e2 * t601 * t1993 * t1860;
    (t22621, t22623, t22625, t22627, t22636, t22641)
}
