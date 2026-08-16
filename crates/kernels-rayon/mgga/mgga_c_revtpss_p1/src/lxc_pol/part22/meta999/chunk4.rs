//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3395/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395(t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t63377: f64) -> f64 {
    let t63747 = 0.72462e1_f64 * t63336 - 0.80513333333333333333e0_f64 * t63338 + 0.26837777777777777778e0_f64 * t63340 + 0.22364814814814814814e0_f64 * t63342 - 0.33547222222222222222e0_f64 * t63346 - 0.89459259259259259259e0_f64 * t63351 + 0.12077e1_f64 * t63355 - 0.40256666666666666666e0_f64 * t63359 + 0.12077e1_f64 * t63361 + 0.12077e1_f64 * t63366 - 0.181155e1_f64 * t63369 - 0.80513333333333333333e0_f64 * t63371 - 0.181155e1_f64 * t63374 - 0.198684e1_f64 * t63377 + 0.12077e1_f64 * t52033;
    t63747
}
