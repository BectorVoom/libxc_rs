//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 565/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk565<F: Float>(t255: F, t9802: F, t2347: F, t761: F, t251: F, t631: F, t675: F, t7242: F, t898: F, t2371: F, t665: F) -> (F, F, F, F) {
    let t9803 = t9802 * t255;
    let t9808 = t761 * t2347;
    let t9890 = 1.0 / t251 / t631 / t898 / t675 / t7242 / 4.0;
    let t9895 = t665 * t2371;
    (t9803, t9808, t9890, t9895)
}
