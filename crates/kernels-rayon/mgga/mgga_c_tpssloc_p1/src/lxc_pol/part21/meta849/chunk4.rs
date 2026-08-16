//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3076/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3076(t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64) -> f64 {
    let t63825 = 0.23744444444444444444e0_f64 * t63380 + 0.15829629629629629629e-1_f64 * t63382 + 0.47488888888888888888e-1_f64 * t63384 - 0.71233333333333333332e-1_f64 * t63388 - 0.42739999999999999999e0_f64 * t63392 - 0.23744444444444444444e-1_f64 * t63396 - 0.47488888888888888888e-1_f64 * t63398 - 0.71233333333333333333e-1_f64 * t63400 + 0.10685e0_f64 * t63404 + 0.4274e0_f64 * t63408 + 0.71233333333333333332e-1_f64 * t63412 + 0.19787037037037037037e-1_f64 * t63417 - 0.52765432098765432099e-1_f64 * t63422;
    t63825
}
