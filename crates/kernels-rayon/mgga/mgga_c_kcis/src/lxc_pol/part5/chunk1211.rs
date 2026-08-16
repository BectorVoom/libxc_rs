//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1211/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1211(t19894: f64, t19943: f64, t20154: f64, t20208: f64, t1142: f64, t13122: f64, t15112: f64, t15113: f64, t18461: f64, t18465: f64, t18468: f64, t18471: f64, t18474: f64, t18495: f64, t18498: f64, t18500: f64, t18504: f64, t18511: f64, t18515: f64, t18517: f64, t18521: f64, t18523: f64, t18528: f64) -> (f64, f64) {
    let t20210 = t19894 + t19943 + t20154 + t20208;
    let t20211 = t1142 * t20210;
    let t20228 = -0.17411041666666666666e-2_f64 * t18461 + 0.11607361111111111111e-2_f64 * t18465 + 0.34822083333333333332e-2_f64 * t18468 - t15112 + t15113 + 0.15476481481481481481e-2_f64 * t18471 - 0.23214722222222222222e-2_f64 * t18474 + 0.15476481481481481481e-2_f64 * t18495 - 0.61905925925925925925e-2_f64 * t18498 - 0.11607361111111111111e-2_f64 * t18500 + 0.46429444444444444443e-2_f64 * t18504 - 0.51588271604938271603e-3_f64 * t13122 - 0.23214722222222222222e-2_f64 * t18511 + 0.19345601851851851852e-2_f64 * t18515 + 0.23214722222222222221e-2_f64 * t18517 - 0.15476481481481481481e-2_f64 * t18521 - 0.46429444444444444444e-2_f64 * t18523 + 0.38691203703703703703e-2_f64 * t18528;
    (t20211, t20228)
}
