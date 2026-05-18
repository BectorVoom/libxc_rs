//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1145/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1145<F: Float>(t43399: F, t43403: F, t43407: F, t43411: F, t43416: F, t43418: F, t43422: F, t43424: F, t43426: F, t43430: F, t43433: F, t43437: F, t43441: F, t43444: F, t43448: F) -> F {
    let t44096 = F::new(2.0) * t43399 + F::new(9.0) / F::new(4.0) * t43403 - t43407 - F::new(8.0) * t43411 + F::new(4.0) / F::new(3.0) * t43416 + F::new(8.0) / F::new(9.0) * t43418 + F::new(4.0) / F::new(3.0) * t43422 - F::new(4.0) / F::new(3.0) * t43424 - F::new(4.0) / F::new(3.0) * t43426 + F::new(4.0) / F::new(3.0) * t43430 + F::new(8.0) / F::new(3.0) * t43433 + F::new(8.0) * t43437 + F::new(2.0) * t43441 + F::new(112.0) / F::new(81.0) * t43444 - F::new(2.0) / F::new(3.0) * t43448;
    t44096
}
