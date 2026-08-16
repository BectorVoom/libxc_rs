//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 529/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk529(t529: f64, t1555: f64, t547: f64, t524: f64, t1596: f64, t544: f64, t3729: f64, t41: f64, t1287: f64, t1558: f64, t382: f64, t4144: f64, t4148: f64, t525: f64, t526: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t530 = t529 < -0.66725e-1_f64;
    let t4346 = 1.0_f64 / t1555 / t547;
    let t4347 = t524 * t4346;
    let t4348 = t1596 * t1596;
    let t4349 = t544 * t544;
    let t4350 = 1.0_f64 / t4349;
    let t4351 = t4348 * t4350;
    let t4354 = t3729 * t41;
    let t4368 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t4354 * t382 - 20.0_f64 / 27.0_f64 * t525 * t1558 * t1287 + 40.0_f64 / 81.0_f64 * t525 * t526 * t4144 - 10.0_f64 / 27.0_f64 * t525 * t526 * t4148);
    (t4346, t4347, t4348, t4349, t4350, t4351, t4354, t4368)
}
