//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 799/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk799(t7426: f64, t8657: f64, t1165: f64, t1439: f64, t7351: f64, t7575: f64, t1992: f64, t525: f64, t7842: f64, t7585: f64, t1562: f64, t7561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8658 = t7426 * t8657;
    let t8661 = t1165 * t7351 * t1439;
    let t8662 = t7575 * t8661;
    let t8665 = t7842 * t1992 * t525;
    let t8666 = t7585 * t8665;
    let t8668 = t7561 * t1562;
    (t8658, t8661, t8662, t8665, t8666, t8668)
}
