//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2145/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2145<F: Float>(t1468: F, t4433: F, t892: F, t1583: F, t4537: F, t27383: F, t6079: F, t775: F, t890: F, t98785: F, t25207: F, t77408: F) -> (F, F, F, F, F, F, F, F) {
    let t106546 = t892 * t1468 * t4433;
    let t106554 = t1583 * t4537;
    let t106555 = t27383 * t106554;
    let t106561 = t6079 * t775;
    let t106562 = t27383 * t106561;
    let t106565 = t6079 * t890;
    let t106566 = t98785 * t106565;
    let t106569 = t25207 * t77408;
    (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569)
}
