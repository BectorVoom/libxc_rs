//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 874/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk874<F: Float>(t15157: F, t15158: F, t15198: F, t1857: F, t19311: F, t19313: F, t19315: F, t19319: F, t19322: F, t19544: F, t19550: F, t19555: F, t19559: F, t19563: F, t19566: F, t19569: F, t19573: F, t19578: F, t3638: F, t6738: F, t9529: F) -> (F,) {
    let t20269 = -0.92858888888888888885e-2 * t19311 - 0.13345e0 * t15198 * t1857 + 0.77382407407407407407e-3 * t19313 - 0.25794135802469135802e-3 * t19315 + 0.10317654320987654321e-2 * t19319 - 0.38691203703703703703e-3 * t19322 + 0.17411041666666666666e-2 * t19544 - t15157 + t15158 + 0.12897067901234567901e-2 * t19550 + 0.66725e-1 * t3638 * t6738 - 0.15476481481481481481e-2 * t19555 + 0.69644166666666666666e-2 * t19559 - 0.92858888888888888888e-2 * t19563 + 0.61905925925925925925e-2 * t19566 + 0.38691203703703703703e-3 * t9529 + 0.15476481481481481481e-2 * t19569 - 0.15476481481481481481e-2 * t19573 + 0.77382407407407407407e-3 * t19578;
    (t20269,)
}
