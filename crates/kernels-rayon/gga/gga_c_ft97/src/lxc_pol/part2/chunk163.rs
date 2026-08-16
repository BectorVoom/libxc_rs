//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 163/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk163(t488: f64, t492: f64, t83: f64, t28: f64, t442: f64, t446: f64, t449: f64, t454: f64, t482: f64, t89: f64) -> (f64, f64, f64) {
    let t493 = t488 * t492;
    let t494 = t83 * t493;
    let t497 = -t442 - t446 * t449 / 9.0_f64 - t446 * t454 / 3.0_f64 + t89 * t28 * t482 / 3.0_f64 - t446 * t494 / 3.0_f64;
    (t493, t494, t497)
}
