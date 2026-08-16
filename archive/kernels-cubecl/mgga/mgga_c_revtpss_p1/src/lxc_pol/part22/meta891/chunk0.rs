//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3079/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3079<F: Float>(t1071: F, t4743: F, t1078: F, t4772: F, t16237: F, t994: F, t11200: F, t1678: F, t3056: F, t4742: F, t378: F, t379: F) -> (F, F, F, F, F, F, F) {
    let t53119 = t4743 * t1071;
    let t53130 = t1078 * t4772;
    let t53157 = t994 * t16237;
    let t53160 = t11200 * t1678;
    let t53166 = t4742 * t3056;
    let t53167 = t53166 * t378;
    let t53174 = t11200 * t379;
    (t53119, t53130, t53157, t53160, t53166, t53167, t53174)
}
