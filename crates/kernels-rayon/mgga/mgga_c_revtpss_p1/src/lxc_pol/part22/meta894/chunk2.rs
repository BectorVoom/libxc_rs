//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3085/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085(t15689: f64, t15692: f64, t53405: f64, t11916: f64, t15932: f64, t11922: f64, t11927: f64, t16026: f64, t11710: f64, t15964: f64, t3091: f64, t11268: f64, t4820: f64) -> (f64, f64, f64, f64, f64) {
    let t53407 = t15689 * t53405 * t15692;
    let t53413 = t15932 * t11916;
    let t53416 = t11927 * t11922 * t16026;
    let t53422 = t3091 * t11710 * t15964;
    let t53427 = t11268 * t4820;
    (t53407, t53413, t53416, t53422, t53427)
}
