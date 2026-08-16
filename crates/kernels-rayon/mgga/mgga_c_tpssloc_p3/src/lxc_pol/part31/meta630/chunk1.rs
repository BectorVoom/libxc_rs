//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1890/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890(t19930: f64, t6952: f64, t1831: f64, t91191: f64, t26257: f64, t5314: f64, t28100: f64, t80853: f64, t80855: f64, t22788: f64, t6431: f64, t6427: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97340 = t6952 * t19930;
    let t97342 = t91191 * t1831;
    let t97344 = t26257 * t5314;
    let t97347 = t80853 * t80855 * t28100;
    let t97352 = t22788 * t6431;
    let t97354 = t22788 * t6427;
    (t97340, t97342, t97344, t97347, t97352, t97354)
}
