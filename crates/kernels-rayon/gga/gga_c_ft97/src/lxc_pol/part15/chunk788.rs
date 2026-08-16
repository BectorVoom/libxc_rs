//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 788/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk788(t21540: f64, t242: f64, t1168: f64, t18391: f64, t14114: f64, t18431: f64, t18452: f64, t18455: f64, t18457: f64, t18538: f64, t18540: f64, t18542: f64, t18544: f64, t21524: f64, t21533: f64, t21537: f64, t446: f64, t9982: f64) -> (f64, f64, f64, f64) {
    let t21541 = t242 * t21540;
    let t21548 = t18391 * t1168;
    let t21549 = t242 * t21548;
    let t21551 = 2.0_f64 / 3.0_f64 * t446 * t21524 + t18431 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t18452 - 2.0_f64 / 9.0_f64 * t18455 - 2.0_f64 / 9.0_f64 * t18457 - 2.0_f64 * t446 * t21533 - 2.0_f64 * t446 * t21537 - t446 * t21541 + 4.0_f64 / 9.0_f64 * t14114 - t9982 + 2.0_f64 / 27.0_f64 * t18538 + t18540 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t18542 - 2.0_f64 / 3.0_f64 * t18544 - t446 * t21549;
    (t21541, t21548, t21549, t21551)
}
