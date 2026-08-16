//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1210/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1210(t20716: f64, t20748: f64, t20751: f64, t17351: f64, t17354: f64, t17357: f64, t17405: f64, t17408: f64, t17411: f64, t17414: f64, t17417: f64, t17487: f64, t17505: f64, t20705: f64, t20708: f64, t20710: f64, t20719: f64, t20745: f64, t20754: f64) -> f64 {
    let t21055 = 0.20659e1_f64 * t20716;
    let t21058 = 0.104195e1_f64 * t20748;
    let t21059 = 0.104195e1_f64 * t20751;
    let t21062 = -0.27785333333333333333e1_f64 * t17405 + 0.104195e1_f64 * t17411 - 0.62517e0_f64 * t17414 - 0.20839e0_f64 * t17417 - 0.16068111111111111111e1_f64 * t20705 + 0.794188125e1_f64 * t20708 - 0.473371875e0_f64 * t20710 + t17505 - 0.48204333333333333334e1_f64 * t17351 + 0.20659e1_f64 * t17354 - 0.516475e0_f64 * t17357 + t21055 - 0.1549425e1_f64 * t20719 + 0.1549425e1_f64 * t20745 + t21058 + t21059 - 0.92617777777777777779e0_f64 * t20754 + t17487 + 0.104195e1_f64 * t17408;
    t21062
}
