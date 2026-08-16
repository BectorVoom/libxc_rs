//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2936/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936(t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64, t60207: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64) -> f64 {
    let t61150 = -4.0_f64 / 3.0_f64 * t60192 + 8.0_f64 / 9.0_f64 * t60194 + t60197 - 2.0_f64 / 3.0_f64 * t60200 + 4.0_f64 / 9.0_f64 * t60202 + 5.0_f64 / 81.0_f64 * t60204 + t60207 / 9.0_f64 + t60223 / 9.0_f64 + t60226 / 18.0_f64 + 2.0_f64 / 27.0_f64 * t60229 + 2.0_f64 * t60232 + t60235;
    t61150
}
