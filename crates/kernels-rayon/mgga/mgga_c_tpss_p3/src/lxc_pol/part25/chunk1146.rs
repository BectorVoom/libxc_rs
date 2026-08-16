//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1146/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1146(t15599: f64, t15795: f64, t450: f64, t1112: f64, t242: f64, t1098: f64, t1111: f64, t12406: f64, t12409: f64, t15569: f64, t15574: f64, t15578: f64, t15582: f64, t15586: f64, t15590: f64, t15596: f64, t3067: f64, t4212: f64, t4228: f64, t9556: f64, t9573: f64) -> (f64, f64) {
    let t15796 = t15599 + t15795;
    let t15797 = t15796 * t450;
    let t15799 = t242 * t1112 * t15797;
    let t15802 = -t9556 * t15569 / 1152.0_f64 + t9573 * t15574 / 2304.0_f64 - t3067 * t15578 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t3067 * t15582 + t12406 + t12409 - t3067 * t15586 / 2304.0_f64 - t3067 * t15590 / 4608.0_f64 + t4212 * t4228 / 54.0_f64 - t1098 * t15596 / 288.0_f64 + t1111 * t15799 / 3072.0_f64;
    (t15796, t15802)
}
