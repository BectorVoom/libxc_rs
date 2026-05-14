//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1383/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1383<F: Float>(t23997: F, t582: F, t27242: F, t8392: F, t1882: F, t27299: F, t26952: F, t50249: F, t5942: F, t157: F, t5855: F, t7368: F, t26996: F, t46862: F, t107273: F, t12334: F, t12590: F, t12609: F, t12709: F, t12710: F, t12754: F, t13216: F, t13221: F, t1570: F, t1901: F, t2185: F, t27256: F, t3188: F, t3450: F, t40945: F, t446: F, t47659: F, t47666: F, t574: F, t5935: F, t5968: F, t5975: F, t95789: F, t95842: F, t96002: F) -> (F, F) {
    let t107284 = t582 * t23997;
    let t107294 = 4.0 / 27.0 * t8392 * t27242;
    let t107296 = 2.0 / 9.0 * t1882 * t27299;
    let t107303 = 4.0 / 9.0 * t1882 * t26952;
    let t107311 = t50249 * t5942;
    let t107316 = t7368 * t157 * t5855;
    let t107323 = t46862 * t26996;
    let t107325 = -2.0 / 9.0 * t1901 * t40945 * t27256 - 2.0 / 9.0 * t1901 * t95789 * t12609 - 4.0 / 9.0 * t1901 * t107284 * t13221 - 4.0 / 9.0 * t1901 * t12709 * t5968 * t1570 * t3188 + t107294 + t107296 - t96002 / 9.0 + t446 * t574 * t5935 * t12754 / 3.0 - t107303 + 4.0 / 3.0 * t446 * t2185 * t5975 * t3450 + 8.0 / 9.0 * t47659 * t107273 * t12710 - 4.0 / 27.0 * t47666 * t107311 * t12590 + 4.0 / 3.0 * t47659 * t107316 * t12334 + 4.0 / 9.0 * t47659 * t95842 * t13216 - 22.0 / 27.0 * t107323;
    (t107311, t107325)
}
