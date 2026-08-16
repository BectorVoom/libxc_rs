//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2774/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774(t16673: f64, t2642: f64, t41424: f64, t5587: f64, t13278: f64, t4236: f64, t13186: f64, t13196: f64, t13222: f64, t13251: f64, t13254: f64, t13306: f64, t13316: f64, t13350: f64, t1510: f64, t16891: f64, t16893: f64, t16896: f64, t16924: f64, t2633: f64, t2643: f64, t2649: f64, t4172: f64, t4178: f64, t4180: f64, t4182: f64, t46698: f64, t46717: f64, t46733: f64, t46742: f64, t46748: f64, t58495: f64, t58552: f64, t9632: f64, t9642: f64, t9646: f64) -> f64 {
    let t58642 = t16673 * t2642;
    let t58668 = t41424 * t5587;
    let t58670 = t13278 * t4236;
    let t58672 = t9642 * t16924 / 192.0_f64 + t13254 * t16893 / 768.0_f64 + t4178 * t4180 * t58495 * t4182 / 768.0_f64 + t4178 * t4180 * t16891 * t9632 / 1536.0_f64 + t58642 * t2649 / 384.0_f64 - t13251 * t13316 / 1536.0_f64 + t13251 * t13306 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t4178 * t9646 * t16896 * t2633 + t2643 * t13222 * t1510 * t58552 / 192.0_f64 - 7.0_f64 / 288.0_f64 * t46698 - 5.0_f64 / 384.0_f64 * t2643 * t13350 * t1510 * t13196 + 7.0_f64 / 1152.0_f64 * t46717 + 7.0_f64 / 1152.0_f64 * t46733 + 7.0_f64 / 384.0_f64 * t46742 - 7.0_f64 / 384.0_f64 * t46748 - 5.0_f64 / 64.0_f64 * t4172 * t13186 - 7.0_f64 / 1152.0_f64 * t58668 + 7.0_f64 / 1152.0_f64 * t58670;
    t58672
}
