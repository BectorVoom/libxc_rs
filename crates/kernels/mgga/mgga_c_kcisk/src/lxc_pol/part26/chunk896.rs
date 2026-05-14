//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 896/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk896<F: Float>(t19030: F, t19075: F, t19737: F, t19759: F, t19761: F, t19790: F, t19808: F, t19810: F, t1556: F, t6579: F, t19832: F, t19837: F, t19846: F, t19856: F, t3951: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21377 = 0.61905925925925925925e-2 * t19030;
    let t21388 = 0.10317654320987654321e-2 * t19075;
    let t21402 = 0.15476481481481481481e-2 * t19737;
    let t21408 = 0.15476481481481481481e-2 * t19759;
    let t21409 = 0.15476481481481481481e-2 * t19761;
    let t21420 = 0.23214722222222222222e-2 * t19790;
    let t21425 = 0.15476481481481481481e-2 * t19808;
    let t21426 = 0.15476481481481481481e-2 * t19810;
    let t21434 = t6579 * t1556;
    let t21438 = 0.23214722222222222222e-2 * t19832;
    let t21440 = 0.15476481481481481481e-2 * t19837;
    let t21446 = 0.23214722222222222222e-2 * t19846;
    let t21449 = 0.23214722222222222222e-2 * t19856;
    let t21487 = t964 * t3951;
    (t21377, t21388, t21402, t21408, t21409, t21420, t21425, t21426, t21434, t21438, t21440, t21446, t21449, t21487)
}
