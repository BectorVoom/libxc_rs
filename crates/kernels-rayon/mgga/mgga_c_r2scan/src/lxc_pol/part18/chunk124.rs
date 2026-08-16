//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 124/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk124(t390: f64, t185: f64, t2: f64, t387: f64, t22: f64, t23: f64, t6: f64, t8: f64, t388: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t391 = 0.29896666666666666667e0_f64 * t390;
    let t392 = t185 * t2;
    let t393 = t392 * t387;
    let t394 = 0.1023875e0_f64 * t393;
    let t398 = t22 * t6 / t23 / t8;
    let t399 = 0.82156666666666666667e-1_f64 * t398;
    let t400 = -0.632975e0_f64 * t388 - t391 - t394 - t399;
    let t401 = 1.0_f64 / t31;
    (t391, t392, t393, t394, t398, t399, t400, t401)
}
