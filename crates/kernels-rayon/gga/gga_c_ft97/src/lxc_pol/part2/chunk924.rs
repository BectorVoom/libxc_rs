//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 924/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk924(t13704: f64, t13708: f64, t13719: f64, t13722: f64, t13728: f64, t13732: f64, t13736: f64, t13739: f64, t13743: f64, t13750: f64, t14317: f64, t14318: f64, t14327: f64, t14329: f64, t9520: f64, t9723: f64, t9727: f64, t9730: f64, t9765: f64, t9768: f64) -> f64 {
    let t14332 = -2.0_f64 / 9.0_f64 * t13704 + 2.0_f64 / 27.0_f64 * t13708 + t9723 / 54.0_f64 + t9727 / 81.0_f64 - t14317 - t14318 - t9730 / 9.0_f64 + t9520 / 18.0_f64 - t13719 - 2.0_f64 / 81.0_f64 * t13722 + 2.0_f64 / 3.0_f64 * t13728 - 11.0_f64 / 27.0_f64 * t13732 + t13736 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t13739 + t13743 / 3.0_f64 - t14327 - t13750 / 6.0_f64 + t14329 - t9768 / 27.0_f64 - t9765 / 27.0_f64;
    t14332
}
