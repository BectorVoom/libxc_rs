//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1391/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1391<F: Float>(t6632: F, t8232: F, t1882: F, t26894: F, t23500: F, t50260: F, t27320: F, t27393: F, t376: F, t89: F, t1637: F, t6687: F, t26864: F, t8392: F, t104496: F, t105662: F, t13050: F, t13065: F, t144: F, t167: F, t1901: F, t2185: F, t2210: F, t23470: F, t23997: F, t26909: F, t27245: F, t40735: F, t446: F, t574: F, t616: F, t6695: F, t9099: F, t925: F, t96162: F) -> (F, F) {
    let t107650 = t8232 * t6632;
    let t107670 = 4.0 / 9.0 * t1882 * t26894;
    let t107675 = t50260 * t23500;
    let t107680 = 2.0 / 9.0 * t1882 * t27320;
    let t107683 = 2.0 / 9.0 * t89 * t376 * t27393;
    let t107685 = t89 * t1637 * t6687;
    let t107691 = 4.0 / 81.0 * t8392 * t26864;
    let t107692 = 2.0 / 3.0 * t446 * t2185 * t167 * t105662 + 8.0 / 27.0 * t107650 + 4.0 / 3.0 * t446 * t2185 * t616 * t26909 + 2.0 / 9.0 * t1901 * t9099 * t27245 + t1901 * t40735 * t6695 / 9.0 + t1901 * t2210 * t96162 * t925 / 9.0 - 2.0 / 3.0 * t446 * t144 * t104496 - t107670 - 2.0 / 3.0 * t446 * t574 * t23997 * t13065 - 2.0 * t446 * t144 * t107675 + t107680 - t107683 + 4.0 / 27.0 * t107685 - 2.0 / 9.0 * t1901 * t23470 * t13050 + t107691;
    (t107675, t107692)
}
