//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 819/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk819<F: Float>(t21989: F, t666: F, t89: F, t21204: F, t835: F, t446: F, t14738: F, t5284: F, t21249: F, t290: F, t21253: F, t5272: F, t7853: F) -> (F, F, F, F, F, F, F) {
    let t21991 = t89 * t666 * t21989;
    let t21993 = t835 * t21204;
    let t21994 = t446 * t21993;
    let t21996 = t14738 * t5284;
    let t21999 = t290 * t21249;
    let t22000 = t21999 * t21253;
    let t22003 = t7853 * t5272;
    (t21991, t21993, t21994, t21996, t21999, t22000, t22003)
}
