//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 910/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk910(t1156: f64, t3573: f64, t1089: f64, t175: f64, t301: f64, t3101: f64, t384: f64, t13690: f64, t13693: f64, t13726: f64, t13736: f64, t13745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13791 = t3573 * t1156;
    let t13802 = t384 * t1089 * t175 * t3101 * t301;
    let t13804 = 35.0_f64 / 9.0_f64 * t13690;
    let t13805 = 130.0_f64 / 27.0_f64 * t13693;
    let t13810 = 35.0_f64 / 36.0_f64 * t13726;
    let t13812 = 910.0_f64 / 81.0_f64 * t13736;
    let t13814 = 100.0_f64 / 9.0_f64 * t13745;
    (t13791, t13802, t13804, t13805, t13810, t13812, t13814)
}
