//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 784/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk784<F: Float>(t8604: F, t8610: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F, t8647: F, t8651: F, t8679: F, t8685: F, t8690: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38271 = F::new(0.85129199786595678796e-5) * t8604;
    let t38272 = F::new(0.85129199786595678796e-5) * t8610;
    let t38274 = F::new(0.13637330827122670864e-1) * t8623;
    let t38275 = F::new(0.81823984962736025184e-1) * t8627;
    let t38276 = F::new(0.13637330827122670864e0) * t8633;
    let t38277 = F::new(0.27274661654245341728e-1) * t8637;
    let t38278 = F::new(0.40911992481368012592e-1) * t8643;
    let t38279 = F::new(0.81823984962736025184e-1) * t8647;
    let t38280 = F::new(0.20455996240684006296e-1) * t8651;
    let t38292 = F::new(0.85129199786595678796e-5) * t8679;
    let t38295 = F::new(0.85129199786595678796e-5) * t8685;
    let t38296 = F::new(0.85129199786595678796e-5) * t8690;
    (t38271, t38272, t38274, t38275, t38276, t38277, t38278, t38279, t38280, t38292, t38295, t38296)
}
