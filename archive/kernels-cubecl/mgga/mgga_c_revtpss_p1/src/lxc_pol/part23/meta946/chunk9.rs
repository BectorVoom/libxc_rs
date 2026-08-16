//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3125/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3125<F: Float>(t1770: F, t6695: F, t1210: F, t12641: F, t1277: F, t1294: F, t1295: F, t17986: F, t18005: F, t18065: F, t18097: F, t1813: F, t1829: F, t20697: F, t20700: F, t20728: F, t20748: F, t20753: F, t21333: F, t21347: F, t21382: F, t21408: F, t24633: F, t24892: F, t3736: F, t5220: F, t5225: F, t5231: F, t5251: F, t5423: F, t5428: F, t5429: F, t56393: F, t6580: F, t6587: F, t6703: F, t72767: F, t72784: F, t72843: F) -> F {
    let t82238 = t1770 * t6695;
    let t82266 = F::cast_from(0.39512695097613069592e1_f64) * t20700 * t5429 + F::cast_from(0.39512695097613069591e1_f64) * t5220 * t21382 + F::cast_from(0.39512695097613069592e1_f64) * t20753 * t5429 - F::cast_from(0.39512695097613069591e1_f64) * t72784 * t1829 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t24633 * t1294 + F::cast_from(0.39512695097613069591e1_f64) * t18065 * t6703 - F::cast_from(0.11853808529283920877e2_f64) * t56393 * t20748 - F::cast_from(0.19756347548806534796e1_f64) * t82238 * t1295 + F::cast_from(0.11853808529283920877e2_f64) * t17986 * t72843 * t21347 + F::cast_from(0.19756347548806534796e1_f64) * t21333 * t1813 + F::cast_from(0.19756347548806534796e1_f64) * t5251 * t20728 + F::cast_from(0.39512695097613069592e1_f64) * t72767 * t5231 + F::cast_from(0.39512695097613069591e1_f64) * t18097 * t6580 + F::cast_from(0.79025390195226139182e1_f64) * t5225 * t21408 - F::cast_from(0.39512695097613069592e1_f64) * t17986 * t3736 * t6587 * t5428 + F::cast_from(0.19756347548806534796e1_f64) * t5220 * t20728 + F::cast_from(0.19756347548806534796e1_f64) * t20697 * t5423 + F::cast_from(0.39512695097613069591e1_f64) * t18005 * t6703 + F::cast_from(0.39512695097613069591e1_f64) * t12641 * t24892;
    t82266
}
