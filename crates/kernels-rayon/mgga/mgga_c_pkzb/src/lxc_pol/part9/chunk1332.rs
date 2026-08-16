//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1332/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1332(t1259: f64, t1306: f64, t19339: f64, t22688: f64, t22721: f64, t22724: f64, t22726: f64, t22729: f64, t22731: f64, t22733: f64, t22822: f64, t22902: f64, t22904: f64, t6359: f64) -> f64 {
    let t23567 = -6.0_f64 * t1259 * t1306 * t19339 * t6359 + t22688 - t22721 + t22724 + t22726 + t22729 + t22731 + t22733 - t22822 + t22902 - t22904;
    t23567
}
