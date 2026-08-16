//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1349/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1349(t5866: f64, t5872: f64, t1021: f64, t10408: f64, t1041: f64, t10413: f64, t10482: f64, t1622: f64, t17177: f64, t17607: f64, t17923: f64, t18030: f64, t21393: f64, t21398: f64, t21516: f64, t248: f64, t28651: f64, t3039: f64, t3070: f64, t3071: f64, t360: f64, t43291: f64, t43292: f64, t43385: f64, t43399: f64, t4644: f64, t48570: f64, t50265: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t61663: f64, t61736: f64, t70122: f64, t70132: f64, t70138: f64, t70153: f64, t76572: f64) -> (f64, f64, f64) {
    let t76722 = t5866 * t5866;
    let t76740 = t5872 * t5872;
    let t76768 = t61736 * t5875 / 256.0_f64 - t70132 / 288.0_f64 - t3039 * t248 * t1021 * t76722 * t360 / 1024.0_f64 + 5.0_f64 / 1296.0_f64 * t4644 * t21516 + 55.0_f64 / 15552.0_f64 * t1041 * t248 * t43399 * t76572 + t70153 * t1622 / 1152.0_f64 + t48570 * t21393 / 128.0_f64 - t50265 * t21398 / 128.0_f64 + t43291 * t248 * t1021 * t76740 * t43292 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t43385 * t248 * t1021 * t76740 * t10482 + t70138 / 576.0_f64 - t10413 * t3071 * t70122 * t17923 / 384.0_f64 + 5.0_f64 / 1152.0_f64 * t3070 * t10408 * t17177 * t28651 * t360 - t61663 / 1152.0_f64 + t17607 * t5857 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t17607 * t5861 + t18030 * t5869 / 512.0_f64;
    (t76722, t76740, t76768)
}
