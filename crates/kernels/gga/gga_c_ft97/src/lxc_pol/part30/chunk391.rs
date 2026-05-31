//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 391/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk391<F: Float>(t505: F, t6334: F, t2665: F, t446: F, t6222: F, t824: F, t193: F, t89: F, t6260: F, t799: F, t27: F, t6312: F, t6316: F, t6321: F, t6325: F, t6329: F, t6333: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6335 = t6334 * t505;
    let t6336 = t2665 * t6335;
    let t6337 = t446 * t6336;
    let t6339 = t6222 * t824;
    let t6340 = t193 * t6339;
    let t6341 = t89 * t6340;
    let t6343 = t799 * t6260;
    let t6345 = t89 * t27 * t6343;
    let t6347 = t6312 / F::cast_from(12.0_f64) + t6316 + t6321 / F::cast_from(18.0_f64) + t6325 / F::cast_from(3.0_f64) - t6329 / F::cast_from(6.0_f64) + t6333 + t6337 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6341 - t6345 / F::cast_from(3.0_f64);
    (t6335, t6336, t6337, t6339, t6340, t6341, t6343, t6345, t6347)
}
