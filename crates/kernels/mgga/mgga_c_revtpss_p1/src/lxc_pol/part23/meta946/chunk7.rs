//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3123/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3123<F: Float>(t1210: F, t1214: F, t1215: F, t12628: F, t1274: F, t1277: F, t1294: F, t1295: F, t13182: F, t1770: F, t18065: F, t18097: F, t1828: F, t1829: F, t20697: F, t20744: F, t20748: F, t20759: F, t21082: F, t21344: F, t21366: F, t21394: F, t24515: F, t24616: F, t25015: F, t3556: F, t5216: F, t5220: F, t5246: F, t5423: F, t5497: F, t56314: F, t56315: F, t56332: F, t56416: F, t6574: F, t6588: F, t6697: F, t6702: F, t6745: F, t72927: F, t73187: F, t82147: F, t82150: F) -> F {
    let t82169 = -F::cast_from(0.11853808529283920877e2_f64) * t1274 * t13182 * t6702 * t5497 - F::cast_from(0.19756347548806534796e1_f64) * t18065 * t6745 + F::cast_from(0.19756347548806534796e1_f64) * t1770 * t21344 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t25015 * t1214 + F::cast_from(0.19756347548806534796e1_f64) * t5216 * t6697 + F::cast_from(0.39512695097613069591e1_f64) * t21394 * t5423 - F::cast_from(0.11853808529283920877e2_f64) * t56332 * t20748 + F::cast_from(0.39512695097613069591e1_f64) * t56416 * t6574 - F::cast_from(0.19756347548806534796e1_f64) * t20697 * t5246 - F::cast_from(0.11853808529283920877e2_f64) * t56314 * t56315 * t20759 + F::cast_from(0.39512695097613069591e1_f64) * t5220 * t21366 - F::cast_from(0.65854491829355115987e0_f64) * t82147 * t1295 - F::cast_from(0.19756347548806534796e1_f64) * t82150 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t12628 * t1277 * t24616 * t1294 - F::cast_from(0.39512695097613069591e1_f64) * t73187 * t1829 - F::cast_from(0.19756347548806534796e1_f64) * t18097 * t6588 - F::cast_from(0.79025390195226139182e1_f64) * t72927 * t20744 + F::cast_from(0.19756347548806534796e1_f64) * t3556 * t24515 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t21082 * t1828;
    t82169
}
