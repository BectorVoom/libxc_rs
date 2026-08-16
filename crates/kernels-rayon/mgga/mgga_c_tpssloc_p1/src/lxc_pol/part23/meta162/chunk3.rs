//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 760/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk760(t1315: f64, t1341: f64, t1363: f64, t1827: f64, t1831: f64, t3733: f64, t3762: f64, t3790: f64, t3803: f64, t3864: f64, t5220: f64, t5235: f64, t5238: f64, t5240: f64, t5255: f64, t5306: f64, t559: f64, t6371: f64, t6375: f64, t6379: f64, t6390: f64, t6396: f64, t6417: f64, t6422: f64, t6427: f64, t6431: f64) -> f64 {
    let t6434 = t3762 + 7.0_f64 / 72.0_f64 * t5220 + t3733 * t6371 / 16.0_f64 - t1315 * t6375 / 48.0_f64 + t6379 * t559 / 3072.0_f64 - t5235 * t1827 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t5238 - t5240 * t1831 / 384.0_f64 + t3790 * t6390 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t5255 + t3803 * t6396 / 384.0_f64 - t1341 * t6417 / 3072.0_f64 - t1341 * t6422 / 3072.0_f64 + t3864 + 7.0_f64 / 576.0_f64 * t5306 + 5.0_f64 / 768.0_f64 * t1363 * t6427 - t1363 * t6431 / 768.0_f64;
    t6434
}
