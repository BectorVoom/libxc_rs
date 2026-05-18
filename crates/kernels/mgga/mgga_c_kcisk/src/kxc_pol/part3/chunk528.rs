//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 528/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk528<F: Float>(t4222: F, t4320: F, t1459: F, t1553: F, t1556: F, t1598: F, t3489: F, t3497: F, t3505: F, t3510: F, t3514: F, t3735: F, t3740: F, t3745: F, t3749: F, t3752: F, t3756: F, t3762: F, t3767: F, t3771: F) -> (F, F, F, F) {
    let t4321 = t4222 + t4320;
    let t4322 = t1459 * t4321;
    let t4324 = t1553 * t1556;
    let t4340 = F::new(0.15476481481481481481e-2) * t3489 - F::new(0.386e0) * t4324 * t1598 - F::new(0.61905925925925925925e-2) * t3497 + F::new(0.11607361111111111111e-2) * t3505 - F::new(0.34822083333333333332e-2) * t3510 + F::new(0.23214722222222222222e-2) * t3514 - F::new(0.17411041666666666666e-2) * t3735 - F::new(0.23214722222222222222e-2) * t3740 - F::new(0.23214722222222222222e-2) * t3745 + F::new(0.15476481481481481481e-2) * t3749 + F::new(0.23214722222222222222e-2) * t3752 + F::new(0.11607361111111111111e-2) * t3756 + F::new(0.19345601851851851852e-2) * t3762 - F::new(0.61905925925925925925e-2) * t3767 - F::new(0.23214722222222222222e-2) * t3771;
    (t4321, t4322, t4324, t4340)
}
