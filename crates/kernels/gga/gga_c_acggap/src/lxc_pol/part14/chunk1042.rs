//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1042/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1042<F: Float>(t1165: F, t30698: F, t38789: F, t604: F, t5712: F, t7561: F, t5717: F, t5722: F, t1894: F, t2095: F, t355: F, t2001: F, t6116: F, t6121: F, t2068: F, t26214: F, t7351: F) -> (F, F, F, F, F, F, F, F) {
    let t40251 = t30698 * t1165 * t604 * t38789;
    let t40253 = t7561 * t5712;
    let t40255 = t7561 * t5717;
    let t40257 = t7561 * t5722;
    let t40260 = t2095 * t1894 * t355;
    let t40262 = t2001 * t6116;
    let t40264 = t2001 * t6121;
    let t40268 = t2068 * t1165 * t7351 * t26214;
    (t40251, t40253, t40255, t40257, t40260, t40262, t40264, t40268)
}
