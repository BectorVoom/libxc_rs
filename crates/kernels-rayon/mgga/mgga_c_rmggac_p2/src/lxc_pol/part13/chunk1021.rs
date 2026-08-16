//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1021/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1021(t8494: f64, t10276: f64, t10277: f64, t10278: f64, t10279: f64, t8073: f64, t8074: f64, t8075: f64, t8076: f64, t8077: f64, t8080: f64, t8498: f64) -> (f64, f64) {
    let t42408 = 0.1702583995731913576e-4_f64 * t8494;
    let t42409 = -t8073 - t10276 + t10277 - t10278 + t10279 + t8074 - t8075 + t8076 - t8077 - t8080 - t42408;
    let t42413 = 0.1702583995731913576e-4_f64 * t8498;
    (t42409, t42413)
}
