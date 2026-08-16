//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1129/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1129(t12278: f64, t15257: f64, t1098: f64, t1125: f64, t12290: f64, t12294: f64, t12319: f64, t15519: f64, t15523: f64, t15527: f64, t15533: f64, t15536: f64, t4265: f64, t4289: f64, t9543: f64) -> f64 {
    let t15539 = t12278 * t15257;
    let t15542 = t15519 / 648.0_f64 - t15523 / 4608.0_f64 + t15527 / 4608.0_f64 - t9543 / 13824.0_f64 + t12290 - t12294 + t4265 * t4289 / 432.0_f64 - t1125 * t15533 / 4608.0_f64 - t12319 + t1098 * t15536 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t1098 * t15539;
    t15542
}
