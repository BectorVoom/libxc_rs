//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1167/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1167<F: Float>(t43444: F, t43392: F, t43394: F, t43399: F, t43411: F, t43416: F, t43418: F, t43422: F, t43424: F, t43426: F, t43430: F, t43433: F, t43437: F, t43441: F) -> F {
    let t44750 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t43444;
    let t44751 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43392 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43394 + t43399 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43411 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43416 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43418 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43422 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43424 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43426 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43430 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43433 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43437 + t43441 / F::cast_from(3.0_f64) + t44750;
    t44751
}
