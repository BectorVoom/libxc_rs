//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 796/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk796<F: Float>(t604: F, t8906: F, t1181: F, t2068: F, t599: F, t8402: F, t2297: F, t301: F, t4256: F, t7450: F, t372: F, t4262: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8907 = t604 * t8906;
    let t8908 = t1181 * t8907;
    let t8909 = t2068 * t8908;
    let t8911 = t599 * t8402;
    let t8912 = t1181 * t8911;
    let t8913 = t2068 * t8912;
    let t8915 = t2297 * t301;
    let t8916 = t4256 * t8915;
    let t8917 = t7450 * t8916;
    let t8919 = t2297 * t372;
    let t8920 = t4262 * t8919;
    (t8907, t8908, t8909, t8911, t8912, t8913, t8915, t8916, t8917, t8919, t8920)
}
