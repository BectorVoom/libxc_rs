//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 708/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk708(t9779: f64, t9784: f64, t9786: f64, t9791: f64, t9793: f64, t9796: f64, t9800: f64, t9804: f64, t9808: f64, t9810: f64, t9813: f64, t9815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10289 = 0.212822999466489197e-4_f64 * t9779;
    let t10290 = 0.1064114997332445985e-4_f64 * t9784;
    let t10291 = 0.1702583995731913576e-4_f64 * t9786;
    let t10292 = 0.85129199786595678799e-5_f64 * t9791;
    let t10293 = 0.5107751987195740728e-4_f64 * t9793;
    let t10294 = 0.2553875993597870364e-4_f64 * t9796;
    let t10295 = 0.2727466165424534173e-1_f64 * t9800;
    let t10296 = 0.68186654135613354325e-2_f64 * t9804;
    let t10297 = 0.20455996240684006298e-1_f64 * t9808;
    let t10298 = 0.13637330827122670865e-1_f64 * t9810;
    let t10299 = 0.5987120850931904282e-1_f64 * t9813;
    let t10301 = 0.5107751987195740728e-4_f64 * t9815;
    (t10289, t10290, t10291, t10292, t10293, t10294, t10295, t10296, t10297, t10298, t10299, t10301)
}
