//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2334/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334(t20949: f64, t2697: f64, t20882: f64, t9638: f64, t13258: f64, t20988: f64, t13251: f64, t16853: f64, t16946: f64, t16949: f64, t16976: f64, t17013: f64, t2643: f64, t2645: f64, t41467: f64, t4172: f64, t4248: f64, t4257: f64, t46550: f64, t46628: f64, t5591: f64, t58461: f64, t58472: f64, t58474: f64, t58495: f64, t9642: f64) -> f64 {
    let t67675 = t2697 * t20949;
    let t67690 = t9638 * t20882;
    let t67692 = t13258 * t20988;
    let t67696 = -15.0_f64 / 128.0_f64 * t4172 * t16853 + 5.0_f64 / 128.0_f64 * t4172 * t16946 + 5.0_f64 / 256.0_f64 * t16976 * t4257 - 35.0_f64 / 384.0_f64 * t67675 + t46550 - t13251 * t17013 / 1024.0_f64 + 35.0_f64 / 384.0_f64 * t58461 - 15.0_f64 / 128.0_f64 * t46628 * t41467 * t4248 * t16949 + t9642 * t20882 / 256.0_f64 + t2643 * t2645 * t58495 * t5591 / 256.0_f64 - 7.0_f64 / 384.0_f64 * t67690 - 7.0_f64 / 768.0_f64 * t67692 - 7.0_f64 / 768.0_f64 * t58472 - 7.0_f64 / 384.0_f64 * t58474;
    t67696
}
