//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 611/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk611<F: Float>(t1598: F, t3489: F, t3497: F, t3505: F, t3510: F, t3514: F, t3735: F, t3740: F, t3745: F, t3749: F, t3752: F, t3756: F, t3762: F, t3767: F, t3771: F, t4324: F) -> (F,) {
    let t4340 = 0.15476481481481481481e-2 * t3489 - 0.386e0 * t4324 * t1598 - 0.61905925925925925925e-2 * t3497 + 0.11607361111111111111e-2 * t3505 - 0.34822083333333333332e-2 * t3510 + 0.23214722222222222222e-2 * t3514 - 0.17411041666666666666e-2 * t3735 - 0.23214722222222222222e-2 * t3740 - 0.23214722222222222222e-2 * t3745 + 0.15476481481481481481e-2 * t3749 + 0.23214722222222222222e-2 * t3752 + 0.11607361111111111111e-2 * t3756 + 0.19345601851851851852e-2 * t3762 - 0.61905925925925925925e-2 * t3767 - 0.23214722222222222222e-2 * t3771;
    (t4340,)
}
