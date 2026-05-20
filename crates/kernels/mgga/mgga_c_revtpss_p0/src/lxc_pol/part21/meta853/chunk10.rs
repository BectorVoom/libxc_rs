//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3222/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3222<F: Float>(t12629: F, t12732: F, t12756: F, t12987: F, t13129: F, t16695: F, t17289: F, t17834: F, t21472: F, t3302: F, t3727: F, t3760: F, t3766: F, t3781: F, t45683: F, t45697: F, t45700: F, t45715: F, t45738: F, t45863: F, t460: F, t471: F, t5332: F, t5446: F, t5466: F, t5478: F, t5481: F, t5486: F, t57737: F, t59096: F, t59514: F) -> F {
    let t59649 = -F::cast_from(0.65854491829355115987e0_f64) * t5478 * t5332 * t3302 * t12732 * t471 + F::cast_from(0.39512695097613069591e1_f64) * t460 * t3766 * t3727 * t5466 - F::cast_from(0.19756347548806534796e1_f64) * t460 * t3781 * t3727 * t5481 - F::cast_from(0.39512695097613069591e1_f64) * t45715 * t17834 - F::cast_from(0.39512695097613069591e1_f64) * t45683 * t17834 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t5486 * t12629 - F::cast_from(0.65854491829355115987e0_f64) * t45738 * t59096 * t13129 - F::cast_from(0.19756347548806534796e1_f64) * t45697 * t5446 - F::cast_from(0.39512695097613069591e1_f64) * t45700 * t5446 - F::cast_from(0.39512695097613069591e1_f64) * t17289 * t3760 - F::cast_from(0.39512695097613069591e1_f64) * t45863 * t59514 * t21472 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t16695 * t57737;
    t59649
}
