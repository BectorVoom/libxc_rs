//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1029/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1029(t18249: f64, t22263: f64, t22266: f64, t22268: f64, t22273: f64, t22277: f64, t22280: f64, t22282: f64, t22287: f64, t22292: f64, t6193: f64, t6208: f64) -> f64 {
    let t23249 = t18249 - 0.15476481481481481481e-2_f64 * t22263 + 0.13345e0_f64 * t6193 * t6208 - 0.10317654320987654321e-1_f64 * t22266 + 0.15476481481481481481e-2_f64 * t22268 - 0.30952962962962962962e-2_f64 * t22273 + 0.23214722222222222221e-2_f64 * t22277 + 0.61905925925925925924e-2_f64 * t22280 - 0.23214722222222222222e-2_f64 * t22282 - 0.23214722222222222222e-2_f64 * t22287 + 0.46429444444444444444e-2_f64 * t22292;
    t23249
}
