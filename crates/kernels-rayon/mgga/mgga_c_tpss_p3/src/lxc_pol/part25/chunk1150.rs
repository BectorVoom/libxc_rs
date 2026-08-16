//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1150/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1150(t1114: f64, t15846: f64, t3931: f64, t14911: f64, t4278: f64, t12490: f64, t14906: f64, t1125: f64, t12431: f64, t12465: f64, t12472: f64, t12477: f64, t12480: f64, t12530: f64, t12537: f64, t4234: f64, t4242: f64, t4265: f64, t4285: f64, t9607: f64) -> f64 {
    let t15868 = t15846 * t1114;
    let t15869 = t3931 * t15868;
    let t15872 = t4278 * t14911;
    let t15873 = t3931 * t15872;
    let t15876 = t12490 * t14906;
    let t15877 = t3931 * t15876;
    let t15880 = -t12465 + t12472 * t4242 / 432.0_f64 - t12477 - t12431 * t4234 / 144.0_f64 + t4265 * t4285 / 216.0_f64 + t9607 * t15869 / 3072.0_f64 - t12480 + t12530 - t12537 + 5.0_f64 / 6912.0_f64 * t1125 * t15873 + 5.0_f64 / 2304.0_f64 * t1125 * t15877;
    t15880
}
