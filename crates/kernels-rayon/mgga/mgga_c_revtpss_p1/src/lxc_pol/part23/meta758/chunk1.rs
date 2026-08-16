//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2551/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551(t1053: f64, t15670: f64, t11262: f64, t3127: f64, t4824: f64, t11671: f64, t4954: f64, t11998: f64, t15822: f64, t1086: f64, t15669: f64, t3090: f64) -> (f64, f64, f64, f64, f64) {
    let t54404 = t15670 * t1053;
    let t54414 = t3127 * t11262 * t4824;
    let t54471 = t4954 * t11671;
    let t54492 = t15822 * t11998;
    let t54500 = t15669 * t1086 * t3090;
    (t54404, t54414, t54471, t54492, t54500)
}
