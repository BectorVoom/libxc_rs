//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 303/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk303(t1165: f64, t1167: f64, t1169: f64, t1152: f64, t1154: f64, t1158: f64, t1161: f64, t14: f64, t351: f64, t394: f64, t31: f64, t4: f64, t96: f64) -> (f64, f64) {
    let t1171 = -0.44044444444444444445e-2_f64 * t1165 + 0.88088888888888888889e-2_f64 * t1167 + 0.55033333333333333333e-2_f64 * t1169;
    let t1174 = -t1152 * t1154 / 18.0_f64 - t1158 * t351 / 6.0_f64 + t394 * t1161 / 9.0_f64 + t14 * t1171 / 2.0_f64;
    let t1179 = 0.14764770444444444444e-2_f64 * t4 * t96 * t31;
    (t1174, t1179)
}
