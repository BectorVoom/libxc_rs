//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 770/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk770<F: Float>(t544: F, t8261: F, t197: F, t3529: F, t107: F, t11279: F, t11433: F, t1397: F, t11429: F, t11425: F, t1415: F, t1: F, t35951: F) -> (F, F, F, F, F, F, F, F) {
    let t37551 = t544 * t8261;
    let t37573 = t197 * t3529;
    let t37575 = t544 * t37573 * t107;
    let t37578 = t11279 * t107;
    let t37579 = t544 * t37578;
    let t37648 = t1397 * t11433;
    let t37654 = t1397 * t11429;
    let t37667 = t1415 * t11425;
    let t37675 = t544 * t35951 * t1;
    (t37551, t37573, t37575, t37579, t37648, t37654, t37667, t37675)
}
