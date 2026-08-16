//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1035/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1035(t40699: f64, t8571: f64, t35691: f64, t35697: f64, t35699: f64, t35703: f64, t35705: f64, t40343: f64, t40350: f64, t40351: f64, t40354: f64, t40357: f64, t40459: f64, t46992: f64, t46995: f64, t46999: f64, t47004: f64, t47006: f64, t47008: f64) -> f64 {
    let t47011 = t8571 * t40699;
    let t47013 = -0.85129199786595678796e-5_f64 * t46992 + 0.10248087766267884742e-3_f64 * t35691 - 0.85129199786595678796e-5_f64 * t46995 + 0.29810146462873361018e-2_f64 * t40343 - 0.99317399751028291929e-5_f64 * t46999 + t40350 - 0.59590439850616975158e-4_f64 * t40351 + 0.59590439850616975158e-4_f64 * t40354 + t40357 - 0.19863479950205658386e-4_f64 * t47004 + 0.99317399751028291929e-5_f64 * t47006 - 0.39914139006212695213e-1_f64 * t47008 - t35697 - t35699 - t35703 - 0.35220688045884876043e-2_f64 * t35705 + 0.85129199786595678796e-5_f64 * t47011 - t40459;
    t47013
}
