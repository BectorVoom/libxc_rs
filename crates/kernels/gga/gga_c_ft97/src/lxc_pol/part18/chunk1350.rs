//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1350/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1350<F: Float>(t105343: F, t105347: F, t105351: F, t105355: F, t105359: F, t105362: F, t105366: F, t105370: F, t105374: F, t105378: F, t105381: F, t105411: F, t105416: F, t105386: F, t105390: F, t105396: F, t105400: F, t105404: F, t105409: F, t105414: F, t105421: F, t95051: F, t95054: F) -> (F, F) {
    let t105964 = 5.0 / 81.0 * t105343 - t105347 / 9.0 - 2.0 / 9.0 * t105351 + t105355 / 27.0 + t105359 / 3.0 - t105362 / 18.0 - 2.0 / 9.0 * t105366 - t105370 / 9.0 - 2.0 / 27.0 * t105374 + t105378 / 3.0 - t105381;
    let t105971 = t105411 / 27.0;
    let t105973 = t105416 / 9.0;
    let t105977 = t105386 / 18.0 - 2.0 / 9.0 * t105390 - t105396 / 9.0 + 4.0 * t105400 - t105404 / 9.0 + t105409 / 9.0 - t105971 - 11.0 / 27.0 * t105414 + t105973 - t95051 / 27.0 + t105421 / 4.0 + t95054 / 9.0;
    (t105964, t105977)
}
