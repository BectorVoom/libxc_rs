//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1069/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1069<F: Float>(t1505: F, t1618: F, t555: F, t127: F, t495: F, t5136: F, t4803: F, t546: F, t1670: F, t5322: F, t16129: F, t81: F) -> (F, F, F, F, F, F) {
    let t16923 = F::new(0.21053605041484726346e2) * t555 * t1505 * t1618;
    let t16929 = t495 * t5136 * t127;
    let t16931 = t4803 * t546;
    let t16935 = t1670 * t5322;
    let t16940 = t16129 * t127;
    let t16942 = t81 * t81;
    (t16923, t16929, t16931, t16935, t16940, t16942)
}
