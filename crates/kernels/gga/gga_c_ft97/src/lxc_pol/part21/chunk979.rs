//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 979/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk979<F: Float>(t30383: F, t574: F, t605: F, t1901: F, t26826: F, t26914: F, t26916: F, t27004: F, t27025: F, t30311: F, t30314: F, t30317: F, t30321: F, t30325: F, t30359: F, t30364: F, t30369: F, t30373: F, t30376: F, t30380: F, t446: F) -> (F, F) {
    let t30385 = t574 * t605 * t30383;
    let t30388 = -4.0 / 9.0 * t26826 - 4.0 / 9.0 * t26914 - 2.0 / 27.0 * t26916 - t446 * t30311 / 3.0 - 2.0 / 3.0 * t446 * t30314 - t446 * t30317 / 3.0 - t446 * t30321 / 3.0 + 2.0 / 3.0 * t446 * t30325 - t446 * t30359 / 3.0 + 2.0 / 9.0 * t27004 - 2.0 / 9.0 * t1901 * t30364 - 2.0 / 27.0 * t27025 - 2.0 / 3.0 * t446 * t30369 - t446 * t30373 / 3.0 + 4.0 / 3.0 * t446 * t30376 + 2.0 / 3.0 * t446 * t30380 + t446 * t30385 / 3.0;
    (t30385, t30388)
}
