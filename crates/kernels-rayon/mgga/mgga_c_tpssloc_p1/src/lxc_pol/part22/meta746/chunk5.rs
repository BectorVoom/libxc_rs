//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2486/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486(t25548: f64, t360: f64, t10403: f64, t10408: f64, t13995: f64, t17177: f64, t17182: f64, t17920: f64, t17925: f64, t17972: f64, t3070: f64, t3071: f64, t3130: f64, t4582: f64, t4594: f64, t4644: f64, t49934: f64, t5681: f64, t62494: f64, t62499: f64, t62510: f64, t62515: f64, t70082: f64, t70391: f64) -> f64 {
    let t70735 = t25548 * t360;
    let t70756 = -t62494 / 3456.0_f64 - t62499 / 216.0_f64 - t10403 * t3071 * t5681 * t70082 / 384.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t17177 * t70735 + 5.0_f64 / 2304.0_f64 * t13995 * t17920 - t49934 * t17925 / 768.0_f64 - t3070 * t3071 * t17182 * t70735 / 768.0_f64 + t3130 * t4582 * t70391 * t4594 / 1536.0_f64 + t4644 * t17972 / 256.0_f64 - t62510 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t62515;
    t70756
}
