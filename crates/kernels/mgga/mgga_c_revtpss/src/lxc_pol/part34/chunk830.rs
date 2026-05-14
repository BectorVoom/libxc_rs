//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 830/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk830<F: Float>(t1011: F, t19912: F, t3172: F, t6262: F, t3127: F, t1062: F, t6317: F, t11922: F, t6272: F, t3115: F, t4817: F, t4834: F, t127: F, t371: F, t6337: F, t3205: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19913 = t1011 * t19912;
    let t19920 = t3172 * t6262;
    let t19921 = t3127 * t19920;
    let t19968 = t6317 * t1062;
    let t19976 = t11922 * t6272;
    let t19977 = t3115 * t19976;
    let t20005 = t4834 * t4817;
    let t20016 = t371 * t127 * t6337;
    let t20017 = t3205 * t20016;
    (t19913, t19920, t19921, t19968, t19976, t19977, t20005, t20016, t20017)
}
