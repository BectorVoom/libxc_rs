//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1631/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1631(t10952: f64, t18714: f64, t23168: f64, t23177: f64, t40902: f64, t4526: f64, t51498: f64, t51646: f64, t51660: f64, t51676: f64, t51686: f64, t5978: f64, t820: f64, t87714: f64, t87729: f64, t87764: f64, t87775: f64, t879: f64) -> f64 {
    let t87895 = -0.18505311230957427423e-1_f64 * t51646 - 0.15805078039045227836e2_f64 * t820 * t51498 * t23168 + 0.15805078039045227836e2_f64 * t820 * t40902 * t87764 - 0.23707617058567841754e2_f64 * t820 * t10952 * t87775 + 0.78548797528808629095e-3_f64 * t51660 - 0.78548797528808629095e-3_f64 * t51676 + 0.68293547082294194357e-1_f64 * t51686 - 0.19756347548806534796e1_f64 * t820 * t879 * t87729 - 0.39512695097613069592e1_f64 * t820 * t18714 * t5978 - 0.65854491829355115987e0_f64 * t820 * t879 * t87714 - 0.26341796731742046395e1_f64 * t820 * t4526 * t23177;
    t87895
}
