//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 949/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk949<F: Float>(t13682: F, t13683: F, t192: F, t2506: F, t3917: F, t41482: F, t41794: F, t41837: F, t41841: F, t41849: F, t42059: F, t42192: F, t42194: F, t42206: F, t42207: F, t42212: F, t42214: F, t42216: F, t42218: F, t42227: F, t462: F, t743: F, t92: F, t9896: F) -> (F,) {
    let t42229 = 8.0 / 3.0 * t42192 + 16.0 / 9.0 * t42194 + 8.0 / 3.0 * t13682 * t13683 * t42059 - 12.0 * t462 * t3917 * t41482 - 4.0 * t462 * t9896 * t41837 + t42206 + 112.0 / 27.0 * t42207 - t92 * t192 * t743 * t41794 + 16.0 / 3.0 * t42212 - 8.0 / 3.0 * t42214 + 8.0 * t42216 + 24.0 * t92 * t192 * t42218 * t41849 + 6.0 * t92 * t192 * t2506 * t41841 + 4.0 / 3.0 * t42227;
    (t42229,)
}
