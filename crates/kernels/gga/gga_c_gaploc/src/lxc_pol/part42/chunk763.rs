//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 763/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk763<F: Float>(t11218: F, t1564: F, t197: F, t3529: F, t107: F, t544: F, t11279: F, t11433: F, t1397: F, t11429: F, t11425: F, t1415: F) -> (F, F, F, F, F, F, F) {
    let t37478 = t1564 * t11218;
    let t37573 = t197 * t3529;
    let t37575 = t544 * t37573 * t107;
    let t37578 = t11279 * t107;
    let t37579 = t544 * t37578;
    let t37648 = t1397 * t11433;
    let t37654 = t1397 * t11429;
    let t37667 = t1415 * t11425;
    (t37478, t37573, t37575, t37579, t37648, t37654, t37667)
}
