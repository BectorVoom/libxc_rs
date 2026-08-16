//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1089/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1089(t17614: f64, t17640: f64, t17684: f64, t17725: f64, t17900: f64, t17967: f64, t18007: f64, t18044: f64, t349: f64, t1052: f64, t1066: f64, t17575: f64, t17579: f64, t17583: f64, t17588: f64, t3026: f64, t3169: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t5920: f64, t5944: f64) -> (f64, f64) {
    let t18047 = t17614 + t17640 + t17684 + t17725 + t17900 + t17967 + t18007 + t18044;
    let t18048 = t349 * t18047;
    let t18050 = 4.0_f64 * t1052 * t17583 - t1066 * t17575 - 2.0_f64 * t1066 * t17588 + 2.0_f64 * t17579 * t388 + t18048 * t388 + 2.0_f64 * t3026 * t5920 - t3026 * t5944 + 2.0_f64 * t3169 * t5920 - t3169 * t5944 + 4.0_f64 * t4557 * t4665 - 2.0_f64 * t4557 * t4694 - 2.0_f64 * t4660 * t4694;
    (t18047, t18050)
}
