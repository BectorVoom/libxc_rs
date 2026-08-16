//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 888/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk888<F: Float>(t1095: F, t3101: F, t372: F, t384: F, t398: F, t1163: F, t1165: F, t3695: F, t407: F, t1160: F, t12746: F, t1167: F) -> (F, F, F, F) {
    let t13161 = t384 * t398 * t1095 * t3101 * t372;
    let t13181 = t1163 * t1165 * t3695 * t407;
    let t13183 = t1160 * t12746;
    let t13184 = t13183 * t1167;
    (t13161, t13181, t13183, t13184)
}
