//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2340/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2340(t91344: f64, t26245: f64, t80783: f64, t80870: f64, t80872: f64, t91304: f64, t91305: f64, t91311: f64, t91312: f64, t91314: f64, t91317: f64, t91319: f64, t91321: f64, t91323: f64, t91328: f64, t91330: f64, t91333: f64, t91336: f64, t91340: f64) -> f64 {
    let t91345 = 0.28260929265898273598e-2_f64 * t91344;
    let t91346 = t80783 * t26245;
    let t91348 = -t91304 + 119.0_f64 / 6912.0_f64 * t91305 + t91311 - 0.52708876011794399171e-3_f64 * t91312 - t91314 + 7.0_f64 / 288.0_f64 * t80870 + 7.0_f64 / 576.0_f64 * t80872 + 5.0_f64 / 192.0_f64 * t91317 + 5.0_f64 / 192.0_f64 * t91319 + 5.0_f64 / 384.0_f64 * t91321 + 0.10093189023535097714e-3_f64 * t91323 + t91328 + 0.16956557559538964158e-1_f64 * t91330 + 0.84782787797694820792e-2_f64 * t91333 - 0.20186378047070195427e-3_f64 * t91336 + 0.12111826828242117256e-2_f64 * t91340 - t91345 + 0.16821981705891829522e-4_f64 * t91346;
    t91348
}
