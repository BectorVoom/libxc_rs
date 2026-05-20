//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1664/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1664<F: Float>(t17633: F, t6638: F, t3626: F, t12884: F, t247: F, t6421: F, t1261: F, t20302: F, t5312: F, t20298: F, t1785: F, t5390: F) -> (F, F, F, F, F, F) {
    let t21227 = t17633 * t6638;
    let t21228 = t3626 * t21227;
    let t21233 = t247 * t12884 * t6421;
    let t21234 = t1261 * t21233;
    let t21236 = t5312 * t20302;
    let t21239 = t5312 * t20298;
    let t21242 = t1785 * t5390;
    (t21228, t21233, t21234, t21236, t21239, t21242)
}
