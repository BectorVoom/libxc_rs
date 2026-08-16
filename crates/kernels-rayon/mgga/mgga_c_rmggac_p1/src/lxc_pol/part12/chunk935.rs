//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 935/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk935(t40092: f64, t7720: f64, t495: f64, t515: f64, t7230: f64, t7231: f64, t9109: f64, t2868: f64, t35566: f64, t40050: f64, t40055: f64, t40057: f64, t40060: f64, t40063: f64, t40068: f64, t40073: f64, t40076: f64, t40082: f64, t40085: f64, t40087: f64, t40089: f64, t5055: f64, t7527: f64, t7530: f64) -> f64 {
    let t40093 = t7720 * t40092;
    let t40098 = t7230 * t7231 * t515 * t9109 * t495;
    let t40100 = 0.44903406381989282115e-1_f64 * t40050 + 0.25538759935978703638e-4_f64 * t40055 - 0.81823984962736025184e-1_f64 * t40057 + 0.13637330827122670864e0_f64 * t40060 - t40063 + 0.53205749866622299248e-5_f64 * t40068 - 0.11971293719990017331e-4_f64 * t40073 - t40076 + 0.35922725105591425692e0_f64 * t5055 * t7527 + 0.23948483403727617128e0_f64 * t2868 * t7530 - t35566 + 0.25538759935978703638e-4_f64 * t40082 + t40085 + t40087 + t40089 + 0.25538759935978703638e-4_f64 * t40093 - 0.1064114997332445985e-4_f64 * t40098;
    t40100
}
