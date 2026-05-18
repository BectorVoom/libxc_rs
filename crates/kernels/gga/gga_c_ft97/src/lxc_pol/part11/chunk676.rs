//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 676/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk676<F: Float>(t9379: F, t9393: F, t143: F, t160: F, t1901: F, t28: F, t446: F, t89: F, t9313: F, t9318: F, t9321: F, t9324: F, t9329: F, t9333: F, t9337: F, t9340: F, t9342: F, t9345: F, t9350: F, t9355: F, t9359: F, t9363: F) -> (F, F, F) {
    let t9394 = t9379 + t9393;
    let t9396 = t143 * t9394 * t160;
    let t9400 = -F::new(2.0) * t446 * t9313 + t446 * t9318 + F::new(4.0) / F::new(9.0) * t9321 - t446 * t9324 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t446 * t9329 - t446 * t9333 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t446 * t9337 + F::new(2.0) / F::new(3.0) * t9340 + F::new(2.0) / F::new(3.0) * t9342 - F::new(2.0) / F::new(3.0) * t1901 * t9345 + t1901 * t9350 / F::new(3.0) + t1901 * t9355 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1901 * t9359 - F::new(2.0) / F::new(9.0) * t1901 * t9363 + t89 * t28 * t9396 / F::new(3.0);
    (t9394, t9396, t9400)
}
