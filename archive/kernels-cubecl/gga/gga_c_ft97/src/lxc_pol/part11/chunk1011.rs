//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1011/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1011<F: Float>(t1882: F, t9402: F, t157: F, t40424: F, t8392: F, t9100: F, t2144: F, t8232: F, t376: F, t89: F, t9396: F, t605: F, t9114: F) -> (F, F, F, F, F, F) {
    let t41246 = t1882 * t9402;
    let t41251 = t40424 * t157;
    let t41262 = t8392 * t9100;
    let t41264 = t8232 * t2144;
    let t41267 = t89 * t376 * t9396;
    let t41269 = t9114 * t605;
    (t41246, t41251, t41262, t41264, t41267, t41269)
}
