//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1376/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1376<F: Float>(t50235: F, t5942: F, t26988: F, t8392: F, t1882: F, t26842: F, t1053: F, t1060: F, t12334: F, t12950: F, t13216: F, t1901: F, t2185: F, t23434: F, t23518: F, t23884: F, t27191: F, t27329: F, t27334: F, t3408: F, t3483: F, t3565: F, t3578: F, t446: F, t47659: F, t49414: F, t558: F, t574: F, t5842: F, t5935: F, t5968: F, t605: F, t9438: F, t95776: F, t95837: F) -> (F,) {
    let t106981 = t50235 * t5942;
    let t107012 = 2.0 / 27.0 * t8392 * t26988;
    let t107019 = 2.0 / 9.0 * t1882 * t26842;
    let t107020 = 2.0 / 3.0 * t446 * t574 * t605 * t27191 * t558 - 4.0 * t1901 * t27334 * t9438 * t5968 * t3483 - 4.0 / 3.0 * t1901 * t49414 * t27329 + 4.0 / 9.0 * t47659 * t106981 * t12334 + 4.0 / 9.0 * t47659 * t95837 * t13216 + 2.0 / 3.0 * t446 * t2185 * t1060 * t23518 + t446 * t574 * t605 * t23884 * t1053 / 3.0 + t446 * t574 * t5935 * t12950 / 3.0 - 4.0 / 9.0 * t95776 + 2.0 / 3.0 * t446 * t574 * t605 * t5968 * t3408 + t446 * t574 * t3578 * t23434 / 3.0 - t107012 + 2.0 / 3.0 * t446 * t574 * t605 * t5842 * t3565 - t107019;
    (t107020,)
}
