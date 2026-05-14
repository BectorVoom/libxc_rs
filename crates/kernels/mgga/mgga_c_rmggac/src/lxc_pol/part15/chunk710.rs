//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 710/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk710<F: Float>(t8572: F, t8578: F, t8583: F, t8585: F, t8588: F, t8590: F, t8593: F, t8595: F, t8598: F, t8604: F, t8610: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38257 = 0.85129199786595678796e-5 * t8572;
    let t38260 = 0.85129199786595678796e-5 * t8578;
    let t38261 = 0.85129199786595678796e-5 * t8583;
    let t38262 = 0.25538759935978703638e-4 * t8585;
    let t38263 = 0.25538759935978703638e-4 * t8588;
    let t38266 = 0.25538759935978703638e-4 * t8590;
    let t38267 = 0.25538759935978703638e-4 * t8593;
    let t38268 = 0.85129199786595678796e-5 * t8595;
    let t38269 = 0.85129199786595678796e-5 * t8598;
    let t38271 = 0.85129199786595678796e-5 * t8604;
    let t38272 = 0.85129199786595678796e-5 * t8610;
    let t38274 = 0.13637330827122670864e-1 * t8623;
    let t38275 = 0.81823984962736025184e-1 * t8627;
    let t38276 = 0.13637330827122670864e0 * t8633;
    let t38277 = 0.27274661654245341728e-1 * t8637;
    let t38278 = 0.40911992481368012592e-1 * t8643;
    (t38257, t38260, t38261, t38262, t38263, t38266, t38267, t38268, t38269, t38271, t38272, t38274, t38275, t38276, t38277, t38278)
}
