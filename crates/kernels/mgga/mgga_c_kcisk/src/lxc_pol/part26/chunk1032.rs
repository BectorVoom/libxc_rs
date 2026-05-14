//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1032/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1032<F: Float>(t19020: F, t19022: F, t19028: F, t21377: F, t21388: F, t25363: F, t25368: F, t25372: F, t25376: F, t25381: F, t25385: F, t25389: F, t25394: F, t25399: F, t25401: F, t25970: F, t25974: F, t26506: F, t26510: F) -> (F,) {
    let t27469 = -0.17411041666666666666e-2 * t25363 - 0.23214722222222222222e-2 * t25368 + 0.12897067901234567901e-2 * t25372 - 0.51588271604938271603e-3 * t19020 - 0.41270617283950617283e-2 * t19022 + 0.23214722222222222221e-2 * t25376 + 0.77382407407407407407e-3 * t19028 + t21377 + 0.61905925925925925925e-2 * t25381 + 0.46429444444444444444e-2 * t25385 + 0.10317654320987654321e-2 * t25389 - 0.23214722222222222222e-2 * t25394 - 0.46429444444444444444e-2 * t25399 + 0.15476481481481481481e-2 * t25401 + 0.17411041666666666666e-2 * t25970 - 0.92858888888888888888e-2 * t25974 + t21388 + 0.23214722222222222222e-2 * t26506 - 0.23214722222222222222e-2 * t26510;
    (t27469,)
}
