//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 421/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk421(t4153: f64, t431: f64, t1037: f64, t409: f64, t1040: f64, t179: f64, t4052: f64, t1045: f64, t973: f64, t1042: f64, t1003: f64, t230: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4155 = 0.5848223622634646207e0_f64 * t431 * t4153;
    let t4157 = 1.0_f64 / t1037 / t409;
    let t4160 = 1.0_f64 / t1040 / t179;
    let t4161 = t4157 * t4052 * t4160;
    let t4163 = 0.10254018858216406658e4_f64 * t431 * t4161;
    let t4167 = t1045 * t973;
    let t4169 = t1045 * t1042;
    let t4179 = 1.0_f64 / t1003 / t230;
    (t4155, t4157, t4160, t4163, t4167, t4169, t4179)
}
