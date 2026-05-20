//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3394/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394<F: Float>(t41307: F, t41361: F, t41363: F, t51967: F, t51973: F, t51978: F, t63299: F, t63304: F, t63308: F, t63311: F, t63315: F, t63320: F, t63325: F, t63328: F, t63332: F) -> F {
    let t63731 = F::new(0.60385e0) * t63299 + F::cast_from(0.40256666666666666666e1_f64) * t63304 - F::new(0.72462e1) * t63308 + t41307 - F::new(0.99342e0) * t63311 + F::new(0.198684e1) * t63315 + F::cast_from(0.20128333333333333334e0_f64) * t51967 - F::cast_from(0.53675555555555555558e0_f64) * t51973 + F::cast_from(0.62621481481481481484e0_f64) * t51978 + F::new(0.11038e0) * t63320 + F::cast_from(0.62621481481481481482e0_f64) * t41361 + F::cast_from(0.26837777777777777778e0_f64) * t41363 - F::cast_from(0.13418888888888888889e1_f64) * t63325 + F::cast_from(0.48307999999999999999e1_f64) * t63328 + F::new(0.33114e0) * t63332;
    t63731
}
