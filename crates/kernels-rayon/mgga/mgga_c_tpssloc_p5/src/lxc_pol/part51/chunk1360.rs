//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1360/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1360(t25980: f64, t7042: f64, t33553: f64, t650: f64, t31759: f64, t7685: f64, t31300: f64, t91655: f64, t120954: f64, t120958: f64, t120962: f64, t120964: f64, t120966: f64, t1976: f64, t27145: f64, t27170: f64, t31246: f64, t33133: f64, t652: f64, t7156: f64, t7220: f64, t7451: f64, t7904: f64, t8450: f64) -> f64 {
    let t120968 = 2.0_f64 * t7042 * t25980;
    let t120973 = t650 * t33553;
    let t120975 = 3.0_f64 * t7685 * t31759;
    let t120979 = 3.0_f64 * t91655 * t31300;
    let t120980 = -2.0_f64 * t1976 * t27170 * t652 + t27145 * t8450 + 3.0_f64 * t31246 * t7904 - t33133 * t7220 - t7156 * t7451 - t120954 + t120958 - t120962 - t120964 - t120966 - t120968 - t120973 + t120975 - t120979;
    t120980
}
