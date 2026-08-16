//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3507/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507(t19912: f64, t3241: f64, t1011: f64, t6292: f64, t697: f64, t11922: f64, t19717: f64, t4899: f64, t11883: f64, t16147: f64, t19705: f64, t3092: f64, t53948: f64, t53955: f64, t53958: f64, t53961: f64, t53964: f64, t53967: f64, t53970: f64, t53974: f64, t55331: f64, t6293: f64) -> f64 {
    let t66215 = t3241 * t19912;
    let t66218 = t1011 * t697 * t6292;
    let t66221 = t4899 * t11922 * t19717;
    let t66227 = -0.30488190661738479624e-2_f64 * t53948 - 0.1270341277572436651e-3_f64 * t53955 - t53958 / 108.0_f64 - t53961 / 216.0_f64 - t53964 / 54.0_f64 + t53967 / 162.0_f64 + t53970 / 324.0_f64 + 7.0_f64 / 972.0_f64 * t53974 + 11.0_f64 / 243.0_f64 * t11883 * t6293 - 2.0_f64 / 243.0_f64 * t66215 - t66218 / 972.0_f64 - 0.57165357490759649296e-3_f64 * t66221 - 0.17149607247227894789e-2_f64 * t55331 * t3092 * t19705 * t16147;
    t66227
}
