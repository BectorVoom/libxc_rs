//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 86/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk86<F: Float>(t244: F, t40: F, t67: F, t62: F, t205: F, t207: F, t211: F, t216: F, t70: F) -> (F, F, F, F, F, F) {
    let t245 = t40 * t244;
    let t249 = t67 * t67;
    let t250 = 1.0 / t249;
    let t251 = t62 * t250;
    let t256 = -0.1176575e1 * t205 - 0.516475e0 * t207 - 0.2103875e0 * t211 - 0.104195e0 * t216;
    let t257 = 1.0 / t70;
    (t245, t249, t250, t251, t256, t257)
}
