//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1093/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1093(t39445: f64, t10772: f64, t10810: f64, t2568: f64, t10768: f64, t8129: f64, t2604: f64, t625: f64, t37637: f64, t24906: f64, t37943: f64, t37945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39446 = 0.47609969197673950972e-2_f64 * t39445;
    let t39458 = t10772 * t10810 * t2568;
    let t39459 = 0.69345773920434148506e0_f64 * t39458;
    let t39464 = t10768 * t8129;
    let t39469 = t2604 * t625;
    let t39470 = t37637 * t39469;
    let t39482 = t37943 * t37945 * t24906;
    (t39446, t39459, t39464, t39469, t39470, t39482)
}
