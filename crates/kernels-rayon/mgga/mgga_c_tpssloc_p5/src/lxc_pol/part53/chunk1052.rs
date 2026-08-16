//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1052/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1052(t40611: f64, t8807: f64, t115925: f64, t120719: f64, t120721: f64, t120728: f64, t120730: f64, t120735: f64, t122654: f64, t124538: f64, t1266: f64, t12725: f64, t1799: f64, t22574: f64, t24432: f64, t26114: f64, t26161: f64, t26163: f64, t26179: f64, t26559: f64, t26872: f64, t27150: f64, t32111: f64, t32200: f64, t32213: f64, t32220: f64, t33916: f64, t4028: f64, t510: f64, t7042: f64, t7216: f64, t7458: f64, t7685: f64, t8721: f64) -> f64 {
    let t124580 = t8807 * t40611;
    let t124584 = 3.0_f64 * t7685 * t32111 - t33916 * t1266 - 3.0_f64 * t7685 * t32213 - 4.0_f64 * t4028 * t32200 - 4.0_f64 * t26114 * t8721 - 4.0_f64 * t26179 * t8721 - 4.0_f64 * t7458 * t32220 - 4.0_f64 * t7042 * t27150 - 4.0_f64 * t12725 * t8721 - t120719 - t120721 - t120728 - t120730 - t120735 - 2.0_f64 * t124538 * t510 + 4.0_f64 * t122654 * t26559 - 6.0_f64 * t115925 * t26872 - 6.0_f64 * t22574 * t24432 * t1799 * t7216 - 6.0_f64 * t26161 * t124580 * t26163;
    t124584
}
