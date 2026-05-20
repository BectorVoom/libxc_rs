//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3418/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3418<F: Float>(t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t63377: F) -> F {
    let t64244 = F::new(0.123954e2) * t63336 - F::cast_from(0.13772666666666666667e1_f64) * t63338 + F::cast_from(0.45908888888888888889e0_f64) * t63340 + F::cast_from(0.38257407407407407407e0_f64) * t63342 - F::cast_from(0.57386111111111111112e0_f64) * t63346 - F::cast_from(0.15302962962962962963e1_f64) * t63351 + F::new(0.20659e1) * t63355 - F::cast_from(0.68863333333333333334e0_f64) * t63359 + F::new(0.20659e1) * t63361 + F::new(0.20659e1) * t63366 - F::new(0.309885e1) * t63369 - F::cast_from(0.13772666666666666667e1_f64) * t63371 - F::new(0.309885e1) * t63374 - F::new(0.250068e1) * t63377 + F::new(0.20659e1) * t52033;
    t64244
}
