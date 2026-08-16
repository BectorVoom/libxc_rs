//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3089/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3089(t11727: f64, t4834: f64, t16143: f64, t3127: f64, t3172: f64, t15772: f64, t3106: f64, t15775: f64, t15905: f64, t43420: f64, t43574: f64, t11922: f64, t15781: f64, t4892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53628 = t4834 * t11727;
    let t53633 = t3127 * t3172 * t16143;
    let t53641 = t3106 * t15772;
    let t53643 = t3106 * t15775;
    let t53654 = t43420 * t15905;
    let t53657 = t43574 * t15905;
    let t53661 = t4892 * t11922 * t15781;
    (t53628, t53633, t53641, t53643, t53654, t53657, t53661)
}
