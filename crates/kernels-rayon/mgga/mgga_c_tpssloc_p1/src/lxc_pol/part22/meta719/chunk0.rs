//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2327/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327(t59039: f64, t16717: f64, t58994: f64, t59045: f64, t59048: f64, t39658: f64, t46436: f64, t46438: f64, t67494: f64, t67495: f64, t67496: f64, t67497: f64, t67498: f64, t67499: f64, t67500: f64, t67501: f64, t67502: f64, t67503: f64) -> (f64, f64, f64, f64, f64) {
    let t67504 = 36.0_f64 * t59039;
    let t67506 = 72.0_f64 * t58994 * t16717;
    let t67507 = 0.17544670867903938621e1_f64 * t59045;
    let t67508 = 0.54934341918019635162e-3_f64 * t59048;
    let t67509 = t67494 + t46436 + t46438 + t67495 + t67496 - t39658 + t67497 + t67498 + t67499 + t67500 - t67501 + t67502 + t67503 + t67504 + t67506 - t67507 - t67508;
    (t67504, t67506, t67507, t67508, t67509)
}
