//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1026/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1026(t2679: f64, t4180: f64, t4181: f64, t4240: f64, t9638: f64, t13242: f64, t2645: f64, t2647: f64, t10007: f64, t4191: f64, t13275: f64, t13277: f64, t13280: f64, t13283: f64, t13287: f64, t13289: f64, t13293: f64, t13297: f64, t13302: f64, t13306: f64, t13312: f64, t1512: f64, t2571: f64, t2618: f64, t2635: f64, t2643: f64, t2686: f64, t4167: f64, t4236: f64, t4250: f64, t9559: f64, t9613: f64, t9642: f64) -> f64 {
    let t13316 = t4180 * t4181 * t2679;
    let t13320 = 7.0_f64 / 2304.0_f64 * t9638 * t4240;
    let t13322 = t2645 * t13242 * t2647;
    let t13326 = t2645 * t4181 * t10007;
    let t13330 = 7.0_f64 / 576.0_f64 * t9638 * t4191;
    let t13331 = -t9613 * t1512 / 3072.0_f64 - t2618 * t4236 / 1536.0_f64 + t13275 + t13277 + t13280 - t4167 * t2686 / 3072.0_f64 + t13283 * t2635 / 1536.0_f64 - t13287 - t9559 * t13289 / 4.0_f64 + t2571 * t13293 / 8.0_f64 + t2571 * t13297 / 16.0_f64 + t2643 * t13302 / 384.0_f64 + t2643 * t13306 / 768.0_f64 + t9642 * t4250 / 384.0_f64 - t2643 * t13312 / 1536.0_f64 - t2643 * t13316 / 3072.0_f64 + t13320 + t2643 * t13322 / 384.0_f64 + t2643 * t13326 / 768.0_f64 - t13330;
    t13331
}
