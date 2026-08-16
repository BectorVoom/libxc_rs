//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2722/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722(t550: f64, t57266: f64, t57298: f64, t12215: f64, t12397: f64, t12419: f64, t1341: f64, t1343: f64, t16018: f64, t16206: f64, t1810: f64, t19868: f64, t19871: f64, t19979: f64, t210: f64, t3719: f64, t3733: f64, t3778: f64, t3803: f64, t3807: f64, t3856: f64, t39952: f64, t39975: f64, t40160: f64, t5246: f64, t5248: f64, t5249: f64, t54063: f64, t57143: f64, t57145: f64, t57147: f64, t57158: f64, t57160: f64, t57170: f64, t57172: f64, t6370: f64, t6390: f64, t6396: f64, t6417: f64, t820: f64) -> (f64, f64) {
    let t57300 = (t57266 + t57298) * t550;
    let t57305 = 119.0_f64 / 6912.0_f64 * t40160 - 5.0_f64 / 768.0_f64 * t3803 * t12419 * t19979 * t3856 - t3803 * t5248 * t19871 * t3856 / 3072.0_f64 + 35.0_f64 / 576.0_f64 * t57143 - 7.0_f64 / 576.0_f64 * t57145 + t5246 * t5248 * t5249 * t57147 / 768.0_f64 + t39975 * t6396 / 384.0_f64 - t3803 * t5248 * t5249 * t16206 / 1536.0_f64 + 7.0_f64 / 6.0_f64 * t57158 - 7.0_f64 / 12.0_f64 * t57160 - t12215 * t210 * t6370 * t3719 / 4.0_f64 + t3733 * t210 * t1810 * t16018 / 8.0_f64 - 7.0_f64 / 24.0_f64 * t57170 + 5.0_f64 / 64.0_f64 * t3803 * t54063 * t57172 * t3807 + t39952 * t6390 / 1536.0_f64 - t12397 * t6417 / 3072.0_f64 - t3778 * t19868 / 1536.0_f64 - t1341 * t1343 * t820 * t57300 / 3072.0_f64;
    (t57300, t57305)
}
