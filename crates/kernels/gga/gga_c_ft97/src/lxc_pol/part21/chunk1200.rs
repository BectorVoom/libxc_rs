//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1200/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1200<F: Float>(t1882: F, t29879: F, t29824: F, t30005: F, t29872: F, t8392: F, t103029: F, t103077: F, t11552: F, t116102: F, t116312: F, t116446: F, t11810: F, t16047: F, t16093: F, t16229: F, t16242: F, t1901: F, t23327: F, t23339: F, t29961: F, t379: F, t446: F, t47768: F, t5631: F, t59510: F, t83: F, t8557: F, t92055: F) -> (F,) {
    let t117436 = t1882 * t29879;
    let t117438 = t1882 * t29824;
    let t117441 = t1882 * t30005;
    let t117443 = t8392 * t29872;
    let t117476 = -2.0 / 9.0 * t117436 - t117438 / 9.0 - 8.0 / 27.0 * t103029 + 2.0 / 9.0 * t117441 - 2.0 / 27.0 * t117443 + 4.0 / 3.0 * t446 * t83 * t116102 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t16047 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t16093 + t1901 * t59510 * t5631 / 9.0 - 2.0 / 9.0 * t1901 * t92055 * t16229 + t1901 * t23327 * t16242 / 9.0 - 2.0 / 9.0 * t1901 * t8557 * t29961 * t379 - 8.0 / 27.0 * t103077 - 4.0 / 9.0 * t1901 * t11552 * t116446 + 10.0 / 81.0 * t1901 * t47768 * t116312;
    (t117476,)
}
