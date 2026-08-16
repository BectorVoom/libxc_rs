//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3363/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363(t52011: f64, t52018: f64, t60927: f64, t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64) -> (f64, f64) {
    let t63377 = t52011 * t52018 * t60927;
    let t63380 = 0.71752e1_f64 * t63336 - 0.79724444444444444445e0_f64 * t63338 + 0.26574814814814814814e0_f64 * t63340 + 0.22145679012345679012e0_f64 * t63342 - 0.33218518518518518518e0_f64 * t63346 - 0.88582716049382716048e0_f64 * t63351 + 0.11958666666666666667e1_f64 * t63355 - 0.39862222222222222222e0_f64 * t63359 + 0.11958666666666666667e1_f64 * t63361 + 0.11958666666666666667e1_f64 * t63366 - 0.17938e1_f64 * t63369 - 0.79724444444444444445e0_f64 * t63371 - 0.17938e1_f64 * t63374 - 0.197176e1_f64 * t63377 + 0.11958666666666666667e1_f64 * t52033;
    (t63377, t63380)
}
