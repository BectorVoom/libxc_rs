//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 528/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk528(t4222: f64, t4320: f64, t1459: f64, t1553: f64, t1556: f64, t1598: f64, t3489: f64, t3497: f64, t3505: f64, t3510: f64, t3514: f64, t3735: f64, t3740: f64, t3745: f64, t3749: f64, t3752: f64, t3756: f64, t3762: f64, t3767: f64, t3771: f64) -> (f64, f64, f64, f64) {
    let t4321 = t4222 + t4320;
    let t4322 = t1459 * t4321;
    let t4324 = t1553 * t1556;
    let t4340 = 0.15476481481481481481e-2_f64 * t3489 - 0.386e0_f64 * t4324 * t1598 - 0.61905925925925925925e-2_f64 * t3497 + 0.11607361111111111111e-2_f64 * t3505 - 0.34822083333333333332e-2_f64 * t3510 + 0.23214722222222222222e-2_f64 * t3514 - 0.17411041666666666666e-2_f64 * t3735 - 0.23214722222222222222e-2_f64 * t3740 - 0.23214722222222222222e-2_f64 * t3745 + 0.15476481481481481481e-2_f64 * t3749 + 0.23214722222222222222e-2_f64 * t3752 + 0.11607361111111111111e-2_f64 * t3756 + 0.19345601851851851852e-2_f64 * t3762 - 0.61905925925925925925e-2_f64 * t3767 - 0.23214722222222222222e-2_f64 * t3771;
    (t4321, t4322, t4324, t4340)
}
