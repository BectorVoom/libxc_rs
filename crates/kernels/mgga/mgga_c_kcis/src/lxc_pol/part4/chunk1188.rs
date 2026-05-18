//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1188/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1188<F: Float>(t1281: F, t5358: F, t13101: F, t13103: F, t13108: F, t13111: F, t13115: F, t13122: F, t13126: F, t13135: F, t13139: F, t13153: F, t13158: F, t13163: F, t13166: F, t13169: F, t13176: F, t13179: F, t13183: F, t9379: F, t9383: F, t9387: F) -> (F, F) {
    let t15109 = t5358 * t1281;
    let t15112 = F::new(0.23214722222222222222e-2) * t13101;
    let t15113 = F::new(0.15476481481481481481e-2) * t13103;
    let t15132 = -t15112 + t15113 - F::new(0.34822083333333333332e-2) * t13108 - F::new(0.17411041666666666666e-2) * t13111 + F::new(0.23214722222222222222e-2) * t13115 + F::new(0.15476481481481481481e-2) * t9379 - F::new(0.25794135802469135802e-3) * t13122 - F::new(0.17024129629629629629e-1) * t13126 + F::new(0.51588271604938271605e-2) * t13135 - F::new(0.15476481481481481481e-2) * t13139 + F::new(0.77382407407407407407e-3) * t9383 + F::new(0.12897067901234567901e-2) * t9387 - F::new(0.23214722222222222222e-2) * t13153 - F::new(0.11607361111111111111e-1) * t13158 + F::new(0.19345601851851851852e-2) * t13163 + F::new(0.11607361111111111111e-2) * t13166 + F::new(0.92858888888888888886e-2) * t13169 - F::new(0.92858888888888888888e-2) * t13176 - F::new(0.61905925925925925925e-2) * t13179 + F::new(0.23214722222222222222e-2) * t13183;
    (t15109, t15132)
}
