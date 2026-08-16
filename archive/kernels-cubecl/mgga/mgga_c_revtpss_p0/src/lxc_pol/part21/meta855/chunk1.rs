//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3236/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3236<F: Float>(t1209: F, t17807: F, t3727: F, t5219: F, t1204: F, t1210: F, t1215: F, t12599: F, t12621: F, t12622: F, t12633: F, t12641: F, t12647: F, t12650: F, t12651: F, t12673: F, t1277: F, t13166: F, t17973: F, t17974: F, t17979: F, t17986: F, t18037: F, t18043: F, t18059: F, t18070: F, t18073: F, t18109: F, t18114: F, t1828: F, t3556: F, t3572: F, t3575: F, t3585: F, t3732: F, t3736: F, t5251: F, t5417: F, t5497: F, t5498: F, t56327: F) -> F {
    let t60087 = t1209 * t17807;
    let t60106 = t5219 * t3727;
    let t60117 = F::cast_from(0.11853808529283920877e2_f64) * t56327 * t17974 * t12599 + F::cast_from(0.19756347548806534796e1_f64) * t1204 * t17979 + F::cast_from(0.79025390195226139182e1_f64) * t3732 * t18109 - F::cast_from(0.39512695097613069591e1_f64) * t17973 * t17974 * t12650 - F::cast_from(0.19756347548806534796e1_f64) * t12673 * t5498 - F::cast_from(0.79025390195226139182e1_f64) * t17986 * t3736 * t5497 * t3575 + F::cast_from(0.39512695097613069591e1_f64) * t12641 * t18073 - F::cast_from(0.19756347548806534796e1_f64) * t60087 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t5251 * t12622 + F::cast_from(0.39512695097613069591e1_f64) * t3556 * t18043 - F::cast_from(0.65854491829355115987e0_f64) * t5417 * t13166 + F::cast_from(0.39512695097613069591e1_f64) * t18059 * t12647 - F::cast_from(0.19756347548806534796e1_f64) * t18037 * t3585 + F::cast_from(0.39512695097613069591e1_f64) * t3572 * t18043 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t1828 * t12621 - F::cast_from(0.19756347548806534796e1_f64) * t60106 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t12633 * t18073 - F::cast_from(0.19756347548806534796e1_f64) * t18114 * t3585 + F::cast_from(0.79025390195226139182e1_f64) * t12641 * t18070 + F::cast_from(0.19756347548806534796e1_f64) * t5251 * t12651;
    t60117
}
