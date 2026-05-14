//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 896/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk896<F: Float>(t6851: F, t869: F, t291: F, t3707: F, t1180: F, t7451: F, t2579: F, t891: F, t2232: F, t2546: F, t2580: F, t7943: F, t147: F, t786: F, t3412: F, t8133: F) -> (F, F, F, F, F, F, F, F) {
    let t16408 = t869 * t6851;
    let t16471 = t3707 * t291;
    let t16676 = t7451 * t1180;
    let t16677 = t2579 * t891;
    let t16720 = t2546 * t2232;
    let t16798 = t2580 * t7943;
    let t16826 = t147 * t786;
    let t17713 = t3412 * t8133;
    (t16408, t16471, t16676, t16677, t16720, t16798, t16826, t17713)
}
