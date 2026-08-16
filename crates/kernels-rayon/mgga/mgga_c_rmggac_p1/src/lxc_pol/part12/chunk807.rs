//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 807/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk807(t8590: f64, t8593: f64, t8595: f64, t8598: f64, t8604: f64, t8610: f64, t8623: f64, t8627: f64, t8633: f64, t8637: f64, t8643: f64, t8647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38266 = 0.25538759935978703638e-4_f64 * t8590;
    let t38267 = 0.25538759935978703638e-4_f64 * t8593;
    let t38268 = 0.85129199786595678796e-5_f64 * t8595;
    let t38269 = 0.85129199786595678796e-5_f64 * t8598;
    let t38271 = 0.85129199786595678796e-5_f64 * t8604;
    let t38272 = 0.85129199786595678796e-5_f64 * t8610;
    let t38274 = 0.13637330827122670864e-1_f64 * t8623;
    let t38275 = 0.81823984962736025184e-1_f64 * t8627;
    let t38276 = 0.13637330827122670864e0_f64 * t8633;
    let t38277 = 0.27274661654245341728e-1_f64 * t8637;
    let t38278 = 0.40911992481368012592e-1_f64 * t8643;
    let t38279 = 0.81823984962736025184e-1_f64 * t8647;
    (t38266, t38267, t38268, t38269, t38271, t38272, t38274, t38275, t38276, t38277, t38278, t38279)
}
