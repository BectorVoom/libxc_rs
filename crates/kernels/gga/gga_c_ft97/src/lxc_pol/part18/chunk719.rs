//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 719/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk719<F: Float>(t12338: F, t1969: F, t446: F, t9065: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12307: F, t12309: F, t12311: F, t12315: F, t12319: F, t12322: F, t12325: F, t12328: F, t12332: F, t12336: F, t8805: F, t9068: F) -> (F, F) {
    let t12339 = t1969 * t12338;
    let t12340 = t446 * t12339;
    let t12343 = 4.0 / 27.0 * t9065;
    let t12345 = t12285 / 18.0 + t12290 / 27.0 - 5.0 / 81.0 * t12293 - 4.0 / 27.0 * t12296 + t12300 / 18.0 + 2.0 / 9.0 * t12304 - t12307 - t12309 + t12311 - t12315 / 9.0 - t12319 / 9.0 - t12322 / 3.0 + 2.0 / 9.0 * t12325 - t12328 + t12332 / 9.0 - 2.0 / 9.0 * t12336 - 2.0 / 9.0 * t12340 - t8805 / 9.0 - t12343 + t9068 / 18.0;
    (t12340, t12345)
}
