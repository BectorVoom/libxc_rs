//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1060/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1060(t25854: f64, t78223: f64, t72087: f64, t76415: f64, t76416: f64, t76425: f64, t76427: f64, t76429: f64, t76476: f64, t78214: f64, t78215: f64, t78216: f64, t78219: f64, t78222: f64) -> f64 {
    let t78225 = 0.35922725105591425692e0_f64 * t25854 * t78223;
    let t78226 = t76415 - t76416 - t78214 + t76425 - t76427 - t76429 + t78215 + t78216 + t78219 + t72087 - t76476 - t78222 + t78225;
    t78226
}
