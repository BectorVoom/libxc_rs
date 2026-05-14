//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 580/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk580<F: Float>(t1851: F, t480: F, t1853: F, t83: F, t1827: F, t1882: F, t1901: F, t28: F, t446: F, t8383: F, t8388: F, t8393: F, t8396: F, t8399: F, t8402: F, t8406: F, t8409: F, t8413: F, t8421: F, t8426: F, t8430: F, t8462: F, t89: F) -> (F, F, F, F) {
    let t8466 = t480 * t1851;
    let t8467 = t8466 * t1853;
    let t8468 = t83 * t8467;
    let t8471 = t1882 * t1827;
    let t8473 = 2.0 / 3.0 * t446 * t8383 + t1901 * t8388 / 3.0 - 2.0 / 9.0 * t8393 - t446 * t8396 - t446 * t8399 - t446 * t8402 / 3.0 - 2.0 / 3.0 * t446 * t8406 - 2.0 / 3.0 * t8409 - 2.0 * t446 * t8413 - 2.0 * t446 * t8421 - 2.0 / 3.0 * t1901 * t8426 - t8430 / 3.0 + t89 * t28 * t8462 / 3.0 + 2.0 * t446 * t8468 + 2.0 / 3.0 * t8471;
    (t8466, t8467, t8468, t8473)
}
