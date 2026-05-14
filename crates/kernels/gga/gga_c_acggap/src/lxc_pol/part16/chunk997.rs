//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 997/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk997<F: Float>(t1181: F, t26459: F, t599: F, t7337: F, t1983: F, t30692: F, t7586: F, t9587: F, t30689: F, t5732: F, t1815: F, t360: F, t604: F, t7413: F, t5755: F, t8511: F) -> (F, F, F, F, F) {
    let t39581 = t7337 * t1181 * t599 * t26459;
    let t39585 = t30692 * t7586 * t1983 * t9587;
    let t39587 = t30689 * t5732;
    let t39592 = t7413 * t1181 * t604 * t1815 * t360;
    let t39594 = t8511 * t5755;
    (t39581, t39585, t39587, t39592, t39594)
}
