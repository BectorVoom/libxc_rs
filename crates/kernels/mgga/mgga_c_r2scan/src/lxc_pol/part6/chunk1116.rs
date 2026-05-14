//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1116/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1116<F: Float>(t1610: F, t5095: F, t5096: F, t5147: F, t5148: F, t5168: F, t2147: F, t6398: F, t6541: F, t6535: F, t6536: F, t2132: F, t6217: F, t1234: F, t6212: F, t6211: F, t6480: F) -> (F, F, F, F, F, F) {
    let t20113 = t5095 * t1610 * t5096;
    let t20116 = t5147 * t5148 * t5168;
    let t20122 = t2147 * t6398 * t6541;
    let t20125 = t6535 * t6398 * t6536;
    let t20127 = t6217 * t2132;
    let t20132 = t6212 * t1234;
    let t20134 = t6480 * t6211 * t20132;
    (t20113, t20116, t20122, t20125, t20127, t20134)
}
