//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1076/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1076(t529: f64, t2153: f64, t2308: f64, t30490: f64, t30498: f64, t31679: f64, t31695: f64, t382: f64, t525: f64, t526: f64, t6442: f64, t8011: f64, t8015: f64, t8292: f64) -> f64 {
    let t530 = t529 < -0.66725e-1_f64;
    let t31702 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t31679 * t382 - 10.0_f64 / 9.0_f64 * t525 * t8292 * t2153 + 40.0_f64 / 27.0_f64 * t525 * t2308 * t8011 - 10.0_f64 / 9.0_f64 * t525 * t2308 * t8015 - 280.0_f64 / 243.0_f64 * t525 * t526 * t30490 + 40.0_f64 / 27.0_f64 * t6442 * t31695 - 10.0_f64 / 27.0_f64 * t525 * t526 * t30498);
    t31702
}
