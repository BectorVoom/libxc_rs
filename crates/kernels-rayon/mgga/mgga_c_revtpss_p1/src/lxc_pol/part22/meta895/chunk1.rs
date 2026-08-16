//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3087/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087(t1011: f64, t4886: f64, t697: f64, t1065: f64, t372: f64, t4866: f64, t11670: f64, t15904: f64, t12167: f64, t11922: f64, t16081: f64, t16083: f64) -> (f64, f64, f64, f64, f64) {
    let t53542 = t1011 * t697 * t4886;
    let t53545 = t372 * t1065 * t4866;
    let t53552 = t11670 * t15904;
    let t53553 = t12167 * t53552;
    let t53557 = t16081 * t11922 * t16083;
    (t53542, t53545, t53552, t53553, t53557)
}
