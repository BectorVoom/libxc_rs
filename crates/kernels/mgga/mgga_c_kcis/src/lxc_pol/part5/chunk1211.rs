//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1211/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1211<F: Float>(t19894: F, t19943: F, t20154: F, t20208: F, t1142: F, t13122: F, t15112: F, t15113: F, t18461: F, t18465: F, t18468: F, t18471: F, t18474: F, t18495: F, t18498: F, t18500: F, t18504: F, t18511: F, t18515: F, t18517: F, t18521: F, t18523: F, t18528: F) -> (F, F) {
    let t20210 = t19894 + t19943 + t20154 + t20208;
    let t20211 = t1142 * t20210;
    let t20228 = -F::cast_from(0.17411041666666666666e-2_f64) * t18461 + F::cast_from(0.11607361111111111111e-2_f64) * t18465 + F::cast_from(0.34822083333333333332e-2_f64) * t18468 - t15112 + t15113 + F::cast_from(0.15476481481481481481e-2_f64) * t18471 - F::cast_from(0.23214722222222222222e-2_f64) * t18474 + F::cast_from(0.15476481481481481481e-2_f64) * t18495 - F::cast_from(0.61905925925925925925e-2_f64) * t18498 - F::cast_from(0.11607361111111111111e-2_f64) * t18500 + F::cast_from(0.46429444444444444443e-2_f64) * t18504 - F::cast_from(0.51588271604938271603e-3_f64) * t13122 - F::cast_from(0.23214722222222222222e-2_f64) * t18511 + F::cast_from(0.19345601851851851852e-2_f64) * t18515 + F::cast_from(0.23214722222222222221e-2_f64) * t18517 - F::cast_from(0.15476481481481481481e-2_f64) * t18521 - F::cast_from(0.46429444444444444444e-2_f64) * t18523 + F::cast_from(0.38691203703703703703e-2_f64) * t18528;
    (t20211, t20228)
}
