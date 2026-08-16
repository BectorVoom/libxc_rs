//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 865/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk865<F: Float>(t1343: F, t6415: F, t820: F, t550: F, t6387: F, t3870: F, t6330: F, t1367: F, t6347: F, t1315: F, t1341: F, t1363: F, t1827: F, t1831: F, t3733: F, t3762: F, t3790: F, t3803: F, t3864: F, t5220: F, t5235: F, t5238: F, t5240: F, t5255: F, t5306: F, t559: F, t6371: F, t6375: F, t6379: F, t6390: F, t6396: F) -> (F, F, F, F, F, F) {
    let t6417 = t1343 * t820 * t6415;
    let t6420 = t6387 * t550;
    let t6422 = t1343 * t820 * t6420;
    let t6427 = t3870 * t820 * t6330;
    let t6431 = t1367 * t820 * t6347;
    let t6434 = t3762 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t5220 + t3733 * t6371 / F::cast_from(16.0_f64) - t1315 * t6375 / F::cast_from(48.0_f64) + t6379 * t559 / F::cast_from(3072.0_f64) - t5235 * t1827 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t5238 - t5240 * t1831 / F::cast_from(384.0_f64) + t3790 * t6390 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t5255 + t3803 * t6396 / F::cast_from(384.0_f64) - t1341 * t6417 / F::cast_from(3072.0_f64) - t1341 * t6422 / F::cast_from(3072.0_f64) + t3864 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t5306 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t6427 - t1363 * t6431 / F::cast_from(768.0_f64);
    (t6417, t6420, t6422, t6427, t6431, t6434)
}
