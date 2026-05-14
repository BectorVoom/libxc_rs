//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1054/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1054<F: Float>(t17739: F, t17750: F, t17757: F, t17765: F, t11663: F, t12325: F, t17139: F, t17143: F, t17150: F, t17154: F, t17159: F, t17744: F, t17748: F, t17755: F, t17761: F, t18672: F, t18776: F, t18793: F, t1994: F, t5440: F, t5445: F, t7553: F, t7648: F, t795: F) -> (F,) {
    let t18826 = 0.10317654320987654321e-2 * t17739;
    let t18829 = 0.15476481481481481481e-2 * t17750;
    let t18831 = 0.30952962962962962962e-2 * t17757;
    let t18833 = 0.25794135802469135802e-2 * t17765;
    let t18834 = -0.92858888888888888886e-2 * t17139 + 0.12897067901234567901e-2 * t17143 - 0.223494e0 * t5445 * t18776 + 0.148996e0 * t12325 * t7553 + 0.386e0 * t1994 * t18793 - 0.23214722222222222222e-2 * t11663 + 0.193e0 * t7648 * t5440 - 0.23214722222222222222e-2 * t17150 - 0.23214722222222222222e-2 * t17154 + t18672 * t795 - 0.11607361111111111111e-2 * t17159 + t18826 - 0.25794135802469135802e-3 * t17744 - 0.15476481481481481481e-2 * t17748 - t18829 + 0.23214722222222222222e-2 * t17755 - t18831 + 0.69644166666666666666e-2 * t17761 + t18833;
    (t18834,)
}
