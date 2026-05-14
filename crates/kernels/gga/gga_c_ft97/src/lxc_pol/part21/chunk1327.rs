//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1327/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1327<F: Float>(t30413: F, t8392: F, t107156: F, t107168: F, t107170: F, t107177: F, t11593: F, t119592: F, t12714: F, t13140: F, t16169: F, t16971: F, t17118: F, t17123: F, t1901: F, t2185: F, t23455: F, t27068: F, t27220: F, t27252: F, t3450: F, t4458: F, t446: F, t51151: F, t569: F, t574: F, t5935: F, t5975: F, t62981: F, t63304: F, t6725: F, t95954: F) -> (F,) {
    let t121355 = t8392 * t30413;
    let t121379 = 8.0 / 27.0 * t11593 * t12714 * t27220 * t16169 + 2.0 / 3.0 * t1901 * t51151 * t119592 - 4.0 / 9.0 * t1901 * t63304 * t27252 - 4.0 / 9.0 * t1901 * t62981 * t27068 + 2.0 / 27.0 * t121355 - 2.0 / 3.0 * t1901 * t13140 * t23455 * t16971 + t107156 - t107168 - t107170 + 4.0 / 3.0 * t446 * t2185 * t6725 * t3450 + 4.0 / 81.0 * t95954 + 2.0 / 3.0 * t446 * t574 * t5935 * t17118 + 2.0 / 3.0 * t446 * t574 * t5935 * t17123 + 2.0 / 9.0 * t446 * t569 * t5975 * t4458 + 16.0 / 27.0 * t107177;
    (t121379,)
}
