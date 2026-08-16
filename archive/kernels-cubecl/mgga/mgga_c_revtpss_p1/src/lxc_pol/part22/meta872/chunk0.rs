//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3033/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033<F: Float>(t2475: F, t808: F, t14787: F, t50768: F, t14476: F, t689: F, t887: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F) -> (F, F, F, F, F) {
    let t51176 = t808 * t2475;
    let t51178 = t50768 * t51176 * t14787;
    let t51196 = t689 * t14476 * t887;
    let t51199 = t2439 * t11028 * t1580;
    let t51203 = t10504 * t15002 * t9285;
    (t51176, t51178, t51196, t51199, t51203)
}
