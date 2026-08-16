//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2771/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771(t120: f64, t16752: f64, t16924: f64, t9638: f64, t17004: f64, t2563: f64, t12971: f64, t13191: f64, t13222: f64, t13229: f64, t13242: f64, t13262: f64, t13263: f64, t13333: f64, t16836: f64, t16839: f64, t16891: f64, t16903: f64, t16912: f64, t17013: f64, t17017: f64, t20986: f64, t232: f64, t2643: f64, t2645: f64, t2679: f64, t41467: f64, t4178: f64, t4180: f64, t4181: f64, t4248: f64, t46558: f64, t46573: f64, t46577: f64, t46628: f64, t47307: f64, t58246: f64, t829: f64, t9642: f64) -> (f64, f64) {
    let t58495 = t120 * t16752;
    let t58504 = t9638 * t16924;
    let t58528 = t2563 * t17004;
    let t58540 = -t9642 * t17013 / 1536.0_f64 - t2643 * t4180 * t16839 * t2679 / 3072.0_f64 - t9642 * t17017 / 1536.0_f64 - t2643 * t4180 * t58495 * t829 / 1536.0_f64 - t2643 * t4180 * t16891 * t2679 / 3072.0_f64 - 7.0_f64 / 288.0_f64 * t58504 + t2643 * t2645 * t13242 * t16912 / 192.0_f64 + t2643 * t2645 * t4181 * t232 * t12971 / 384.0_f64 + t9642 * t16903 / 384.0_f64 + t16836 * t13333 / 256.0_f64 + t47307 * t4180 * t16839 * t58246 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t13262 * t4180 * t16839 * t13263 + 7.0_f64 / 72.0_f64 * t46558 + 7.0_f64 / 72.0_f64 * t58528 - 5.0_f64 / 32.0_f64 * t46628 * t41467 * t4248 * t13191 - t4178 * t13222 * t20986 * t13229 / 192.0_f64 - 119.0_f64 / 864.0_f64 * t46573 + 595.0_f64 / 1296.0_f64 * t46577;
    (t58495, t58540)
}
