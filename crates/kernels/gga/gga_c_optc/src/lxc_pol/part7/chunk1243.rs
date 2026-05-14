//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1243/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1243<F: Float>(t1122: F, t27448: F, t553: F, t3105: F, t8446: F, t3103: F, t3120: F, t3236: F, t8415: F, t2856: F, t9057: F, t4435: F, t9074: F, t1179: F, t2586: F, t9193: F) -> (F, F, F, F, F, F) {
    let t27449 = t553 * t1122 * t27448;
    let t27453 = t8446 * t3105;
    let t27455 = t3103 * t27453 * t3120;
    let t27461 = t8415 * t3236;
    let t27465 = t2856 * t9057;
    let t27470 = t4435 * t27453 * t9074;
    let t27473 = t1179 * t2586 * t9193;
    (t27449, t27455, t27461, t27465, t27470, t27473)
}
