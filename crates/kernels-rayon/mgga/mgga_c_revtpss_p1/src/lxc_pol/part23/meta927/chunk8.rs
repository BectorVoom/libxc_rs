//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3017/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3017(t1086: f64, t23959: f64, t11249: f64, t24007: f64, t23997: f64, t3153: f64, t1083: f64, t1087: f64, t1089: f64, t1090: f64, t12127: f64, t1668: f64, t1678: f64, t1685: f64, t19447: f64, t19452: f64, t19477: f64, t19488: f64, t19503: f64, t20112: f64, t24167: f64, t3223: f64, t378: f64, t43341: f64, t4954: f64, t4998: f64, t55747: f64, t55988: f64, t64907: f64, t78721: f64, t79863: f64) -> (f64, f64, f64) {
    let t80243 = t23959 * t1086;
    let t80248 = t24007 * t11249;
    let t80264 = t23997 * t3153;
    let t80274 = 0.65854491829355115987e0_f64 * t1087 * t378 * t78721 * t1089 + 0.65854491829355115987e0_f64 * t80243 * t1090 + 0.79025390195226139182e1_f64 * t55747 * t19447 - 0.19756347548806534796e1_f64 * t43341 * t80248 * t19452 + 0.19756347548806534796e1_f64 * t1087 * t1678 * t19477 * t1089 - 0.65854491829355115987e0_f64 * t79863 * t1083 + 0.19756347548806534796e1_f64 * t4954 * t19488 + 0.19756347548806534796e1_f64 * t1087 * t20112 * t1668 * t1089 + 0.19756347548806534796e1_f64 * t12127 * t80264 * t4998 - 0.39512695097613069591e1_f64 * t55988 * t19503 - 0.19756347548806534796e1_f64 * t3223 * t24167 - 0.19756347548806534796e1_f64 * t64907 * t1685;
    (t80248, t80264, t80274)
}
