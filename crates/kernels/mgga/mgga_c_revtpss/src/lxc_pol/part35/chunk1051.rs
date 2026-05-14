//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1051/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1051<F: Float>(t1711: F, t6075: F, t23421: F, t33: F, t113096: F, t25759: F, t23148: F, t1583: F, t6416: F, t23429: F, t1544: F, t113107: F, t27799: F, t1497: F, t29547: F, t77: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114140 = t1711 * t6075;
    let t114150 = t33 * t23421;
    let t114165 = t25759 * t113096;
    let t114171 = t33 * t23148;
    let t114184 = t6416 * t1583;
    let t114188 = t33 * t23429;
    let t114192 = t6416 * t1544;
    let t114196 = t27799 * t113107;
    let t114246 = t77 * t29547 * t1497;
    (t114140, t114150, t114165, t114171, t114184, t114188, t114192, t114196, t114246)
}
