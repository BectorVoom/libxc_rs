//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3218/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3218(t12640: f64, t1811: f64, t3601: f64, t5412: f64, t17807: f64, t473: f64, t3766: f64, t5216: f64, t13141: f64, t1770: f64, t1214: f64, t1234: f64, t1248: f64, t1285: f64, t1287: f64, t1291: f64, t12966: f64, t12987: f64, t13144: f64, t17331: f64, t17345: f64, t17822: f64, t17917: f64, t3666: f64, t3727: f64, t3759: f64, t3767: f64, t3769: f64, t3770: f64, t460: f64, t487: f64, t489: f64, t5284: f64, t58730: f64, t59453: f64) -> (f64, f64, f64) {
    let t59464 = t12640 * t1811;
    let t59476 = t5412 * t3601;
    let t59488 = t473 * t17807;
    let t59492 = t5216 * t3766;
    let t59498 = t1770 * t13141;
    let t59510 = 0.65854491829355115987e0_f64 * t1285 * t487 * t58730 * t1287 + 0.39512695097613069591e1_f64 * t3767 * t59476 * t3769 + 0.19756347548806534796e1_f64 * t1285 * t3727 * t5284 * t1287 + 0.19756347548806534796e1_f64 * t1285 * t17807 * t1248 * t1287 - 0.19756347548806534796e1_f64 * t1234 * t59488 * t1214 + 0.39512695097613069591e1_f64 * t59492 * t3770 - 0.11853808529283920877e2_f64 * t12987 * t3759 * t17345 - 0.39512695097613069591e1_f64 * t59498 * t13144 + 0.39512695097613069591e1_f64 * t12966 * t17917 - 0.39512695097613069591e1_f64 * t3666 * t17822 + 0.65854491829355115987e0_f64 * t460 * t489 * t59453 + 0.19756347548806534796e1_f64 * t17331 * t1291;
    (t59464, t59476, t59510)
}
