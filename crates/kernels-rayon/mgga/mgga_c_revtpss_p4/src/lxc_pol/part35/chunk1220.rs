//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1220/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1220(t103449: f64, t103463: f64, t103471: f64, t103490: f64, t106275: f64, t110600: f64, t110613: f64, t110615: f64, t113373: f64, t115573: f64, t2067: f64, t231: f64, t27199: f64, t30401: f64, t30406: f64, t7070: f64, t7076: f64, t8012: f64, t95891: f64, t95893: f64, t95899: f64) -> f64 {
    let t115658 = 0.15421710918628844643e0_f64 * t110600 - 0.39029762157531132076e-1_f64 * t103449 + t95891 + 0.51405703062096148814e-2_f64 * t103463 - t95893 + 0.13010442282307799193e1_f64 * t106275 * t8012 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t115573 * t231 + 0.26020884564615598386e1_f64 * t27199 * t30401 + 0.13010442282307799193e1_f64 * t27199 * t30406 - 0.4336814094102599731e0_f64 * t113373 * t2067 + t95899 + 0.14456046980341999104e-2_f64 * t103471 - 0.29272321618148349057e-1_f64 * t110613 - 0.43368140941025997312e-1_f64 * t110615 + 0.21951497276451705329e-1_f64 * t103490;
    t115658
}
