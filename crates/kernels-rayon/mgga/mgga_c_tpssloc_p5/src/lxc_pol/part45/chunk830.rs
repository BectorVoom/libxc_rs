//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 830/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk830(t23173: f64, t7084: f64, t814: f64, t829: f64, t2679: f64, t7101: f64, t235: f64, t24234: f64, t2051: f64, t226: f64, t23156: f64, t23160: f64, t23166: f64, t23169: f64, t23178: f64, t23182: f64, t23187: f64, t2613: f64, t7104: f64, t808: f64, t812: f64) -> f64 {
    let t24265 = 0.16449340668482264365e-1_f64 * t23173;
    let t24269 = t814 * t7084;
    let t24270 = t24269 * t829;
    let t24273 = t7101 * t2679;
    let t24278 = t235 * t24234;
    let t24280 = -0.6579736267392905746e-1_f64 * t23156 - 0.3289868133696452873e-1_f64 * t23160 + 0.3289868133696452873e-1_f64 * t23166 + 0.15352717957250113407e0_f64 * t23169 - t24265 - 0.3289868133696452873e-1_f64 * t23178 - 0.16449340668482264365e-1_f64 * t23182 + 0.16449340668482264365e-1_f64 * t23187 - 2.0_f64 * t812 * t24270 - t812 * t24273 + 2.0_f64 * t808 * t7104 + t2613 * t2051 + t226 * t24278;
    t24280
}
