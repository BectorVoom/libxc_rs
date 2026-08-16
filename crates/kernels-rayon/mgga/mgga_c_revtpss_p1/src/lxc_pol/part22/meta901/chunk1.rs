//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3096/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096(t3105: f64, t4857: f64, t1012: f64, t43222: f64, t16190: f64, t3173: f64, t15711: f64, t3188: f64, t1011: f64, t15145: f64, t15987: f64, t15149: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53926 = t4857 * t3105;
    let t53944 = t1012 * t43222;
    let t53948 = t16190 * t3173;
    let t53955 = t3188 * t15711;
    let t53958 = t1011 * t15987 * t15145;
    let t53961 = t1011 * t15987 * t15149;
    (t53926, t53944, t53948, t53955, t53958, t53961)
}
