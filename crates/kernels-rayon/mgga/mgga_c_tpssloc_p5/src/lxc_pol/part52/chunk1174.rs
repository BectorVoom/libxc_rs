//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1174/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1174(t3806: f64, t5248: f64, t550: f64, t31170: f64, t553: f64, t835: f64, t544: f64, t8467: f64, t1369: f64, t8466: f64, t31154: f64, t31157: f64, t31161: f64, t31163: f64, t31166: f64) -> (f64, f64, f64, f64, f64) {
    let t31172 = t5248 * t3806 * t550;
    let t31173 = t31170 * t31172;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    let t31178 = 7.0_f64 / 2304.0_f64 * t31177;
    let t31179 = t8466 * t1369;
    let t31181 = -t31154 - 0.48447307312968469025e-2_f64 * t31157 - t31161 - 0.80745512188280781708e-3_f64 * t31163 + t31166 / 1536.0_f64 - t31173 / 1536.0_f64 - t31178 - t31179 / 384.0_f64;
    (t31172, t31175, t31176, t31178, t31181)
}
