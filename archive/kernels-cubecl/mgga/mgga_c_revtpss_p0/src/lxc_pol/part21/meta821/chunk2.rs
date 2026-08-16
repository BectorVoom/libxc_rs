//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3040/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3040<F: Float>(t3316: F, t4743: F, t19602: F, t994: F, t19607: F, t1082: F, t1087: F, t1089: F, t1093: F, t11620: F, t12047: F, t12052: F, t12070: F, t12086: F, t12100: F, t12124: F, t12128: F, t15886: F, t16496: F, t16509: F, t16566: F, t16568: F, t1678: F, t3204: F, t3278: F, t3319: F, t381: F, t4857: F, t4954: F, t52977: F, t54112: F, t54479: F, t55586: F) -> F {
    let t55985 = t4743 * t3316;
    let t55988 = t994 * t19602;
    let t55991 = t994 * t19607;
    let t56001 = F::cast_from(0.19756347548806534796e1_f64) * t16566 * t54479 * t16568 + F::cast_from(0.65854491829355115987e0_f64) * t52977 * t381 + F::cast_from(0.19756347548806534796e1_f64) * t15886 * t1093 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t1678 * t11620 * t1089 - F::cast_from(0.19756347548806534796e1_f64) * t4857 * t12100 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t54112 + F::cast_from(0.39512695097613069591e1_f64) * t3278 * t16496 - F::cast_from(0.19756347548806534796e1_f64) * t55985 * t3319 - F::cast_from(0.39512695097613069591e1_f64) * t55988 * t12124 + F::cast_from(0.19756347548806534796e1_f64) * t55991 * t12128 + F::cast_from(0.39512695097613069591e1_f64) * t16509 * t12086 + F::cast_from(0.65854491829355115987e0_f64) * t4954 * t12070 + F::cast_from(0.65854491829355115987e0_f64) * t12047 * t55586 * t12052;
    t56001
}
