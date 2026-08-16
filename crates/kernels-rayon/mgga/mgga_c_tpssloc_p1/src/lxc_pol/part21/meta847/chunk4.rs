//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3067/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3067(t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64) -> f64 {
    let t63706 = 0.12361111111111111111e0_f64 * t63380 + 0.82407407407407407407e-2_f64 * t63382 + 0.24722222222222222222e-1_f64 * t63384 - 0.37083333333333333333e-1_f64 * t63388 - 0.22249999999999999999e0_f64 * t63392 - 0.12361111111111111111e-1_f64 * t63396 - 0.24722222222222222222e-1_f64 * t63398 - 0.37083333333333333334e-1_f64 * t63400 + 0.55625000000000000001e-1_f64 * t63404 + 0.2225e0_f64 * t63408 + 0.37083333333333333334e-1_f64 * t63412 + 0.10300925925925925926e-1_f64 * t63417 - 0.27469135802469135803e-1_f64 * t63422;
    t63706
}
