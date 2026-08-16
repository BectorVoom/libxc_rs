//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3221/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3221(t1234: f64, t1280: f64, t12987: f64, t17192: f64, t17307: f64, t17958: f64, t21082: f64, t21448: f64, t21473: f64, t21491: f64, t21592: f64, t24912: f64, t24951: f64, t3666: f64, t45779: f64, t5443: f64, t5446: f64, t5459: f64, t5462: f64, t5466: f64, t5477: f64, t5481: f64, t5486: f64, t59674: f64, t6564: f64, t69637: f64, t72267: f64, t72386: f64, t82525: f64) -> f64 {
    let t84710 = 0.39512695097613069592e1_f64 * t6564 * t5462 * t5466 - 0.19756347548806534796e1_f64 * t6564 * t5477 * t5481 + 0.19756347548806534796e1_f64 * t59674 * t21473 - 0.39512695097613069591e1_f64 * t17192 * t21448 - 0.19756347548806534796e1_f64 * t72267 * t5446 - 0.19756347548806534796e1_f64 * t1234 * t5486 * t21082 - 0.11853808529283920877e2_f64 * t12987 * t1280 * t82525 - 0.19756347548806534796e1_f64 * t3666 * t24951 - 0.39512695097613069591e1_f64 * t17958 * t21491 + 0.79025390195226139182e1_f64 * t17307 * t21592 + 0.39512695097613069592e1_f64 * t69637 * t5443 - 0.39512695097613069591e1_f64 * t45779 * t24912 - 0.39512695097613069591e1_f64 * t72386 * t5459;
    t84710
}
