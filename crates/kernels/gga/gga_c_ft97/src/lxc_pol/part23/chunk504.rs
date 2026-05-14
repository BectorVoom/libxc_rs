//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 504/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk504<F: Float>(t505: F, t6334: F, t2665: F, t446: F, t6222: F, t824: F, t193: F, t89: F, t6260: F, t799: F, t27: F, t6312: F, t6316: F, t6321: F, t6325: F, t6329: F, t6333: F) -> (F, F, F, F, F, F, F) {
    let t6335 = t6334 * t505;
    let t6336 = t2665 * t6335;
    let t6337 = t446 * t6336;
    let t6339 = t6222 * t824;
    let t6340 = t193 * t6339;
    let t6341 = t89 * t6340;
    let t6343 = t799 * t6260;
    let t6345 = t89 * t27 * t6343;
    let t6347 = t6312 / 12.0 + t6316 + t6321 / 18.0 + t6325 / 3.0 - t6329 / 6.0 + t6333 + t6337 / 9.0 + 2.0 / 3.0 * t6341 - t6345 / 3.0;
    (t6336, t6337, t6339, t6341, t6343, t6345, t6347)
}
