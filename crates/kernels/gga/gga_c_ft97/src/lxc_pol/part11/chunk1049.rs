//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1049/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1049<F: Float>(t2354: F, t41833: F, t446: F, t2373: F, t2413: F, t9770: F, t2459: F, t2372: F, t27: F, t89: F, t375: F, t9709: F) -> (F, F, F, F, F, F) {
    let t41835 = t446 * t2354 * t41833;
    let t41837 = t2413 * t2373;
    let t41839 = t446 * t9770 * t41837;
    let t41841 = t2459 * t2459;
    let t41844 = t89 * t27 * t2372 * t41841;
    let t41846 = t89 * t375 * t9709;
    (t41835, t41837, t41839, t41841, t41844, t41846)
}
