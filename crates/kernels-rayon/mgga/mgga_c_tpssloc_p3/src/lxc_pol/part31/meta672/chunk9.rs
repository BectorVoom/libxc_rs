//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2022/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2022(t80780: f64, t91206: f64, t91221: f64, t91223: f64, t93674: f64, t93682: f64, t97310: f64, t97315: f64, t97318: f64, t97320: f64, t97322: f64, t97326: f64, t97333: f64, t97337: f64, t97340: f64, t97342: f64, t97344: f64, t97347: f64) -> f64 {
    let t102694 = -t97310 / 48.0_f64 + 0.67287926823567318088e-4_f64 * t97315 + t97318 / 768.0_f64 + t97320 / 192.0_f64 + t97322 / 96.0_f64 - 0.40372756094140390853e-3_f64 * t97326 - 0.126501302428306558e-1_f64 * t91206 - t93674 - t91221 - t91223 + t93682 - 0.24223653656484234512e-2_f64 * t97333 + 0.80745512188280781706e-3_f64 * t97337 - 0.63250651214153279004e-2_f64 * t80780 - t97340 / 192.0_f64 - t97342 / 96.0_f64 - t97344 / 96.0_f64 - 0.80745512188280781707e-3_f64 * t97347;
    t102694
}
