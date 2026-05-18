//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 708/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk708<F: Float>(t9779: F, t9784: F, t9786: F, t9791: F, t9793: F, t9796: F, t9800: F, t9804: F, t9808: F, t9810: F, t9813: F, t9815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10289 = F::new(0.212822999466489197e-4) * t9779;
    let t10290 = F::new(0.1064114997332445985e-4) * t9784;
    let t10291 = F::new(0.1702583995731913576e-4) * t9786;
    let t10292 = F::new(0.85129199786595678799e-5) * t9791;
    let t10293 = F::new(0.5107751987195740728e-4) * t9793;
    let t10294 = F::new(0.2553875993597870364e-4) * t9796;
    let t10295 = F::new(0.2727466165424534173e-1) * t9800;
    let t10296 = F::new(0.68186654135613354325e-2) * t9804;
    let t10297 = F::new(0.20455996240684006298e-1) * t9808;
    let t10298 = F::new(0.13637330827122670865e-1) * t9810;
    let t10299 = F::new(0.5987120850931904282e-1) * t9813;
    let t10301 = F::new(0.5107751987195740728e-4) * t9815;
    (t10289, t10290, t10291, t10292, t10293, t10294, t10295, t10296, t10297, t10298, t10299, t10301)
}
