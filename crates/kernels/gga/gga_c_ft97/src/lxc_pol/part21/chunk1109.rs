//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1109/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1109<F: Float>(t23997: F, t582: F, t27242: F, t8392: F, t1882: F, t27299: F, t26952: F, t50249: F, t5942: F, t157: F, t5855: F, t7368: F, t26996: F, t46862: F, t27269: F, t26969: F) -> (F, F, F, F, F, F, F, F, F) {
    let t107284 = t582 * t23997;
    let t107294 = 4.0 / 27.0 * t8392 * t27242;
    let t107296 = 2.0 / 9.0 * t1882 * t27299;
    let t107303 = 4.0 / 9.0 * t1882 * t26952;
    let t107311 = t50249 * t5942;
    let t107316 = t7368 * t157 * t5855;
    let t107323 = t46862 * t26996;
    let t107336 = 4.0 / 9.0 * t1882 * t27269;
    let t107361 = 2.0 / 9.0 * t1882 * t26969;
    (t107284, t107294, t107296, t107303, t107311, t107316, t107323, t107336, t107361)
}
