//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2678/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2678(t15700: f64, t19992: f64, t53405: f64, t16226: f64, t19997: f64, t11710: f64, t19777: f64, t3091: f64, t19644: f64, t1011: f64, t140: f64, t19916: f64) -> (f64, f64, f64, f64, f64) {
    let t66644 = t15700 * t53405 * t19992;
    let t66647 = t16226 * t53405 * t19997;
    let t66655 = t3091 * t11710 * t19777;
    let t66660 = t3091 * t11710 * t19644;
    let t66686 = t1011 * t140 * t19916;
    (t66644, t66647, t66655, t66660, t66686)
}
