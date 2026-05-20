//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3417/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3417<F: Float>(t41361: F, t41363: F, t41690: F, t51967: F, t51973: F, t51978: F, t63299: F, t63304: F, t63308: F, t63311: F, t63315: F, t63320: F, t63325: F, t63328: F, t63332: F) -> F {
    let t64228 = F::new(0.103295e1) * t63299 + F::cast_from(0.68863333333333333334e1_f64) * t63304 - F::new(0.123954e2) * t63308 + t41690 - F::new(0.125034e1) * t63311 + F::new(0.250068e1) * t63315 + F::cast_from(0.34431666666666666666e0_f64) * t51967 - F::cast_from(0.91817777777777777776e0_f64) * t51973 + F::cast_from(0.10712074074074074074e1_f64) * t51978 + F::cast_from(0.13892666666666666667e0_f64) * t63320 + F::cast_from(0.10712074074074074074e1_f64) * t41361 + F::cast_from(0.45908888888888888888e0_f64) * t41363 - F::cast_from(0.22954444444444444444e1_f64) * t63325 + F::cast_from(0.82636000000000000001e1_f64) * t63328 + F::new(0.41678e0) * t63332;
    t64228
}
