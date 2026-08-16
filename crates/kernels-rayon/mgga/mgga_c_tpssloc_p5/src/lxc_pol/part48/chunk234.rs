//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 234/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk234(t1036: f64, t370: f64, t368: f64, t372: f64, t364: f64, t354: f64, t270: f64, t283: f64, t61: f64, t248: f64, t884: f64, t1000: f64, t1005: f64, t1020: f64, t1025: f64, t1032: f64, t350: f64, t378: f64, t964: f64, t973: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1038 = t370 * t1036 / 4608.0_f64;
    let t1039 = t368 * t372;
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = 1.0_f64 / t283 / t270;
    let t1044 = t61 * t1043;
    let t1046 = t248 * t1044 * t884;
    let t1049 = -t964 * t350 / 36.0_f64 + t997 + t973 * t1000 / 288.0_f64 + t1005 * t378 / 3072.0_f64 + t1020 * t1025 / 3072.0_f64 - t1032 * t378 / 576.0_f64 + t1038 + t1041 * t1046 / 4608.0_f64;
    (t1040, t1041, t1043, t1044, t1046, t1049)
}
