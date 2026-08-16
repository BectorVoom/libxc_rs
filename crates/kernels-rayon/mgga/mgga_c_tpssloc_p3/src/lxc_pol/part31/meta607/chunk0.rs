//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1852/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1852(t90549: f64, t90584: f64, t90604: f64, t90609: f64, t90645: f64, t90686: f64, t90701: f64, t90707: f64, t90749: f64, t90759: f64, t90781: f64, t90789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93362 = 0.3289868133696452873e-1_f64 * t90549;
    let t93388 = 0.15352717957250113407e0_f64 * t90584;
    let t93404 = 0.76763589786250567036e-1_f64 * t90604;
    let t93407 = 0.9869604401089358619e-1_f64 * t90609;
    let t93439 = 0.16449340668482264365e-1_f64 * t90645;
    let t93452 = 0.3289868133696452873e-1_f64 * t90686;
    let t93461 = 0.16449340668482264365e-1_f64 * t90701;
    let t93467 = 0.76763589786250567036e-1_f64 * t90707;
    let t93473 = 0.15352717957250113407e0_f64 * t90749;
    let t93476 = 0.76763589786250567036e-1_f64 * t90759;
    let t93483 = 0.16449340668482264365e-1_f64 * t90781;
    let t93488 = 0.9869604401089358619e-1_f64 * t90789;
    (t93362, t93388, t93404, t93407, t93439, t93452, t93461, t93467, t93473, t93476, t93483, t93488)
}
