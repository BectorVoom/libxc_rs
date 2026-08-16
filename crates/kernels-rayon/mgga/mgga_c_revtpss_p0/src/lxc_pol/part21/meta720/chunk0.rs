//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2559/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2559(t4038: f64, t9318: f64, t1337: f64, t40101: f64, t9323: f64, t1340: f64, t40097: f64, t39816: f64, t19: f64, t2237: f64, t521: f64, t1331: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46989 = t4038 * t9318;
    let t46992 = 0.18989649058080861537e-2_f64 * t1337 * t40101;
    let t46993 = t4038 * t9323;
    let t46996 = 0.46785788981077169656e1_f64 * t1340 * t40097;
    let t46998 = 0.69263436422725855036e2_f64 * t1340 * t39816;
    let t47003 = 840.0_f64 * t19 * t2237 * t521;
    let t47005 = t9342 * t1331;
    (t46989, t46992, t46993, t46996, t46998, t47003, t47005)
}
