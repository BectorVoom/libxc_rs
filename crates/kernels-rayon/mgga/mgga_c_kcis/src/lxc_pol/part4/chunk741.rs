//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 741/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk741(t4114: f64, t1592: f64, t3725: f64, t3729: f64, t3731: f64, t3736: f64, t3740: f64, t3957: f64, t4112: f64, t4117: f64, t4127: f64, t4315: f64, t4390: f64, t626: f64) -> (f64, f64) {
    let t4399 = 0.38691203703703703703e-3_f64 * t4114;
    let t4402 = 0.66725e-1_f64 * t1592 * t4315 + t4390 * t626 + 0.11607361111111111111e-2_f64 * t3725 - 0.23214722222222222222e-2_f64 * t3729 + 0.15476481481481481481e-2_f64 * t3731 - 0.34822083333333333332e-2_f64 * t3736 + 0.23214722222222222222e-2_f64 * t3740 - 0.17411041666666666666e-2_f64 * t3957 + 0.17411041666666666666e-2_f64 * t4112 - t4399 + 0.23214722222222222222e-2_f64 * t4117 + 0.34822083333333333332e-2_f64 * t4127;
    (t4399, t4402)
}
