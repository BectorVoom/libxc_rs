//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 664/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk664(t4314: f64, t7509: f64, t1592: f64, t4414: f64, t5681: f64, t5684: f64, t5686: f64, t6906: f64, t6910: f64, t6915: f64, t6920: f64, t6925: f64, t6930: f64, t6934: f64, t7031: f64, t7035: f64, t7039: f64, t7498: f64) -> (f64, f64) {
    let t7510 = t7509 * t4314;
    let t7515 = 0.15476481481481481481e-2_f64 * t6906 - 0.61905925925925925925e-2_f64 * t6910 - 0.23214722222222222222e-2_f64 * t6915 - 0.66725e-1_f64 * t1592 * t7498 - 0.23214722222222222222e-2_f64 * t6920 - 0.38691203703703703703e-3_f64 * t6925 + 0.34822083333333333332e-2_f64 * t6930 + 0.92858888888888888886e-2_f64 * t6934 + 0.23214722222222222222e-2_f64 * t7031 + 0.11607361111111111111e-2_f64 * t7035 + 0.19345601851851851852e-2_f64 * t7039 + 0.15476481481481481481e-2_f64 * t5681 + 0.890445125e-2_f64 * t4414 * t7510 - 0.61905925925925925925e-2_f64 * t5684 + 0.23214722222222222222e-2_f64 * t5686;
    (t7510, t7515)
}
