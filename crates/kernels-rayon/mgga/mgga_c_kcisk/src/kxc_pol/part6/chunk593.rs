//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 593/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk593(t529: f64, t1459: f64, t8286: f64, t2331: f64, t4350: f64, t41: f64, t7828: f64, t2153: f64, t2308: f64, t382: f64, t525: f64, t526: f64, t8011: f64, t8015: f64) -> (f64, f64, f64, f64, f64) {
    let t530 = t529 < -0.66725e-1_f64;
    let t8287 = t1459 * t8286;
    let t8288 = t2331 * t2331;
    let t8289 = t8288 * t4350;
    let t8292 = t7828 * t41;
    let t8306 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t8292 * t382 - 20.0_f64 / 27.0_f64 * t525 * t2308 * t2153 + 40.0_f64 / 81.0_f64 * t525 * t526 * t8011 - 10.0_f64 / 27.0_f64 * t525 * t526 * t8015);
    (t8287, t8288, t8289, t8292, t8306)
}
