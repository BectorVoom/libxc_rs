//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3201/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201(t15569: f64, t15572: f64, t11665: f64, t15714: f64, t15740: f64, t15749: f64, t18364: f64, t3242: f64, t3508: f64, t3577: f64, t3578: f64, t45250: f64, t4733: f64, t4950: f64, t5012: f64, t52615: f64, t53433: f64, t53440: f64, t53452: f64, t66372: f64, t66378: f64, t66380: f64, t66566: f64, t66571: f64, t66575: f64, t66583: f64, t66597: f64) -> f64 {
    let t66599 = t15569 * t15572;
    let t66601 = -t66566 / 3456.0_f64 + 5.0_f64 / 6912.0_f64 * t11665 * t18364 - t45250 + t66571 / 324.0_f64 - 2.0_f64 / 243.0_f64 * t53433 + t66575 / 162.0_f64 - 5.0_f64 / 1944.0_f64 * t53440 + 5.0_f64 / 6912.0_f64 * t15740 * t15714 + t52615 * t4950 / 216.0_f64 + 5.0_f64 / 1728.0_f64 * t66378 * t66583 * t3508 * t3242 * t66380 - 5.0_f64 / 3456.0_f64 * t66372 * t66583 * t15749 - t53452 / 1728.0_f64 - t3577 * t3578 * t5012 * t4733 / 1152.0_f64 - t66597 / 1728.0_f64 + t66599 / 324.0_f64;
    t66601
}
