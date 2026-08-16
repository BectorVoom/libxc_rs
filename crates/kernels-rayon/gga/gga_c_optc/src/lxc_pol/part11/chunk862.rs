//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk862(t12998: f64, t13005: f64, t13050: f64, t13054: f64, t16220: f64, t16221: f64, t16249: f64, t16262: f64, t16295: f64, t16301: f64, t16336: f64, t16582: f64, t16602: f64, t16614: f64, t16619: f64, t172: f64, t188: f64, t6318: f64, t6321: f64, t6324: f64, t6328: f64, t6330: f64, t6332: f64, t6457: f64, t6465: f64, t6526: f64, t6638: f64, t6644: f64, t6696: f64, t6741: f64, t6747: f64, t6750: f64, t6753: f64, t6771: f64, t9431: f64, t95: f64, t9523: f64, t9527: f64, param_c1: f64) -> f64 {
    let t16623 = param_c1 * (t188 * t16295 / 2.0_f64 + t188 * t16301 / 2.0_f64 + t16336 + t16614 + t16602 - t6747 + t16619 + 35.0_f64 / 3.0_f64 * t9431 + t6753 - t6638 - t6750 - t6332 + 35.0_f64 / 3.0_f64 * t9527 + t16582 - t6318 - t6321 + t6771 + t6465 - 7.0_f64 / 2.0_f64 * t13054 + 3.0_f64 / 2.0_f64 * t13005 - 7.0_f64 * t13050 - 7.0_f64 * t9523 + t6457 + t6526 - t6324 + 0.15506928860942058298e-1_f64 * t95 * t16221 * t172 - t6328 - t6330 + t6741 - t16220 + t16249 + t6696 + t16262 - t6644 - 7.0_f64 / 2.0_f64 * t12998);
    t16623
}
