//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3112/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3112(t1025: f64, t127: f64, t15649: f64, t371: f64, t225: f64, t53166: f64, t1053: f64, t15655: f64, t15666: f64, t3224: f64, t11991: f64, t4817: f64) -> (f64, f64, f64, f64, f64) {
    let t54693 = t1025 * t371 * t127 * t15649;
    let t54695 = t53166 * t225;
    let t54699 = t15655 * t1053;
    let t54704 = t3224 * t15666;
    let t54708 = t11991 * t4817;
    (t54693, t54695, t54699, t54704, t54708)
}
