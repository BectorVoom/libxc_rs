//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 917/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk917<F: Float>(t20029: F, t3150: F, t4820: F, t4879: F, t11725: F, t247: F, t6092: F, t1063: F, t3109: F, t6100: F, t1647: F, t1678: F) -> (F, F, F, F, F) {
    let t20030 = t3150 * t20029;
    let t20034 = t4879 * t4820;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    let t20055 = t1063 * t20054;
    let t20175 = t1647 * t1678;
    (t20030, t20034, t20051, t20055, t20175)
}
