//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 972/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk972(t10076: f64, t10145: f64, t1427: f64, t1357: f64, t4078: f64, t689: f64, t1445: f64, t3899: f64, t10115: f64, t562: f64, t2435: f64, t3903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10146 = t10076 + t10145;
    let t10147 = t1427 * t10146;
    let t10150 = t1357 * t4078;
    let t10151 = t689 * t10150;
    let t10153 = t3899 * t1445;
    let t10154 = t689 * t10153;
    let t10157 = 0.11044544084478153697e-3_f64 * t10115 * t562;
    let t10160 = t2435 * t3903;
    (t10146, t10147, t10150, t10151, t10153, t10154, t10157, t10160)
}
