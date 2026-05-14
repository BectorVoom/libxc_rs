//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1302/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1302<F: Float>(t1882: F, t30373: F, t106300: F, t106384: F, t106413: F, t106415: F, t107284: F, t119297: F, t12968: F, t13153: F, t144: F, t17007: F, t17040: F, t17081: F, t17365: F, t17369: F, t17376: F, t1901: F, t2142: F, t23443: F, t26991: F, t27015: F, t27329: F, t30472: F, t30477: F, t379: F, t40911: F, t446: F, t574: F, t63180: F, t95789: F, t96239: F) -> (F,) {
    let t120469 = t1882 * t30373;
    let t120497 = t1901 * t23443 * t17007 / 9.0 + 2.0 / 9.0 * t1901 * t13153 * t26991 + 4.0 / 3.0 * t1901 * t12968 * t27015 * t17081 + t120469 / 9.0 + 2.0 / 9.0 * t1901 * t40911 * t30477 * t379 - 2.0 / 9.0 * t1901 * t95789 * t17365 - 4.0 / 9.0 * t1901 * t107284 * t17369 - 4.0 / 9.0 * t1901 * t106300 * t17376 + t106384 - 4.0 / 3.0 * t1901 * t63180 * t27329 + 4.0 / 3.0 * t446 * t144 * t119297 + 2.0 / 27.0 * t1901 * t96239 * t17040 + 2.0 / 3.0 * t446 * t574 * t2142 * t30472 + t106413 - t106415;
    (t120497,)
}
