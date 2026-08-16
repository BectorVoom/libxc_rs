//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2527/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2527(t2770: f64, t340: f64, t2403: f64, t4389: f64, t4386: f64, t13543: f64, t699: f64, t13547: f64, t13556: f64, t13529: f64, t13533: f64, t344: f64, t42308: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48143 = t340 * t2770;
    let t48155 = t2403 * t4389;
    let t48157 = t2403 * t4386;
    let t48159 = t699 * t13543;
    let t48161 = t699 * t13547;
    let t48163 = t699 * t13556;
    let t48165 = t699 * t13529;
    let t48167 = t699 * t13533;
    let t48180 = t60 * t42308 * t344;
    (t48143, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48180)
}
