//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2977/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977(t1041: f64, t10868: f64, t248: f64, t5681: f64, t10949: f64, t14080: f64, t14172: f64, t14187: f64, t1622: f64, t17643: f64, t17734: f64, t17972: f64, t3117: f64, t4582: f64, t4583: f64, t4588: f64, t4636: f64, t49716: f64, t49721: f64, t49732: f64, t49740: f64, t50334: f64, t55662: f64, t55666: f64, t62044: f64) -> f64 {
    let t62137 = t1041 * t248 * t10868 * t5681;
    let t62145 = t10949 * t17734 / 384.0_f64 + t3117 * t17972 / 384.0_f64 + t49716 / 576.0_f64 + t49721 / 2304.0_f64 + t49732 / 72.0_f64 - t1041 * t4582 * t4583 * t55666 / 1152.0_f64 - t1041 * t4582 * t4583 * t55662 / 2304.0_f64 - 5.0_f64 / 2304.0_f64 * t1041 * t4582 * t14172 * t62044 + 5.0_f64 / 6912.0_f64 * t3117 * t17643 + 5.0_f64 / 6912.0_f64 * t1041 * t4582 * t4588 * t55666 + 5.0_f64 / 13824.0_f64 * t1041 * t4582 * t4588 * t55662 + 5.0_f64 / 5184.0_f64 * t1041 * t4582 * t14187 * t62044 + t62137 / 10368.0_f64 + 19.0_f64 / 1296.0_f64 * t50334 * t1622 - t49740 * t1622 / 216.0_f64 - t14080 * t4636 / 216.0_f64;
    t62145
}
