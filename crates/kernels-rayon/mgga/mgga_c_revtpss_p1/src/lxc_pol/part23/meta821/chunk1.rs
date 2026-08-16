//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2672/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672(t1058: f64, t19858: f64, t15688: f64, t16509: f64, t19869: f64, t3201: f64, t6318: f64, t1011: f64, t15987: f64, t18926: f64, t18930: f64, t15689: f64, t19985: f64, t53405: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t66093 = t19858 * t1058;
    let t66114 = t16509 * t15688;
    let t66139 = t19869 * t1058;
    let t66141 = t6318 * t3201;
    let t66155 = t1011 * t15987 * t18926;
    let t66158 = t1011 * t15987 * t18930;
    let t66176 = t15689 * t53405 * t19985;
    (t66093, t66114, t66139, t66141, t66155, t66158, t66176)
}
