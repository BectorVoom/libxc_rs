//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2934/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2934(t11670: f64, t15687: f64, t3317: f64, t127: f64, t15690: f64, t15689: f64, t15692: f64, t11916: f64, t15932: f64, t11922: f64, t11927: f64, t16026: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53401 = t11670 * t15687;
    let t53402 = t3317 * t53401;
    let t53405 = t127 * t15690;
    let t53407 = t15689 * t53405 * t15692;
    let t53413 = t15932 * t11916;
    let t53416 = t11927 * t11922 * t16026;
    (t53401, t53402, t53405, t53407, t53413, t53416)
}
