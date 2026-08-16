//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 783/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk783(t21457: f64, t2354: f64, t446: f64, t13722: f64, t13739: f64, t17720: f64, t21433: f64, t21437: f64, t21440: f64, t21444: f64, t21448: f64, t21451: f64, t21455: f64, t9699: f64) -> (f64, f64, f64) {
    let t21458 = t2354 * t21457;
    let t21459 = t446 * t21458;
    let t21462 = -2.0_f64 / 27.0_f64 * t13722 - t9699 - t17720 / 9.0_f64 - 5.0_f64 / 81.0_f64 * t21433 - t21437 / 3.0_f64 + t21440 / 3.0_f64 + t21444 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t21448 - t21451 / 9.0_f64 + t21455 / 6.0_f64 + t21459 / 6.0_f64 - 2.0_f64 / 9.0_f64 * t13739;
    (t21458, t21459, t21462)
}
