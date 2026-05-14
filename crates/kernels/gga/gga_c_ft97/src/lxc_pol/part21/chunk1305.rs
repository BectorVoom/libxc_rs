//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1305/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1305<F: Float>(t1882: F, t30317: F, t1060: F, t106561: F, t106600: F, t106894: F, t16666: F, t17496: F, t1901: F, t2142: F, t26768: F, t26836: F, t27073: F, t30494: F, t3408: F, t3565: F, t3578: F, t446: F, t4714: F, t49614: F, t574: F, t5975: F, t605: F, t62985: F, t6615: F, t6695: F, t6718: F, t6725: F, t95632: F, t95789: F) -> (F,) {
    let t120602 = t1882 * t30317;
    let t120647 = t120602 / 9.0 - 2.0 / 3.0 * t446 * t574 * t1060 * t26768 - 4.0 / 9.0 * t1901 * t106894 * t16666 - 2.0 / 9.0 * t1901 * t95789 * t17496 + t106561 + 2.0 / 3.0 * t446 * t574 * t605 * t6718 * t3408 + 2.0 / 3.0 * t446 * t574 * t3578 * t26836 + 8.0 / 27.0 * t95632 - 2.0 / 3.0 * t446 * t574 * t6725 * t3408 + 2.0 / 9.0 * t1901 * t49614 * t6695 - t446 * t574 * t5975 * t4714 / 3.0 + 4.0 / 27.0 * t1901 * t62985 * t27073 - t106600 + 2.0 / 3.0 * t446 * t574 * t605 * t6615 * t3565 + t446 * t574 * t2142 * t30494 / 3.0;
    (t120647,)
}
