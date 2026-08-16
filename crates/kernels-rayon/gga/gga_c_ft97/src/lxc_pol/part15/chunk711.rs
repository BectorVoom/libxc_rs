//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 711/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk711(t11949: f64, t20105: f64, t20109: f64, t20119: f64, t20126: f64, t20132: f64, t20139: f64, t20143: f64, t20147: f64, t20154: f64, t20331: f64, t20390: f64, t8455: f64) -> f64 {
    let t20460 = t20147 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t20154 + 2.0_f64 / 9.0_f64 * t20132 - 2.0_f64 / 9.0_f64 * t20139 + t20143 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t20105 - 2.0_f64 / 3.0_f64 * t20109 - t11949 - t8455 + t20331 / 8.0_f64 + t20390 / 6.0_f64 + 2.0_f64 * t20119 - 10.0_f64 / 81.0_f64 * t20126;
    t20460
}
