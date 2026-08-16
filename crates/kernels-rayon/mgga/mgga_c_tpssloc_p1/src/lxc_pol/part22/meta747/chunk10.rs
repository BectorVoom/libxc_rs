//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2498/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2498(t1052: f64, t1065: f64, t14529: f64, t14552: f64, t1603: f64, t1634: f64, t1635: f64, t18047: f64, t18074: f64, t18165: f64, t21614: f64, t21676: f64, t21677: f64, t3026: f64, t3169: f64, t3174: f64, t349: f64, t388: f64, t43604: f64, t4665: f64, t5920: f64, t5944: f64, t60971: f64, t61061: f64, t61621: f64, t70938: f64, t990: f64) -> f64 {
    let t71049 = 24.0_f64 * t1052 * t1065 * t21676 * t43604 + 6.0_f64 * t1052 * t1634 * t18165 * t3174 + 3.0_f64 * t1603 * t18047 * t388 + t21614 * t388 * t990 + t349 * t388 * t70938 + 6.0_f64 * t14529 * t5920 + 6.0_f64 * t14552 * t5920 - 3.0_f64 * t14552 * t5944 - 6.0_f64 * t1635 * t60971 - 3.0_f64 * t1635 * t61061 - 3.0_f64 * t1635 * t61621 + 6.0_f64 * t18074 * t4665 - 6.0_f64 * t21677 * t3026 - 6.0_f64 * t21677 * t3169;
    t71049
}
