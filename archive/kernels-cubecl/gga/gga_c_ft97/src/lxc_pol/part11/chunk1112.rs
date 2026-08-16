//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1112/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1112<F: Float>(t10406: F, t1882: F, t2413: F, t2682: F, t10248: F, t446: F, t824: F, t9587: F, t2665: F, t41482: F, t835: F, t2857: F, t41464: F) -> (F, F, F, F, F, F, F) {
    let t43365 = t1882 * t10406;
    let t43367 = t2413 * t2682;
    let t43369 = t446 * t10248 * t43367;
    let t43371 = t9587 * t824;
    let t43373 = t446 * t2665 * t43371;
    let t43376 = t446 * t835 * t41482;
    let t43379 = t446 * t2857 * t41464;
    (t43365, t43367, t43369, t43371, t43373, t43376, t43379)
}
