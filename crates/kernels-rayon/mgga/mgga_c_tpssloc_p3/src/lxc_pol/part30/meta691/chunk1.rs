//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2203/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2203(t1408: f64, t4119: f64, t193: f64, t7637: f64, t1530: f64, t4303: f64, t25373: f64, t22960: f64, t67123: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25028: f64, t2522: f64, t25358: f64, t25372: f64, t25375: f64, t25381: f64, t28448: f64, t28462: f64, t6542: f64, t6670: f64, t7541: f64, t7545: f64, t86836: f64, t97990: f64, t98000: f64, t98004: f64, t98008: f64, t98012: f64, t98015: f64) -> (f64, f64, f64) {
    let t98020 = t1408 * t4119;
    let t98027 = t193 * t7637;
    let t98030 = t1530 * t4303;
    let t98031 = t25373 * t98030;
    let t98034 = t22960 * t67123;
    let t98039 = -t1877 * t6670 * t97990 + 3.0_f64 / 2.0_f64 * t2522 * t28448 * t6542 + 3.0_f64 * t2522 * t7541 * t25028 - 3.0_f64 * t25372 * t98000 + 3.0_f64 * t22959 * t98004 - 3.0_f64 * t22959 * t98008 - 3.0_f64 / 2.0_f64 * t22959 * t98012 - 3.0_f64 * t22959 * t98015 - t1877 * t25358 * t25381 + 3.0_f64 * t2522 * t1915 * t98020 - t1877 * t23290 * t28462 / 2.0_f64 + 2.0_f64 * t98027 * t25375 + 2.0_f64 * t25372 * t98031 - 3.0_f64 / 2.0_f64 * t22959 * t98034 - t1877 * t86836 * t7545;
    (t98027, t98030, t98039)
}
