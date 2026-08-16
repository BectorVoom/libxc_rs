//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1631/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1631<F: Float>(t10952: F, t18714: F, t23168: F, t23177: F, t40902: F, t4526: F, t51498: F, t51646: F, t51660: F, t51676: F, t51686: F, t5978: F, t820: F, t87714: F, t87729: F, t87764: F, t87775: F, t879: F) -> F {
    let t87895 = -F::cast_from(0.18505311230957427423e-1_f64) * t51646 - F::cast_from(0.15805078039045227836e2_f64) * t820 * t51498 * t23168 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t40902 * t87764 - F::cast_from(0.23707617058567841754e2_f64) * t820 * t10952 * t87775 + F::cast_from(0.78548797528808629095e-3_f64) * t51660 - F::cast_from(0.78548797528808629095e-3_f64) * t51676 + F::cast_from(0.68293547082294194357e-1_f64) * t51686 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t879 * t87729 - F::cast_from(0.39512695097613069592e1_f64) * t820 * t18714 * t5978 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t87714 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t4526 * t23177;
    t87895
}
