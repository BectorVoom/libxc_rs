//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3221/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3221<F: Float>(t1234: F, t1280: F, t12987: F, t17192: F, t17307: F, t17958: F, t21082: F, t21448: F, t21473: F, t21491: F, t21592: F, t24912: F, t24951: F, t3666: F, t45779: F, t5443: F, t5446: F, t5459: F, t5462: F, t5466: F, t5477: F, t5481: F, t5486: F, t59674: F, t6564: F, t69637: F, t72267: F, t72386: F, t82525: F) -> F {
    let t84710 = F::cast_from(0.39512695097613069592e1_f64) * t6564 * t5462 * t5466 - F::cast_from(0.19756347548806534796e1_f64) * t6564 * t5477 * t5481 + F::cast_from(0.19756347548806534796e1_f64) * t59674 * t21473 - F::cast_from(0.39512695097613069591e1_f64) * t17192 * t21448 - F::cast_from(0.19756347548806534796e1_f64) * t72267 * t5446 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t5486 * t21082 - F::cast_from(0.11853808529283920877e2_f64) * t12987 * t1280 * t82525 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t24951 - F::cast_from(0.39512695097613069591e1_f64) * t17958 * t21491 + F::cast_from(0.79025390195226139182e1_f64) * t17307 * t21592 + F::cast_from(0.39512695097613069592e1_f64) * t69637 * t5443 - F::cast_from(0.39512695097613069591e1_f64) * t45779 * t24912 - F::cast_from(0.39512695097613069591e1_f64) * t72386 * t5459;
    t84710
}
