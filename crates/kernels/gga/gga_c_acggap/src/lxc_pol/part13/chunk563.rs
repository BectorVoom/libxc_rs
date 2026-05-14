//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 563/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk563<F: Float>(t1036: F, t4349: F, t171: F, t3221: F, t495: F, t922: F, t175: F, t864: F, t1089: F, t1165: F, t1552: F, t3176: F, t1140: F, t1526: F, t509: F, t987: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4350 = t1036 * t4349;
    let t4352 = t3221 * t171;
    let t4353 = t495 * t922;
    let t4355 = t4352 * t175 * t4353;
    let t4358 = t495 * t864;
    let t4360 = t1089 * t175 * t4358;
    let t4361 = t1036 * t4360;
    let t4364 = t1165 * t1552 * t3176;
    let t4368 = 7.0 / 144.0 * t1140 * t1526;
    let t4369 = t987 * t509;
    (t4350, t4352, t4353, t4355, t4358, t4360, t4361, t4364, t4368, t4369)
}
