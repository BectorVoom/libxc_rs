//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 830/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk830(t1815: f64, t599: f64, t1181: f64, t7413: f64, t1165: f64, t1849: f64, t7351: f64, t7575: f64, t1713: f64, t142: f64, t7450: f64, t2313: f64, t507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9648 = t599 * t1815;
    let t9649 = t1181 * t9648;
    let t9650 = t7413 * t9649;
    let t9653 = t1165 * t7351 * t1849;
    let t9654 = t7575 * t9653;
    let t9659 = t599 * t1713;
    let t9660 = t142 * t9659;
    let t9661 = t7450 * t9660;
    let t9663 = t507 * t2313;
    (t9648, t9649, t9650, t9653, t9654, t9659, t9660, t9661, t9663)
}
