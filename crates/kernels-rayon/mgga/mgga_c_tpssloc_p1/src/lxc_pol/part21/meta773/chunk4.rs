//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2679/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679(t54405: f64, t12466: f64, t1297: f64, t15868: f64, t15904: f64, t1799: f64, t193: f64, t19577: f64, t19596: f64, t19994: f64, t20077: f64, t3719: f64, t3914: f64, t3918: f64, t3919: f64, t39476: f64, t5126: f64, t5160: f64, t55191: f64, t55266: f64, t56219: f64, t56275: f64, t6301: f64, t6347: f64) -> (f64, f64) {
    let t56279 = 8.0_f64 * t54405;
    let t56294 = 3.0_f64 * t12466 * t3918 * t6347 + 3.0_f64 * t1297 * t193 * t56275 - 12.0_f64 * t15868 * t19577 * t3918 - 6.0_f64 * t15904 * t19596 * t3918 + 6.0_f64 * t1799 * t3918 * t55191 - t19596 * t3914 * t5160 + 12.0_f64 * t19994 * t3919 * t5126 - 3.0_f64 * t20077 * t3719 * t3918 + 12.0_f64 * t55266 * t6301 - t39476 - t56219 - t56279;
    (t56279, t56294)
}
