//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1372/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1372(t10482: f64, t23508: f64, t11060: f64, t3120: f64, t11045: f64, t42332: f64, t42340: f64, t42341: f64, t43288: f64, t43292: f64, t1049: f64, t1058: f64, t1060: f64, t10857: f64, t11034: f64, t11037: f64, t11040: f64, t11049: f64, t11055: f64, t11059: f64, t11081: f64, t3187: f64, t3200: f64, t3201: f64, t43483: f64, t43489: f64, t43504: f64, t43525: f64, t43553: f64) -> f64 {
    let t43554 = t23508 * t10482;
    let t43558 = t11060 * t3120;
    let t43562 = t42332 * t11045;
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43584 = 4.0_f64 * t1049 * t1058 * t1060 * t10857 + 24.0_f64 * t11059 * t11060 * t43483 + 36.0_f64 * t11059 * t3187 * t43558 - 6.0_f64 * t3200 * t3201 * t43489 - 3.0_f64 * t3200 * t3201 * t43525 - 36.0_f64 * t43504 * t43553 * t43554 + 24.0_f64 * t43504 * t43576 * t43577 + 24.0_f64 * t11034 * t11055 - 12.0_f64 * t11037 * t11040 - 12.0_f64 * t11037 * t11081 + 4.0_f64 * t11049 * t43562;
    t43584
}
