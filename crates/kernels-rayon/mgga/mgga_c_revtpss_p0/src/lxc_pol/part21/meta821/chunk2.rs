//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3040/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3040(t3316: f64, t4743: f64, t19602: f64, t994: f64, t19607: f64, t1082: f64, t1087: f64, t1089: f64, t1093: f64, t11620: f64, t12047: f64, t12052: f64, t12070: f64, t12086: f64, t12100: f64, t12124: f64, t12128: f64, t15886: f64, t16496: f64, t16509: f64, t16566: f64, t16568: f64, t1678: f64, t3204: f64, t3278: f64, t3319: f64, t381: f64, t4857: f64, t4954: f64, t52977: f64, t54112: f64, t54479: f64, t55586: f64) -> f64 {
    let t55985 = t4743 * t3316;
    let t55988 = t994 * t19602;
    let t55991 = t994 * t19607;
    let t56001 = 0.19756347548806534796e1_f64 * t16566 * t54479 * t16568 + 0.65854491829355115987e0_f64 * t52977 * t381 + 0.19756347548806534796e1_f64 * t15886 * t1093 + 0.65854491829355115987e0_f64 * t1087 * t1678 * t11620 * t1089 - 0.19756347548806534796e1_f64 * t4857 * t12100 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t54112 + 0.39512695097613069591e1_f64 * t3278 * t16496 - 0.19756347548806534796e1_f64 * t55985 * t3319 - 0.39512695097613069591e1_f64 * t55988 * t12124 + 0.19756347548806534796e1_f64 * t55991 * t12128 + 0.39512695097613069591e1_f64 * t16509 * t12086 + 0.65854491829355115987e0_f64 * t4954 * t12070 + 0.65854491829355115987e0_f64 * t12047 * t55586 * t12052;
    t56001
}
