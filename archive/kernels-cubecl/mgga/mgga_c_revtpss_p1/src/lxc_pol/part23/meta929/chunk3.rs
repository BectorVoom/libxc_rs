//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3036/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3036<F: Float>(t378: F, t79862: F, t1000: F, t1073: F, t1076: F, t1079: F, t11121: F, t1652: F, t1695: F, t19380: F, t19385: F, t19403: F, t19425: F, t20188: F, t20195: F, t20204: F, t23607: F, t23959: F, t24047: F, t24068: F, t3063: F, t3269: F, t386: F, t42060: F, t43637: F, t4747: F, t4752: F, t4758: F, t4935: F, t4941: F, t5015: F, t53015: F, t6350: F, t6392: F, t64687: F, t64711: F, t64764: F, t78554: F, t79388: F, t995: F, t996: F, t999: F) -> F {
    let t80921 = t79862 * t378;
    let t80967 = F::cast_from(0.79025390195226139182e1_f64) * t4752 * t20195 - F::cast_from(0.65854491829355115987e0_f64) * t80921 * t1000 + F::cast_from(0.19756347548806534796e1_f64) * t20204 * t4941 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t19380 * t1695 - F::cast_from(0.39512695097613069591e1_f64) * t43637 * t24068 - F::cast_from(0.11853808529283920877e2_f64) * t53015 * t20188 - F::cast_from(0.39512695097613069591e1_f64) * t3063 * t23607 + F::cast_from(0.65854491829355115987e0_f64) * t79388 * t386 + F::cast_from(0.65854491829355115987e0_f64) * t23959 * t1073 - F::cast_from(0.39512695097613069591e1_f64) * t64764 * t1652 - F::cast_from(0.11853808529283920877e2_f64) * t4752 * t19425 + F::cast_from(0.39512695097613069591e1_f64) * t995 * t11121 * t24047 * t999 - F::cast_from(0.79025390195226139182e1_f64) * t64711 * t19403 - F::cast_from(0.11853808529283920877e2_f64) * t4935 * t19425 + F::cast_from(0.19756347548806534796e1_f64) * t4747 * t19385 + F::cast_from(0.15805078039045227836e2_f64) * t42060 * t996 * t78554 + F::cast_from(0.39512695097613069592e1_f64) * t64687 * t4758 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t5015 * t6392 - F::cast_from(0.11853808529283920877e2_f64) * t1076 * t11121 * t6350 * t5015;
    t80967
}
