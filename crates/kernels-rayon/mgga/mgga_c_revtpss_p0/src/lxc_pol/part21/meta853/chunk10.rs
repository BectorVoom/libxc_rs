//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3222/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3222(t12629: f64, t12732: f64, t12756: f64, t12987: f64, t13129: f64, t16695: f64, t17289: f64, t17834: f64, t21472: f64, t3302: f64, t3727: f64, t3760: f64, t3766: f64, t3781: f64, t45683: f64, t45697: f64, t45700: f64, t45715: f64, t45738: f64, t45863: f64, t460: f64, t471: f64, t5332: f64, t5446: f64, t5466: f64, t5478: f64, t5481: f64, t5486: f64, t57737: f64, t59096: f64, t59514: f64) -> f64 {
    let t59649 = -0.65854491829355115987e0_f64 * t5478 * t5332 * t3302 * t12732 * t471 + 0.39512695097613069591e1_f64 * t460 * t3766 * t3727 * t5466 - 0.19756347548806534796e1_f64 * t460 * t3781 * t3727 * t5481 - 0.39512695097613069591e1_f64 * t45715 * t17834 - 0.39512695097613069591e1_f64 * t45683 * t17834 - 0.39512695097613069591e1_f64 * t12987 * t5486 * t12629 - 0.65854491829355115987e0_f64 * t45738 * t59096 * t13129 - 0.19756347548806534796e1_f64 * t45697 * t5446 - 0.39512695097613069591e1_f64 * t45700 * t5446 - 0.39512695097613069591e1_f64 * t17289 * t3760 - 0.39512695097613069591e1_f64 * t45863 * t59514 * t21472 + 0.19756347548806534796e1_f64 * t12756 * t16695 * t57737;
    t59649
}
