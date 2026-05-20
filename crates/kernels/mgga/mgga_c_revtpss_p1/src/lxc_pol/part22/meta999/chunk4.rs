//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3395/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395<F: Float>(t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t63377: F) -> F {
    let t63747 = F::new(0.72462e1) * t63336 - F::cast_from(0.80513333333333333333e0_f64) * t63338 + F::cast_from(0.26837777777777777778e0_f64) * t63340 + F::cast_from(0.22364814814814814814e0_f64) * t63342 - F::cast_from(0.33547222222222222222e0_f64) * t63346 - F::cast_from(0.89459259259259259259e0_f64) * t63351 + F::new(0.12077e1) * t63355 - F::cast_from(0.40256666666666666666e0_f64) * t63359 + F::new(0.12077e1) * t63361 + F::new(0.12077e1) * t63366 - F::new(0.181155e1) * t63369 - F::cast_from(0.80513333333333333333e0_f64) * t63371 - F::new(0.181155e1) * t63374 - F::new(0.198684e1) * t63377 + F::new(0.12077e1) * t52033;
    t63747
}
