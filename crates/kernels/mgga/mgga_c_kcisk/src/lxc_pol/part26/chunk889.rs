//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 889/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk889<F: Float>(t2201: F, t3119: F, t2206: F, t3123: F, t16216: F, t5817: F, t16222: F, t5824: F, t16210: F, t5828: F, t2198: F, t3114: F, t1387: F, t2059: F, t14100: F, t14082: F, t220: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20752 = t3119 * t2201;
    let t20754 = t3123 * t2206;
    let t20756 = t16216 * t5817;
    let t20759 = t16222 * t5824;
    let t20761 = t16210 * t5828;
    let t20763 = t3114 * t2198;
    let t20781 = t1387 * t2059;
    let t20783 = t14100 * t2059;
    let t20785 = t14082 * t220;
    (t20752, t20754, t20756, t20759, t20761, t20763, t20781, t20783, t20785)
}
