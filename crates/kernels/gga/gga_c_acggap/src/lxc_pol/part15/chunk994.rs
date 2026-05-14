//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 994/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk994<F: Float>(t31362: F, t9589: F, t4680: F, t7337: F, t9588: F, t1181: F, t26995: F, t599: F, t26459: F, t1983: F, t30692: F, t7586: F, t9587: F, t30689: F, t5732: F, t1815: F, t360: F, t604: F, t7413: F) -> (F, F, F, F, F, F, F) {
    let t39567 = t31362 * t9589;
    let t39570 = t7337 * t4680 * t9588;
    let t39574 = t7337 * t1181 * t599 * t26995;
    let t39581 = t7337 * t1181 * t599 * t26459;
    let t39585 = t30692 * t7586 * t1983 * t9587;
    let t39587 = t30689 * t5732;
    let t39592 = t7413 * t1181 * t604 * t1815 * t360;
    (t39567, t39570, t39574, t39581, t39585, t39587, t39592)
}
