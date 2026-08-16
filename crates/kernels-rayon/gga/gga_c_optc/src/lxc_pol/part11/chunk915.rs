//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 915/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk915(t11459: f64, t11470: f64, t14670: f64, t14739: f64, t14744: f64, t14753: f64, t14758: f64, t17176: f64, t17181: f64, t17186: f64, t17191: f64, t17196: f64, t17201: f64, t2668: f64, t3884: f64, t8231: f64, t913: f64, t940: f64, t953: f64) -> f64 {
    let t17206 = 0.25190352229182098644e-1_f64 * t953 * t17176 + 0.1559479530529405812e2_f64 * t14670 - 0.30228422675018518374e-1_f64 * t953 * t17181 + 0.11360101276506094136e1_f64 * t913 * t17186 + 0.5848048239485271795e1_f64 * t940 * t17191 - 0.57954409931925052365e-1_f64 * t14739 + 0.38636273287950034909e-1_f64 * t14744 - 0.4395493670620718481e3_f64 * t3884 * t17196 - 0.75734008510040627575e0_f64 * t11459 - 0.389869882632351453e1_f64 * t11470 + t8231 - 0.15486228121497046737e2_f64 * t2668 * t17201 - 0.4395493670620718481e3_f64 * t14753 + 0.8790987341241436962e3_f64 * t14758;
    t17206
}
