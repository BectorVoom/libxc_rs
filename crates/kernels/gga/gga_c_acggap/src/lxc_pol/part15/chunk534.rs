//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 534/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk534<F: Float>(t506: F, t864: F, t368: F, t398: F, t1036: F, t171: F, t3221: F, t495: F, t1089: F, t175: F, t1140: F, t1526: F, t509: F, t987: F, t1165: F, t1532: F, t4162: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4347 = t506 * t864;
    let t4349 = t398 * t368 * t4347;
    let t4350 = t1036 * t4349;
    let t4352 = t3221 * t171;
    let t4358 = t495 * t864;
    let t4360 = t1089 * t175 * t4358;
    let t4361 = t1036 * t4360;
    let t4368 = 7.0 / 144.0 * t1140 * t1526;
    let t4369 = t987 * t509;
    let t4372 = t1165 * t1532 * t4162;
    (t4347, t4349, t4350, t4352, t4358, t4360, t4361, t4368, t4369, t4372)
}
