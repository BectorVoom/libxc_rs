//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 874/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk874<F: Float>(t1: F, t438: F, t8905: F, t450: F, t140: F, t446: F, t7369: F, t3183: F, t1122: F, t3105: F) -> (F, F, F, F, F) {
    let t8907 = t8905 * t1 * t438;
    let t8908 = t450 * t8907;
    let t8912 = t446 * t7369 * t140;
    let t8913 = t3183 * t8912;
    let t8914 = t3105 * t1122;
    (t8907, t8908, t8912, t8913, t8914)
}
