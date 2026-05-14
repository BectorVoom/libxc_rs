//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1219/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1219<F: Float>(t1882: F, t29893: F, t29869: F, t102753: F, t103823: F, t103881: F, t110: F, t116082: F, t116807: F, t1307: F, t16480: F, t16485: F, t16550: F, t1825: F, t1901: F, t1909: F, t26371: F, t26374: F, t29569: F, t29956: F, t3114: F, t3189: F, t446: F, t452: F, t47659: F, t488: F, t499: F, t5718: F, t59506: F, t83: F, t91739: F, t925: F, t971: F) -> (F,) {
    let t118262 = t1882 * t29893;
    let t118301 = t1882 * t29869;
    let t118303 = 4.0 / 9.0 * t47659 * t91739 * t16485 - t446 * t452 * t16550 * t1307 / 3.0 + 2.0 / 9.0 * t118262 - t446 * t83 * t116807 / 3.0 + 2.0 / 9.0 * t1901 * t103823 * t3114 + 4.0 / 9.0 * t1901 * t103823 * t3189 - t446 * t452 * t499 * t29569 / 3.0 - t446 * t452 * t110 * t116082 / 3.0 - 4.0 * t1901 * t26371 * t971 * t26374 + t1901 * t59506 * t5718 / 9.0 - t103881 + t446 * t452 * t488 * t1307 * t16480 / 3.0 + t446 * t452 * t1825 * t29956 / 3.0 + 2.0 / 9.0 * t1901 * t1909 * t102753 * t925 - 2.0 / 9.0 * t118301;
    (t118303,)
}
