//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2684/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684(t11922: f64, t16081: f64, t19749: f64, t20020: f64, t3211: f64, t15656: f64, t4845: f64, t19675: f64, t372: f64, t11947: f64, t20016: f64, t11875: f64, t19757: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67025 = t16081 * t11922 * t19749;
    let t67044 = t3211 * t20020;
    let t67048 = t15656 * t4845;
    let t67052 = t372 * t19675;
    let t67072 = t11947 * t20016;
    let t67152 = t11875 * t11922 * t19757;
    (t67025, t67044, t67048, t67052, t67072, t67152)
}
