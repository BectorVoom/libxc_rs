//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 189/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk189<F: Float>(t457: F, t473: F, t91: F, t353: F, t366: F, t435: F) -> (F, F) {
    let t475 = t91 * t457 * t473;
    let t477 = t353 / F::new(9.0);
    let t480 = t475 / F::new(6.0) - t477 - t366 / F::new(9.0) - t435 / F::new(3.0);
    (t475, t480)
}
