//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3222/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3222(t1287: f64, t1770: f64, t17949: f64, t17958: f64, t20850: f64, t20956: f64, t21451: f64, t21455: f64, t21459: f64, t21599: f64, t25005: f64, t3755: f64, t45634: f64, t45718: f64, t45739: f64, t5284: f64, t5452: f64, t5466: f64, t5481: f64, t59686: f64, t59817: f64, t60008: f64, t60019: f64, t6717: f64, t82493: f64) -> f64 {
    let t84741 = -0.65854491829355115987e0_f64 * t3755 * t82493 * t1287 + 0.39512695097613069591e1_f64 * t60019 * t21599 - 0.39512695097613069591e1_f64 * t59686 * t6717 + 0.79025390195226139182e1_f64 * t1770 * t21451 * t5466 - 0.39512695097613069591e1_f64 * t1770 * t21455 * t5481 - 0.19756347548806534796e1_f64 * t17958 * t21459 - 0.19756347548806534796e1_f64 * t20850 * t5452 + 0.19756347548806534796e1_f64 * t17949 * t20956 * t45739 * t5284 + 0.39512695097613069591e1_f64 * t59817 * t21599 - 0.39512695097613069591e1_f64 * t60008 * t6717 + 0.19756347548806534796e1_f64 * t45718 * t25005 + 0.19756347548806534796e1_f64 * t45634 * t25005;
    t84741
}
