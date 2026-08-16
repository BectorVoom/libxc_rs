//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1364/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1364(t96005: f64, t1646: f64, t28110: f64, t3616: f64, t5310: f64, t96018: f64, t13173: f64, t15231: f64, t2192: f64, t2197: f64, t26960: f64, t26961: f64, t28123: f64, t3515: f64, t3611: f64, t46577: f64, t92964: f64, t92976: f64, t92981: f64, t96000: f64, t96003: f64, t96021: f64) -> (f64, f64) {
    let t97173 = 0.15476481481481481481e-2_f64 * t96005;
    let t97188 = t5310 * t28110 * t1646 * t3616;
    let t97193 = 0.23214722222222222222e-2_f64 * t96018;
    let t97195 = 0.38691203703703703703e-3_f64 * t96000 + 0.11607361111111111111e-2_f64 * t96003 + t97173 - 0.34752604166666666667e-3_f64 * t46577 * t2192 * t2197 - t92964 - 0.61782407407407407408e-3_f64 * t26960 * t15231 * t28123 * t13173 + 0.11584201388888888889e-3_f64 * t26960 * t3515 * t26961 * t1646 * t3611 + 0.11584201388888888889e-3_f64 * t26960 * t97188 - 0.11584201388888888889e-3_f64 * t92976 - 0.41270617283950617284e-2_f64 * t92981 - t97193 + 0.92858888888888888886e-2_f64 * t96021;
    (t97188, t97195)
}
