//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1776/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1776(t6502: f64, t1169: f64, t1188: f64, t12429: f64, t12486: f64, t12553: f64, t17032: f64, t17097: f64, t24431: f64, t24436: f64, t3452: f64, t3477: f64, t3479: f64, t3496: f64, t3521: f64, t3523: f64, t6487: f64, t69488: f64, t90319: f64, t90329: f64, t90332: f64, t90336: f64, t90339: f64, t90341: f64, t90343: f64, t90352: f64, t90357: f64, t90670: f64) -> f64 {
    let t90756 = t6502 * t6502;
    let t90775 = 0.6233709278045326953e3_f64 * t12553 * t90357 * t3523 - 12.0_f64 * t69488 * t6487 + 24.0_f64 * t17032 * t24431 - 24.0_f64 * t12429 * t90670 * t1169 - 6.0_f64 * t3452 * t90756 * t1169 + 0.96491876992155210402e2_f64 * t3477 * t90756 * t3479 + 0.14035736694323150897e2_f64 * t17097 * t24436 - 0.14035736694323150897e2_f64 * t12486 * t90357 * t1188 - 0.35089341735807877242e1_f64 * t3496 * t90352 * t1188 + 0.51947577317044391277e2_f64 * t3521 * t90352 * t3523 - t90329 + t90332 + t90336 - t90339 - t90341 - t90343 - 0.19751673498613801407e-1_f64 * t90319;
    t90775
}
