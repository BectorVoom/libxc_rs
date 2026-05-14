//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1388/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1388<F: Float>(t1882: F, t26957: F, t1384: F, t39652: F, t6645: F, t8232: F, t26965: F, t104584: F, t1060: F, t12605: F, t12645: F, t1391: F, t144: F, t1647: F, t1901: F, t2185: F, t2190: F, t23490: F, t26590: F, t27021: F, t27334: F, t446: F, t569: F, t574: F, t6725: F, t9099: F, t9432: F, t96220: F, t96222: F, t96224: F, t96227: F, t96229: F) -> (F,) {
    let t107533 = 2.0 / 9.0 * t1882 * t26957;
    let t107542 = t39652 * t1384;
    let t107547 = t8232 * t6645;
    let t107552 = 2.0 / 9.0 * t1882 * t26965;
    let t107562 = -2.0 * t446 * t9432 * t1060 * t23490 + 2.0 / 3.0 * t446 * t574 * t26590 * t2190 - t107533 - 8.0 / 27.0 * t96220 + 2.0 / 3.0 * t446 * t144 * t104584 + 4.0 / 3.0 * t446 * t2185 * t1391 * t12645 + 8.0 * t1901 * t27334 * t107542 * t12605 - 4.0 / 27.0 * t107547 - 8.0 / 27.0 * t96222 - 8.0 / 27.0 * t96224 + t107552 + 2.0 / 9.0 * t446 * t569 * t6725 * t1647 + 8.0 / 27.0 * t96227 + t96229 / 9.0 + 2.0 / 9.0 * t1901 * t9099 * t27021;
    (t107562,)
}
