//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 999/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk999(t11161: f64, t11170: f64, t11197: f64, t11200: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11221: f64, t11224: f64, t11314: f64, t11317: f64) -> f64 {
    let t11328 = -t11314 - 0.52945875e1_f64 * t11197 + 0.94674375e0_f64 * t11200 - t11317 + 0.62517e0_f64 * t11206 + 0.104195e0_f64 * t11209 + 0.34731666666666666667e0_f64 * t11211 + 0.69463333333333333335e-1_f64 * t11213 - 0.41678000000000000001e0_f64 * t11215 - 0.20839e0_f64 * t11217 + 0.46308888888888888889e-1_f64 * t11221 - 0.20839e0_f64 * t11224 - 0.103295e1_f64 * t11161 + 0.309885e1_f64 * t11170;
    t11328
}
