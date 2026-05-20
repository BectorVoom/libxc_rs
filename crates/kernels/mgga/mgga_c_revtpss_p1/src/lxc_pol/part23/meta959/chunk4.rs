//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3225/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3225<F: Float>(t12050: F, t1214: F, t12751: F, t1287: F, t16695: F, t1770: F, t17853: F, t17854: F, t17861: F, t17949: F, t1825: F, t20956: F, t21333: F, t21527: F, t21583: F, t21587: F, t24934: F, t24964: F, t3666: F, t3755: F, t5284: F, t5458: F, t59705: F, t6717: F, t6731: F, t70890: F, t72429: F, t72432: F, t82476: F, t82725: F, t83792: F, t84645: F) -> F {
    let t84851 = -F::cast_from(0.19756347548806534796e1_f64) * t3755 * t82476 * t1287 - F::cast_from(0.39512695097613069591e1_f64) * t59705 * t6717 - F::cast_from(0.11853808529283920877e2_f64) * t17853 * t20956 * t17854 * t5284 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t82725 * t5458 + F::cast_from(0.11853808529283920877e2_f64) * t72429 * t21583 - F::cast_from(0.11853808529283920877e2_f64) * t72432 * t21587 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t16695 * t84645 * t1214 + F::cast_from(0.39512695097613069591e1_f64) * t17861 * t6731 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t24934 + F::cast_from(0.19756347548806534796e1_f64) * t1770 * t21527 - F::cast_from(0.65854491829355115987e0_f64) * t3666 * t24964 + F::cast_from(0.19756347548806534796e1_f64) * t21333 * t1825 + F::cast_from(0.19756347548806534796e1_f64) * t17949 * t70890 * t12050 * t83792;
    t84851
}
