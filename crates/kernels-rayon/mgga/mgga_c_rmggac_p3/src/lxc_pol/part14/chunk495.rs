//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 495/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk495(t4120: f64, t4124: f64, t1004: f64, t589: f64, t4165: f64, t4167: f64, t4169: f64, t194: f64, t618: f64, t1412: f64, t171: f64, t433: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5385 = 4.0_f64 * t4120;
    let t5388 = 32.0_f64 * t4124;
    let t5389 = t1004 * t589;
    let t5392 = 0.5848223622634646207e0_f64 * t4165;
    let t5393 = 0.11696447245269292414e1_f64 * t4167;
    let t5394 = 0.34631718211362927518e2_f64 * t4169;
    let t5395 = t194 * t618;
    let t5400 = t1412 * t171;
    let t5402 = 0.11696447245269292414e1_f64 * t5400 * t433;
    (t5385, t5388, t5389, t5392, t5393, t5394, t5395, t5402)
}
