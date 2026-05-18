//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1118/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1118<F: Float>(t2670: F, t41468: F, t666: F, t89: F, t3704: F, t793: F, t2345: F, t2660: F, t43399: F, t43403: F, t43407: F, t43411: F, t43416: F, t43418: F, t43422: F, t43424: F, t43426: F, t43430: F, t43433: F, t43437: F) -> (F, F, F, F) {
    let t43441 = t89 * t666 * t2670 * t41468;
    let t43444 = t89 * t3704 * t793;
    let t43448 = t89 * t2345 * t2660 * t41468;
    let t43450 = F::new(2.0) / F::new(3.0) * t43399 + F::new(3.0) / F::new(4.0) * t43403 - t43407 / F::new(3.0) - F::new(8.0) / F::new(3.0) * t43411 + F::new(4.0) / F::new(9.0) * t43416 + F::new(8.0) / F::new(27.0) * t43418 + F::new(4.0) / F::new(9.0) * t43422 - F::new(4.0) / F::new(9.0) * t43424 - F::new(4.0) / F::new(9.0) * t43426 + F::new(4.0) / F::new(9.0) * t43430 + F::new(8.0) / F::new(9.0) * t43433 + F::new(8.0) / F::new(3.0) * t43437 + F::new(2.0) / F::new(3.0) * t43441 + F::new(112.0) / F::new(243.0) * t43444 - F::new(2.0) / F::new(9.0) * t43448;
    (t43441, t43444, t43448, t43450)
}
