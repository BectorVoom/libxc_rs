//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1050/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1050(t25304: f64, t7283: f64, t1426: f64, t3999: f64, t25821: f64, t2106: f64, t530: f64, t10309: f64, t7342: f64, t38: f64, t624: f64, t2247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26069 = t25304 * t7283;
    let t26079 = t1426 * t3999;
    let t26148 = 22.0_f64 / 9.0_f64 * t25821;
    let t26161 = t530 * t2106;
    let t26175 = t10309 * t7342;
    let t26178 = t38 * t624;
    let t26179 = t2247 * t26178;
    (t26069, t26079, t26148, t26161, t26175, t26178, t26179)
}
