//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1097/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1097(t105: f64, t2357: f64, t2255: f64, t661: f64, t2362: f64, t4279: f64, t108: f64, t580: f64, t22: f64, t4283: f64, t13472: f64, t13475: f64, t13476: f64, t13479: f64, t13482: f64, t13485: f64, t13493: f64, t1505: f64, t1507: f64, t2344: f64, t2359: f64, t2363: f64, t4270: f64, t4274: f64, t656: f64, t97: f64) -> f64 {
    let t13496 = t105 * t2357;
    let t13497 = t2255 * t661;
    let t13500 = t4279 * t2362;
    let t13503 = t108 * t580;
    let t13506 = t4283 * t22;
    let t13509 = 200.0_f64 / 27.0_f64 * t2344 * t1505 - 100.0_f64 / 27.0_f64 * t656 * t4270 - 50.0_f64 / 9.0_f64 * t656 * t4274 - 10.0_f64 / 27.0_f64 * t97 * t13472 + 20.0_f64 / 9.0_f64 * t13475 * t13476 + 10.0_f64 / 9.0_f64 * t97 * t13479 + 5.0_f64 / 3.0_f64 * t97 * t13482 - 5.0_f64 * t97 * t13485 - 50.0_f64 / 27.0_f64 * t1507 * t2359 - 25.0_f64 / 9.0_f64 * t1507 * t2363 - 10.0_f64 / 27.0_f64 * t105 * t13493 - 20.0_f64 / 9.0_f64 * t13496 * t13497 + 10.0_f64 / 9.0_f64 * t105 * t13500 - 5.0_f64 / 3.0_f64 * t105 * t13503 + 5.0_f64 * t105 * t13506;
    t13509
}
