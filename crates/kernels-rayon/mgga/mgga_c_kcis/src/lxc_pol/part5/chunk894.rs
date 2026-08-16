//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 894/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk894(t1592: f64, t2110: f64, t4399: f64, t5764: f64, t5766: f64, t6193: f64, t626: f64, t7102: f64, t7106: f64, t7109: f64, t7196: f64, t7199: f64, t7205: f64, t7208: f64, t7260: f64, t7264: f64, t7490: f64, t7510: f64) -> f64 {
    let t7532 = 0.66725e-1_f64 * t1592 * t7510 + 0.15476481481481481481e-2_f64 * t5764 - 0.23214722222222222222e-2_f64 * t5766 - t4399 - 0.23214722222222222222e-2_f64 * t7102 + 0.17024129629629629629e-1_f64 * t7106 - 0.92858888888888888886e-2_f64 * t7109 + 0.17411041666666666666e-2_f64 * t7196 + 0.23214722222222222222e-2_f64 * t7199 + 0.11607361111111111111e-2_f64 * t7205 - 0.34822083333333333332e-2_f64 * t7208 - 0.17411041666666666666e-2_f64 * t7260 - 0.13345e0_f64 * t6193 * t2110 + t7490 * t626 - 0.61905925925925925925e-2_f64 * t7264;
    t7532
}
