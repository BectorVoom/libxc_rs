//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 905/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk905(t13811: f64, t10119: f64, t13798: f64, t13801: f64, t13804: f64, t13807: f64, t13814: f64, t13817: f64, t13820: f64, t13823: f64, t14004: f64, t13977: f64, t13986: f64, t13999: f64) -> f64 {
    let t14005 = 4.0_f64 / 9.0_f64 * t13811;
    let t14010 = -2.0_f64 / 9.0_f64 * t13798 - 10.0_f64 / 27.0_f64 * t13801 + 8.0_f64 / 9.0_f64 * t13804 + t13807 / 3.0_f64 - t14004 - t10119 - t14005 - 2.0_f64 / 3.0_f64 * t13814 - 2.0_f64 * t13817 + 4.0_f64 / 3.0_f64 * t13820 - 2.0_f64 / 3.0_f64 * t13823;
    let t14012 = t13977 + t13986 + t13999 + t14010;
    t14012
}
