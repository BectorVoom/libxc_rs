//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 969/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk969(t41914: f64, t41947: f64, t41949: f64, t41951: f64, t10430: f64, t2890: f64, t9267: f64, t2478: f64, t3545: f64, t6576: f64, t2482: f64, t3536: f64, t9263: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46368 = 0.59584149919750711116e-1_f64 * t41914;
    let t46370 = 0.17875244975925213335e0_f64 * t41947;
    let t46371 = 0.17875244975925213335e0_f64 * t41949;
    let t46372 = 0.17875244975925213335e0_f64 * t41951;
    let t46378 = t9267 * t2890 * t10430;
    let t46379 = 0.19171462976960374838e1_f64 * t46378;
    let t46381 = t6576 * t3545 * t2478;
    let t46382 = 0.19171462976960374838e0_f64 * t46381;
    let t46384 = t9263 * t3536 * t2482;
    (t46368, t46370, t46371, t46372, t46379, t46382, t46384)
}
