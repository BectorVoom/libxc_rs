//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2053/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2053(t15135: f64, t2908: f64, t141: f64, t11341: f64, t15140: f64, t15145: f64, t930: f64, t15149: f64, t1593: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15177 = t2908 * t15135;
    let t15178 = t141 * t15177;
    let t15180 = t11341 * t15140;
    let t15181 = t141 * t15180;
    let t15183 = t930 * t15145;
    let t15184 = t141 * t15183;
    let t15186 = t930 * t15149;
    let t15187 = t141 * t15186;
    let t15189 = t2435 * t1593;
    (t15177, t15178, t15180, t15181, t15183, t15184, t15186, t15187, t15189)
}
