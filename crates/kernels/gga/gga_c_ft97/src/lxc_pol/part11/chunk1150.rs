//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1150/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1150<F: Float>(t10681: F, t1882: F, t10467: F, t8392: F, t10482: F, t10478: F, t863: F, t10548: F, t10769: F, t10505: F, t2360: F, t2842: F) -> (F, F, F, F, F, F, F, F) {
    let t44160 = t1882 * t10681;
    let t44174 = t8392 * t10467;
    let t44176 = t8392 * t10482;
    let t44178 = t10478 * t863;
    let t44190 = t1882 * t10548;
    let t44195 = t1882 * t10769;
    let t44202 = t8392 * t10505;
    let t44204 = t2842 * t2360;
    (t44160, t44174, t44176, t44178, t44190, t44195, t44202, t44204)
}
