//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 905/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk905(t1217: f64, t810: f64, t1261: f64, t2368: f64, t1216: f64, t308: f64, t2372: f64, t40: f64, t1243: f64, t1258: f64, t1262: f64, t2359: f64, t2363: f64, t295: f64, t305: f64, t6648: f64, t803: f64, t8316: f64, t8319: f64, t8320: f64, t8323: f64, t8326: f64, t8329: f64, t8337: f64, t8340: f64, t991: f64, t997: f64) -> (f64, f64, f64, f64) {
    let t8341 = t1217 * t810;
    let t8344 = t2368 * t1261;
    let t8347 = t308 * t1216;
    let t8350 = t2372 * t40;
    let t8353 = 200.0_f64 / 27.0_f64 * t1243 * t991 - 100.0_f64 / 27.0_f64 * t803 * t2359 - 50.0_f64 / 9.0_f64 * t803 * t2363 - 10.0_f64 / 27.0_f64 * t295 * t8316 + 20.0_f64 / 9.0_f64 * t8319 * t8320 + 10.0_f64 / 9.0_f64 * t295 * t8323 + 5.0_f64 / 3.0_f64 * t295 * t8326 - 5.0_f64 * t295 * t8329 - 50.0_f64 / 27.0_f64 * t997 * t1258 - 25.0_f64 / 9.0_f64 * t997 * t1262 - 10.0_f64 / 27.0_f64 * t305 * t8337 - 20.0_f64 / 9.0_f64 * t8340 * t8341 + 10.0_f64 / 9.0_f64 * t305 * t8344 - 5.0_f64 / 3.0_f64 * t305 * t8347 + 5.0_f64 * t305 * t8350 + t6648;
    (t8344, t8347, t8350, t8353)
}
