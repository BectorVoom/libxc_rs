//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2722/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722<F: Float>(t550: F, t57266: F, t57298: F, t12215: F, t12397: F, t12419: F, t1341: F, t1343: F, t16018: F, t16206: F, t1810: F, t19868: F, t19871: F, t19979: F, t210: F, t3719: F, t3733: F, t3778: F, t3803: F, t3807: F, t3856: F, t39952: F, t39975: F, t40160: F, t5246: F, t5248: F, t5249: F, t54063: F, t57143: F, t57145: F, t57147: F, t57158: F, t57160: F, t57170: F, t57172: F, t6370: F, t6390: F, t6396: F, t6417: F, t820: F) -> (F, F) {
    let t57300 = (t57266 + t57298) * t550;
    let t57305 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t40160 - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3803 * t12419 * t19979 * t3856 - t3803 * t5248 * t19871 * t3856 / F::cast_from(3072.0_f64) + F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t57143 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57145 + t5246 * t5248 * t5249 * t57147 / F::cast_from(768.0_f64) + t39975 * t6396 / F::cast_from(384.0_f64) - t3803 * t5248 * t5249 * t16206 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(6.0_f64) * t57158 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t57160 - t12215 * t210 * t6370 * t3719 / F::cast_from(4.0_f64) + t3733 * t210 * t1810 * t16018 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t57170 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t3803 * t54063 * t57172 * t3807 + t39952 * t6390 / F::cast_from(1536.0_f64) - t12397 * t6417 / F::cast_from(3072.0_f64) - t3778 * t19868 / F::cast_from(1536.0_f64) - t1341 * t1343 * t820 * t57300 / F::cast_from(3072.0_f64);
    (t57300, t57305)
}
