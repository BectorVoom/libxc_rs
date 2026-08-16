//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 961/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk961(t40458: f64, t35705: f64, t35707: f64, t35713: f64, t35717: f64, t35720: f64, t35724: f64, t35729: f64, t40414: f64, t40420: f64, t40425: f64, t40431: f64, t40437: f64, t40442: f64, t40448: f64, t40451: f64, t40456: f64) -> f64 {
    let t40459 = 0.79828278012425390426e-1_f64 * t40458;
    let t40463 = -0.70441376091769752086e-2_f64 * t35705 + 0.1064114997332445985e-4_f64 * t40414 + 0.53205749866622299248e-5_f64 * t40420 + 0.53205749866622299248e-5_f64 * t40425 - 0.85129199786595678796e-5_f64 * t40431 - 0.85129199786595678796e-5_f64 * t40437 + 0.25538759935978703638e-4_f64 * t40442 - 0.25538759935978703638e-4_f64 * t40448 + 0.85129199786595678796e-5_f64 * t40451 - 0.1064114997332445985e-4_f64 * t40456 - t40459 + 0.60975299583150056628e-3_f64 * t35707 + t35713 + t35717 - 0.86737941314158990624e-4_f64 * t35720 - 0.86737941314158990624e-4_f64 * t35724 - t35729;
    t40463
}
