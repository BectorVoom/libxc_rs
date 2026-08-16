//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1479/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1479<F: Float>(t11465: F, t3006: F, t3015: F, t981: F, t11602: F, t3022: F, t3329: F, t3325: F, t1071: F, t3043: F, t1076: F, t1078: F, t1079: F, t1097: F, t11123: F, t11128: F, t11174: F, t11177: F, t11178: F, t11184: F, t11187: F, t11190: F, t12040: F, t12178: F, t16312: F, t16603: F, t3047: F, t3052: F, t3058: F, t3059: F, t3063: F, t3066: F, t3075: F, t3076: F, t3261: F, t3264: F, t3268: F, t3269: F, t995: F, t999: F) -> (F, F, F, F) {
    let t41947 = F::cast_from(0.62337092780453269531e3_f64) * t981 * t11465 * t3006 * t3015;
    let t41949 = F::cast_from(0.14035736694323150897e2_f64) * t3022 * t11602;
    let t41950 = t3329 * t3329;
    let t41983 = t3325 * t3325;
    let t41993 = t3043 * t1071;
    let t42000 = -F::cast_from(0.15805078039045227836e2_f64) * t16603 * t3268 * t999 * t11177 - F::cast_from(0.39512695097613069592e1_f64) * t11190 * t3076 - F::cast_from(0.15805078039045227836e2_f64) * t11187 * t12178 - F::cast_from(0.79025390195226139183e1_f64) * t3058 * t1079 * t3059 * t3325 - F::cast_from(0.15805078039045227836e2_f64) * t16312 * t1078 * t3075 * t3066 + F::cast_from(0.15805078039045227836e2_f64) * t3052 * t11178 + F::cast_from(0.39512695097613069592e1_f64) * t995 * t1079 * t3075 * t3325 - F::cast_from(0.79025390195226139183e1_f64) * t11128 * t3076 + F::cast_from(0.79025390195226139183e1_f64) * t3047 * t11184 + F::cast_from(0.15805078039045227836e2_f64) * t3264 * t11178 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t41983 - F::cast_from(0.15805078039045227836e2_f64) * t3047 * t12040 + F::cast_from(0.39512695097613069592e1_f64) * t3043 * t3261 + F::cast_from(0.79025390195226139183e1_f64) * t3063 * t11184 - F::cast_from(0.79025390195226139183e1_f64) * t41993 * t1097 - F::cast_from(0.26341796731742046395e1_f64) * t3063 * t11174 - F::cast_from(0.15805078039045227836e2_f64) * t3052 * t11123;
    (t41947, t41949, t41950, t42000)
}
