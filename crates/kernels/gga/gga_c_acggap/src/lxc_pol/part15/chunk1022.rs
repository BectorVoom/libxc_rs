//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1022/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1022<F: Float>(t1165: F, t2068: F, t39596: F, t7351: F, t31350: F, t5737: F, t7337: F, t8480: F, t8902: F, t30698: F, t38789: F, t604: F, t5712: F, t7561: F, t5717: F, t5722: F) -> (F, F, F, F, F, F, F) {
    let t40241 = t2068 * t1165 * t7351 * t39596;
    let t40243 = t31350 * t5737;
    let t40246 = t7337 * t8480 * t8902;
    let t40251 = t30698 * t1165 * t604 * t38789;
    let t40253 = t7561 * t5712;
    let t40255 = t7561 * t5717;
    let t40257 = t7561 * t5722;
    (t40241, t40243, t40246, t40251, t40253, t40255, t40257)
}
