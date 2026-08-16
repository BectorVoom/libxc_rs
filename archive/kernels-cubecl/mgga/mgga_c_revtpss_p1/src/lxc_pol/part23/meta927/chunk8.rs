//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3017/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3017<F: Float>(t1086: F, t23959: F, t11249: F, t24007: F, t23997: F, t3153: F, t1083: F, t1087: F, t1089: F, t1090: F, t12127: F, t1668: F, t1678: F, t1685: F, t19447: F, t19452: F, t19477: F, t19488: F, t19503: F, t20112: F, t24167: F, t3223: F, t378: F, t43341: F, t4954: F, t4998: F, t55747: F, t55988: F, t64907: F, t78721: F, t79863: F) -> (F, F, F) {
    let t80243 = t23959 * t1086;
    let t80248 = t24007 * t11249;
    let t80264 = t23997 * t3153;
    let t80274 = F::cast_from(0.65854491829355115987e0_f64) * t1087 * t378 * t78721 * t1089 + F::cast_from(0.65854491829355115987e0_f64) * t80243 * t1090 + F::cast_from(0.79025390195226139182e1_f64) * t55747 * t19447 - F::cast_from(0.19756347548806534796e1_f64) * t43341 * t80248 * t19452 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t1678 * t19477 * t1089 - F::cast_from(0.65854491829355115987e0_f64) * t79863 * t1083 + F::cast_from(0.19756347548806534796e1_f64) * t4954 * t19488 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t20112 * t1668 * t1089 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t80264 * t4998 - F::cast_from(0.39512695097613069591e1_f64) * t55988 * t19503 - F::cast_from(0.19756347548806534796e1_f64) * t3223 * t24167 - F::cast_from(0.19756347548806534796e1_f64) * t64907 * t1685;
    (t80248, t80264, t80274)
}
