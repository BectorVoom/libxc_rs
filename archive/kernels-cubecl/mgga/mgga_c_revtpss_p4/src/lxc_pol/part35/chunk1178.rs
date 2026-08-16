//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1178/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1178<F: Float>(t1583: F, t5966: F, t25207: F, t23279: F, t27159: F, t1544: F, t6075: F, t27383: F, t1468: F, t29598: F, t98658: F, t198: F, t23114: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113096 = t5966 * t1583;
    let t113097 = t25207 * t113096;
    let t113100 = t27159 * t23279;
    let t113103 = t1544 * t6075;
    let t113104 = t25207 * t113103;
    let t113107 = t1583 * t6075;
    let t113108 = t27383 * t113107;
    let t113111 = t1468 * t6075;
    let t113115 = t98658 * t29598;
    let t113123 = t198 * t23114;
    (t113096, t113097, t113100, t113103, t113104, t113107, t113108, t113111, t113115, t113123)
}
