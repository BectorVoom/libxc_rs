//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 999/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk999<F: Float>(t1505: F, t1618: F, t555: F, t127: F, t495: F, t5136: F, t4803: F, t546: F, t1670: F, t5322: F, t5119: F, t545: F, t83: F, t16129: F, t81: F, t79: F) -> (F, F, F, F, F, F, F, F) {
    let t16923 = 0.21053605041484726346e2 * t555 * t1505 * t1618;
    let t16929 = t495 * t5136 * t127;
    let t16931 = t4803 * t546;
    let t16935 = t1670 * t5322;
    let t16938 = t83 * t5119 * t545;
    let t16940 = t16129 * t127;
    let t16942 = t81 * t81;
    let t16946 = 840.0 * t79 / t16942 * t127;
    (t16923, t16929, t16931, t16935, t16938, t16940, t16942, t16946)
}
