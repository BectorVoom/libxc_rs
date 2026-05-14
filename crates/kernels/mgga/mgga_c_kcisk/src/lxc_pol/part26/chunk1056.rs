//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1056/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1056<F: Float>(t14224: F, t14226: F, t14250: F, t21434: F, t22035: F, t22036: F, t22037: F, t22038: F, t2332: F, t26976: F, t26990: F, t27006: F, t27008: F, t27013: F, t27028: F, t27030: F, t27035: F, t27037: F, t27040: F, t4324: F, t8404: F) -> (F,) {
    let t28031 = -0.11607361111111111111e-2 * t26976 + 0.38691203703703703703e-3 * t14224 - 0.25794135802469135802e-3 * t14226 - 0.11607361111111111111e-2 * t26990 - 0.386e0 * t21434 * t2332 + 0.10317654320987654321e-2 * t14250 - 0.30952962962962962963e-2 * t27006 + 0.12897067901234567901e-2 * t27008 + 0.34822083333333333332e-2 * t27013 + t22035 - t22036 + t22037 - t22038 - 0.193e0 * t4324 * t8404 + 0.15476481481481481481e-2 * t27028 - 0.46429444444444444444e-2 * t27030 + 0.38691203703703703703e-2 * t27035 + 0.10317654320987654321e-2 * t27037 - 0.41270617283950617283e-2 * t27040;
    (t28031,)
}
