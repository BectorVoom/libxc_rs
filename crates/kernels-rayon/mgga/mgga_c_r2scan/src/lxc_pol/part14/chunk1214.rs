//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1214/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1214(t37765: f64, t37770: f64, t37788: f64, t39686: f64, t39689: f64, t39692: f64, t39695: f64, t39697: f64, t39700: f64, t39703: f64, t39706: f64, t39708: f64) -> f64 {
    let t41511 = -0.51220160311720645768e0_f64 * t37765 - 0.21951497276451705328e-1_f64 * t37770 - 0.46230515946956099004e0_f64 * t37788 + 0.13099107994629972538e-1_f64 * t39686 - 0.87327386630866483588e-2_f64 * t39689 - 0.26198215989259945076e-1_f64 * t39692 + 0.1047928639570397803e0_f64 * t39695 - 0.87327386630866483588e-2_f64 * t39697 - 0.87327386630866483588e-2_f64 * t39700 + 0.87327386630866483588e-2_f64 * t39703 - 0.43663693315433241794e-2_f64 * t39706 - 0.87327386630866483588e-2_f64 * t39708;
    t41511
}
