//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3225/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3225(t12050: f64, t1214: f64, t12751: f64, t1287: f64, t16695: f64, t1770: f64, t17853: f64, t17854: f64, t17861: f64, t17949: f64, t1825: f64, t20956: f64, t21333: f64, t21527: f64, t21583: f64, t21587: f64, t24934: f64, t24964: f64, t3666: f64, t3755: f64, t5284: f64, t5458: f64, t59705: f64, t6717: f64, t6731: f64, t70890: f64, t72429: f64, t72432: f64, t82476: f64, t82725: f64, t83792: f64, t84645: f64) -> f64 {
    let t84851 = -0.19756347548806534796e1_f64 * t3755 * t82476 * t1287 - 0.39512695097613069591e1_f64 * t59705 * t6717 - 0.11853808529283920877e2_f64 * t17853 * t20956 * t17854 * t5284 - 0.65854491829355115987e0_f64 * t3755 * t82725 * t5458 + 0.11853808529283920877e2_f64 * t72429 * t21583 - 0.11853808529283920877e2_f64 * t72432 * t21587 - 0.39512695097613069591e1_f64 * t12751 * t16695 * t84645 * t1214 + 0.39512695097613069591e1_f64 * t17861 * t6731 - 0.19756347548806534796e1_f64 * t3666 * t24934 + 0.19756347548806534796e1_f64 * t1770 * t21527 - 0.65854491829355115987e0_f64 * t3666 * t24964 + 0.19756347548806534796e1_f64 * t21333 * t1825 + 0.19756347548806534796e1_f64 * t17949 * t70890 * t12050 * t83792;
    t84851
}
