//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 686/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk686<F: Float>(t571: F, t7852: F, t336: F, t3565: F, t570: F, t1072: F, t154: F, t7322: F, t1: F, t145: F, t203: F, t2020: F, t2025: F, t1090: F, t1181: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7853 = t7852 * t571;
    let t7855 = t336 * t3565;
    let t7856 = t570 * t7855;
    let t7858 = t154 * t1072;
    let t7859 = t7322 * t7858;
    let t7861 = t145 * t1 * t203;
    let t7862 = t7859 * t7861;
    let t7864 = t2020 * t2025;
    let t7865 = 7.0 / 144.0 * t7864;
    let t7867 = t1181 * t604 * t1090;
    (t7853, t7855, t7856, t7858, t7859, t7861, t7862, t7864, t7865, t7867)
}
