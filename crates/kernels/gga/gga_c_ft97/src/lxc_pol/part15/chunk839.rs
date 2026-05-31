//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 839/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk839<F: Float>(t22301: F, t22329: F, t845: F, t91: F, t1234: F, t5337: F, t10631: F, t4191: F, t5362: F, t10797: F, t14895: F, t19246: F, t19249: F, t19278: F, t19298: F, t19301: F, t19304: F, t21981: F, t22164: F) -> (F, F, F, F, F) {
    let t22330 = t22301 + t22329;
    let t22332 = t91 * t845 * t22330;
    let t22334 = t5337 * t1234;
    let t22336 = t91 * t10631 * t22334;
    let t22339 = t91 * t4191 * t5362;
    let t22345 = -F::cast_from(6.0_f64) * t21981 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14895 + t19246 - F::cast_from(2.0_f64) * t19249 - t22164 + t22332 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t22336 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t22339 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19278 - t10797 + t19298 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19301 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19304;
    (t22330, t22332, t22336, t22339, t22345)
}
