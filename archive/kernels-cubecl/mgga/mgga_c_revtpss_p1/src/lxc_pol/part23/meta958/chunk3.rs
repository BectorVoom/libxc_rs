//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3216/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216<F: Float>(t1234: F, t12751: F, t1291: F, t17821: F, t21507: F, t21542: F, t21587: F, t24698: F, t25002: F, t3766: F, t3769: F, t3781: F, t45683: F, t45738: F, t45740: F, t460: F, t490: F, t5326: F, t5465: F, t5466: F, t5481: F, t6587: F, t6695: F, t72343: F, t72732: F, t82293: F, t82321: F, t83232: F, t84487: F) -> F {
    let t84541 = -F::cast_from(0.39512695097613069592e1_f64) * t12751 * t84487 * t5465 + F::cast_from(0.65854491829355115987e0_f64) * t83232 * t490 + F::cast_from(0.65854491829355115987e0_f64) * t24698 * t1291 - F::cast_from(0.39512695097613069591e1_f64) * t45683 * t25002 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t82321 * t3769 - F::cast_from(0.11853808529283920877e2_f64) * t72343 * t21587 + F::cast_from(0.19756347548806534796e1_f64) * t72732 * t21507 - F::cast_from(0.65854491829355115987e0_f64) * t45738 * t82293 * t45740 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t17821 * t6587 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t21542 + F::cast_from(0.39512695097613069592e1_f64) * t460 * t3766 * t6695 * t5466 - F::cast_from(0.19756347548806534796e1_f64) * t460 * t3781 * t6695 * t5481;
    t84541
}
