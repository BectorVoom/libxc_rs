//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2770/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770(t16898: f64, t9638: f64, t13258: f64, t16893: f64, t16918: f64, t4191: f64, t46657: f64, t4240: f64, t120: f64, t13076: f64, t13171: f64, t13251: f64, t13326: f64, t16662: f64, t16839: f64, t16896: f64, t16901: f64, t16976: f64, t17009: f64, t2643: f64, t2645: f64, t2679: f64, t2684: f64, t2707: f64, t41448: f64, t4178: f64, t4180: f64, t4181: f64, t46549: f64, t46551: f64, t5624: f64, t829: f64, t9642: f64, t9646: f64, t9990: f64) -> f64 {
    let t58461 = t9638 * t16898;
    let t58472 = t13258 * t16893;
    let t58474 = t9638 * t16918;
    let t58480 = t46657 * t4191;
    let t58482 = t46657 * t4240;
    let t58486 = -t16976 * t2707 / 768.0_f64 + 5.0_f64 / 768.0_f64 * t9990 * t5624 + 595.0_f64 / 864.0_f64 * t46549 - 35.0_f64 / 288.0_f64 * t46551 - t9642 * t17009 / 768.0_f64 - t2643 * t4180 * t4181 * t13171 / 1536.0_f64 - 5.0_f64 / 768.0_f64 * t2643 * t9646 * t16896 * t2684 + 5.0_f64 / 384.0_f64 * t4178 * t9646 * t16839 * t41448 + 35.0_f64 / 576.0_f64 * t58461 + t2643 * t2645 * t120 * t16662 * t829 / 384.0_f64 + t2643 * t2645 * t16901 * t2679 / 768.0_f64 - 7.0_f64 / 1152.0_f64 * t58472 - 7.0_f64 / 576.0_f64 * t58474 - t13251 * t13076 / 1536.0_f64 + t9642 * t16918 / 384.0_f64 - 7.0_f64 / 288.0_f64 * t58480 + 7.0_f64 / 1152.0_f64 * t58482 + t13251 * t13326 / 384.0_f64;
    t58486
}
