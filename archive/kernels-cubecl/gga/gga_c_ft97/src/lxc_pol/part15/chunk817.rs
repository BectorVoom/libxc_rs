//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 817/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk817<F: Float>(t21970: F, t446: F, t1091: F, t5299: F, t2665: F, t1212: F, t5225: F) -> (F, F, F, F, F) {
    let t21971 = t446 * t21970;
    let t21973 = t1091 * t5299;
    let t21974 = t2665 * t21973;
    let t21975 = t446 * t21974;
    let t21978 = t5225 * t1212;
    (t21971, t21973, t21974, t21975, t21978)
}
