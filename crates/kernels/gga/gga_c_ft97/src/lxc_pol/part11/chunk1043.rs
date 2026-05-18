//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1043/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1043<F: Float>(t11401: F, t191: F, t26: F, t458: F, t9573: F, t9597: F, t2360: F, t322: F, t17: F, t41448: F, t41536: F, t92: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41743 = t11401 * t191;
    let t41744 = t26 * t41743;
    let t41745 = F::new(280.0) / F::new(81.0) * t41744;
    let t41746 = t458 * t9573;
    let t41748 = t458 * t9597;
    let t41751 = F::new(1.0) / t322 / t2360;
    let t41752 = t17 * t41751;
    let t41753 = t41536 * t41448;
    let t41755 = t92 * t41752 * t41753;
    (t41743, t41744, t41745, t41746, t41748, t41751, t41752, t41753, t41755)
}
