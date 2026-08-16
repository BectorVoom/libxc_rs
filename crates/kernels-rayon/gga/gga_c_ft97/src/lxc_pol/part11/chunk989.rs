//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 989/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk989(t40312: f64, t40315: f64, t40497: f64, t40500: f64, t40503: f64, t40512: f64, t40297: f64, t40301: f64, t40306: f64, t40309: f64, t40318: f64, t40321: f64, t40506: f64, t40509: f64) -> f64 {
    let t40575 = 4.0_f64 / 27.0_f64 * t40312;
    let t40576 = 8.0_f64 / 81.0_f64 * t40315;
    let t40579 = 56.0_f64 / 243.0_f64 * t40497;
    let t40580 = 8.0_f64 / 27.0_f64 * t40500;
    let t40581 = 4.0_f64 / 9.0_f64 * t40503;
    let t40584 = 8.0_f64 / 9.0_f64 * t40512;
    let t40585 = 20.0_f64 / 27.0_f64 * t40297 - 8.0_f64 / 27.0_f64 * t40301 + 4.0_f64 / 3.0_f64 * t40306 - 4.0_f64 / 3.0_f64 * t40309 - t40575 - t40576 + 2.0_f64 / 27.0_f64 * t40318 + 20.0_f64 / 243.0_f64 * t40321 + t40579 + t40580 - t40581 + 2.0_f64 / 9.0_f64 * t40506 + 4.0_f64 / 3.0_f64 * t40509 + t40584;
    t40585
}
