//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 808/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk808(t299: f64, t10943: f64, t7858: f64, t7906: f64, t383: f64, t7857: f64, t1598: f64, t66: f64, t1593: f64, t1595: f64, t1630: f64, t14: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t10944 = piecewise3(t300, 0.0_f64, t10943);
    let t11109 = t7906 * t7858;
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11140 = t1593 * t1595;
    let t11153 = t1630 * t1595;
    let t11174 = 1.0_f64 / t14 / t7741;
    (t10944, t11109, t11120, t11121, t11140, t11153, t11174)
}
