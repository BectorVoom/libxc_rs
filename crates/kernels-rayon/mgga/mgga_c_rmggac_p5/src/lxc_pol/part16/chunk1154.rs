//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1154/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1154(t10364: f64, t10365: f64, t10366: f64, t10369: f64, t37179: f64, t42520: f64, t42521: f64, t42527: f64, t7762: f64, t8192: f64, t8193: f64, t10377: f64, t10378: f64, t10380: f64, t37183: f64, t42535: f64, t42536: f64, t42537: f64, t42539: f64, t42540: f64, t8197: f64, t9499: f64) -> (f64, f64) {
    let t49867 = t10364 + t10365 - t10366 - t42520 + t10369 + t42521 + t8192 + t8193 - 0.20496175532535769483e-3_f64 * t7762 + t37179 - t42527;
    let t49872 = t42535 + t42536 + t42537 + 4.0_f64 * t9499 - t8197 + t10377 - t10378 + t42539 + t42540 + t37183 + t10380;
    (t49867, t49872)
}
