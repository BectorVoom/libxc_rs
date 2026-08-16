//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 551/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk551(t14434: f64, t352: f64, t2228: f64, t36: f64, t305: f64, t664: f64, t8264: f64, t118: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14435 = t14434 * t352;
    let t14438 = t2228 * t36;
    let t14439 = t305 * t14438;
    let t14440 = 0.14967802127329760705e-1_f64 * t14439;
    let t14441 = t8264 * t664;
    let t14443 = 0.39914139006212695214e-1_f64 * t118 * t14441;
    let t14444 = t698 * t664;
    (t14435, t14438, t14440, t14441, t14443, t14444)
}
