//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 839/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk839<F: Float>(t41809: F, t447: F, t6963: F, t6964: F, t2859: F, t3137: F, t4752: F, t10557: F, t9431: F, t2487: F, t41749: F, t6711: F) -> (F, F, F, F, F) {
    let t41810 = t41809 * t447;
    let t41813 = F::new(0.71500979903700853338e0) * t6963 * t6964 * t41810;
    let t41829 = F::new(0.7150097990370085334e0) * t2859 * t4752 * t3137;
    let t41831 = F::new(0.42900587942220512003e1) * t10557 * t9431;
    let t41834 = F::new(0.87421871174939309262e2) * t2487 * t6711 * t41749;
    (t41810, t41813, t41829, t41831, t41834)
}
