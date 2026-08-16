//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1455/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1455(t2508: f64, t2580: f64, t32458: f64, t32461: f64, t32464: f64, t32466: f64, t32471: f64, t32474: f64, t32477: f64, t32480: f64, t32483: f64, t32485: f64, t32488: f64, t32490: f64, t39058: f64, t39091: f64, t7226: f64) -> f64 {
    let t39435 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t39091 - 0.46143157380853345701e-1_f64 * t2508 * t7226 * t39058 + t32458 - t32461 + t32464 + t32466 + t32471 + t32474 - t32477 + t32480 - t32483 - t32485 - t32488 - t32490;
    t39435
}
