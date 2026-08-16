//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2780/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780(t119: f64, t12971: f64, t13222: f64, t13229: f64, t13254: f64, t13262: f64, t13347: f64, t13365: f64, t1484: f64, t1516: f64, t16901: f64, t16932: f64, t16937: f64, t16946: f64, t210: f64, t2553: f64, t2623: f64, t2643: f64, t2645: f64, t2684: f64, t2701: f64, t4172: f64, t4191: f64, t4261: f64, t46570: f64, t47037: f64, t47044: f64, t5527: f64, t58139: f64, t58845: f64, t58847: f64, t58853: f64, t58859: f64, t58873: f64, t58885: f64, t787: f64, t820: f64, t843: f64, t9607: f64) -> f64 {
    let t58887 = -t46570 * t1516 / 384.0_f64 - t13365 * t4261 / 192.0_f64 - t4172 * t13347 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t58845 + 7.0_f64 / 288.0_f64 * t58847 - t787 * t210 * t119 * t58139 / 48.0_f64 + t13262 * t13222 * t58853 * t13229 / 64.0_f64 + 35.0_f64 / 288.0_f64 * t47037 + 7.0_f64 / 2304.0_f64 * t58859 + 5.0_f64 / 192.0_f64 * t2623 * t16946 + 5.0_f64 / 384.0_f64 * t843 * t2701 * t820 * t1484 * t12971 - 5.0_f64 / 128.0_f64 * t843 * t9607 * t820 * t5527 * t2553 - 7.0_f64 / 288.0_f64 * t58873 + t47044 * t4191 / 192.0_f64 + t2643 * t2645 * t16901 * t2684 / 768.0_f64 - t13254 * t16932 / 192.0_f64 + t13254 * t16937 / 384.0_f64 - 7.0_f64 / 576.0_f64 * t58885;
    t58887
}
