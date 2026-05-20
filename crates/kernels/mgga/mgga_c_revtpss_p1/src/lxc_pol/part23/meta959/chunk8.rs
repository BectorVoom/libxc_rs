//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3229/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3229<F: Float>(t24698: F, t487: F, t83107: F, t1215: F, t12633: F, t12641: F, t1271: F, t1295: F, t1775: F, t17986: F, t18054: F, t18087: F, t18114: F, t1829: F, t20741: F, t20759: F, t21389: F, t21407: F, t21408: F, t24509: F, t24906: F, t25016: F, t25019: F, t3561: F, t3732: F, t45449: F, t495: F, t5231: F, t5251: F, t5417: F, t6588: F, t6703: F, t6745: F, t72874: F, t72894: F, t72933: F, t73205: F, t83232: F) -> F {
    let t84952 = t24698 * t487;
    let t84967 = t83107 * t487;
    let t84992 = -F::cast_from(0.19756347548806534796e1_f64) * t18054 * t6745 - F::cast_from(0.19756347548806534796e1_f64) * t18114 * t6588 - F::cast_from(0.65854491829355115987e0_f64) * t84952 * t1295 - F::cast_from(0.39512695097613069591e1_f64) * t12633 * t24906 + F::cast_from(0.79025390195226139182e1_f64) * t72874 * t5231 - F::cast_from(0.39512695097613069591e1_f64) * t45449 * t25019 + F::cast_from(0.39512695097613069591e1_f64) * t18087 * t6703 - F::cast_from(0.39512695097613069591e1_f64) * t12641 * t24906 + F::cast_from(0.79025390195226139182e1_f64) * t5417 * t21408 - F::cast_from(0.65854491829355115987e0_f64) * t84967 * t1215 + F::cast_from(0.65854491829355115987e0_f64) * t83232 * t495 + F::cast_from(0.65854491829355115987e0_f64) * t24698 * t1271 - F::cast_from(0.65854491829355115987e0_f64) * t3561 * t25016 - F::cast_from(0.19756347548806534796e1_f64) * t73205 * t1775 - F::cast_from(0.39512695097613069591e1_f64) * t5251 * t20741 - F::cast_from(0.79025390195226139182e1_f64) * t17986 * t21389 * t21407 - F::cast_from(0.19756347548806534796e1_f64) * t72894 * t1829 - F::cast_from(0.39512695097613069592e1_f64) * t17986 * t21389 * t20759 - F::cast_from(0.19756347548806534796e1_f64) * t72933 * t1829 + F::cast_from(0.39512695097613069591e1_f64) * t3732 * t24509;
    t84992
}
