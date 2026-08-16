//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1411/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1411(t105208: f64, t106892: f64, t107493: f64, t107543: f64, t1873: f64, t20347: f64, t3941: f64, t1458: f64, t28017: f64, t5493: f64, t7467: f64, t75784: f64) -> (f64, f64, f64, f64, f64) {
    let t107545 = t105208 + t106892 + t107493 + t107543;
    let t107552 = 27.0_f64 * t3941 * t1873 * t20347;
    let t107555 = 81.0_f64 * t3941 * t28017 * t1458;
    let t107558 = 81.0_f64 * t3941 * t7467 * t5493;
    let t107566 = 0.135e2_f64 * t75784 * t1873;
    (t107545, t107552, t107555, t107558, t107566)
}
