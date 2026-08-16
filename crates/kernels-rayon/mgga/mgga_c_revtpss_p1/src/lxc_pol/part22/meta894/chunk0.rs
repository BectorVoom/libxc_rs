//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3083/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083(t1063: f64, t15193: f64, t247: f64, t3109: f64, t11710: f64, t15600: f64, t3091: f64, t127: f64, t4823: f64, t11774: f64, t3096: f64, t11670: f64, t15687: f64) -> (f64, f64, f64, f64, f64) {
    let t53363 = t1063 * t247 * t3109 * t15193;
    let t53389 = t3091 * t11710 * t15600;
    let t53391 = t127 * t4823;
    let t53393 = t11774 * t53391 * t3096;
    let t53401 = t11670 * t15687;
    (t53363, t53389, t53391, t53393, t53401)
}
