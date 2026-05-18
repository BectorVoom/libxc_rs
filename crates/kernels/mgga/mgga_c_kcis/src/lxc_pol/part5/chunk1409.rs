//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1409/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1409<F: Float>(t18249: F, t22263: F, t22266: F, t22268: F, t22273: F, t22277: F, t22280: F, t22282: F, t22287: F, t22292: F, t6193: F, t6208: F) -> F {
    let t23249 = t18249 - F::new(0.15476481481481481481e-2) * t22263 + F::new(0.13345e0) * t6193 * t6208 - F::new(0.10317654320987654321e-1) * t22266 + F::new(0.15476481481481481481e-2) * t22268 - F::new(0.30952962962962962962e-2) * t22273 + F::new(0.23214722222222222221e-2) * t22277 + F::new(0.61905925925925925924e-2) * t22280 - F::new(0.23214722222222222222e-2) * t22282 - F::new(0.23214722222222222222e-2) * t22287 + F::new(0.46429444444444444444e-2) * t22292;
    t23249
}
