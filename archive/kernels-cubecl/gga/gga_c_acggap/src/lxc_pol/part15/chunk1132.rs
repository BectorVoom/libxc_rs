//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1132/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1132<F: Float>(t1181: F, t1815: F, t360: F, t604: F, t7413: F, t5755: F, t8511: F, t1839: F, t372: F, t2068: F, t6283: F, t7332: F) -> (F, F, F, F, F) {
    let t39592 = t7413 * t1181 * t604 * t1815 * t360;
    let t39594 = t8511 * t5755;
    let t39596 = t1839 * t372;
    let t39599 = t2068 * t1181 * t604 * t39596;
    let t39601 = t7332 * t6283;
    (t39592, t39594, t39596, t39599, t39601)
}
