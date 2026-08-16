//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1358/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1358(t2371: f64, t28264: f64, t572: f64, t2327: f64, t7002: f64, t1459: f64, t26120: f64, t26124: f64, t26127: f64, t13240: f64, t13244: f64, t13247: f64, t1461: f64, t2040: f64, t26106: f64, t4162: f64, t4165: f64, t573: f64, t7324: f64, t95119: f64, t95131: f64, t95136: f64, t95140: f64, t95143: f64, t95147: f64, t95149: f64, t95153: f64, t95157: f64, param_d: f64) -> f64 {
    let t95160 = 18.0_f64 * t572 * t28264 * t2371;
    let t95163 = 18.0_f64 * t572 * t2327 * t7002;
    let t95171 = 18.0_f64 * t1459 * t26120;
    let t95173 = 36.0_f64 * t1459 * t26124;
    let t95175 = 18.0_f64 * t1459 * t26127;
    let t95176 = t573 * t95119 * param_d + 6.0_f64 * t13240 * t2040 + 18.0_f64 * t13244 * t2040 + 3.0_f64 * t13247 * t2040 + 9.0_f64 * t1461 * t26106 + 18.0_f64 * t4162 * t7324 + 9.0_f64 * t4165 * t7324 + t95131 + t95136 + t95140 + t95143 + t95147 + t95149 + t95153 + t95157 + t95160 + t95163 + t95171 + t95173 + t95175;
    t95176
}
