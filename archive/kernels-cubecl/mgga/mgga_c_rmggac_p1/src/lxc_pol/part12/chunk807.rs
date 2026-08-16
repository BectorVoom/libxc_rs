//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 807/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk807<F: Float>(t8590: F, t8593: F, t8595: F, t8598: F, t8604: F, t8610: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F, t8647: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38266 = F::cast_from(0.25538759935978703638e-4_f64) * t8590;
    let t38267 = F::cast_from(0.25538759935978703638e-4_f64) * t8593;
    let t38268 = F::cast_from(0.85129199786595678796e-5_f64) * t8595;
    let t38269 = F::cast_from(0.85129199786595678796e-5_f64) * t8598;
    let t38271 = F::cast_from(0.85129199786595678796e-5_f64) * t8604;
    let t38272 = F::cast_from(0.85129199786595678796e-5_f64) * t8610;
    let t38274 = F::cast_from(0.13637330827122670864e-1_f64) * t8623;
    let t38275 = F::cast_from(0.81823984962736025184e-1_f64) * t8627;
    let t38276 = F::cast_from(0.13637330827122670864e0_f64) * t8633;
    let t38277 = F::cast_from(0.27274661654245341728e-1_f64) * t8637;
    let t38278 = F::cast_from(0.40911992481368012592e-1_f64) * t8643;
    let t38279 = F::cast_from(0.81823984962736025184e-1_f64) * t8647;
    (t38266, t38267, t38268, t38269, t38271, t38272, t38274, t38275, t38276, t38277, t38278, t38279)
}
