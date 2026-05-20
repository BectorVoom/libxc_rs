//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3222/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3222<F: Float>(t1287: F, t1770: F, t17949: F, t17958: F, t20850: F, t20956: F, t21451: F, t21455: F, t21459: F, t21599: F, t25005: F, t3755: F, t45634: F, t45718: F, t45739: F, t5284: F, t5452: F, t5466: F, t5481: F, t59686: F, t59817: F, t60008: F, t60019: F, t6717: F, t82493: F) -> F {
    let t84741 = -F::cast_from(0.65854491829355115987e0_f64) * t3755 * t82493 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t60019 * t21599 - F::cast_from(0.39512695097613069591e1_f64) * t59686 * t6717 + F::cast_from(0.79025390195226139182e1_f64) * t1770 * t21451 * t5466 - F::cast_from(0.39512695097613069591e1_f64) * t1770 * t21455 * t5481 - F::cast_from(0.19756347548806534796e1_f64) * t17958 * t21459 - F::cast_from(0.19756347548806534796e1_f64) * t20850 * t5452 + F::cast_from(0.19756347548806534796e1_f64) * t17949 * t20956 * t45739 * t5284 + F::cast_from(0.39512695097613069591e1_f64) * t59817 * t21599 - F::cast_from(0.39512695097613069591e1_f64) * t60008 * t6717 + F::cast_from(0.19756347548806534796e1_f64) * t45718 * t25005 + F::cast_from(0.19756347548806534796e1_f64) * t45634 * t25005;
    t84741
}
