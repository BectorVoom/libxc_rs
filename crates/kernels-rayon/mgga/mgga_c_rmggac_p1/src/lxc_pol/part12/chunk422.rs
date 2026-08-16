//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 422/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk422(t1040: f64, t179: f64, t4052: f64, t4157: f64, t431: f64, t1034: f64, t171: f64, t433: f64, t1045: f64, t973: f64, t1042: f64, t500: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4160 = 1.0_f64 / t1040 / t179;
    let t4161 = t4157 * t4052 * t4160;
    let t4163 = 0.10254018858216406658e4_f64 * t431 * t4161;
    let t4164 = t1034 * t171;
    let t4165 = t4164 * t433;
    let t4167 = t1045 * t973;
    let t4169 = t1045 * t1042;
    let t4173 = t500 * t998;
    (t4160, t4163, t4165, t4167, t4169, t4173)
}
