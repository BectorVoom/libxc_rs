//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 841/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk841(t1268: f64, t6842: f64, t1240: f64, t1857: f64, t430: f64, t5003: f64, t5345: f64, t6558: f64, t6561: f64, t6564: f64, t6616: f64, t6622: f64, t6627: f64, t6631: f64, t6738: f64, t6835: f64) -> (f64, f64) {
    let t6843 = t6842 * t1268;
    let t6855 = 0.11607361111111111111e-2_f64 * t6558 - 0.34822083333333333332e-2_f64 * t6561 + 0.23214722222222222222e-2_f64 * t6564 - 0.17411041666666666666e-2_f64 * t6616 - 0.66725e-1_f64 * t1240 * t6843 - 0.13345e0_f64 * t5345 * t1857 + 0.15476481481481481481e-2_f64 * t5003 + 0.66725e-1_f64 * t1240 * t6738 + t6835 * t430 - 0.23214722222222222222e-2_f64 * t6622 + 0.15476481481481481481e-2_f64 * t6627 - 0.23214722222222222222e-2_f64 * t6631;
    (t6843, t6855)
}
