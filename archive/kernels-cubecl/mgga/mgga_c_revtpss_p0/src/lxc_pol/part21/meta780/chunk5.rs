//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2786/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2786<F: Float>(t14476: F, t689: F, t887: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F, t10505: F, t137: F, t41011: F) -> (F, F, F, F) {
    let t51196 = t689 * t14476 * t887;
    let t51199 = t2439 * t11028 * t1580;
    let t51203 = t10504 * t15002 * t9285;
    let t51207 = t41011 * t15002 * t137 * t10505;
    (t51196, t51199, t51203, t51207)
}
