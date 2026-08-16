//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3086/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086(t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64) -> f64 {
    let t63994 = 40.0_f64 / 9.0_f64 * t63380 + 8.0_f64 / 27.0_f64 * t63382 + 8.0_f64 / 9.0_f64 * t63384 - 4.0_f64 / 3.0_f64 * t63388 - 8.0_f64 * t63392 - 4.0_f64 / 9.0_f64 * t63396 - 8.0_f64 / 9.0_f64 * t63398 - 4.0_f64 / 3.0_f64 * t63400 + 2.0_f64 * t63404 + 8.0_f64 * t63408 + 4.0_f64 / 3.0_f64 * t63412 + 10.0_f64 / 27.0_f64 * t63417 - 80.0_f64 / 81.0_f64 * t63422;
    t63994
}
