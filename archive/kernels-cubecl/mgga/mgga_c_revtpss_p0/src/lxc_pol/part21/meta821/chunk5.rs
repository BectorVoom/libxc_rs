//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3043/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3043<F: Float>(t342: F, t379: F, t1000: F, t1076: F, t1079: F, t1096: F, t1097: F, t11128: F, t11174: F, t11177: F, t11187: F, t11210: F, t11220: F, t12173: F, t15579: F, t15648: F, t16295: F, t16314: F, t16340: F, t1651: F, t16591: F, t16592: F, t1696: F, t3047: F, t3052: F, t3058: F, t3060: F, t3269: F, t3271: F, t41993: F, t42041: F, t4773: F, t4778: F, t4947: F, t5016: F, t53108: F, t54112: F, t55458: F, t55461: F, t55464: F, t55475: F, t55524: F, t55562: F, t55607: F, t55643: F, t55676: F, t55711: F, t55746: F, t55783: F, t55822: F, t55854: F, t55894: F, t55926: F, t55966: F, t56001: F, t56041: F, t56075: F, t995: F, t996: F) -> F {
    let t56087 = t342 * t379;
    let t56099 = F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t1651 * t12173 - F::cast_from(0.79025390195226139182e1_f64) * t55458 * t16314 - F::cast_from(0.19756347548806534796e1_f64) * t55461 * t1097 + F::cast_from(0.39512695097613069591e1_f64) * t55464 * t3060 + F::cast_from(0.39512695097613069591e1_f64) * t11187 * t16295 - F::cast_from(0.19756347548806534796e1_f64) * t3052 * t16592 - F::cast_from(0.39512695097613069591e1_f64) * t11128 * t4773 - F::cast_from(0.65854491829355115987e0_f64) * t42041 * t1696 - F::cast_from(0.39512695097613069591e1_f64) * t55475 * t1000 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t54112 - F::cast_from(0.65854491829355115987e0_f64) * t4778 * t11174 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t16591 * t1096 + F::cast_from(0.39512695097613069591e1_f64) * t16340 * t3271 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1079 * (t55524 + t55562 + t55607 + t55643 + t55676 + t55711 + t55746 + t55783 + t55822 + t55854 + t55894 + t55926 + t55966 + t56001 + t56041 + t56075) - F::cast_from(0.19756347548806534796e1_f64) * t41993 * t1696 + F::cast_from(0.19756347548806534796e1_f64) * t3047 * t15579 - F::cast_from(0.11853808529283920877e2_f64) * t56087 * t53108 * t11177 - F::cast_from(0.19756347548806534796e1_f64) * t11210 * t5016 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t15648 * t1096 + F::cast_from(0.79025390195226139182e1_f64) * t11220 * t4947;
    t56099
}
