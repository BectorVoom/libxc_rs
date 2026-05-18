//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1106/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1106<F: Float>(t7433: F, t9641: F, t1851: F, t7614: F, t1967: F, t9681: F, t1988: F, t9531: F, t429: F, t4352: F, t598: F, t9529: F) -> (F, F, F, F, F) {
    let t39100 = t7433 * t9641;
    let t39107 = t7614 * t1851;
    let t39112 = t1967 * t9681;
    let t39114 = t1988 * t9531;
    let t39118 = t598 * t4352 * t429 * t9529;
    (t39100, t39107, t39112, t39114, t39118)
}
