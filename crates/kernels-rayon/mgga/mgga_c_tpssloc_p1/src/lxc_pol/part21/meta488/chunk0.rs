//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2090/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090(t16758: f64, t829: f64, t4234: f64, t4282: f64, t5550: f64, t9573: f64, t213: f64, t5527: f64, t221: f64, t776: f64, t4119: f64, t4128: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16759 = t16758 * t829;
    let t16762 = t4282 * t4234;
    let t16769 = t9573 * t5550;
    let t16771 = t213 * t5527;
    let t16773 = t221 * t16771 * t776;
    let t16777 = t221 * t4128 * t4119;
    (t16759, t16762, t16769, t16771, t16773, t16777)
}
