//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1287/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1287(t1394: f64, t22275: f64, t28503: f64, t102029: f64, t102032: f64, t102035: f64, t102038: f64, t7978: f64, t98193: f64, t99098: f64, t99100: f64, t99108: f64, t99117: f64, t99129: f64, t99131: f64) -> (f64, f64) {
    let t102041 = t1394 * t28503 * t22275;
    let t102045 = -0.69505208333333333334e-3_f64 * t7978 * t102029 - t99098 - t99100 + t99108 - t99117 + 0.7722800925925925926e-4_f64 * t102032 + 0.7722800925925925926e-4_f64 * t102035 - 0.25794135802469135802e-3_f64 * t102038 + 0.23214722222222222221e-2_f64 * t102041 - 0.41270617283950617283e-2_f64 * t98193 + t99129 - 0.15445601851851851852e-3_f64 * t99131;
    (t102041, t102045)
}
