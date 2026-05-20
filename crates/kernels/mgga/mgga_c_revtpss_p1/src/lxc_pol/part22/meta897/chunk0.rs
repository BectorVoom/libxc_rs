//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3089/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3089<F: Float>(t11727: F, t4834: F, t16143: F, t3127: F, t3172: F, t15772: F, t3106: F, t15775: F, t15905: F, t43420: F, t43574: F, t11922: F, t15781: F, t4892: F) -> (F, F, F, F, F, F, F) {
    let t53628 = t4834 * t11727;
    let t53633 = t3127 * t3172 * t16143;
    let t53641 = t3106 * t15772;
    let t53643 = t3106 * t15775;
    let t53654 = t43420 * t15905;
    let t53657 = t43574 * t15905;
    let t53661 = t4892 * t11922 * t15781;
    (t53628, t53633, t53641, t53643, t53654, t53657, t53661)
}
