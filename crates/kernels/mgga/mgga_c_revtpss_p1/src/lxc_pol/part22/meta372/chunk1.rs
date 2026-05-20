//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1918/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1918<F: Float>(t11249: F, t13045: F, t3603: F, t13032: F, t3609: F, t1032: F, t3552: F, t1246: F, t247: F, t3372: F, t3634: F, t1261: F) -> (F, F, F, F, F, F, F) {
    let t13046 = t11249 * t13045;
    let t13053 = t11249 * t3603;
    let t13058 = t13032 * t3609;
    let t13068 = t3552 * t1032;
    let t13069 = t13068 * t1246;
    let t13085 = t247 * t3634 * t3372;
    let t13086 = t1261 * t13085;
    (t13046, t13053, t13058, t13068, t13069, t13085, t13086)
}
