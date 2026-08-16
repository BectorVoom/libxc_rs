//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 791/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk791(t21453: f64, t2493: f64, t21457: f64, t21442: f64, t9916: f64, t21181: f64, t9920: f64, t2486: f64, t21570: f64, t21573: f64, t21577: f64, t21581: f64, t21585: f64, t21589: f64, t21592: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21595 = t2493 * t21453;
    let t21597 = t2493 * t21457;
    let t21599 = t9916 * t21442;
    let t21602 = t9920 * t21181;
    let t21603 = t2486 * t21602;
    let t21606 = -t92 * t21570 - t462 * t21573 / 3.0_f64 - 6.0_f64 * t92 * t21577 + 6.0_f64 * t462 * t21581 - 10.0_f64 / 27.0_f64 * t462 * t21585 - 2.0_f64 * t462 * t21589 + 2.0_f64 * t462 * t21592 + t462 * t21595 + t462 * t21597 + 2.0_f64 / 3.0_f64 * t462 * t21599 + 4.0_f64 / 3.0_f64 * t462 * t21603;
    (t21595, t21597, t21599, t21602, t21603, t21606)
}
