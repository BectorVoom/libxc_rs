//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1209/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1209<F: Float>(t157: F, t506: F, t929: F, t1163: F, t1165: F, t1539: F, t20906: F, t1036: F, t1772: F, t368: F, t398: F, t864: F) -> (F, F, F) {
    let t22048 = t506 * t929 * t157;
    let t22068 = t1163 * t1165 * t20906 * t1539;
    let t22080 = t1036 * t398 * t368 * t1772 * t864;
    (t22048, t22068, t22080)
}
