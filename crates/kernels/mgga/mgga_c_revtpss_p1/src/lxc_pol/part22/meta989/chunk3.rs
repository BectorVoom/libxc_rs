//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3363/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363<F: Float>(t52011: F, t52018: F, t60927: F, t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F) -> (F, F) {
    let t63377 = t52011 * t52018 * t60927;
    let t63380 = F::new(0.71752e1) * t63336 - F::cast_from(0.79724444444444444445e0_f64) * t63338 + F::cast_from(0.26574814814814814814e0_f64) * t63340 + F::cast_from(0.22145679012345679012e0_f64) * t63342 - F::cast_from(0.33218518518518518518e0_f64) * t63346 - F::cast_from(0.88582716049382716048e0_f64) * t63351 + F::cast_from(0.11958666666666666667e1_f64) * t63355 - F::cast_from(0.39862222222222222222e0_f64) * t63359 + F::cast_from(0.11958666666666666667e1_f64) * t63361 + F::cast_from(0.11958666666666666667e1_f64) * t63366 - F::new(0.17938e1) * t63369 - F::cast_from(0.79724444444444444445e0_f64) * t63371 - F::new(0.17938e1) * t63374 - F::new(0.197176e1) * t63377 + F::cast_from(0.11958666666666666667e1_f64) * t52033;
    (t63377, t63380)
}
