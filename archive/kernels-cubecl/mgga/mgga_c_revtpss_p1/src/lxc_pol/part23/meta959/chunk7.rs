//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3228/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3228<F: Float>(t1210: F, t1211: F, t1214: F, t1274: F, t1277: F, t13182: F, t1775: F, t17973: F, t17974: F, t18037: F, t1829: F, t20710: F, t21348: F, t21365: F, t21366: F, t21394: F, t21618: F, t21621: F, t24524: F, t24525: F, t25019: F, t3732: F, t3737: F, t45427: F, t5220: F, t5225: F, t5231: F, t5237: F, t5245: F, t5251: F, t5422: F, t56327: F, t56607: F, t6574: F, t6580: F, t6702: F, t72802: F, t73137: F, t73222: F, t73236: F, t84175: F, t84392: F, t84425: F, t84461: F, t84506: F, t84541: F, t84570: F, t84605: F, t84641: F, t84679: F, t84710: F, t84741: F, t84778: F, t84816: F, t84851: F, t84887: F, t84917: F) -> F {
    let t84947 = -F::cast_from(0.39512695097613069591e1_f64) * t3732 * t24525 + F::cast_from(0.19756347548806534796e1_f64) * t21621 * t5237 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1211 * t84175 - F::cast_from(0.79025390195226139182e1_f64) * t17973 * t17974 * t21365 + F::cast_from(0.19756347548806534796e1_f64) * t5220 * t20710 - F::cast_from(0.19756347548806534796e1_f64) * t5225 * t21618 + F::cast_from(0.11853808529283920877e2_f64) * t56327 * t73236 * t5422 + F::cast_from(0.39512695097613069592e1_f64) * t72802 * t5231 - F::cast_from(0.11853808529283920877e2_f64) * t5225 * t21348 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1277 * (t84392 + t84425 + t84461 + t84506 + t84541 + t84570 + t84605 + t84641 + t84679 + t84710 + t84741 + t84778 + t84816 + t84851 + t84887 + t84917) + F::cast_from(0.39512695097613069591e1_f64) * t18037 * t6580 + F::cast_from(0.39512695097613069591e1_f64) * t21394 * t5237 - F::cast_from(0.39512695097613069591e1_f64) * t45427 * t25019 + F::cast_from(0.39512695097613069591e1_f64) * t5251 * t21366 - F::cast_from(0.19756347548806534796e1_f64) * t73137 * t1775 + F::cast_from(0.39512695097613069591e1_f64) * t1210 * t13182 * t24524 * t1214 + F::cast_from(0.39512695097613069591e1_f64) * t56607 * t6574 - F::cast_from(0.19756347548806534796e1_f64) * t73222 * t1829 - F::cast_from(0.39512695097613069591e1_f64) * t1210 * t3737 * t5245 * t6702;
    t84947
}
