//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2677/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677(t12364: f64, t5234: f64, t1354: f64, t16288: f64, t3858: f64, t1351: f64, t3734: f64, t12012: f64, t12168: f64, t12413: f64, t1341: f64, t1343: f64, t1363: f64, t16101: f64, t16208: f64, t16224: f64, t16311: f64, t16394: f64, t1799: f64, t221: f64, t3719: f64, t3778: f64, t3803: f64, t3805: f64, t3870: f64, t40160: f64, t5187: f64, t5246: f64, t5248: f64, t5250: f64, t5301: f64, t53958: f64, t54284: f64, t54293: f64, t54295: f64, t54527: f64, t820: f64) -> f64 {
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54534 = 119.0_f64 / 4608.0_f64 * t54533;
    let t54535 = t16288 * t3858;
    let t54542 = t1351 * t3734;
    let t54552 = -t16394 * t12413 / 1024.0_f64 + t5246 * t5248 * t53958 * t5250 / 512.0_f64 - 3.0_f64 / 4.0_f64 * t16101 * t221 * t54284 + 5.0_f64 / 768.0_f64 * t1363 * t3870 * t820 * t1799 * t12012 + 7.0_f64 / 768.0_f64 * t54293 + 7.0_f64 / 1536.0_f64 * t54295 - t3778 * t16208 / 1024.0_f64 - t1341 * t1343 * t820 * t54527 / 3072.0_f64 - t54534 + 7.0_f64 / 1536.0_f64 * t54535 + t3803 * t3805 * t5301 * t12168 / 768.0_f64 + 119.0_f64 / 2304.0_f64 * t40160 + 5.0_f64 / 128.0_f64 * t5246 * t16224 * t16311 * t54542 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t5187 * t3719;
    t54552
}
