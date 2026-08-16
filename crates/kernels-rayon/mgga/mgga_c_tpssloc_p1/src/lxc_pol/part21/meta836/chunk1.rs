//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2971/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971(t1041: f64, t13969: f64, t17687: f64, t14085: f64, t4571: f64, t13765: f64, t13995: f64, t18086: f64, t3069: f64, t10403: f64, t10413: f64, t10891: f64, t14041: f64, t14130: f64, t14218: f64, t17718: f64, t2776: f64, t3041: f64, t3070: f64, t3071: f64, t3073: f64, t3121: f64, t3132: f64, t42483: f64, t4582: f64, t4650: f64, t47779: f64, t48611: f64, t49658: f64, t49661: f64, t49666: f64, t5685: f64, t5867: f64, t61855: f64) -> f64 {
    let t61923 = t1041 * t13969 * t17687;
    let t61929 = t14085 * t4571;
    let t61940 = t13995 * t13765;
    let t61950 = t18086 * t3069;
    let t61965 = -5.0_f64 / 1728.0_f64 * t61923 + 5.0_f64 / 384.0_f64 * t1041 * t4582 * t47779 * t61855 + t61929 / 1728.0_f64 + t10891 * t17718 / 288.0_f64 - 4.0_f64 / 243.0_f64 * t49658 - t49661 / 243.0_f64 + t49666 / 3456.0_f64 + t42483 * t48611 * t14218 * t4650 / 768.0_f64 + t61940 / 1728.0_f64 - t13995 * t14130 / 1152.0_f64 + t13995 * t14041 / 2304.0_f64 - t3070 * t3071 * t5867 * t2776 / 2304.0_f64 + t61950 * t3073 / 2304.0_f64 + t3070 * t3071 * t5685 * t3121 / 4608.0_f64 + t10403 * t3071 * t5685 * t3132 / 2304.0_f64 - t10413 * t3071 * t5685 * t3041 / 4608.0_f64;
    t61965
}
