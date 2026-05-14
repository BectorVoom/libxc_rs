//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1016/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1016<F: Float>(t1504: F, t27130: F, t19848: F, t4229: F, t6370: F, t14344: F, t8271: F, t1501: F, t8283: F, t25365: F, t4231: F, t4230: F, t1483: F, t8252: F, t26416: F, t381: F) -> (F, F, F, F, F, F, F, F) {
    let t27131 = t1504 * t27130;
    let t27133 = t19848 * t4229;
    let t27134 = t27133 * t6370;
    let t27136 = t14344 * t8271;
    let t27138 = t1501 * t8283;
    let t27140 = t4231 * t25365;
    let t27141 = t4230 * t27140;
    let t27143 = t1483 * t8252;
    let t27145 = t381 * t26416;
    (t27131, t27134, t27136, t27138, t27140, t27141, t27143, t27145)
}
