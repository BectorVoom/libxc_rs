//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1027/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1027<F: Float>(t2320: F, t41482: F, t701: F, t2446: F, t41468: F, t420: F, t8608: F, t9591: F) -> (F, F, F) {
    let t41484 = t701 * t2320 * t41482;
    let t41488 = t701 * t420 * t2446 * t41468;
    let t41490 = t9591 * t8608;
    (t41484, t41488, t41490)
}
