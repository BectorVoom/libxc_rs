//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 771/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk771<F: Float>(t11922: F, t6272: F, t3115: F, t4817: F, t4834: F, t127: F, t371: F, t6337: F, t3205: F, t6276: F, t1025: F, t4845: F, t4858: F, t3172: F, t6307: F, t3150: F) -> (F, F, F, F, F, F) {
    let t19976 = t11922 * t6272;
    let t19977 = t3115 * t19976;
    let t20005 = t4834 * t4817;
    let t20016 = t371 * t127 * t6337;
    let t20017 = t3205 * t20016;
    let t20020 = t371 * t127 * t6276;
    let t20021 = t1025 * t20020;
    let t20025 = t4858 * t4845;
    let t20029 = t3172 * t6307;
    let t20030 = t3150 * t20029;
    (t19977, t20005, t20017, t20021, t20025, t20030)
}
