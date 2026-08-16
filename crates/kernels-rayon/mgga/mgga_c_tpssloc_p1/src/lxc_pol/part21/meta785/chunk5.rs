//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2724/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724(t550: f64, t56913: f64, t3862: f64, t6379: f64, t5293: f64, t53945: f64, t19921: f64, t3866: f64, t19926: f64, t12215: f64, t12397: f64, t12429: f64, t1307: f64, t1341: f64, t1343: f64, t1352: f64, t1363: f64, t16394: f64, t16405: f64, t19631: f64, t19843: f64, t19972: f64, t19996: f64, t20000: f64, t210: f64, t3733: f64, t3734: f64, t3783: f64, t3803: f64, t3870: f64, t40025: f64, t40282: f64, t5248: f64, t53990: f64, t54162: f64, t54582: f64, t56817: f64, t6370: f64, t6374: f64, t6422: f64, t820: f64) -> (f64, f64) {
    let t57354 = t56913 * t550;
    let t57383 = t6379 * t3862;
    let t57392 = t53945 * t5293;
    let t57396 = t3866 * t19921;
    let t57398 = t3866 * t19926;
    let t57400 = -t12397 * t6422 / 3072.0_f64 - t1341 * t1343 * t820 * t57354 / 1536.0_f64 + 5.0_f64 / 384.0_f64 * t3783 * t19996 + 5.0_f64 / 384.0_f64 * t1363 * t3870 * t820 * t19631 * t1307 + t3733 * t210 * t19843 * t1307 / 8.0_f64 - t53990 * t20000 / 256.0_f64 - t54162 * t5293 / 768.0_f64 + 119.0_f64 / 1728.0_f64 * t40282 + 5.0_f64 / 4.0_f64 * t40025 * t210 * t6370 * t3734 - t12215 * t210 * t6374 * t3734 / 4.0_f64 + 119.0_f64 / 13824.0_f64 * t57383 + 455.0_f64 / 324.0_f64 * t54582 - t3803 * t5248 * t56817 * t1352 / 1536.0_f64 - t12429 * t19972 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t57392 - 5.0_f64 / 384.0_f64 * t16394 * t16405 + 35.0_f64 / 96.0_f64 * t57396 - 35.0_f64 / 288.0_f64 * t57398;
    (t57354, t57400)
}
