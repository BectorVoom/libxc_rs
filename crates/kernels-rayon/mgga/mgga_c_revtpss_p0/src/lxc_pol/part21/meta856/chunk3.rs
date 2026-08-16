//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3250/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3250(t10317: f64, t10328: f64, t10331: f64, t10336: f64, t13343: f64, t13346: f64, t13389: f64, t1494: f64, t2258: f64, t2259: f64, t2260: f64, t2263: f64, t2312: f64, t4196: f64, t4217: f64, t4238: f64, t608: f64, t641: f64, t7719: f64, t85: f64) -> f64 {
    let t60417 = -t2259 * t4217 * t85 / 4.0_f64 - t13343 * t641 / 4.0_f64 - t13346 * t641 / 2.0_f64 - t4196 * t2312 / 4.0_f64 - t10317 * t7719 * t2258 / 4.0_f64 - t10328 * t1494 / 12.0_f64 - t10331 * t1494 / 4.0_f64 - t2260 * t4238 / 4.0_f64 - t10336 * t1494 / 4.0_f64 - t2263 * t4238 / 2.0_f64 - t608 * t13389 / 4.0_f64;
    t60417
}
