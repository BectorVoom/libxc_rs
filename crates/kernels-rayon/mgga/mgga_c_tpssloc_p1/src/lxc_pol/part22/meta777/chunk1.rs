//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2656/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656(t16288: f64, t6417: f64, t12385: f64, t20497: f64, t120: f64, t12369: f64, t12429: f64, t1307: f64, t1352: f64, t1363: f64, t16278: f64, t16394: f64, t19735: f64, t19871: f64, t19951: f64, t19989: f64, t20356: f64, t20416: f64, t20454: f64, t3803: f64, t3805: f64, t40070: f64, t5246: f64, t5248: f64, t53918: f64, t53920: f64, t54023: f64, t54162: f64, t6390: f64, t6396: f64, t6422: f64, t74120: f64, t820: f64) -> f64 {
    let t74217 = t16288 * t6417;
    let t74228 = t12385 * t20497;
    let t74253 = 7.0_f64 / 1536.0_f64 * t74217 - t16278 * t6422 / 1024.0_f64 + t54023 * t6390 / 512.0_f64 + 35.0_f64 / 128.0_f64 * t1363 * t40070 * t820 * t20356 * t1307 - 7.0_f64 / 768.0_f64 * t74228 - t53918 - t53920 + t12429 * t20454 / 256.0_f64 + t3803 * t3805 * t19871 * t19989 / 256.0_f64 + t54162 * t6396 / 128.0_f64 + t16394 * t19951 / 128.0_f64 - t5246 * t3805 * t74120 * t12369 / 128.0_f64 + 3.0_f64 / 512.0_f64 * t5246 * t5248 * t19871 * t19735 + t3803 * t3805 * t120 * t20416 * t1352 / 768.0_f64;
    t74253
}
