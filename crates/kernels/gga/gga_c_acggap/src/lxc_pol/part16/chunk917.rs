//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 917/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk917<F: Float>(t36065: F, t1181: F, t4623: F, t604: F, t7426: F, t30090: F, t8897: F, t31362: F, t8903: F, t7839: F, t8908: F, t8912: F, t8970: F, t31567: F, t36019: F, t599: F) -> (F, F, F, F, F, F, F, F) {
    let t36066 = 11.0 / 288.0 * t36065;
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36082 = 0.62896184579208304136e-3 * t36081;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    let t36086 = 0.10718504529517434243e-2 * t36085;
    let t36087 = t7839 * t8908;
    let t36088 = 0.42874018118069736972e-3 * t36087;
    let t36089 = t7839 * t8912;
    let t36090 = 0.21437009059034868486e-3 * t36089;
    let t36096 = t7839 * t8970;
    let t36097 = 0.31448092289604152068e-3 * t36096;
    let t36115 = t31567 * t1181 * t599 * t36019;
    (t36066, t36082, t36083, t36086, t36088, t36090, t36097, t36115)
}
