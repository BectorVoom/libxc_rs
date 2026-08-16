//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2807/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2807(t16888: f64, t9638: f64, t12971: f64, t13005: f64, t13191: f64, t13198: f64, t13350: f64, t1495: f64, t1510: f64, t17003: f64, t210: f64, t221: f64, t2553: f64, t2571: f64, t2643: f64, t41410: f64, t4172: f64, t47333: f64, t47353: f64, t5567: f64, t5571: f64, t5587: f64, t59198: f64, t59279: f64, t59282: f64, t59288: f64, t59298: f64, t59308: f64, t59310: f64, t776: f64, t9559: f64, t9642: f64) -> f64 {
    let t59322 = t9638 * t16888;
    let t59324 = 35.0_f64 / 96.0_f64 * t59279 + 7.0_f64 / 2304.0_f64 * t59282 + 5.0_f64 / 384.0_f64 * t4172 * t13198 + t41410 * t5587 / 1536.0_f64 + 119.0_f64 / 13824.0_f64 * t59288 - t9559 * t210 * t5567 * t2553 / 4.0_f64 + t2571 * t210 * t1495 * t12971 / 8.0_f64 - 7.0_f64 / 24.0_f64 * t59298 + t2571 * t210 * t17003 * t776 / 8.0_f64 + t2571 * t210 * t5571 * t2553 / 16.0_f64 - 7.0_f64 / 2304.0_f64 * t59308 - 7.0_f64 / 12.0_f64 * t59310 - 7.0_f64 / 24.0_f64 * t47333 - 5.0_f64 / 192.0_f64 * t9642 * t16888 - 5.0_f64 / 192.0_f64 * t2643 * t13350 * t1510 * t13191 + 35.0_f64 / 576.0_f64 * t47353 - t13005 * t221 * t59198 + 35.0_f64 / 288.0_f64 * t59322;
    t59324
}
