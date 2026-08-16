//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2265/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2265(t2635: f64, t46881: f64, t13337: f64, t838: f64, t2693: f64, t4163: f64, t13080: f64, t13084: f64, t13223: f64, t13251: f64, t13254: f64, t13262: f64, t13350: f64, t1495: f64, t210: f64, t2553: f64, t2571: f64, t2643: f64, t2645: f64, t4158: f64, t4248: f64, t46870: f64, t46875: f64, t46876: f64, t46878: f64, t9516: f64, t9642: f64, t9647: f64, t9649: f64, t9976: f64) -> f64 {
    let t46882 = t46881 * t2635;
    let t46884 = t13337 * t838;
    let t46886 = t4163 * t2693;
    let t46887 = 119.0_f64 / 4608.0_f64 * t46886;
    let t46910 = -7.0_f64 / 384.0_f64 * t46870 + t46875 + 595.0_f64 / 10368.0_f64 * t46876 + t46878 * t2635 / 512.0_f64 - 7.0_f64 / 768.0_f64 * t46882 - 7.0_f64 / 1536.0_f64 * t46884 + t46887 - t13254 * t13084 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t9642 * t13080 - 5.0_f64 / 256.0_f64 * t13251 * t9649 + t13262 * t2645 * t4248 * t9976 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t13223 * t9647 + 3.0_f64 / 16.0_f64 * t2571 * t210 * t4158 * t2553 + t2571 * t210 * t1495 * t9516 / 16.0_f64;
    t46910
}
