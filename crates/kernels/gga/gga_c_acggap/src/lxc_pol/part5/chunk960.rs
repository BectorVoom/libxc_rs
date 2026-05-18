//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 960/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk960<F: Float>(t12531: F, t527: F, t1008: F, t4667: F, t1106: F, t1181: F, t1586: F, t3391: F, t3730: F, t540: F, t1526: F, t3573: F) -> (F, F, F, F, F) {
    let t15350 = t12531 * t527;
    let t15362 = t1008 * t4667;
    let t15366 = t3391 * t1181 * t1586 * t1106;
    let t15370 = t3391 * t1181 * t540 * t3730;
    let t15378 = t3573 * t1526;
    (t15350, t15362, t15366, t15370, t15378)
}
