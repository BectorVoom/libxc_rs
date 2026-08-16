//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1141/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1141(t1160: f64, t1304: f64, t11346: f64, t1535: f64, t2857: f64, t3742: f64, t11267: f64, t11279: f64, t11283: f64, t11315: f64, t11320: f64, t11329: f64, t11336: f64, t11343: f64, t2821: f64, t2824: f64, t2834: f64, t2838: f64, t3665: f64, t3680: f64, t3688: f64, t4491: f64, t7643: f64, t7800: f64, t9639: f64, t9663: f64, t9670: f64, t9678: f64, t9700: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t11347 = t1304 * t1160;
    let t11348 = t11347 * sigma2;
    let t11349 = t11346 * t11348;
    let t11353 = t1535 * t2857 * sigma2;
    let t11354 = t11353 * t3742;
    let t11357 = -16.0_f64 / 9.0_f64 * t9700 * t4491 - 8.0_f64 / 3.0_f64 * t7643 * t11267 - 40.0_f64 / 3.0_f64 * t7800 * t11315 * t2824 - 8.0_f64 / 3.0_f64 * t2834 * t11320 + 400.0_f64 / 9.0_f64 * t3680 * t11279 + 8.0_f64 / 3.0_f64 * t2838 * t11283 + 400.0_f64 / 9.0_f64 * t3688 * t11279 - 100.0_f64 / 9.0_f64 * t11329 * t3665 - 8.0_f64 / 9.0_f64 * t2821 * t11320 + 64.0_f64 / 27.0_f64 * t9670 * t11336 + 64.0_f64 / 9.0_f64 * t9639 * t11336 - 128.0_f64 / 81.0_f64 * t9678 * t11343 + 800.0_f64 / 81.0_f64 * t9663 * t11349 - 64.0_f64 / 27.0_f64 * t9663 * t11354;
    (t11348, t11349, t11353, t11354, t11357)
}
