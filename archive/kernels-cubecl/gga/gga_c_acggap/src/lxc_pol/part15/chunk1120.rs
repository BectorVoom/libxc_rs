//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1120/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1120<F: Float>(t1181: F, t26757: F, t599: F, t7413: F, t6237: F, t7561: F, t7433: F, t9633: F, t30371: F, t5940: F, t7575: F, t8480: F, t8609: F) -> (F, F, F, F, F) {
    let t39362 = t7413 * t1181 * t599 * t26757;
    let t39364 = t7561 * t6237;
    let t39366 = t7433 * t9633;
    let t39368 = t30371 * t5940;
    let t39373 = t7575 * t8480 * t8609;
    (t39362, t39364, t39366, t39368, t39373)
}
