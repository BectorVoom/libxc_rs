//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 639/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk639(t7155: f64, t7189: f64, t1404: f64, t1924: f64, t1979: f64, t4018: f64, t4023: f64, t486: f64, t510: f64, t538: f64, t5787: f64, t5799: f64, t7028: f64, t7113: f64, t7116: f64, t7119: f64, t7123: f64, t7142: f64) -> (f64, f64) {
    let t7190 = t7155 + t7189;
    let t7192 = t4018 + 0.46853067927761790996e-2_f64 * t5787 + 0.93706135855523581992e-2_f64 * t5799 + 0.46853067927761790996e-2_f64 * t4023 * t7113 + 0.93706135855523581992e-2_f64 * t1404 * t7116 - 0.23426533963880895498e-2_f64 * t1404 * t7119 + 0.14055920378328537299e-1_f64 * t510 * t7123 - 0.46853067927761790996e-2_f64 * t510 * t7142 - t7028 * t538 - 2.0_f64 * t1924 * t1979 - t486 * t7190;
    (t7190, t7192)
}
