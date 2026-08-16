//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 427/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk427(t377: f64, t4287: f64, t364: f64, t1076: f64, t163: f64, t158: f64, t1080: f64, t4221: f64, t4130: f64, t4133: f64, t4136: f64, t4138: f64, t4142: f64, t4144: f64, t4146: f64, t4149: f64) -> (f64, f64, f64, f64) {
    let t4288 = t4287 * t377;
    let t4290 = 1.0_f64 * t364 * t4288;
    let t4292 = 1.0_f64 / t1076 / t163;
    let t4293 = t158 * t4292;
    let t4294 = t4221 * t1080;
    let t4305 = -0.47063e1_f64 * t4130 + 0.31375333333333333334e1_f64 * t4133 - 0.36604555555555555556e1_f64 * t4136 - 0.16068111111111111111e1_f64 * t4138 + 0.28051666666666666666e0_f64 * t4142 - 0.56103333333333333332e0_f64 * t4144 - 0.6545388888888888889e0_f64 * t4146 - 0.46308888888888888888e0_f64 * t4149;
    (t4290, t4293, t4294, t4305)
}
