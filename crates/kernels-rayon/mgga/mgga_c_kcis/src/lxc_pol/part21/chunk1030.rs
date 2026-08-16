//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1030/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1030(t14104: f64, t14567: f64, t14576: f64, t10255: f64, t10257: f64, t10450: f64, t10452: f64, t10473: f64, t11209: f64, t14095: f64, t14100: f64, t14102: f64, t14108: f64, t14113: f64, t14377: f64, t14384: f64, t14388: f64, t14390: f64, t14574: f64, t15611: f64, t3644: f64) -> f64 {
    let t15648 = 0.15476481481481481481e-2_f64 * t14104;
    let t15659 = 0.23214722222222222222e-2_f64 * t14567;
    let t15662 = 0.15476481481481481481e-2_f64 * t14576;
    let t15663 = -0.61905925925925925926e-2_f64 * t10255 + 0.11349419753086419753e-1_f64 * t10257 + 0.69644166666666666664e-2_f64 * t14095 + 0.34822083333333333332e-2_f64 * t14100 + 0.46429444444444444443e-2_f64 * t14102 - t15648 + 0.20635308641975308642e-2_f64 * t14108 - 0.38691203703703703703e-3_f64 * t14113 - 0.2671335375e-1_f64 * t3644 * t15611 - 0.17411041666666666666e-2_f64 * t14377 - 0.77382407407407407406e-3_f64 * t10450 + 0.11607361111111111111e-2_f64 * t10452 + 0.46429444444444444444e-2_f64 * t14384 - 0.38691203703703703704e-2_f64 * t14388 - 0.25794135802469135802e-3_f64 * t14390 + t15659 + t11209 + 0.20635308641975308642e-2_f64 * t10473 + 0.19345601851851851852e-2_f64 * t14574 - t15662;
    t15663
}
