//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2518/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518(t2475: f64, t808: f64, t11028: f64, t1580: f64, t2439: f64, t10504: f64, t15002: f64, t9285: f64, t10505: f64, t137: f64, t41011: f64, t11015: f64, t4325: f64) -> (f64, f64, f64, f64, f64) {
    let t51176 = t808 * t2475;
    let t51199 = t2439 * t11028 * t1580;
    let t51203 = t10504 * t15002 * t9285;
    let t51207 = t41011 * t15002 * t137 * t10505;
    let t51208 = 0.69394917116090352834e-2_f64 * t51207;
    let t51211 = t4325 * t11015;
    (t51176, t51199, t51203, t51208, t51211)
}
