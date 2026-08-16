//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2698/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698(t19924: f64, t19994: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t5122: f64, t5126: f64, t54431: f64, t54436: f64, t74484: f64, t74485: f64, t74486: f64) -> f64 {
    let t75237 = 36.0_f64 * t19924 * t5122 * t5126 + 18.0_f64 * t19994 * t5122 * t5126 - t39585 + t39590 - t39593 + t39595 - t54431 - t54436 + t74484 + t74485 - t74486;
    t75237
}
