//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 634/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk634<F: Float>(t1901: F, t28: F, t446: F, t89: F, t9313: F, t9318: F, t9321: F, t9324: F, t9329: F, t9333: F, t9337: F, t9340: F, t9342: F, t9345: F, t9350: F, t9355: F, t9359: F, t9363: F, t9396: F) -> (F,) {
    let t9400 = -2.0 * t446 * t9313 + t446 * t9318 + 4.0 / 9.0 * t9321 - t446 * t9324 / 9.0 - 10.0 / 81.0 * t446 * t9329 - t446 * t9333 / 3.0 - 2.0 / 9.0 * t446 * t9337 + 2.0 / 3.0 * t9340 + 2.0 / 3.0 * t9342 - 2.0 / 3.0 * t1901 * t9345 + t1901 * t9350 / 3.0 + t1901 * t9355 / 3.0 + 2.0 / 3.0 * t1901 * t9359 - 2.0 / 9.0 * t1901 * t9363 + t89 * t28 * t9396 / 3.0;
    (t9400,)
}
