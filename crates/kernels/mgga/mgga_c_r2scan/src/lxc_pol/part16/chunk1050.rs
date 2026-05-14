//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1050/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1050<F: Float>(t10760: F, t19877: F, t29467: F, t29731: F, t6093: F, t11724: F, t26278: F, t11675: F, t26282: F, t12550: F, t2207: F, t3328: F, t1058: F, t9418: F, t37700: F, t37707: F, t39580: F, t39602: F, t39608: F) -> (F,) {
    let t43178 = t19877 * t10760 * t29467;
    let t43181 = t6093 * t10760 * t29731;
    let t43183 = t26278 * t11724;
    let t43185 = t26282 * t11675;
    let t43188 = t2207 * t12550 * t3328;
    let t43191 = t2207 * t1058 * t9418;
    let t43193 = -t39580 + 0.58544643236296698112e-1 * t37700 - 0.22511059664845582436e0 * t37707 + 0.26198215989259945075e-1 * t43178 - 0.65495539973149862688e-2 * t43181 - 0.43663693315433241792e-2 * t43183 - 0.13099107994629972538e-1 * t43185 + t39602 + 0.65495539973149862688e-2 * t43188 - t39608 - 0.69861909304693186867e-1 * t43191;
    (t43193,)
}
