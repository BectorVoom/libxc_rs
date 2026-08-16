//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1187/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1187(t1058: f64, t2207: f64, t9418: f64, t37700: f64, t37707: f64, t39580: f64, t39602: f64, t39608: f64, t43178: f64, t43181: f64, t43183: f64, t43185: f64, t43188: f64) -> f64 {
    let t43191 = t2207 * t1058 * t9418;
    let t43193 = -t39580 + 0.58544643236296698112e-1_f64 * t37700 - 0.22511059664845582436e0_f64 * t37707 + 0.26198215989259945075e-1_f64 * t43178 - 0.65495539973149862688e-2_f64 * t43181 - 0.43663693315433241792e-2_f64 * t43183 - 0.13099107994629972538e-1_f64 * t43185 + t39602 + 0.65495539973149862688e-2_f64 * t43188 - t39608 - 0.69861909304693186867e-1_f64 * t43191;
    t43193
}
