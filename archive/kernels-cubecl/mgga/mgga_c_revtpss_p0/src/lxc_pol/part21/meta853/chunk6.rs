//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3218/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3218<F: Float>(t12640: F, t1811: F, t3601: F, t5412: F, t17807: F, t473: F, t3766: F, t5216: F, t13141: F, t1770: F, t1214: F, t1234: F, t1248: F, t1285: F, t1287: F, t1291: F, t12966: F, t12987: F, t13144: F, t17331: F, t17345: F, t17822: F, t17917: F, t3666: F, t3727: F, t3759: F, t3767: F, t3769: F, t3770: F, t460: F, t487: F, t489: F, t5284: F, t58730: F, t59453: F) -> (F, F, F) {
    let t59464 = t12640 * t1811;
    let t59476 = t5412 * t3601;
    let t59488 = t473 * t17807;
    let t59492 = t5216 * t3766;
    let t59498 = t1770 * t13141;
    let t59510 = F::cast_from(0.65854491829355115987e0_f64) * t1285 * t487 * t58730 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t59476 * t3769 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t3727 * t5284 * t1287 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t17807 * t1248 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t59488 * t1214 + F::cast_from(0.39512695097613069591e1_f64) * t59492 * t3770 - F::cast_from(0.11853808529283920877e2_f64) * t12987 * t3759 * t17345 - F::cast_from(0.39512695097613069591e1_f64) * t59498 * t13144 + F::cast_from(0.39512695097613069591e1_f64) * t12966 * t17917 - F::cast_from(0.39512695097613069591e1_f64) * t3666 * t17822 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t489 * t59453 + F::cast_from(0.19756347548806534796e1_f64) * t17331 * t1291;
    (t59464, t59476, t59510)
}
